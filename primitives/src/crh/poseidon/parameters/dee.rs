use crate::crh::{
    PoseidonParameters,
    FieldBasedHashParameters, PoseidonHash, batched_crh::PoseidonBatchHash,
    PoseidonQuinticSBox,
};
use algebra::fields::tweedle::*;

use algebra::biginteger::BigInteger256 as BigInteger;
use algebra::field_new;

#[derive(Clone)]
/// x^5-POSEIDON-128 parameters for scalar field of the Tweedle Dee (= Fr).
/// 
/// The number of rounds are computed by ./evidence/calc_round_numbers.py, round constants and matrix 
/// are generated using the script ./evidence/generate_parameters_grain.
pub struct FrPoseidonParameters;

impl FieldBasedHashParameters for FrPoseidonParameters {
    type Fr = Fr;
    // Number of partial rounds
    const R: usize = 2;  // The rate of the hash function
}

// x^5-POSEIDON-128 parameters for the scalar field of the Tweedle Dee curve
//
// The number of rounds are computed by ./evidence/calc_round_numbers.py, round constants and matrix 
// are generated using the script ./evidence/generate_parameters_grain.
impl PoseidonParameters for FrPoseidonParameters {

    const T: usize = 3; // Size of the internal state (in field elements)
    const R_F: i32 = 4; // Half number of full rounds (the R_f in the paper) 
    const R_P: i32 = 56; // Number of partial rounds.

    // The zero element of the field
    const ZERO: Fr = field_new!(Fr, BigInteger([0x0, 0x0, 0x0, 0x0]));
    
    // The constant 3 to add to the position corresponding to the capacity (Montgomery rep.)
    // (we don't use the domain separator for now)
    const C2: Fr = field_new!(Fr, 
        BigInteger([
            0x123baa49fffffff5,
            0xd90b134e59472506,
            0xffffffffffffffff,
            0x3fffffffffffffff
        ])
    );

