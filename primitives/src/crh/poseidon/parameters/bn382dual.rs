use crate::crh::{
    PoseidonParameters,
    FieldBasedHashParameters, PoseidonHash, batched_crh::PoseidonBatchHash,
    PoseidonQuinticSBox,
};
use algebra::fields::bn_382::Fq as Fr;

use algebra::biginteger::BigInteger384 as BigInteger;
use algebra::field_new;

#[derive(Clone)]
pub struct BN382FqPoseidonParameters;

impl FieldBasedHashParameters for BN382FqPoseidonParameters {
    type Fr = Fr;
    const R:usize = 2;  // The rate of the hash function
}

/// x^5-POSEIDON-128 parameters for scalar field (= Fr) of the BN382 dual curve.
/// 
/// The number of rounds are computed by ./evidence/calc_round_numbers.py, round constants and matrix 
/// are generated using the script ./evidence/generate_parameters_grain.
impl PoseidonParameters for BN382FqPoseidonParameters {

    const T:usize = 3;  // Size of the internal state (in field elements)
    const R_F:i32 = 4;  // Half number of full rounds (the R_f in the paper) 
    const R_P:i32 = 56; // Number of partial rounds

    // The zero element of the field
    const ZERO: Fr = field_new!(Fr, BigInteger([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]));
    // The constant 3 to add to the position corresponding to the capacity
    // (we don't use the domain separator for now)
    const C2:Fr = Fr::new(
        BigInteger([
            0xffffffffffffffeb,
            0xffffffe07f020607,
            0x7e04041040c0e07f,
            0x30a99058ab4842e0,
            0xb46b0b54830f49c9,
            0xba0bdc30c3ce150,
        ])
    );
    
