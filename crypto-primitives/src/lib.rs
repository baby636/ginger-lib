#[macro_use]
extern crate bench_utils;

#[macro_use]
extern crate derivative;

pub mod commitment;
pub mod crh;
pub mod merkle_tree;
pub mod nizk;
pub mod prf;
pub mod signature;
pub mod vrf;

pub use self::{
    commitment::CommitmentScheme,
    crh::FixedLengthCRH,
    merkle_tree::{MerkleHashTree, MerkleTreePath},
    nizk::NIZK,
    prf::PRF,
    signature::SignatureScheme,
};

#[cfg(feature = "r1cs")]
pub use self::{
    commitment::CommitmentGadget, crh::FixedLengthCRHGadget,
    merkle_tree::constraints::MerkleTreePathGadget, nizk::NIZKVerifierGadget, prf::PRFGadget,
    signature::SigRandomizePkGadget,
};

pub type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum CryptoError {
    IncorrectInputLength(usize),
    NotPrimeOrder,
    FailedVerification,
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CryptoError::IncorrectInputLength(len) => format!("input length is wrong: {}", len),
            CryptoError::NotPrimeOrder => "element is not prime order".to_owned(),
            CryptoError::FailedVerification => "verification failed".to_owned(),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for CryptoError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

pub fn leading_zeros(bits: Vec<bool>) -> u32 {
    let mut ctr = 0;
    for b in bits.iter() {
        if !b {
            ctr += 1;
        } else {
            break;
        }
    }
    ctr
}