    // State vector after permutation of zero state vector (Montgomery rep.)
	const AFTER_ZERO_PERM: &'static [Fr] = &[
        Fr::new(BigInteger([0x85614442a60ac11a,0x55a43ca8180d2e08,0x43f61ff197080ac4,0x19d87eb89a42aaf1,])),
        Fr::new(BigInteger([0xa2f6b5a9a16d3790,0xc947563b131a126c,0x52c19607bb4b6640,0xc4604a460df1c57,])),
        Fr::new(BigInteger([0x7d8f3c1679a9cbe2,0xb09fdc38ee15fe77,0x810720bf23be8578,0x2ab876d1a0abfa95,]))
    ];

    // Array of round constants
    const ROUND_CST: &'static [Fr] = &[
        // Constants converted to Montgomery representation.
        // For rounds 4 + 57 + 4 = 65

        field_new!(Fr,BigInteger([0x5b40ba7b683c32b4,0x5c84c551ca7a85da,0x4c7048d27b81a93b,0x3635a5ecd9890320,])),
        field_new!(Fr,BigInteger([0xfaf17623b724080f,0x5147e68371f072d8,0x7b6db0e06026db4e,0x35568295f90299f1,])),
        field_new!(Fr,BigInteger([0xdf23982dd9ab92ee,0x930337384ab1cf0f,0xa24fb33c6e0edf07,0x16f9cec4b30643d5,])),
        field_new!(Fr,BigInteger([0x17a1f4e0ed300856,0xa2261565e18f8435,0x57023bb1ef5e4acc,0x29f74889b22b4f42,])),
        field_new!(Fr,BigInteger([0x250b3c444dce8028,0xfc3256f01f8709aa,0xda2ced599d05a12b,0x2709632801b2fad5,])),
        field_new!(Fr,BigInteger([0x45c1f93e5da9687d,0xf2a5b62ea2bc014e,0x8609bd7fb780d77,0x21984a539dd58517,])),
        field_new!(Fr,BigInteger([0x842f4e299261f8ee,0x95e3ff6ae96ee780,0xe898ea9fcb19fb8a,0xe86cd5182be4ede,])),
        field_new!(Fr,BigInteger([0x801561c22bef8ded,0x16f26b1df6fda550,0xb27e9b31bcde644,0x120e1d4f13fed959,])),
        field_new!(Fr,BigInteger([0xc8f8a96d4a4d6411,0xade9be9ac79de2bb,0x28d9f1b7afd1fa0d,0x2bd0d5cc0a8ec40d,])),
        field_new!(Fr,BigInteger([0xc0232d0b802af29c,0xa7479c9b6937b309,0x9251392cf5507e3f,0x222efa635791e7f3,])),
        field_new!(Fr,BigInteger([0x770afc56a6e9e7be,0x5eea5cfd5cbee98f,0xc7634888d523d361,0xc8db4523f0f6b37,])),
        field_new!(Fr,BigInteger([0x3c4bef6b32ecfd5f,0x8faf86e61ae51801,0xa7aaf638e5419e61,0x1e8ac93cce7af18c,])),
        field_new!(Fr,BigInteger([0xdba1e3b777225723,0x1c443440e9a57d18,0x355d465ae9214b,0x1ca702200dfb0714,])),
        field_new!(Fr,BigInteger([0x7398405fa18eed64,0xae86c4847d5522c8,0xb83609d939ea39f8,0x3dca88087d7ca780,])),
        field_new!(Fr,BigInteger([0x649ac2dbb3d71084,0x9908ffd822f3b422,0x9c28cee534c92fbf,0x19848720b9c422c2,])),
        field_new!(Fr,BigInteger([0x768a4011e50059bd,0xcf9257823d2b4b52,0xe67ddb151a7e2620,0x1ee345166e7b78e4,])),
        field_new!(Fr,BigInteger([0xd5278574024ca4bb,0xd27510c90177e064,0xa68b6eb08e653734,0x2a4ec10e3c350990,])),
        field_new!(Fr,BigInteger([0xf2e2eeec26f0c782,0x54022078c1ea0f08,0x1f94580b1710ae0f,0x23acae47a518b433,])),
        field_new!(Fr,BigInteger([0x3edcc3626063e349,0x24e01b80ad328575,0x6bbcd01882c1dc37,0x3f5be3b0c20a0297,])),
        field_new!(Fr,BigInteger([0x6a6a2fcdf45f0f1d,0x4b4b6abdcb2485a4,0xff62dc01f4d5c6d3,0xefb1b1545a3d537,])),
        field_new!(Fr,BigInteger([0xc4d8909cb5f1508e,0xdce57a9ddf63bdc7,0x4e40f59531a4210b,0x2aacd0bab17f7e21,])),
        field_new!(Fr,BigInteger([0xf0b98b8c922917d9,0x2df4f5b90cc69f6c,0x879b95c03948656f,0x247cfd314dff147b,])),
        field_new!(Fr,BigInteger([0x4b19813492891dc5,0xb770bcb2c942e03f,0x7f36c6866501a12e,0x5c528ac418866d8,])),
        field_new!(Fr,BigInteger([0x83b5862abd3a5869,0x6e9924a15a35e8e2,0x4ee02518bf9271b4,0x1810ab5d4db5526f,])),
        field_new!(Fr,BigInteger([0x57a29928a3f89a83,0x44ce075e9efa5ab8,0xe5986769c012be2,0x1fa92e8d9ceb62a6,])),
        field_new!(Fr,BigInteger([0x7f2a76be51108f7a,0xb22948b99ae4b573,0xb04820032d22d414,0x2d821e02f729b3b4,])),
        field_new!(Fr,BigInteger([0xd35d701a0855451d,0x86e98f69c3c8223f,0xd0dc2c5c4df590c1,0x14bbe1533e129537,])),
        field_new!(Fr,BigInteger([0xaeb51b1954d37e64,0x41dde0df17f6ed05,0x69e41530ca17714b,0x14b01168e046deff,])),
        field_new!(Fr,BigInteger([0xee99c169bd39e2be,0x9da34706ab01fe81,0xdd867f2a9f033fea,0xdf49fbdc0a3e246,])),
        field_new!(Fr,BigInteger([0x68befc07dbfbe05c,0x21e68712d7529c29,0x5826ac469b436c5f,0x1ffca5b9f5623d08,])),
        field_new!(Fr,BigInteger([0x531c7f1193f74903,0x3004cbceb702a130,0xde051b2c457bea98,0xfe4dee460343735,])),
        field_new!(Fr,BigInteger([0xd78591c4c24767e0,0xc00f76aae9605c7c,0x282d9be2a5dbcb3,0x2b8320190d37fc1d,])),
        field_new!(Fr,BigInteger([0xc8ba5b82a4b8f36f,0xcd83353c928feb30,0xdc7d210da77da39f,0xd8b9729a1595850,])),
        field_new!(Fr,BigInteger([0x69ab2f87d14a19d1,0xa50d9f2c2652b492,0x2e2d4ed8796f2095,0x24f017b692739e37,])),
        field_new!(Fr,BigInteger([0xd275c2c9c91d7810,0x422f2a583d3ed019,0x6c2d97876c962800,0x35c2b094b945aca8,])),
        field_new!(Fr,BigInteger([0xc209a45eab978019,0x4aea5d9c7feba34e,0x80330115db35a489,0xc8c3afb44b5e433,])),
        field_new!(Fr,BigInteger([0xa31ea13afb07e3a9,0x576fda31d615e9c0,0x6c3fb6e0ccbc51e2,0x3589d714414c6ae6,])),
        field_new!(Fr,BigInteger([0xf066573facee6ae5,0x3f5780997add0273,0x96bf0cbfa9eb2d77,0xe1f6cdcc5cc4f82,])),
        field_new!(Fr,BigInteger([0x82175011fcbc1132,0xd9f8835383b444ed,0xc936550b32a08e9f,0x3d2f6e9af6c701b3,])),
        field_new!(Fr,BigInteger([0x6d1257d391aaab40,0x9d6bc81892ded952,0x8d494d23f2b6a450,0x2ab76afef15e4907,])),
        field_new!(Fr,BigInteger([0x4f2f36b61527cc9b,0x7c24b84bc9efaddb,0xf566d68c350e5ad1,0xb70be812bbd7984,])),
        field_new!(Fr,BigInteger([0x5c0bfb236fcd408d,0xa168f50ed8a2d3bc,0x3dd1b7f6d6a6c3de,0x9e7f31ca061f58a,])),
        field_new!(Fr,BigInteger([0x529b2af2b6d2692a,0x5f7726eb988e29fb,0xf526ad2131c13565,0x2c2f89f61e0821b7,])),
        field_new!(Fr,BigInteger([0xc9a0042e7e8d9161,0x1ef6405cd54e9f53,0x8fb81ce5138d5e4f,0x255daaf8cd7fba2e,])),
        field_new!(Fr,BigInteger([0xa23054850faa19a,0xabce1c215fce82cf,0x11e602ed29969952,0x141e8a5470240d16,])),
        field_new!(Fr,BigInteger([0xc5479d44f4495019,0x9517b52757c8df1f,0x90c8782d13821a3e,0xca01f04d0e6b51,])),
        field_new!(Fr,BigInteger([0x7808ea626b554cdc,0x5984f3c9d12a886f,0xdbf2dc8b8b4dc62c,0x13544ea0756f1e52,])),
        field_new!(Fr,BigInteger([0x3143068f358657d9,0xd2884887574cfec4,0x6771d946a01d9ba,0x1ce6165523ee56e3,])),
        field_new!(Fr,BigInteger([0x7c56376c7c49d8fb,0x4bb497eaecf25f94,0x900676807703d160,0x3983839a9fa907eb,])),
        field_new!(Fr,BigInteger([0x5962f38ae6258169,0x3636dfd3338ecf98,0x15a21dc9ef0b7069,0x3dbdfb167eadf9d6,])),
        field_new!(Fr,BigInteger([0xc1bf4a7451f40c0d,0x7d6c04fcaba2bc10,0x9efea209f61ff3a5,0x2b7e866b1e3486b5,])),
        field_new!(Fr,BigInteger([0xfbf25fef5e70191a,0x3ea2facb4e33fbf1,0xde3883ca4737c10d,0x128605fc3aa70821,])),
        field_new!(Fr,BigInteger([0xe4e730382838e84e,0xa339aef76b143312,0xdfb55dafcad4eead,0x3b57e837ae88e30a,])),
        field_new!(Fr,BigInteger([0x899cc5ce432282ac,0xa40e3a240d681196,0xd7123ac7f4bbe80,0x3dbbdb06f786c87a,])),
        field_new!(Fr,BigInteger([0x3603ae0a30c4e0ab,0x5fb0069dc23c4e0c,0x40a0b4367d1e1940,0xf76071a11359b34,])),
        field_new!(Fr,BigInteger([0xb2f5de4238440b3f,0x2df163160875eb5c,0x138986efd82b9bf9,0xc04833310c446b7,])),
        field_new!(Fr,BigInteger([0x7fb78f69bb24da10,0xa0dcdac828496beb,0x73473024f29ed6c,0x2c427c31ebb40280,])),
        field_new!(Fr,BigInteger([0x9b047a9aed6b285b,0xe7edb85af1244f99,0x4d5951ee0a4d944c,0x1ba6aab4f1f6dda7,])),
        field_new!(Fr,BigInteger([0x3ac3876bc3af6824,0x7ed62f6843cfe5bb,0x52ca0242d0afd07e,0x28bb2ebe7801bf3,])),
        field_new!(Fr,BigInteger([0xb2ec58c3fdb06bfb,0xd2124576148ab023,0x45e45cf3920e7a5f,0x1fa6736497b2ec7d,])),
        field_new!(Fr,BigInteger([0x683fbe5ba3701159,0x328d9a42a438cc89,0x10379ca4e7235ef8,0x1ec08556c43de00a,])),
        field_new!(Fr,BigInteger([0xfa98e83104c4281,0x78a7b989ceb63e6f,0x60e7a3401a353e4b,0x3a078be8189ac774,])),
        field_new!(Fr,BigInteger([0x21c40b74fcf0f0ff,0xf4331168872abf24,0x1e3858d5c495a389,0x2f1d896ba58b68fa,])),
        field_new!(Fr,BigInteger([0x8ec1cf148008e452,0x7a9341e95edd3d25,0x2ba8ef7c8dc857de,0x1a9b993468c6de97,])),
        field_new!(Fr,BigInteger([0x240d302dc52be2a,0x727cd6a5b639c1e7,0x1d77e1844488a06f,0x180d136543d0830d,])),
        field_new!(Fr,BigInteger([0xdbec2d05d799f1fa,0x57de7a4308c59f0e,0xa546dcf352194bd2,0x2f41451801c55d15,])),
        field_new!(Fr,BigInteger([0x5a6c23347fb73855,0xe435e2a4919f6741,0x5a8f0f4e3fb3fd48,0xfaea39d9f9ccd99,])),
        field_new!(Fr,BigInteger([0xb608ac024a19526b,0xc66db73c8de7a6ec,0x1410e5990760f215,0x2a04c75ce368123,])),
        field_new!(Fr,BigInteger([0x5dd1ec60ca5c4617,0x42f946e8ce1c9dd3,0xcc106a7eefbaa2f3,0x29982299d1ae0609,])),
        field_new!(Fr,BigInteger([0xef89c5de6185a1bc,0x729621e91c955a34,0xf80c8d198f1a55cd,0x246fdff7fca32b0f,])),
        field_new!(Fr,BigInteger([0xa437f9c38f5d3aa6,0x36026d6082870a81,0xada88b6442c914,0x7e35821968aa3f7,])),
        field_new!(Fr,BigInteger([0x468c722d26e6cc2f,0x2308f0e5395bfd20,0x8a5462e0bcca01cf,0x2c494a8af1252391,])),
        field_new!(Fr,BigInteger([0x9d570b88634578bf,0xf230b4402ac38819,0x18ffd5d42abbda4b,0x3df4128c07f339e7,])),
        field_new!(Fr,BigInteger([0xdf9a8ebc79ed3ca1,0xc0b9ce370274144c,0x8305ab76c01f900b,0x2aca620b1d284876,])),
        field_new!(Fr,BigInteger([0xcee0177dc89e8b79,0xd4c2b4a9b5419721,0xd367325278cc7f8a,0x356443ae1f0c10e0,])),
        field_new!(Fr,BigInteger([0x6f0ba77c0ffdd52f,0x138ba73e76a99e8a,0x8fc163237cf24127,0x197c71ca636246ef,])),
        field_new!(Fr,BigInteger([0x2821aeb46f75005b,0x2b8a1579e26d66db,0x4ed63ae3a9f04713,0x3ecdf3895ab694bb,])),
        field_new!(Fr,BigInteger([0x4e1c8067a64047b4,0xb0d76895f9d49a93,0xa8baefb3f95ca250,0x255b57dd4beb9c97,])),
        field_new!(Fr,BigInteger([0x652fcabcef1e5880,0x4e0d5e21bd34817,0x5919495683c909c9,0x3a48a1d8e0c1ac49,])),
        field_new!(Fr,BigInteger([0x5eb4c07f344eb756,0x13b503408e43fa77,0xaa3f79e6b2a21f58,0x39cd92dc336d98f4,])),
        field_new!(Fr,BigInteger([0xbabf83f00ba73378,0x58b409a005b39442,0xaf29495cde3171d0,0x39d4fddfe7a495fd,])),
        field_new!(Fr,BigInteger([0x88aa71ff4748eae7,0x15dcf8529e1b6e72,0x9dccb71d7792352f,0x29162325d6f93ddb,])),
        field_new!(Fr,BigInteger([0x58acb61d9baa4321,0x85c77bffa34019fc,0x25c55410de450ca3,0x373b0cb2d2b4ac16,])),
        field_new!(Fr,BigInteger([0x33a41d77a7608a21,0x4481ac32eb1027b9,0x1724325035373431,0x3b30aa0324f469cf,])),
        field_new!(Fr,BigInteger([0xce2c196c128ab161,0x7fcb3bf93a8233ef,0xe6a4e7d30e51b75,0xf5105739e560354,])),
        field_new!(Fr,BigInteger([0xb0ce75473b9c4756,0x4c327642dbc3e75f,0xe3ef959238048022,0x5f9cd07fb1b8439,])),
        field_new!(Fr,BigInteger([0xb24396c283123a29,0x116e5579eed619be,0x5d0f913ba5c91d75,0x19afb9d6135204cd,])),
        field_new!(Fr,BigInteger([0x38aa8cc700a2809d,0xbeb96eba49709a2a,0x372ac244e3ee29c3,0x2d7f802d1fa5f51d,])),
        field_new!(Fr,BigInteger([0xc3f4536bd1fca99d,0x4844f4e1be1b90d7,0x412a6aa7a9e84517,0x21e53326dbfbffbf,])),
        field_new!(Fr,BigInteger([0xf1385daff2a2f8ff,0xa4a80f1ddabbc58d,0x920706dd6fe726b0,0xe720ae572c3e8f,])),
        field_new!(Fr,BigInteger([0x395d623573714aeb,0xa7809c21b198a83,0x6f17eaabaf874dc3,0x2386323fefc3d0ee,])),
        field_new!(Fr,BigInteger([0xc4ca6a67617f9d9e,0x5921adbddcd1b3c1,0xdddb4bf80d1425a1,0x8c1f9e7337e332d,])),
        field_new!(Fr,BigInteger([0x18ae91f714426b6b,0xbcb7e69b5dbc1b8d,0x478f7087437efd7b,0x1f0e5d8508265b58,])),
        field_new!(Fr,BigInteger([0x2072743b5ed4d4e2,0x98cc3ebb858ce950,0x8d3edbd0198f8abf,0x1fff876354f7e8b9,])),
        field_new!(Fr,BigInteger([0x8c3d8559db6171f2,0x155f3df1ef7f27ab,0xf8bff91f29054e8,0x3afa4f78c62f4c27,])),
        field_new!(Fr,BigInteger([0xe07e955f891dd11d,0x9b3d78bedef1980d,0xd892d3929006398d,0x10b1f7cc5b24f2a8,])),
        field_new!(Fr,BigInteger([0xcb42a29a7699ac02,0x1ab5af42ef8615a7,0xb63959c4aba00405,0x32af39352a3b653,])),
        field_new!(Fr,BigInteger([0x43e8bbdba7be6c56,0xe0cb740d7ef4aa85,0x32da6558373c5eb3,0x24e230f9a0f7b47b,])),
        field_new!(Fr,BigInteger([0x61b487e28794a309,0x78d6a8f0ac2fe1d4,0xd930bba6ca33b916,0x225f7481b01be62c,])),
        field_new!(Fr,BigInteger([0xdf2dfdd71c9b2219,0x683f835d1f5f5358,0xa30e95f9c86c417b,0xe5031e83b6ce98b,])),
        field_new!(Fr,BigInteger([0x11baf726f345d1b0,0x7448d7020694b66b,0xb215793ab845ad64,0x1390dea862255442,])),
        field_new!(Fr,BigInteger([0x5c3b10e7e8b76d23,0x683d97736a642ee1,0x3c9d71ac14cfa293,0x11771bd30622685d,])),
        field_new!(Fr,BigInteger([0x5241adbb730f7c1c,0x54dd3ea4e2d99abc,0x252349664178cfeb,0x35680d800c726b0,])),
        field_new!(Fr,BigInteger([0x24a4c9059e0664a3,0x16966cf8daebdd3d,0xf79952fa5f5d66b0,0x1c5fd4ac6dc3a82d,])),
        field_new!(Fr,BigInteger([0x159db5003cafe2dd,0xed947b30c1c4c5f5,0x4f1efc23572fd36,0x33e175fcecb32ef,])),
        field_new!(Fr,BigInteger([0x7167bfbb4ec88831,0x2192e7554c53bc9,0xdc2c734c34c847ef,0x3a0c56c757ffd17e,])),
        field_new!(Fr,BigInteger([0x4be153e4b781ccfb,0x482b9b003141437a,0xad0a61ce105d2377,0x3da50efd2fbe3547,])),
        field_new!(Fr,BigInteger([0x37fe98a457b4d383,0xb84afb053ed9ccbf,0xc812b03968f051c5,0x3440b797f43d3fd7,])),
        field_new!(Fr,BigInteger([0x6e65d3bbd01ec57c,0x59bb8720a8017a3b,0x61ee625cfa32fc7b,0x2058eb99d35bd8c2,])),
        field_new!(Fr,BigInteger([0x4fb52802b9a65ef1,0x7ab6a7c8d938b810,0x22fd912083ea64cc,0x8951291638300dd,])),
        field_new!(Fr,BigInteger([0x5c86c97f3a293b3,0xfdd511e4b386a858,0xd96b567ced51dd86,0xd007d439444ee0d,])),
        field_new!(Fr,BigInteger([0x25e231c9c268a448,0x82211fcee85324dd,0xe5dc9e4d574104b,0x349f388ff4b39804,])),
        field_new!(Fr,BigInteger([0xc04e9aab4caea7a5,0x29323bc7287a055f,0x380ec44a8d950a11,0x8edc2b0f4dd49fb,])),
        field_new!(Fr,BigInteger([0xa4ec4b979c36ca86,0x3d04e4888ab2bc1,0x65d0e4a41702b77b,0x21a54769a20ad5a5,])),
        field_new!(Fr,BigInteger([0xe43a76d502a5dcf3,0x3d7d516ade3d4d85,0xf1a192ef8fb46ed9,0x16b82615513f3e,])),
        field_new!(Fr,BigInteger([0x1f27e6946eb3d237,0x90ea64c64592bb87,0xae18c2871965853b,0x95ed0bcc2552da8,])),
        field_new!(Fr,BigInteger([0xee0e74467719bd5c,0x71178d708d070d2a,0x1d5221ea476ddab1,0x3f4544eb857e6191,])),
        field_new!(Fr,BigInteger([0xe5b4c57255a42c98,0x60f5059d98537d1a,0x779592cb513f1037,0x5fb02fcd2bf22b2,])),
        field_new!(Fr,BigInteger([0x34afcb4d83bb75b0,0xe8b9a17e67c535a2,0x162a08bea063ad7f,0x173521cf52c3f703,])),
        field_new!(Fr,BigInteger([0x111fe0618da68686,0x72f18a8d1016a031,0x21573c0d83d8449d,0x1a1b2ddda60b2864,])),
        field_new!(Fr,BigInteger([0xecaeee5299feead7,0xaa2ac57af25fcf32,0x8aaed58c5c612676,0x278d19038a95a4ce,])),
        field_new!(Fr,BigInteger([0x4fd5a37933de40b2,0x48c3e4e86e6e293d,0x2bc19accce69855,0x3c2522dbcacc59ae,])),
        field_new!(Fr,BigInteger([0x900407bb54697e8b,0x8cda43619dac3e50,0xaa61a4853d1eb224,0x2dbbc58fc1045d3b,])),
        field_new!(Fr,BigInteger([0x4d319547aa417a31,0xbe595b394222cbf8,0x4810d1221885d698,0x3927befead0d255f,])),
        field_new!(Fr,BigInteger([0x6ef1931f91cc693e,0xbfd985c2d5a000b5,0x435ec5d0f7025e5a,0x1b6aadca0975c95e,])),
        field_new!(Fr,BigInteger([0x568719957fc6d3ea,0xdeab08bd7666e2a7,0xbefeaad6482ff79e,0x3b76bd6838874f64,])),
        field_new!(Fr,BigInteger([0xb545b477f84a9e35,0x6ace83435105609f,0xe06a154db6abcadf,0xaca980d1168ec6f,])),
        field_new!(Fr,BigInteger([0x1dc2bb904e3dc01a,0x7e8dbfc1655bcaab,0x25620195822cb50c,0x1e25bb92541cafcb,])),
        field_new!(Fr,BigInteger([0x13618fb6c1d1b02c,0x9b6067028fd1a00b,0x9789e13f148a731b,0x23d0fcbcf6ae6ff5,])),
        field_new!(Fr,BigInteger([0xa81e57dd6825855a,0x4ce86237606e33f1,0xf16a3a3f3188995,0x3bdd80b309dd37b1,])),
        field_new!(Fr,BigInteger([0xc85beb683624f94c,0x6f4252ccce0a7dcc,0xfc3378ed2cddbc89,0x372ed965844dc975,])),
        field_new!(Fr,BigInteger([0x57e21f1f7c543558,0x5488a759ec2ef3da,0x60885a9a7dc16ee1,0x39d019452439c6d4,])),
        field_new!(Fr,BigInteger([0xf82a8816dd040e44,0x95e825ac7adb359b,0x5b04e3a5845e1611,0x62354abd118cb9c,])),
        field_new!(Fr,BigInteger([0x5fe01ca6a8d4ab0c,0x7d80471e1674d1ce,0x72216b91c704740a,0x2056e8352f496387,])),
        field_new!(Fr,BigInteger([0x307d2640247e39c0,0x6be654cb1497f2a4,0xd453ca99df6af5b4,0x32ce7fcc0649737d,])),
        field_new!(Fr,BigInteger([0x2227b48e8b87838,0x9f02324c92f7c5e5,0x3ad9aad80fe64ed,0x2b06ce250895635d,])),
        field_new!(Fr,BigInteger([0xe33d93c6dc374234,0x4be10d1976c46cbe,0xcaaa44b7e3db976d,0x56cb6118685b436,])),
        field_new!(Fr,BigInteger([0xfb022765684373d4,0xaa65d769b1c7326d,0x1615e1b8394b938d,0x2b51b83c2ae929a0,])),
        field_new!(Fr,BigInteger([0xcecc85a407f6b66f,0x15ea963585bb4902,0xbc46166254b25807,0x2994c0eebd765a08,])),
        field_new!(Fr,BigInteger([0xf118c2aca29b807f,0x90d173ca245d1e93,0x5a51b0eeb97ec1a8,0x3cde6ce85af570e9,])),
        field_new!(Fr,BigInteger([0xd164861a1998b5de,0xe29b4e25c55e8dc,0x313ef23ae01c8eca,0x3f2964f0fbb924e3,])),
        field_new!(Fr,BigInteger([0x527b83ec285477b3,0xa57dbd1546a72bdf,0xabed7bec6ae182dc,0x250f1283efb3402e,])),
        field_new!(Fr,BigInteger([0x1a1f0d16e2ad5558,0x9913ecff1f996cec,0xcab6e3fe0c7ff1a7,0x32629665a97a0736,])),
        field_new!(Fr,BigInteger([0x3ffcc30c3ce947ad,0x43e76b15dbcd7b63,0x7094b67e3b9ceb7e,0xdac5c94e50cb421,])),
        field_new!(Fr,BigInteger([0x506bd32ba9a41489,0xecdf0e10e4f49c21,0x4b547abb2ac41851,0x2cb618b912b3275b,])),
        field_new!(Fr,BigInteger([0x8bfb64bfed59a43d,0xad01893d081d4a6b,0x625ea0318fb7b20,0x39b29446a805b481,])),
        field_new!(Fr,BigInteger([0x790e8f0fad9e8023,0x20d178d7d71a9cb5,0x86e9807744c00c5b,0x369aa6516d15022a,])),
        field_new!(Fr,BigInteger([0x5d236755881b6034,0xbff7a7d0de6eb989,0x41a3a6fac1592ae4,0x222fe06074625307,])),
        field_new!(Fr,BigInteger([0x4aeb6b34bc223516,0xef266585b07894af,0x5609ba88ed579aee,0xbb4f4c48be0cdd4,])),
        field_new!(Fr,BigInteger([0x54e9913fecea53e2,0xb0eb64c208568f6b,0x50c2d29fce02db3e,0x21d7131de1e1a5d3,])),
        field_new!(Fr,BigInteger([0x736a579502cda13a,0xaecf0a297d2cbd69,0x2c868f8e57238ca9,0xe821e4908b566c9,])),
        field_new!(Fr,BigInteger([0xedc6f03c4c5f083c,0x9ebb9f9ad7e6ed78,0x22777c2a480f6214,0x2e22941434a6506,])),
        field_new!(Fr,BigInteger([0x184e084e85f85d1f,0x190449a7dde4c2c1,0xe8ad81171857b7b1,0x186076659f3abd36,])),
        field_new!(Fr,BigInteger([0xa9bacbf9c4c65188,0xb52fd9e3080a9f6e,0x8428a1dc6e6bd3ee,0x373eda6df718f452,])),
        field_new!(Fr,BigInteger([0x2972b908b89cca5f,0x5bae7f65f450acab,0x52254ec09db43b3d,0x154a84f5e57d386a,])),
        field_new!(Fr,BigInteger([0xd54af914ac566405,0xc6852d2c81e91329,0x20650b0a5ca0764e,0xfd0742d537fed3f,])),
        field_new!(Fr,BigInteger([0x1c3749c7c2c5287d,0xd4bcf2b93cb992d8,0x358b6d7f6ae4583d,0x3fd85a692d050055,])),
        field_new!(Fr,BigInteger([0x607e31eb73a3a386,0x6ca076126d546ea,0x9ac086eaf45d7f3f,0x1cb8c8eeb2ba03f,])),
        field_new!(Fr,BigInteger([0x793b8ef2ecd39cdc,0xc4e0ef88d7a70e2d,0x8bd423711d21dc10,0x2868e5830cd05303,])),
        field_new!(Fr,BigInteger([0x806fd2e7e9f5163f,0xff971fcb05f2ab2b,0xb4e9eaa384a12a5b,0xd587f09ad218e2b,])),
        field_new!(Fr,BigInteger([0xc54eb1aa8981f94a,0x1cd5f841cd632951,0xd5f377b55b8e76a0,0x25fa7cac45ce43fe,])),
        field_new!(Fr,BigInteger([0x3e08c772d914cd28,0x935c3aad62487c58,0xa1ef643a2f5d82a9,0x32d4a044a4c30eb6,])),
        field_new!(Fr,BigInteger([0xc1bdb46862179331,0x32d662df696d345f,0x5b813b6f10eaabf0,0x32a2d759af2ab145,])),
        field_new!(Fr,BigInteger([0x81f15d1941e37eaf,0x85d465ade1fdaa9a,0xc742919021a71c8d,0x2b55fd86318a8a64,])),
        field_new!(Fr,BigInteger([0xd192ca1f2b2d9b2e,0xadaff0f2a200dd4e,0x9c916850d1b82ee7,0xc80e142a869ffa8,])),
        field_new!(Fr,BigInteger([0xf1cb558c36627467,0x7f91625550390817,0x6ab1bf431c34b292,0x1728fc3415dc43ff,])),
        field_new!(Fr,BigInteger([0x28daae31dedcd4f0,0x1f3c06c8b85b04dc,0x8a78a38a2b6d3d64,0x1cb6422e7277e29,])),
        field_new!(Fr,BigInteger([0x4e413ace6b75a4f3,0x9cc21be3f8aaef09,0xe0d1fe8d244095e7,0x25ae2e0fa0d16f05,])),
        field_new!(Fr,BigInteger([0xf18c6c5c70f0b16c,0xca1ce4c4ed58dde2,0xb9eebeae9f9b79da,0x288db0b85a834294,])),
        field_new!(Fr,BigInteger([0x408abb99bfceac0c,0x5c93e60a53d96d93,0x19eeb51561642058,0x23e057c909b4e7f,])),
        field_new!(Fr,BigInteger([0x959eee5939fa1d8d,0xa5ff39f32da04b94,0xb3d943403ab22524,0x5786ff2bc6634e8,])),
        field_new!(Fr,BigInteger([0xce4a6a9ed5c8be2f,0x4e91726d3a6165da,0xb9764561d8c0a61c,0x37fd4ba55133cb50,])),
        field_new!(Fr,BigInteger([0x30f7ac315201424e,0xf48df3350d1bb102,0xb44bd6fc8af28d5a,0x22362ab1e657b9a2,])),
        field_new!(Fr,BigInteger([0xbee5eb66070b29b5,0x876fa788634aaef0,0xcb31f2fe228c8caa,0x58fa540d0e81bbc,])),
        field_new!(Fr,BigInteger([0x20e02f2c84960d63,0x407397d2cd20e853,0xf111f6b11760f99b,0x2d2b58eed5623103,])),
        field_new!(Fr,BigInteger([0xe527a73a5cdb4e81,0xc1720d1b51f2182f,0x40d50001a5d1255b,0x3f929202e89e3433,])),
        field_new!(Fr,BigInteger([0x409848273fe1c7be,0x144ab08444211029,0x8a590797aca79994,0x22ba45bbc0e9b9f8,])),
        field_new!(Fr,BigInteger([0xcf2f8d8ad5b04792,0x95679d53d91ee4ea,0xed849b08ad6ca24c,0x21eae5337d2a5df8,])),
        field_new!(Fr,BigInteger([0xa9171cbcf537e388,0x8dcc539fab02c19f,0xd060595ad18dd32f,0x3c24dd140ba9714e,])),
        field_new!(Fr,BigInteger([0xafde8f05b7844422,0x56f3153a5924e1ba,0xc3ed59c72a2fdfa3,0x13e1243401fef48e,])),
        field_new!(Fr,BigInteger([0x51b23dd6592db68,0x8ec00ede32811417,0x78886dda3aca7d8a,0x353eca04fa8ee369,])),
        field_new!(Fr,BigInteger([0x800c229a1e14eb30,0x26de2bd2e5ab7287,0xf916231941575d5c,0x2b06b5f500362a3b,])),
        field_new!(Fr,BigInteger([0x60464d7ef79fe789,0x8199995aa96441ca,0x629c1a42b02cd714,0xd46c254408f437,])),
        field_new!(Fr,BigInteger([0xd83d818d1d5f7b,0xea3394b5eda41f71,0xb6e82bb0ba4d6cc2,0x2e418c94923b665d,])),
        field_new!(Fr,BigInteger([0xd713edb83238e015,0xda6cd708e2510972,0xff024b5398f41da,0xc5bc2c83d4ddd1c,])),
        field_new!(Fr,BigInteger([0xf546196560d08f11,0xd876ebb1b0f37609,0x7a0637e14cf66af8,0x7b96c1fce1a8c6b,])),
        field_new!(Fr,BigInteger([0xe657d91aa5e2276e,0x4ffdbfc4da2dd176,0xc8a99f1671106de5,0x1da55bb86670e7e5,])),
        field_new!(Fr,BigInteger([0xe7b37627ef49db09,0x8e87d458b42d6b53,0x27b36ee6f601cdb1,0x36b2bc604961c208,])),
        field_new!(Fr,BigInteger([0x77c590bd7cfe0788,0x99ec8f62fae2c79c,0xd37d21cce509ee88,0x2e01b20390ff227c,])),
        field_new!(Fr,BigInteger([0xf0e6625100d16470,0x29cd5b26a2297d4f,0xc519c471d87c7845,0x2cd220ee9a9ca3f3,])),
        field_new!(Fr,BigInteger([0x3288e8d42d7051de,0x5940b5fc34fd4c4c,0x598fad9caba6d447,0x2d36b62734d1e219,])),
        field_new!(Fr,BigInteger([0x53e04377615a99d,0xd83121cbf4205acc,0x53b34762460e3c33,0x3aaa4ce4a6f388ab,])),
    ];

    // The MDS matrix constants
    const MDS_CST: &'static [Fr] = &[
        // Constants in Montgomery representation 
        field_new!(Fr,BigInteger([0x507c7c3801b065f4,0x55fad147f6ed8180,0xd824a2b1bf437c06,0x1dbf404c94728386,])),
        field_new!(Fr,BigInteger([0x706e8fa3b2cec138,0x99388f0ebd78e15,0xfaad8b4043083408,0xb25966789f42f5c,])),
        field_new!(Fr,BigInteger([0x6dfd051c6ee53b7f,0x1763b22c11853d87,0x67ea6c399e3b51ba,0xf16c3ae0f454f9,])),
        field_new!(Fr,BigInteger([0xfb30fbf09660ae7d,0xfb8e9f44f0253a7b,0x188d045b94fb48c0,0x303cc8c8367fc508,])),
        field_new!(Fr,BigInteger([0x6677cf4973709ad6,0x6140d5ad5d36bb65,0x5c78685811d582fb,0x1c493d93a2f4c8c5,])),
        field_new!(Fr,BigInteger([0xb591cd839a7494ef,0xd119192e17ee213f,0xfd146e4ff6037865,0x2ac07a9b7e18dc5e,])),
        field_new!(Fr,BigInteger([0x40985a4c90087b8d,0x3bbff3a8fa6519,0xb829e164c293b46d,0x23f178572c4f5003,])),
        field_new!(Fr,BigInteger([0x1bda396a735997f,0xb071a1ceecefb0a5,0xdd21e9b4ebfa39f6,0x448e9117e96ab82,])),
        field_new!(Fr,BigInteger([0xf8bfeb926376c2e4,0xbfe9ff853fbe09d7,0x95ee2f2bc6f81adf,0x29278abf7b87b845,])),
    ];
}

pub type FrQuinticSbox = PoseidonQuinticSBox<Fr, FrPoseidonParameters>;
pub type FrPoseidonHash = PoseidonHash<Fr, FrPoseidonParameters, FrQuinticSbox>;
pub type FrBatchPoseidonHash = PoseidonBatchHash<Fr, FrPoseidonParameters, FrQuinticSbox>;
