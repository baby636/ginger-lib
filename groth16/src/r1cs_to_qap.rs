use algebra_core::{One, PairingEngine, Zero};
use ff_fft::{cfg_iter, cfg_iter_mut, EvaluationDomain};

use crate::{generator::KeypairAssembly, prover::ProvingAssignment, Vec};
use core::ops::AddAssign;
use r1cs_core::{Index, SynthesisError};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub(crate) struct R1CStoQAP;

impl R1CStoQAP {
    #[inline]
    pub(crate) fn instance_map_with_evaluation<E: PairingEngine>(
        assembly: &KeypairAssembly<E>,
        t: &E::Fr,
    ) -> Result<(Vec<E::Fr>, Vec<E::Fr>, Vec<E::Fr>, E::Fr, usize, usize), SynthesisError> {
        let domain_size = assembly.num_constraints + (assembly.num_inputs - 1) + 1;
        let domain = EvaluationDomain::<E::Fr>::new(domain_size)
            .ok_or(SynthesisError::PolynomialDegreeTooLarge)?;
        let domain_size = domain.size();

        let zt = domain.evaluate_vanishing_polynomial(*t);

        // Evaluate all Lagrange polynomials
        let coefficients_time = start_timer!(|| "Evaluate Lagrange coefficients");
        let u = domain.evaluate_all_lagrange_coefficients(*t);
        end_timer!(coefficients_time);

        let qap_num_variables = (assembly.num_inputs - 1) + assembly.num_aux;

        let mut a = vec![E::Fr::zero(); qap_num_variables + 1];
        let mut b = vec![E::Fr::zero(); qap_num_variables + 1];
        let mut c = vec![E::Fr::zero(); qap_num_variables + 1];

        for i in 0..assembly.num_inputs {
            a[i] = u[assembly.num_constraints + i];
        }

        for i in 0..assembly.num_constraints {
            for &(ref coeff, index) in assembly.at[i].iter() {
                let index = match index {
                    Index::Input(i) => i,
                    Index::Aux(i) => assembly.num_inputs + i,
                };

                a[index] += &(u[i] * coeff);
            }
            for &(ref coeff, index) in assembly.bt[i].iter() {
                let index = match index {
                    Index::Input(i) => i,
                    Index::Aux(i) => assembly.num_inputs + i,
                };

                b[index] += &(u[i] * coeff);
            }
            for &(ref coeff, index) in assembly.ct[i].iter() {
                let index = match index {
                    Index::Input(i) => i,
                    Index::Aux(i) => assembly.num_inputs + i,
                };

                c[index] += &(u[i] * coeff);
            }
        }

        Ok((a, b, c, zt, qap_num_variables, domain_size))
    }

    #[inline]
    pub(crate) fn witness_map<E: PairingEngine>(
        prover: &ProvingAssignment<E>,
    ) -> Result<(Vec<E::Fr>, Vec<E::Fr>, usize), SynthesisError> {
        #[inline]
        fn evaluate_constraint<E: PairingEngine>(
            terms: &[(E::Fr, Index)],
            assignment: &[E::Fr],
            num_input: usize,
        ) -> E::Fr {
            let mut acc = E::Fr::zero();
            for &(coeff, index) in terms {
                let val = match index {
                    Index::Input(i) => assignment[i],
                    Index::Aux(i) => assignment[num_input + i],
                };
                acc += &(val * &coeff);
            }
            acc
        }

        let zero = E::Fr::zero();
        let one = E::Fr::one();

        let mut full_input_assignment = prover.input_assignment.clone();
        full_input_assignment.extend(prover.aux_assignment.clone());

        let domain =
            EvaluationDomain::<E::Fr>::new(prover.num_constraints + (prover.num_inputs - 1) + 1)
                .ok_or(SynthesisError::PolynomialDegreeTooLarge)?;
        let domain_size = domain.size();

        let mut a = vec![zero; domain_size];
        let mut b = vec![zero; domain_size];

        cfg_iter_mut!(a[..prover.num_constraints])
            .zip(cfg_iter_mut!(b[..prover.num_constraints]))
            .zip(cfg_iter!(&prover.at))
            .zip(cfg_iter!(&prover.bt))
            .for_each(|(((a, b), at_i), bt_i)| {
                *a = evaluate_constraint::<E>(&at_i, &full_input_assignment, prover.num_inputs);
                *b = evaluate_constraint::<E>(&bt_i, &full_input_assignment, prover.num_inputs);
            });

        for i in 0..prover.num_inputs {
            a[prover.num_constraints + i] = if i > 0 { full_input_assignment[i] } else { one };
        }

        domain.ifft_in_place(&mut a);
        domain.ifft_in_place(&mut b);

        domain.coset_fft_in_place(&mut a);
        domain.coset_fft_in_place(&mut b);

        let mut ab = domain.mul_polynomials_in_evaluation_domain(&a, &b);
        drop(a);
        drop(b);

        let mut c = vec![zero; domain_size];
        cfg_iter_mut!(c[..prover.num_constraints])
            .enumerate()
            .for_each(|(i, c)| {
                *c = evaluate_constraint::<E>(
                    &prover.ct[i],
                    &full_input_assignment,
                    prover.num_inputs,
                );
            });

        domain.ifft_in_place(&mut c);
        domain.coset_fft_in_place(&mut c);

        cfg_iter_mut!(ab)
            .zip(c)
            .for_each(|(ab_i, c_i)| *ab_i -= &c_i);

        domain.divide_by_vanishing_poly_on_coset_in_place(&mut ab);
        domain.coset_ifft_in_place(&mut ab);

        let mut h: Vec<E::Fr> = vec![zero; domain_size - 1];
        cfg_iter_mut!(h)
            .enumerate()
            .for_each(|(i, e)| e.add_assign(&ab[i]));

        Ok((full_input_assignment, h, domain_size))
    }
}