    // State vector after permutation of zero state vector
    const AFTER_ZERO_PERM: &'static [Fr] = &[
        Fr::new(BigInteger([0x27dcc9c1f001c02d,0x7fc9de4b5ab915ed,0x7c6832557c4a410d,0x320b95a8fa27bf32,0xe5c89c9c09bd67e5,0x65748e22de4f8c5,])),
        Fr::new(BigInteger([0x7cdb27778c5d6796,0xad588ee542be3389,0x68e926bfdd6398ec,0xe432240624573240,0x2766c91ade70f83f,0x170646120652b37c,])),
        Fr::new(BigInteger([0xcada65af3ba4e9c4,0x7e4561e9933627cd,0x8cb8757ddb2e0730,0x610ecc5beda633e0,0x984de49537e8c3ec,0x1349deb07a8f6f52,]))
    ];

    // Array of round constants
    const ROUND_CST: &'static[Fr]  = &[
        // Constants in Montgomery representation.
        field_new!(Fr,BigInteger([0x7d3e06817fe2fa1e,0xe4c855556b4aacda,0xd3c7466dfe3ef0ad,0xdc8dfad17c55598d,0xfedeaecb451cc31c,0x1bae49fccd9255b6,])),
        field_new!(Fr,BigInteger([0x754b46e688e41941,0x5218c793c3fcd5f,0xba2d939611dd08a0,0xf2c8cd45b84d1652,0xfec52f665bbf0be7,0x1d8fb23e0ed07701,])),
        field_new!(Fr,BigInteger([0x179cb43091dd32ca,0x7018ca5f70f11350,0x7aa891c65140ab9d,0x7b58774f3f1be5a7,0x58d49c97590ce49c,0x22d837d6cf90598d,])),
        field_new!(Fr,BigInteger([0x6e7264f540297200,0x7e22d05a2ac6eead,0x750bb0bb0d9beca2,0xce0d22d4f9b03517,0x2d3abd81cc62a5d5,0x1e364e55e0f8e6f8,])),
        field_new!(Fr,BigInteger([0x286abfb064d58d5a,0x694641c59e8226c0,0x32e216e3da299c02,0xde59f2f1fb4e0e13,0x46582fcca06d8bc5,0xd701682260dc1ec,])),
        field_new!(Fr,BigInteger([0x3124c62cde9d20e2,0x9fac0ad8132a843,0x16a273f6243d658b,0x10bbd72c62bd55c7,0x28009a3b766ce48e,0xb2e1ad3015607f3,])),
        field_new!(Fr,BigInteger([0xcd85dea0420e7da4,0x7655466abe3e791e,0x89ba0ed5c5df0b7a,0x696eb193eedd7a53,0x21ef87ad83bc2098,0x1436fa2898d85c31,])),
        field_new!(Fr,BigInteger([0x2eb1bef93fa95c1b,0x6744676e7c75d573,0xfe40bfb6449c47ed,0xfb6f06274deb6d2f,0xa121c9375338d467,0x18c0c15fee7893a3,])),
        field_new!(Fr,BigInteger([0x9e87fc5f2c70948a,0xf15a84b5d6674772,0xeeb5b16e7841c954,0x195365c9174167ce,0x44cc3beded5210e8,0x9b30e2f37cf4d6d,])),
        field_new!(Fr,BigInteger([0xed5ac23cb2d7b2e7,0xb59479b2299532fd,0xd7346e61af7d6075,0x5379a9f9af0c76be,0xbc5e3f64e23b4510,0x1aa9539db9e2ca68,])),
        field_new!(Fr,BigInteger([0x7cca5d75471e18a5,0xcf738bac19bfd23c,0x8c7dae93f38d07d8,0x9928bea7b544d67,0x7bcbddc8941fbafa,0x107a0460dc3257d7,])),
        field_new!(Fr,BigInteger([0x509d7d069c8144b4,0x40d6f4f4838412b4,0x5f808597a65e824f,0xf85fc0b1ae528ba6,0x35417bc445096105,0x44c8ecab50e4106,])),
        field_new!(Fr,BigInteger([0xda7bcdb9334589ce,0x279866595323b253,0xc031ba9e9316bb4c,0x4d45d0a51e50ee99,0xd053f804a1ee09fa,0x1bd56aee6613e74e,])),
        field_new!(Fr,BigInteger([0xc19bd1f4d113085d,0xa029a66df291c4,0xdb6338960b5b1cc5,0xe91b2ccb73bb8a6f,0xe0c9fe626bd1f126,0x2395daf5ba537f1d,])),
        field_new!(Fr,BigInteger([0xe4de707dc5215efc,0xa7fea7ecec777a24,0x4fccf05790993d31,0xc61a8ea4220b5f3f,0x27d4a4665ac75ba9,0x6e3246f4a3b382e,])),
        field_new!(Fr,BigInteger([0x9d67087a4ea62ebf,0x59699096f1144725,0x2af84cf3d0380a9e,0xd87ee2a6d17347d8,0x4a81eb07be1b056c,0x75a5a6801316477,])),
        field_new!(Fr,BigInteger([0x9a6fd8809f12d30b,0x3ca61d2f47a53089,0x6c4d38eea4287956,0x154fa56675395c0c,0x8f865519001514f6,0x313964af599095e,])),
        field_new!(Fr,BigInteger([0x4ff63f6fc5645fd1,0x7b82f0c9b003a384,0x405ce50b7a794585,0x29e412a76ea8e5bd,0x692bacb4a43e915,0x78c400e81eaea2a,])),
        field_new!(Fr,BigInteger([0x2389a14f0e3f6b65,0x8598cc77c8681c81,0x3440e4d2dbe05338,0xaae1f848c7032be5,0x6e4f9f5529c13580,0x1de775da7d81ff42,])),
        field_new!(Fr,BigInteger([0x84fe9561a4fe2594,0x45c754538473d54e,0x2081e726351ab13e,0x8cb323441756a713,0xcb4b80882f845807,0x1ee0fcb705dc2430,])),
        field_new!(Fr,BigInteger([0x9c0aaa37d9119bc1,0x575c078a99cbd829,0x560f505a0478cedb,0x603f002e733c554f,0xaf0e6f0f83da8ce5,0xf5194f8e715f5d4,])),
        field_new!(Fr,BigInteger([0xf3561b7c1cf04,0xe56f7e055b0ec90b,0x9fb224e52785822,0x4c1790f1a8f9110b,0x35c79e9c35302307,0x18664851ea84735a,])),
        field_new!(Fr,BigInteger([0x50bdc2348c15b026,0xc77491daac514fc4,0x8955bc6b09ac737d,0xad2ea27060414f4d,0x7c997db5fac52dd0,0x1f2a5df9da5f6e32,])),
        field_new!(Fr,BigInteger([0xc681eed0e78148a7,0x57621f4c24529a1d,0x5876dd8e8ea07bd3,0xd28bb407c841cbf6,0x8b359037c71d366d,0xc328179635cd9f8,])),
        field_new!(Fr,BigInteger([0xb1df1f49c37ca695,0x1b56a5b8a3ba95f1,0x9a1808171c05e8e1,0x596925481ab62566,0xa9894f79cde80b77,0x1818e624cc575377,])),
        field_new!(Fr,BigInteger([0x43548437e0c1fc50,0x59a017d1d250161f,0xa2321e1ad533ce71,0x3430291f3dfd7b49,0x40e675e0cdd03d1d,0x1fb60b75dee10176,])),
        field_new!(Fr,BigInteger([0x9650122a9d5e917a,0xa8c5eb643e9680dc,0xebd8e0cf4b27e181,0x81878f28988986f5,0x84ecb59806b665fe,0x117feaed33fcaf64,])),
        field_new!(Fr,BigInteger([0x762852a42e0e383b,0x2b9d56b451dfd3a1,0xed90a6dd9cfa1ca0,0xc5e6550af40ad5f6,0x6670b6c3cc0f13ba,0x5e52fd37e326076,])),
        field_new!(Fr,BigInteger([0xe1e4e2b36aee2c68,0x72612c101843cef7,0x76566b953d138574,0x6286ec06a22d97c4,0xfb0718535ee4c307,0x9e580b297e1295b,])),
        field_new!(Fr,BigInteger([0x65f93a2ead7fa104,0x1139a51aa674b95c,0xab9fa9ccab64de12,0x4969cab7168e67e1,0x2c66ed95aa8833e9,0x22277dba6061a1d8,])),
        field_new!(Fr,BigInteger([0x12cb752b436c79c3,0xb5575ed259af50,0x7dccf11c775c50d2,0x256f41f44c42588,0xdddd7ae2731d6bd,0x1a7da4605f2bcf7f,])),
        field_new!(Fr,BigInteger([0xf17d407d066ce17e,0xdfea3d5a11d38819,0x717c2c4500c8a8c0,0xc4688c81e31a6bfb,0x2efddb7e9a1a49ec,0x11e0e744d9b9bf65,])),
        field_new!(Fr,BigInteger([0x2f760135fa494ee,0xe4c672bdaa35fa34,0x1a97d2b5972454fb,0xb81957273e6ab4ff,0xaae7da73e2b6266f,0x1e686ac49d2a0af0,])),
        field_new!(Fr,BigInteger([0x1eba31296647c903,0xbd60962fb86746ae,0x1b4d0d9f30fb2a43,0x5b25913bd3ddc434,0xbc01fc18fb238c25,0x116e17520ecc512a,])),
        field_new!(Fr,BigInteger([0x59d0c63a50610ec1,0x3fe0bd51fa0924ce,0xc6d8ebda99506139,0xe99dc0342673ea71,0x9c64cdfb11223be2,0x230326e1cac30a51,])),
        field_new!(Fr,BigInteger([0x19dd66a19f2d898f,0xba24837495c205bd,0xddd6a3e566c9364,0xf17d2d050307ed13,0x8569697716a78d55,0x407aa69c480f0b3,])),
        field_new!(Fr,BigInteger([0x2243ad2201738dfb,0x47fa258210c3c4d3,0xe16a2f543c5e1563,0x388ab7de843b5472,0x1e0cfdaa9aee9db9,0x110ccf2550e558c5,])),
        field_new!(Fr,BigInteger([0xaeac429db7c9fc3e,0x7987de7d8d4db1ae,0x2014fa7be205236b,0x25907e49da254eb,0x3f5ffb11ff83ff7a,0x1f3ac24ab99ac449,])),
        field_new!(Fr,BigInteger([0x6cc00c3862aec83d,0x53d9edc4ca869837,0x6a6faf39cc7cca99,0x79d17dc8f0fd7a8f,0x9ed37f5afbd892fe,0x1f954915257adceb,])),
        field_new!(Fr,BigInteger([0x225313eda783d91c,0x76933610b1c2a124,0x521b514063a98ce4,0x13097fb4ee798ed4,0x313901195a1ea8b8,0xff5d28d7fe4999f,])),
        field_new!(Fr,BigInteger([0xa1c3174820850d5,0x168f64a76996a0d4,0x303146faa39efcbf,0xa8fc079df528ce32,0x34a6e6f873f78b5d,0x1e309d65bec39e58,])),
        field_new!(Fr,BigInteger([0x5aba4a3eba88b40a,0xf127a7a57f966e88,0xa865a53933a2c98,0x7701a2b048d8493d,0x493e5cceb2dd3b4b,0x145d682927bfe049,])),
        field_new!(Fr,BigInteger([0x6ba71c33b869a2b,0xe3c311ecbc20b673,0xc8ffec8a168b0beb,0xd45919cf48d19ca1,0xff2aeb83156f1e0c,0x65a7c17c04d9b8,])),
        field_new!(Fr,BigInteger([0x4483c5ac6052733c,0x65bafd9ec9cfeabb,0x2d9af7ffe46491f4,0x5107fa9836303c50,0xb1626909c20a8843,0xb77644d31505c4a,])),
        field_new!(Fr,BigInteger([0xfa37aca0ad180976,0xadd5ca29c549ea0,0x4ca36d8a5becbf99,0xb35cc97506fba437,0x801b618d8f7a65cc,0x88bb0ff7c887260,])),
        field_new!(Fr,BigInteger([0x74334a2589275f8a,0x495e1acbf42feda3,0x6f598447f9edd1e2,0x6e8bcbee242e2acd,0x737217d76399b6c,0x7605effd6db690e,])),
        field_new!(Fr,BigInteger([0x79558d730cdb0cf4,0x4d76ad94a57ea3dc,0x5a50daa4eae5be50,0x74dc4e343537adc0,0xaf57e89b8c8f3e4f,0xf0262b7cd58877b,])),
        field_new!(Fr,BigInteger([0x8d5a4bfd9e8e4c01,0xc48f87f84b1837d0,0x9746d06f3c208d0f,0xa55b05cc96e1f278,0x9feb469338fd0639,0x164d9b54c6bbaa53,])),
        field_new!(Fr,BigInteger([0xd711a95e74aa16e5,0xb21b42826191468a,0xef2215a6e2465cf5,0x3be11d38cd2abc2e,0x6922aa004be7acbe,0x9a438ce38f57452,])),
        field_new!(Fr,BigInteger([0x6f0c791d5a19f8ad,0x275fc4f4cf3f0749,0x14e6278ceb5603f,0x590ca23d0742e311,0xd523652098158b3a,0x144386ed9e2bd037,])),
        field_new!(Fr,BigInteger([0xf1b912e706f95fc1,0x9376a60c0fc8251d,0xd16509aa8d5702aa,0xf2ad8b42c152b137,0x2afc63ed502bf64e,0x1b6b0e5558cf617,])),
        field_new!(Fr,BigInteger([0xbe0ad82d9087197c,0x52bb9b60c3921550,0xd6087209d2c93fe5,0xf237aa4f495c4e6b,0xd7ed19ea6caae622,0x1ff295b91a998386,])),
        field_new!(Fr,BigInteger([0xe7b97858a6116d4b,0xd252a504b677fc67,0xec18f05d02c43c78,0xa34d9af2785c6751,0x7441dd9d2c7386c8,0xb755708ab1d63e1,])),
        field_new!(Fr,BigInteger([0x6a1966d3e49fafc5,0x8c1d2f21edfda2aa,0x1e82cd1e3a21a87d,0xfd8c44699c59c071,0xcb6db201aeb8e231,0xa57ca087cf89d1a,])),
        field_new!(Fr,BigInteger([0x9b878aa4f5c861a4,0xc7a50a6ef2667e80,0x3b33bc9fdda7f2b4,0x2b2b093522416676,0x33c874bb886eab7e,0x2f7225321705c9f,])),
        field_new!(Fr,BigInteger([0x38dfe7c8970c5a78,0x190711550d76e4fd,0x8af31c1ea6981255,0xe44676fac09c007b,0x104542df1c5818ad,0x22a3b7d8efcf0800,])),
        field_new!(Fr,BigInteger([0xad5aad220f28bf51,0xb787f8a1009b43bb,0x9f5c78b850cf435b,0x2a17d2b78b00b5b2,0x2a4689cf92603212,0x885788fc73b9dec,])),
        field_new!(Fr,BigInteger([0x596522cf3842f886,0x6ba78ebad4ad6c5f,0x5e915622de2ac7a8,0xd2e59e5e9b7803e9,0x12c15ef046080ddb,0x6196d0e51609c2f,])),
        field_new!(Fr,BigInteger([0x7e431c78ef003b9e,0x9bec5430fd198efd,0x7adfe197a648c9c,0xadc6814bdb8bf143,0x3ef245fbeea19ee0,0x1b502b659f6836ba,])),
        field_new!(Fr,BigInteger([0x7dd783443d5ade4a,0x8d91ab427b47d701,0x559737434af8cb42,0x5de98c39e51c61c2,0x6795b74aabd89d60,0x160214431f119d36,])),
        field_new!(Fr,BigInteger([0x251c10d7fa7c47f7,0x3c5fed691f68b593,0x3ea6ba7614ec69a8,0xa83d2c9a7604b3c7,0x503e43021f5084dd,0x30d842ee24af4af,])),
        field_new!(Fr,BigInteger([0x24a001965c5a1ba1,0x21a3948e442d7a1b,0xf262851a2eaeb09a,0x9a271685559ac491,0x5eabb60c7b9cdf7c,0x1368a35e372e7d9d,])),
        field_new!(Fr,BigInteger([0xcadffb361e7dd4e3,0xadc86c733c0b39a0,0x6c02ba0221296118,0xd1c3748fee443c9b,0xc04c5a63e15d102f,0x1a9f44a94d17649f,])),
        field_new!(Fr,BigInteger([0xadb5ac70082c132f,0x667ea02a0bf6f1dc,0x33a436e53c7eda95,0xea430c4a49f27027,0x9f7c45e34cbf6009,0x1ee595a24a59d641,])),
        field_new!(Fr,BigInteger([0x28ac144f3a0b7e60,0x81fc47eb0a5deef1,0x1a14e4dd531e46cc,0x7dd2f07f98c3421e,0x531bc81951825408,0xa2c68961991d3ff,])),
        field_new!(Fr,BigInteger([0x63dddc915e48446f,0x3a0f3d957ed21daf,0xabc04d220488efca,0x5f6b1f817b891852,0xc59271c2ace370cb,0xa4e9ffabdd62291,])),
        field_new!(Fr,BigInteger([0x8db624d513968f95,0xc170025059125c0c,0x5abbac40d20de48,0xe3e20a404b528996,0xcd3929d5524f33dc,0x9c3cf17f05ad0e,])),
        field_new!(Fr,BigInteger([0x9b277edb6efb3130,0x988064c61e7619fd,0x2fbfd271f9b310e,0xef68cd1a6799c767,0xba33b0055fb32250,0x196e91c97e27ce2a,])),
        field_new!(Fr,BigInteger([0xad6bccb6962563a2,0x3541e76cf2b27ec6,0x4da50d8dc11d476a,0xbd918c9990d0819c,0x1be2192580c32d03,0x13d8d818e68a3503,])),
        field_new!(Fr,BigInteger([0xc3ea077e59173225,0x9b2cda8e512b43d9,0x8d8cc70481f2de1e,0xf81ea731024e9e40,0xd7815d8494506e6f,0x111cc2c3474c379d,])),
        field_new!(Fr,BigInteger([0xc3990a99670c6376,0x733fede82f6a9f32,0x25ccf1bdc3a7b6fe,0xfb1688881f90f542,0x1d1c1fe21fc1053f,0x1185a4198ce71f31,])),
        field_new!(Fr,BigInteger([0x56e6d5ac098ed4f7,0xf6a50ebd6524904c,0x78f752af811e0af7,0x5c6785c73ca6a1c2,0x3573984c71537f33,0x9a4f531f29dce14,])),
        field_new!(Fr,BigInteger([0x2acbb144412bd40e,0x12793dc870c184e0,0x99293f4107113fd0,0xf1f7d677ef74423d,0xca28475098096a20,0xcdb39d0b13228e8,])),
        field_new!(Fr,BigInteger([0x7326571976ac830f,0x2dbb7b7357c055e,0xfb3f0c006d85055f,0xb9bf39bb94ae555d,0x9586cd1894236411,0x14275986001e5b8e,])),
        field_new!(Fr,BigInteger([0x4ea34215e89e9594,0xb04b3fcabac985d0,0xc385ba3b30bb9004,0xd895542bb41b31f7,0xd4ee182cc63f49f,0x14429ec401d439d9,])),
        field_new!(Fr,BigInteger([0x7d9256120eea4336,0xc3b7c11a24f3ad9d,0x379d9ffa093019d7,0xcb24c948ffc31f42,0xfaa6a3f44513d31c,0xa73bb43e27c4d9,])),
        field_new!(Fr,BigInteger([0x5715f91b450a2b22,0xbe6056637c16d403,0x18e0ee010694b3eb,0xeea2d89ffd0325c,0xf4d046cd2663d58c,0x9390ecf851c6bb5,])),
        field_new!(Fr,BigInteger([0xcf582579e37b65e4,0x7cb61273ef51698d,0xe21caf10ff0db9e3,0xd23478bd96a9fd46,0x50a060a3b4d52f99,0x1de0c3208db25112,])),
        field_new!(Fr,BigInteger([0xd6799804a7c64aae,0x61cb60d3fa12a952,0x6361bc0acea399e1,0x64048d38061bea1a,0x459654a8836b40c4,0x110b87f12f17d2cb,])),
        field_new!(Fr,BigInteger([0xd4576d455000661,0xafbd7dab30d92892,0x42124cd19ef60497,0x677fc6071d62784a,0x2cd20ec12410380b,0xcdc0ec2f73389a1,])),
        field_new!(Fr,BigInteger([0xdcd901cc066ad6f1,0x2c65a94ceb06c216,0xd2020b3627e37199,0xdc36db63303c1f95,0x77f4b5945b03b180,0x1a4d8e85086a7018,])),
        field_new!(Fr,BigInteger([0x924d686e1f4de468,0xfb951524e511a931,0xdf0c374b77a287e4,0x571839b1986e69e2,0xeb1386c4838d6f40,0x1ba077f86ab31ce9,])),
        field_new!(Fr,BigInteger([0x46a80563869043b,0xe2a9cff6e164dafe,0xb06e9dc8460c4df5,0x42b109c6b7aa652b,0xc3f2a1ba965ef49b,0x188fb3cf5d26ef98,])),
        field_new!(Fr,BigInteger([0xb4ad19e94842f68a,0x30bd2dbe0a36b781,0x81dcc8d903d96637,0x17d5654d4230b8e8,0xf916fc51d11081bf,0x18d9ebe7791394b5,])),
        field_new!(Fr,BigInteger([0x8e1a2fe58ea5b4e5,0xa293946be1872304,0x60c7c8d04a55d07d,0x3b4f31f25b4b992e,0xb0c1889e90604cd9,0x21de7a9924782247,])),
        field_new!(Fr,BigInteger([0x73708f4080f1bac5,0x9bcd5349856d3b85,0xbaf65543926b79c0,0x1e5a0e846c2be200,0x75a71228b1c408d7,0xb00a7c04513b482,])),
        field_new!(Fr,BigInteger([0x270e3bc92fe43b76,0x4508e2c719a621cb,0x28150cbdd98573a4,0xc51f19fba8a857db,0x1d616ccb11df5cf3,0x5fa245a0bce6684,])),
        field_new!(Fr,BigInteger([0xaa5758d6b77fa096,0xfb86385a61da33a1,0xb8760cb02d62871a,0xb777e60379664c6e,0x4e35b0262a8dd1b0,0x4b7a81c8fbd5223,])),
        field_new!(Fr,BigInteger([0xcb46ae413dd897b3,0x3c2f05238b865685,0xe2c71aed8f17cf48,0x24489fb04292964f,0x7297b7b70f73d062,0x912823646e0441d,])),
        field_new!(Fr,BigInteger([0x223a2f3352f8e722,0xcdd30eecbf3a95e4,0x17661fd46a883cdc,0xc4558484a5ee007f,0xd7b36a7acb002d96,0x149056a9ce282692,])),
        field_new!(Fr,BigInteger([0xdac8476fc388dbd3,0x41ce264f30113429,0x4b75791e88afcc5e,0xaf0feb0d78958a1a,0x456677e7084f6510,0x971d78775774c05,])),
        field_new!(Fr,BigInteger([0xea6db25c7f53a2de,0xeee885144aef66aa,0x2a8c170053fbed18,0x8bac8127939f0bc3,0xdebc8e0d27c0bed6,0xce25ea5ca6a23fa,])),
        field_new!(Fr,BigInteger([0x2c4f6fb62d7c30a0,0x593571582f4c201b,0x7a68b9459c6eeff2,0x582e48599c7e5b87,0x2de3f60125b3c492,0x57b622e2b54bc08,])),
        field_new!(Fr,BigInteger([0x9767d665a4befb9b,0x36869114281a8fe0,0xa96fe9de70b3d14c,0xeca0a53acbe1e9b1,0x92a46fc52c530cbc,0x1f7223adb838d6ca,])),
        field_new!(Fr,BigInteger([0xd14afbb062ec7466,0x7f573318281e44d,0xc0f3907c7d65602f,0xcc358ab3ede53284,0x3f108fc02249b5d6,0x2037000b310a41af,])),
        field_new!(Fr,BigInteger([0xe3a7f5a60c842ace,0x291591469e9e388a,0x1970ce92c091bc14,0x281e0bfb36af26d,0x4cb460106ebc8464,0x1a4972a7abd72a9c,])),
        field_new!(Fr,BigInteger([0xeb37f3044a018cfa,0x740b4f1a24b705ba,0xab5191ec02196fb8,0x5c602ba23ab4b6be,0x14cc18a48880bc74,0x45901d587c632b5,])),
        field_new!(Fr,BigInteger([0x4d03f8feb4e29412,0x1312da11e9d6c3c7,0xa3f3f447ae8c2b18,0xa3d46cb6d3aff0e7,0x6b3e6e402cd32755,0x2115dbd506bd9ff3,])),
        field_new!(Fr,BigInteger([0x5b3cc958ff25a816,0x40e1fafa7d8d0df,0xa303af264ac204cb,0x89d91f2e5a0012a8,0x8786e0c8fe120512,0x1ca3fdd74b72c550,])),
        field_new!(Fr,BigInteger([0x9d8b280bb3ffe753,0x85997448639561c9,0xa271b44e64ea857e,0x548f79af1b6a409e,0xc9327aae6474fda2,0x2365e24750970de4,])),
        field_new!(Fr,BigInteger([0x46ecdd73a74b7d1e,0x3a7e9631fcd5239,0x455987279a668aa,0x558388ecb2c5db85,0x5512f9cffcadb1ec,0xcbf236439e73c6f,])),
        field_new!(Fr,BigInteger([0xc05b829150faa3a7,0x91a4c4c6640887f0,0x9b78c67388127cc5,0x96f6c4c796961820,0x3c32c42953807d1,0x1f80990d61726ba7,])),
        field_new!(Fr,BigInteger([0x146f4d6db64519e5,0x4f1c67e5e1696854,0xa74bb64530ffebb8,0xb2147dfb425d992c,0xa1c937be431909cb,0x1ff0475c68d6f42f,])),
        field_new!(Fr,BigInteger([0xd3ac2f1875a1e4f8,0x541ac2ac1a33a705,0x5856458d6dbf42d,0x9c5c28e8c894b748,0x981fa30f407cbd38,0x1124b071e2840bdf,])),
        field_new!(Fr,BigInteger([0xbf7a8f2ba373e34e,0xb0e6d67cfa9b525d,0x5e82bbcefe9a8f65,0x94bfdb0881563f6,0x4f95c9e067a860e2,0xb59e5fd70c41736,])),
        field_new!(Fr,BigInteger([0x99bd7e328fd74e6e,0x6f40a78a33d0477c,0xd7a3ce8774f5c5d4,0xbefa4c401655b3e0,0xceed628ce16401a9,0x1771f0c9ab9e01a0,])),
        field_new!(Fr,BigInteger([0xaa33d3105c11d030,0xb4206b4274144118,0x71af41702c20aa1d,0x4b81d549f01a3bf,0x9e695344a20872cd,0xd9da4aef601b64d,])),
        field_new!(Fr,BigInteger([0x19a6aab308d8fc38,0x657996de7cdc0288,0xbabc456e42bebfc1,0xa78a28e83141031d,0x4786071361f1cdb,0x19eb1a863087b891,])),
        field_new!(Fr,BigInteger([0xf4fbb1fa6558c0b7,0x6b24b123ee40321b,0x92074c3648a99b35,0xf3be03c28c26611d,0x1df77967a66b292f,0x1a1daaedee4baa4c,])),
        field_new!(Fr,BigInteger([0x3a82f327f9606694,0x7291e98c9363473d,0x7b0b80ef86a287a7,0xdf1dcf2ccdc7506e,0xaff515139c07264d,0x23d7670fd063e7c0,])),
        field_new!(Fr,BigInteger([0xed50a3abeb5389fd,0x165e2b2728cd4440,0x7aa64cc11dc70781,0x4ff0c136113d2d3b,0x823c918132409328,0x139d6ff9a0fbe6f,])),
        field_new!(Fr,BigInteger([0xba6c752de6036dba,0x287bb7c856fc951,0x5e811f9882ea22db,0x7b92e32100367c87,0x8aac4c9e772db556,0x12be4d6b3d608d6e,])),
        field_new!(Fr,BigInteger([0x61e0975aa3cbc721,0xe837cffcf71f4387,0x2a5eb9ee35e8eb2,0x6157fe03aa9edc19,0xf9454bf7e9d8d856,0x1e0d0c1935346420,])),
        field_new!(Fr,BigInteger([0xc9479abf4cb7c45,0x6a3f08b15901988c,0x4cd230c93832ff25,0x7a94cd892bfbae6d,0xb7c7ea2bf8fa825c,0x1477cc8413069412,])),
        field_new!(Fr,BigInteger([0x8db7f9df9aca2466,0xa003378e6f470e35,0xe1595479552688c8,0x140789fcf6f470d2,0xf43fe45a3ff8c0dd,0x13eb14c4ba26f086,])),
        field_new!(Fr,BigInteger([0xba11304404e55714,0xd7a5ebffa7108c41,0x7fb72486c2e5fb72,0xec3d576731687f6e,0x77944f261dc6ac2d,0x1c539ae5469b07db,])),
        field_new!(Fr,BigInteger([0xa56715ce64d6d51d,0x2b0fdc813d82ce81,0x67dbaa19d64ebb14,0xf28425a404173a4a,0xb44df750f88769f,0x165d737d22d40d8d,])),
        field_new!(Fr,BigInteger([0x38d54f187254b673,0xdb8329159cfc7d0a,0x341b5e6877f9e5dc,0x308eeaa500c5fa7c,0x147c7b04a5686654,0x1e0c1343a2f46ae9,])),
        field_new!(Fr,BigInteger([0x7d0a7817339648c7,0x6d2bbe2568bd122c,0xb1d022c56deacb8d,0xf3a2157e7c0fb9d2,0x32a928690f5785eb,0x23d36f0efa20bec6,])),
        field_new!(Fr,BigInteger([0x1ba73d3f2039a551,0xb20ce85ee313b7a6,0xfda26ce92468b557,0xa536e7778b08ab31,0x55b343ab2a03877e,0xde0d7750338de90,])),
        field_new!(Fr,BigInteger([0xd14e2e58bd3a3600,0x52cc43605ccb7878,0xd45ef99362259a19,0xdbd620a074e674f4,0xf1308b7999e86648,0xd99b203be30299d,])),
        field_new!(Fr,BigInteger([0x2d258c64a39258ae,0x95770a7d649b2751,0x5ceff0d392b75775,0xb0d5088aea922240,0x2eda9ef8e22238b1,0x1eb3a2847562c989,])),
        field_new!(Fr,BigInteger([0x64c5a9976c34367b,0xb36428fcf75e189c,0x515c48a6a206d639,0xec7b8d3827ea6418,0x242306fe1c4c55e8,0x1e63d46e2cfa5ac8,])),
        field_new!(Fr,BigInteger([0x45a449fb632a6ea1,0xc758b2694916c9ae,0xc4cb7c8fa5904b63,0xca43dd46ba5aa36e,0xc34770489b356a87,0xc2fbf2fca9a77d2,])),
        field_new!(Fr,BigInteger([0xeec6c112251e481,0x1c6857cf9941a068,0x688ea495b579093e,0x51d0da22ae2e8e88,0xb39e86163126d812,0x227aae148627caf8,])),
        field_new!(Fr,BigInteger([0xe74a3f48e86fea2e,0xd0aacde6489a7b13,0xfae7d18731b7ca32,0x9f383d76e9c81ec4,0x9b5c879911035711,0x991664507056f46,])),
        field_new!(Fr,BigInteger([0x323b0d71397ebaa5,0xd7f3d3ea938134ef,0xa064e9462c589493,0xbd21225d39e11944,0xaee1c83237ab6fdb,0x744d5aab0f1154a,])),
        field_new!(Fr,BigInteger([0xb9f070ac75463fab,0x52c1d55cc512c1ef,0x17a0e056654f5771,0xdf40372a1d2cda87,0xe3f2dc081747fa47,0x1eacdfbdf93b83f,])),
        field_new!(Fr,BigInteger([0x3fb40908a6af991b,0xdb8e3ad754fb1b05,0xf0ca408ff4260bb6,0x724a1491dad7e2da,0x799badb10eec717b,0x16d7513dd78dbe80,])),
        field_new!(Fr,BigInteger([0x622d9fad56102378,0xc006938896526de9,0x35f38c1290e7706f,0xfac8725935829b6d,0x5d04e3dfa16fc2d4,0x3b154e70a17f9e6,])),
        field_new!(Fr,BigInteger([0x281ee6cd03d6a761,0xa1042b8794f7ef58,0xcb66d5b539a7adb6,0xc36ff447f5378b46,0x7fae60474ae15653,0x1f9bf90f2a259054,])),
        field_new!(Fr,BigInteger([0x2ff1497169822a09,0xfbe99c36690a05cd,0x3652ed9ef2dca99e,0x927baf0f74563ae2,0x73a8b390017e502e,0x1440521166668284,])),
        field_new!(Fr,BigInteger([0xd1916119637fa5e9,0xd454c90c89389c66,0xf479a0c8deb8865a,0x32982975bfbb0739,0xe0a2dade98190398,0xb9b1fcda1ed0d88,])),
        field_new!(Fr,BigInteger([0x7a5c42fe5004bbee,0x870b992532c7ec69,0x6e5e0bb83dacc6c5,0xcd304188c3674c3a,0x65020c69284cc361,0x217f5f53e3379ddd,])),
        field_new!(Fr,BigInteger([0x186fb6dcf55b541a,0x33808954d7e6e696,0xf72b728bc765d0e0,0xeb287542941dc3b5,0xa460f5242f3d2a99,0x12dea74e20511847,])),
        field_new!(Fr,BigInteger([0x56a8b5c7ab465750,0xa85596f010b9f395,0x8c0d1516ac13fc3d,0xebc4ab3e7779f074,0x46908169bee2a8ae,0x211c205a812f6e62,])),
        field_new!(Fr,BigInteger([0x4f1a7a692b818a56,0xcbd0e353e8f7f4cc,0x886917652a9c3fee,0xa9037c8e67477fe,0xac035ddf8e176ce5,0x23d8568d4f9f1b8f,])),
        field_new!(Fr,BigInteger([0x59d5db51e5b3aca4,0x39ed6b782cbd472,0x8b59b0612ce3ff84,0xab70aa79f61680de,0x5ed9fa83db412c51,0xb6bcf9d8f4fdedd,])),
        field_new!(Fr,BigInteger([0xc46d1e4e9b063124,0x22fb889539d315a,0x605ae51ce39fb701,0xce27f6dc690c5d31,0x5b21edbe47138fd0,0xbf9244e5d8c724a,])),
        field_new!(Fr,BigInteger([0xa4b893db7265d28f,0x27840a56c73e0e90,0x665109d3d3c5bcbf,0x36e48aabe2c53f02,0xe91dd6d198a41348,0x15e5c81319388e50,])),
        field_new!(Fr,BigInteger([0xd3b35835ce2f5568,0xc9d87caafdd70880,0x63c0aa7690deb97c,0xddbe32dba15ec989,0x6c0ecd498f7abd9e,0xa97e9163209e830,])),
        field_new!(Fr,BigInteger([0x6d724e6e54fce9f7,0xf3cab2cf84bcb7a7,0x6512228511d8b645,0xcc09e60d2fcc95b8,0x8b69c789dfc5d84e,0x1e318fa8f5e2435,])),
        field_new!(Fr,BigInteger([0x67bf7dcbbe25b1b6,0xc9210e60b8edf434,0x2cb9649613583586,0x999917c7c1769441,0xb039646cb40b3cc2,0x60d7ab47bd4fa32,])),
        field_new!(Fr,BigInteger([0x185c925eb60646e3,0xec905576c7c038f8,0x623462f3ee26f348,0x3d3f2a4cffdf186,0x714a88b6868b0c93,0x5cefe3f902ec4ce,])),
        field_new!(Fr,BigInteger([0xbdd8c952c824ee91,0xc50e750144e8daa3,0x6f72775ecd9cfc99,0x4b8202acab657528,0xe91b332b3a32cb01,0xdcd5bb998a59f25,])),
        field_new!(Fr,BigInteger([0x35ff043eb0a259ca,0x8a19d1a07add11d9,0xd5a7f65a7e98e76f,0x1d1f5e3d7b67acff,0x1f3160260ad5d071,0x1f49f4f4c4bb0a73,])),
        field_new!(Fr,BigInteger([0xcd2d09df5d799ab3,0x301c5e78c75ec61f,0x3dd659aed620cb5a,0xf49ec1aafcddde8b,0x9a5900a61f200d79,0x12471025c1903c3a,])),
        field_new!(Fr,BigInteger([0x1b70d555e2ca135,0x8b25fe3158b211dc,0x423cb7743e56cdd2,0xd83ec2b68f32a3cb,0xafa31e76172fe97b,0x1284a8d884504da0,])),
        field_new!(Fr,BigInteger([0xb4e20ef9c167092a,0x25e23e7643b2353c,0x2e9a487d6fcf0e27,0x22017a054baa0dc1,0xdbe5fb651269e627,0x175f982ef538845e,])),
        field_new!(Fr,BigInteger([0xa230125cae01ca06,0xb60ddd805060bb65,0x781f6358fdf35cc5,0x27ec26272ca9279f,0x223b9b925145c7d0,0x15321c7999c2c790,])),
        field_new!(Fr,BigInteger([0x299cf1feb0967100,0xe1c814fbe77f0aad,0x74f1a6571a1bf4c0,0xc5b00355e3f71462,0x42959e6fca317d1a,0xf6f766752fcd031,])),
        field_new!(Fr,BigInteger([0xc90630575fe8926f,0x4beecf53beecc9e7,0x3ec5c23ff79d26e7,0x82b9ae9b2074a975,0x3275d3335a5b61a4,0x17e60476ddc00394,])),
        field_new!(Fr,BigInteger([0x61ca02d84e446d4,0x8bf76e3afbf222cf,0xe3f845d9b5c526d9,0x31a417dccde139c4,0xaf451639027fa0a8,0xa5d949c5ba734a4,])),
        field_new!(Fr,BigInteger([0x684b5d5dd06452,0x81873d5c3a927f4b,0xf3ae7d878b53045e,0x5b12585266e9ffc2,0xb6a33967c4fcd23c,0x1fee25b968a19460,])),
        field_new!(Fr,BigInteger([0x28ac05b45525aee6,0xe6ec0ec78f89e6be,0x763da0a94ffb1777,0x9fd9806cf8e0377,0x39853681df51f01d,0x10e884b847588f2e,])),
        field_new!(Fr,BigInteger([0x9ba33a2b8e6d3e77,0x2375fd431e4f63ed,0x287db763b1775b1f,0xb444aa043a658005,0x108dc7af1268421e,0x203df7ff4b1018b9,])),
        field_new!(Fr,BigInteger([0x77cd2254a1ba4727,0xe06d00860400b3e,0x3a834592427d51b0,0x812b436e8d62d10e,0x84f6075d99f256fe,0x1c5a16937066e01e,])),
        field_new!(Fr,BigInteger([0x9a9f5e24b2de8f9,0x5686257bbbd39f6f,0x838a747f765013ea,0xfe45b64bc9bd029,0xdb964863079a7c10,0xc919816e59ac26c,])),
        field_new!(Fr,BigInteger([0xbe21b3621c08586a,0x2724d1f62e184324,0xa02bb0baa99fa2be,0xde816314ae73aed3,0x3667d7ef0501a531,0x20737843e7abb3e2,])),
        field_new!(Fr,BigInteger([0x935770455d764bed,0x978193d2ea32671c,0xc080a64d91aa30a3,0xf373a04cd5ee7205,0xf6107d1a4eb5e7d7,0x12360610e1533edd,])),
        field_new!(Fr,BigInteger([0xe0708f8fafeb7e9b,0xf940ab183d24b34c,0x2c6306a03e773c24,0x4e8e976d3e59aea,0x5020c2ebb299d0d0,0x1aa2a3dda80c1b3f,])),
        field_new!(Fr,BigInteger([0xf853f29bcdb2b758,0xdfffd08ae4750820,0x685519bec7ef0e89,0x2eac74c3e917edf0,0xaaea4e1001653ce1,0x98d7d48d1a81695,])),
        field_new!(Fr,BigInteger([0x7425e6b4fb03ff8e,0x2105558124998e6e,0xb798777e99bb4557,0xe28f58384b8d7d16,0x5143f5e56d13cfe8,0xcae8a8711f32f8f,])),
        field_new!(Fr,BigInteger([0xb6c46eb0b3d4b33b,0x7c2cb08899d9edcf,0xbd7c3b37d41cf642,0xac04c8564fe08058,0x6b1caede28480bac,0x21fbc48be334a5a9,])),
        field_new!(Fr,BigInteger([0x9f938a64de6abc09,0x4627eef381040b02,0xd9d5fa5d8d96c84,0x33ad95824019e9d9,0xd917accdd2b13a7b,0xb2d9874e2a62cb1,])),
        field_new!(Fr,BigInteger([0xa5ffbdeb2ea110be,0x766bde9ef5bd7107,0xd9f698f2bf09048,0xe276b168a207b6e1,0xb690c8fc42b71e91,0x54df5846a5e813e,])),
        field_new!(Fr,BigInteger([0x24db87d6e4d6709,0xbf1197219274c2f3,0x471d23fef55f04aa,0xecfcd4a4fa627c4d,0x95421db58854eeb4,0x1d20e6d69f5e945a,])),
        field_new!(Fr,BigInteger([0x3348360c91c59160,0x2a1db34c49353e2c,0x44ee4803e36b9e87,0xedb38be161db8745,0x28e53ccbd28bceb6,0x93366d44b72a28c,])),
        field_new!(Fr,BigInteger([0xaf2ca5db68a78c48,0x509dd080dbc8d3cc,0xa81246a0f655ecb0,0xb426bee485d33879,0xce02523041e91e2c,0x1d89a23c703a2b30,])),
        field_new!(Fr,BigInteger([0x19d38b7daaea3530,0x49657e8d58725c81,0xaa4d32ba5d860a1e,0xb229e9c836f1b38c,0x6cbc121177f093a6,0x21ad1de692351f86,])),
        field_new!(Fr,BigInteger([0xa948ecbfca226399,0xe953e15ae5a5369d,0xc33a0a0ef1d1f5b,0x8af264c523f5b377,0x84ac6d77cb262d37,0x13821cdca29e6d4e,])),
        field_new!(Fr,BigInteger([0x62312591f619fdc0,0x5d33a1df255325a7,0xc7e5649004526bdb,0x60ec4c76f23c340f,0x46a13ab95d03d2ad,0x1069188d2c9a5a53,])),
        field_new!(Fr,BigInteger([0x3010e9acec9096d7,0x2b203ae92f8ed6eb,0xbd7a5a549decffaa,0xb22773183e3ea1d7,0x476b4cf2ed6f4126,0x189c2d6451dbe104,])),
        field_new!(Fr,BigInteger([0x75513c8a0fa7fafa,0x5e35eb4662658a04,0x573426a661704df4,0x494f8eb7d41ef30d,0x46bb978e4987b42c,0x1162126adcc68ff7,])),
        field_new!(Fr,BigInteger([0xa06cc2f15a8dcbfe,0x7f57f8d71e46e63d,0x612d8f679804b0ec,0x49a7e74b1ca8b3f9,0x91fbf9a3ff6c31ed,0xe0c50bcc47c86c2,])),
        field_new!(Fr,BigInteger([0xd8df2f0db3aef3ce,0x5d04e468adcf12,0x308980a74c1e4ce3,0xb5637748d790029b,0x3e7e1a564eb69c80,0x918465ddb6b1f44,])),
        field_new!(Fr,BigInteger([0xf2fb32edb2515c6,0xf0cd212a371e1e7e,0x42a08dbf3d6f4cbe,0x649d9ea1b64ffe30,0x8c9fb237c238eba,0x1a4303f272d8bfa5,])),
        field_new!(Fr,BigInteger([0xa1eef4849620442,0xac71ab32dc6f2775,0x84e1d19794f4dcc0,0xd7fec7abdf034aec,0x7b56d5f965eaea8a,0xd1f5994cd986d19,])),
        field_new!(Fr,BigInteger([0x5c43385f49919aa0,0x7f250b2827c9a0c5,0x6cef909a571df578,0x83061f78d24752d4,0x607cf5724015ea5f,0xa6b4c97124db01d,])),
        field_new!(Fr,BigInteger([0x84cc4779f63ea86d,0xb9236de88b1b527d,0xadcd29b3e5aa0584,0xff0f794959835122,0x759445ae35be11df,0x1ebfd9ae8a59b8de,])),
        field_new!(Fr,BigInteger([0x774979a98aa1a428,0xd28d6db4966dbf5d,0x9a81829346be995a,0x8ec9014e3a48293f,0xbba6121dcc7f287b,0x6b9821b672e5458,])),
        field_new!(Fr,BigInteger([0xe8832dd2179b057d,0x1e20be80b160ae61,0x851cf07e9dd9b05,0x811cb0153f9d5b2f,0x1226419b4aa0b45b,0xec23a9e2774b9f6,])),
        field_new!(Fr,BigInteger([0x35fed0092c2121bd,0x86591ea9267c848b,0x116024014ba4cf84,0xb6199a51a489c6f9,0x822564eff591e51b,0xd55fb2c5e4ece87,])),
        field_new!(Fr,BigInteger([0xc6dcc91fccd7aac7,0xcf9ab43fad117528,0xdeb1ccf32d4c5880,0xa9c96ac7913281be,0x158f32784daa2e36,0x111426d7ea57ce16,])),
        field_new!(Fr,BigInteger([0x7b1ce60e4e42885,0x57263ac69ce54243,0x1f6978230085216b,0x93706bb6f8fc3f89,0x367aca6758325d23,0x12b511e38d0d16d8,])),
        field_new!(Fr,BigInteger([0x3bc7aa5c339dc68,0x73365851862aa04a,0x9e0057832f283402,0xec4624b3010b16de,0x4c899e803dfa6683,0xd3fc3bc3b2d083d,])),
        field_new!(Fr,BigInteger([0x7be36f2d192d31d0,0x245a34a523dccf46,0x57b8423ff597eb2c,0xb1aa67289cf52bcb,0x5eee2e2d650639e8,0x2bfa6fbdda246cb,])),
        field_new!(Fr,BigInteger([0x32079c180dfce428,0x10617c87f09b343d,0x95034dde23517d1a,0xafb1d3a4c2920d91,0xe2eb69935360d32f,0x3682b6074e77476,])),
        field_new!(Fr,BigInteger([0xd8d5136b2b6feac2,0xcae96ba0e8c7e57f,0x10f720c7818d1583,0xff669a8147cc34ca,0x7c1c1c408f32b9e0,0x1050bdfeedcd74a6,])),
        field_new!(Fr,BigInteger([0x7f8efa59bb904972,0xfb2614eb7cb968c0,0x41673203aea2b0f0,0x292e62ce6587a915,0x26e1377d438055c3,0x1041c909c7a54986,])),
        field_new!(Fr,BigInteger([0x78ca1fec56fecc31,0xaac71b8644aee19a,0x49cb7370ea9445c7,0x108ef1c4ae528cff,0xead47030db3e7e12,0x98c367b2789e318,])),
        field_new!(Fr,BigInteger([0x690b3ca2b6c19d51,0x58f12a81d28bd9d9,0xea9c0b5e33186f40,0xc05b840a2ba60075,0x653af412e1c7ebb3,0x1606f2057e47242,])),

    ];

    // The MDS matrix constants
    const MDS_CST: &'static [Fr] = &[
        // Constants in Montgomery representation
        field_new!(Fr,BigInteger([0x397857d68200d574,0xd28c82874875bbbf,0xbf3116276a5e626e,0xb46843e785373cf3,0x554aa7d66761bfbb,0x5d19bdb71778541,])),
        field_new!(Fr,BigInteger([0x42ed8e29c99e77d4,0x9b3dc99fa8df07a8,0xbea39276a88b451,0x68cdc36cda06aeb0,0x2df13cc054c8b0a5,0x4ba3b0a7edcfcd2,])),
        field_new!(Fr,BigInteger([0x7bda16389c0b7c78,0x588241e53a63bd27,0xcd74903a17166291,0x6b2a803c4b730a56,0xaa47fc73540f793d,0x1047d90f1d8ea82a,])),
        field_new!(Fr,BigInteger([0xd95f6116b09b3c00,0x90922c88c1601a0a,0x333b6fb8ab58e678,0xd0610aab079c52d3,0xabdb85ae6f7328e6,0x1840e3671d26102c,])),
        field_new!(Fr,BigInteger([0x8b12b7a7a81b57a4,0x50d95243b50466e7,0x7536012d01d2f5d3,0x342d728a0c0c024a,0x88e3f4607910e62d,0x51e550fd0093c84,])),
        field_new!(Fr,BigInteger([0x7af610cb8bfc9412,0x558d20d6cdf0db03,0x12500c5fd3e2c8be,0x612de2568ed650cc,0x9eae6a30c7e85c0c,0xceb5127234d64e9,])),
        field_new!(Fr,BigInteger([0x43a12d2da4f0badd,0x4fd4c419e435fa92,0xeea1e6fbe17e2c8,0x74b696b28d5da145,0x2585e1ab409b40b8,0x18cb0c351a6caf2,])),
        field_new!(Fr,BigInteger([0xbf302ec4ffb0388f,0x7d4f228f6851cedd,0x19dbefc2e70045b4,0xe2bc0b0a9e85b446,0x28292dabfa5a02a0,0x619cb1cbd979687,])),
        field_new!(Fr,BigInteger([0x72a8e842794dff45,0x48523ee8e5a68cd9,0xa863bebe796b98dd,0xc03bb379b5704529,0xe051c36ffb1a63ee,0x1a68d07f893e235e,])),
    ];
}

pub type BN382FqQuinticSbox = PoseidonQuinticSBox<Fr, BN382FqPoseidonParameters>;
pub type BN382FqPoseidonHash = PoseidonHash<Fr, BN382FqPoseidonParameters, BN382FqQuinticSbox>;
pub type BN382FqBatchPoseidonHash = PoseidonBatchHash<Fr, BN382FqPoseidonParameters, BN382FqQuinticSbox>;

