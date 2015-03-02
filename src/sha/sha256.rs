
/// Please fix std::simd::u32x4
#[simd]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct u32x4(u32, u32, u32, u32);

/// The `Sha256` public frontend to a SHA-1 message digest algorithm.
#[derive(Clone, Debug)]
pub struct Sha256([u32; 8], Vec<u8>);

impl Sha256 {
    
    /// Construct a `Sha256` object with the given state.
    pub fn new(state: &[u32; 8]) -> Sha256 {
        Sha256(*state, Vec::new())
    }
}

pub mod consts {
    use super::u32x4;
    
    pub const BLOCK_LEN: usize = 16; // words, i.e. [u32; 16]
    pub const STATE_LEN: usize = 8;  // words, i.e. [u32; 8]

    pub const H: [u32; STATE_LEN] = [
        0x6a09e667, // floor(mod(sqrt(2), 1)*2**32)
        0xbb67ae85, // floor(mod(sqrt(3), 1)*2**32)
        0x3c6ef372, // floor(mod(sqrt(5), 1)*2**32)
        0xa54ff53a, // floor(mod(sqrt(7), 1)*2**32)
        0x510e527f, // floor(mod(sqrt(11), 1)*2**32)
        0x9b05688c, // floor(mod(sqrt(13), 1)*2**32)
        0x1f83d9ab, // floor(mod(sqrt(17), 1)*2**32)
        0x5be0cd19, // floor(mod(sqrt(19), 1)*2**32)
    ];

    /// Constants used by Intel SHA-256 instruction set extensions.
    pub const H_U32X4: (u32x4, u32x4) = (
        u32x4(H[2], H[3], H[6], H[7]),
        u32x4(H[0], H[1], H[4], H[5]));

    pub const K: [u32; 64] = [
        0x428a2f98, // floor(mod(cbrt(2), 1)*2**32)
        0x71374491, // floor(mod(cbrt(3), 1)*2**32)
        0xb5c0fbcf, // floor(mod(cbrt(5), 1)*2**32)
        0xe9b5dba5, // floor(mod(cbrt(7), 1)*2**32)
        0x3956c25b, // floor(mod(cbrt(11), 1)*2**32)
        0x59f111f1, // floor(mod(cbrt(13), 1)*2**32)
        0x923f82a4, // floor(mod(cbrt(17), 1)*2**32)
        0xab1c5ed5, // floor(mod(cbrt(19), 1)*2**32)
        0xd807aa98, // floor(mod(cbrt(23), 1)*2**32)
        0x12835b01, // floor(mod(cbrt(29), 1)*2**32)
        0x243185be, // floor(mod(cbrt(31), 1)*2**32)
        0x550c7dc3, // floor(mod(cbrt(37), 1)*2**32)
        0x72be5d74, // floor(mod(cbrt(41), 1)*2**32)
        0x80deb1fe, // floor(mod(cbrt(43), 1)*2**32)
        0x9bdc06a7, // floor(mod(cbrt(47), 1)*2**32)
        0xc19bf174, // floor(mod(cbrt(53), 1)*2**32)
        0xe49b69c1, // floor(mod(cbrt(59), 1)*2**32)
        0xefbe4786, // floor(mod(cbrt(61), 1)*2**32)
        0x0fc19dc6, // floor(mod(cbrt(67), 1)*2**32)
        0x240ca1cc, // floor(mod(cbrt(71), 1)*2**32)
        0x2de92c6f, // floor(mod(cbrt(73), 1)*2**32)
        0x4a7484aa, // floor(mod(cbrt(79), 1)*2**32)
        0x5cb0a9dc, // floor(mod(cbrt(83), 1)*2**32)
        0x76f988da, // floor(mod(cbrt(89), 1)*2**32)
        0x983e5152, // floor(mod(cbrt(97), 1)*2**32)
        0xa831c66d, // floor(mod(cbrt(101), 1)*2**32)
        0xb00327c8, // floor(mod(cbrt(103), 1)*2**32)
        0xbf597fc7, // floor(mod(cbrt(107), 1)*2**32)
        0xc6e00bf3, // floor(mod(cbrt(109), 1)*2**32)
        0xd5a79147, // floor(mod(cbrt(113), 1)*2**32)
        0x06ca6351, // floor(mod(cbrt(127), 1)*2**32)
        0x14292967, // floor(mod(cbrt(131), 1)*2**32)
        0x27b70a85, // floor(mod(cbrt(137), 1)*2**32)
        0x2e1b2138, // floor(mod(cbrt(139), 1)*2**32)
        0x4d2c6dfc, // floor(mod(cbrt(149), 1)*2**32)
        0x53380d13, // floor(mod(cbrt(151), 1)*2**32)
        0x650a7354, // floor(mod(cbrt(157), 1)*2**32)
        0x766a0abb, // floor(mod(cbrt(163), 1)*2**32)
        0x81c2c92e, // floor(mod(cbrt(167), 1)*2**32)
        0x92722c85, // floor(mod(cbrt(173), 1)*2**32)
        0xa2bfe8a1, // floor(mod(cbrt(179), 1)*2**32)
        0xa81a664b, // floor(mod(cbrt(181), 1)*2**32)
        0xc24b8b70, // floor(mod(cbrt(191), 1)*2**32)
        0xc76c51a3, // floor(mod(cbrt(193), 1)*2**32)
        0xd192e819, // floor(mod(cbrt(197), 1)*2**32)
        0xd6990624, // floor(mod(cbrt(199), 1)*2**32)
        0xf40e3585, // floor(mod(cbrt(211), 1)*2**32)
        0x106aa070, // floor(mod(cbrt(223), 1)*2**32)
        0x19a4c116, // floor(mod(cbrt(227), 1)*2**32)
        0x1e376c08, // floor(mod(cbrt(229), 1)*2**32)
        0x2748774c, // floor(mod(cbrt(233), 1)*2**32)
        0x34b0bcb5, // floor(mod(cbrt(239), 1)*2**32)
        0x391c0cb3, // floor(mod(cbrt(241), 1)*2**32)
        0x4ed8aa4a, // floor(mod(cbrt(251), 1)*2**32)
        0x5b9cca4f, // floor(mod(cbrt(257), 1)*2**32)
        0x682e6ff3, // floor(mod(cbrt(263), 1)*2**32)
        0x748f82ee, // floor(mod(cbrt(269), 1)*2**32)
        0x78a5636f, // floor(mod(cbrt(271), 1)*2**32)
        0x84c87814, // floor(mod(cbrt(277), 1)*2**32)
        0x8cc70208, // floor(mod(cbrt(281), 1)*2**32)
        0x90befffa, // floor(mod(cbrt(283), 1)*2**32)
        0xa4506ceb, // floor(mod(cbrt(293), 1)*2**32)
        0xbef9a3f7, // floor(mod(cbrt(307), 1)*2**32)
        0xc67178f2, // floor(mod(cbrt(311), 1)*2**32)
    ];

    /// Constants used by Intel SHA-256 instruction set extensions.
    pub const K_U32X4: [u32x4; 16] = [
        u32x4(K[3], K[2], K[1], K[0]),
        u32x4(K[7], K[6], K[5], K[4]),
        u32x4(K[11], K[10], K[9], K[8]),
        u32x4(K[15], K[14], K[13], K[12]),
        u32x4(K[19], K[18], K[17], K[16]),
        u32x4(K[23], K[22], K[21], K[20]),
        u32x4(K[27], K[26], K[25], K[24]),
        u32x4(K[31], K[30], K[29], K[28]),
        u32x4(K[35], K[34], K[33], K[32]),
        u32x4(K[39], K[38], K[37], K[36]),
        u32x4(K[43], K[42], K[41], K[40]),
        u32x4(K[47], K[46], K[45], K[44]),
        u32x4(K[51], K[50], K[49], K[48]),
        u32x4(K[55], K[54], K[53], K[52]),
        u32x4(K[59], K[58], K[57], K[56]),
        u32x4(K[63], K[62], K[61], K[60]),
    ];
}

pub mod ops {
    use bswap::beu32;
    use std::num::Int;
    use std::io::prelude::*;
    use std::iter::{Unfold, IteratorExt};
    use utils::StdPad;
    use super::u32x4;

    #[inline]
    fn swap(v0: u32x4) -> u32x4 {
        u32x4(v0.2, v0.3, v0.0, v0.1)
    }

    #[inline]
    fn load(v0: u32x4, v1: u32x4) -> u32x4 {
        u32x4(v1.3, v0.0, v0.1, v0.2)
    }

    /// Emulates `llvm.x86.sha256msg1` intrinsic.
    #[inline]
    pub fn expand_step1(v0: u32x4, v1: u32x4) -> u32x4 {
        
        macro_rules! sigma0x4 {
            ($a:expr) => 
            (((($a >> u32x4( 7,  7,  7,  7)) | ($a << u32x4(25, 25, 25, 25))) ^
              (($a >> u32x4(18, 18, 18, 18)) | ($a << u32x4(14, 14, 14, 14))) ^
               ($a >> u32x4( 3,  3,  3,  3))))
        }
        
        v0 + sigma0x4!(load(v0, v1))
    }

    /// Emulates `llvm.x86.sha256msg2` intrinsic.
    #[inline]
    pub fn expand_step2(v4: u32x4, v3: u32x4) -> u32x4 {
        
        macro_rules! sigma1 {
            ($a:expr) => (($a.rotate_right(17) ^ $a.rotate_right(19) ^ ($a >> 10)))
        }
        
        let u32x4(x3, x2, x1, x0) = v4;
        let u32x4(w15, w14, _, _) = v3;
        let w16 = x0 + sigma1!(w14);
        let w17 = x1 + sigma1!(w15);
        let w18 = x2 + sigma1!(w16);
        let w19 = x3 + sigma1!(w17);
        u32x4(w19, w18, w17, w16)
    }

    /// Combines the SHA-256 message schedule intrinsics into one function.
    #[inline]
    pub fn expand_round_x4(work: &[u32x4]) -> u32x4 {
        expand_step2(expand_step1(work[0], work[1]) + load(work[2], work[3]), work[3])
    }

    /// Emulates `llvm.x86.sha256rnds2` intrinsic.
    #[inline]
    pub fn digest_round_x2(state: (u32x4, u32x4), work: u32x4) -> u32x4 {
        let (u32x4(mut c, mut d, mut g, mut h), u32x4(a, b, e, f)) = state;
        let u32x4(_, _, wk1, wk0) = work;
        
        macro_rules! big_sigma0 {
            ($a:expr) => (($a.rotate_right(2) ^ $a.rotate_right(13) ^ $a.rotate_right(22)))
        }
        macro_rules! big_sigma1 {
            ($a:expr) => (($a.rotate_right(6) ^ $a.rotate_right(11) ^ $a.rotate_right(25)))
        }
        macro_rules! bool3ary_202 {
            ($a:expr, $b:expr, $c:expr) => (($c ^ ($a & ($b ^ $c))))
        }
        macro_rules! bool3ary_232 {
            ($a:expr, $b:expr, $c:expr) => (($a & $b) ^ ($a & $c) ^ ($b & $c))
        }
        
        // 2 rounds
        h += big_sigma1!(e) + bool3ary_202!(e, f, g) + wk0; d += h;
        h += big_sigma0!(a) + bool3ary_232!(a, b, c);
        g += big_sigma1!(d) + bool3ary_202!(d, e, f) + wk1; c += g;
        g += big_sigma0!(h) + bool3ary_232!(h, a, b);
        
        u32x4(g, h, c, d)
    }

    /// Performs 4 digest rounds.
    #[inline]
    pub fn digest_round_x4(state: (u32x4, u32x4), work: u32x4) -> (u32x4, u32x4) {
        let (mut cdgh, mut abef) = state;
        
        // 4 rounds
        cdgh = digest_round_x2((cdgh, abef), work);
        abef = digest_round_x2((abef, cdgh), swap(work));
        
        (cdgh, abef)
    }

    #[inline]
    fn expand_copy(work: &mut [u32x4; 4], buf: &[u8]) {
        for (i, chunk) in buf.chunks(16).enumerate() {
            work[i] = u32x4(beu32::decode(&chunk[12..16]),
                            beu32::decode(&chunk[8..12]),
                            beu32::decode(&chunk[4..8]),
                            beu32::decode(&chunk[..4]));
        }
    }
    
    #[inline]
    fn expand_next(work: &mut [u32x4; 4]) -> u32x4 {
        let item = expand_round_x4(&work[..]);
        work[0] = work[1];
        work[1] = work[2];
        work[2] = work[3];
        work[3] = item;
        work[3]
    }
    
    /// Process a block with the SHA-2 SHA-256 algorithm.
    ///
    /// Internally, this uses functions which resemble the new Intel SHA instruction sets,
    /// and so it's data locality properties may improve performance. However, to benefit
    /// the most from this implementation, replace these functions with x86 intrinsics to
    /// get a possible speed boost.
    pub fn digest_block_u32x4(state: (u32x4, u32x4), buf: &[u8]) -> (u32x4, u32x4) {
        use super::consts::K_U32X4;
        assert_eq!(buf.len(), 64);
        let mut work = [u32x4(0, 0, 0, 0); 4];
        expand_copy(&mut work, buf);
        
        // Decode message buffer.
        let state2 = work.iter().cloned()

            // Expand message schedule.
            .chain(Unfold::new(work, |w: &mut [u32x4; 4]| Some(expand_next(w))))

            // Add SHA-256 round constants.
            .enumerate().map(|(i, work): (usize, u32x4)| K_U32X4[i] + work)
            
            // Perform 64 digest rounds.
            .take(16).fold(state, digest_round_x4);

        // Accumulate state
        (state.0 + state2.0,
         state.1 + state2.1)
    }
    
    pub fn digest_block_u32(state: [u32; 8], buf: &[u8]) -> [u32; 8] {
        let [a, b, c, d, e, f, g, h] = state;
        let mut state4 = (u32x4(c, d, g, h), u32x4(a, b, e, f));
        state4 = digest_block_u32x4(state4, buf);
        let (u32x4(c, d, g, h), u32x4(a, b, e, f)) = state4;
        [a, b, c, d, e, f, g, h]
    }
    
    pub fn digest_u32x4(buf: &[u8]) -> (u32x4, u32x4) {
        let mut pad_buf = [0u8; 128];
        let pad_len = StdPad::new(buf.len())
            .read(&mut pad_buf[..]).unwrap();
        let pad = &pad_buf[..pad_len];
        
	    buf
            // Pad the message to a multiple of 64 bytes
            .iter().cloned().chain(pad.iter().cloned()).collect::<Vec<u8>>()
            
            // Digest these blocks of 64 bytes.
            .chunks(64).fold(super::consts::H_U32X4, digest_block_u32x4)
    }
    
    pub fn digest_u32(buf: &[u8]) -> [u32; 8] {
        let mut pad_buf = [0u8; 128];
        let pad_len = StdPad::new(buf.len())
            .read(&mut pad_buf[..]).unwrap();
        let pad = &pad_buf[..pad_len];
        
	    buf
            // Pad the message to a multiple of 64 bytes
            .iter().cloned().chain(pad.iter().cloned()).collect::<Vec<u8>>()
            
            // Digest these blocks of 64 bytes.
            .chunks(64).fold(super::consts::H, digest_block_u32)
    }
}

pub mod impls {
    use std::default::Default;
    use std::ffi::IntoBytes;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::beu32;
    use serialize::hex::ToHex;
    use utils::{Reset, Digest, DigestExt};
    use super::ops;
    use super::{
        Sha256,
    };
    
    impl Default for Sha256 {

        /// Construct a default `Sha256` object.
        fn default() -> Sha256 {
            Sha256::new(&super::consts::H)
        }
    }

    impl Reset for Sha256 {

        /// Reset the state
        fn reset(&mut self) {
            let state = super::consts::H;
            self.0 = state;
        }
    }

    impl Read for Sha256 {

        /// Read state as big-endian
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            beu32::encode_slice(buf, &self.0[..]);
            Ok(buf.len())
        }
    }

    impl Write for Sha256 {
        
        /// Write to the buffer
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Write::write(&mut self.1, buf)
        }
        
        /// Digest the buffer
        fn flush(&mut self) -> io::Result<()> {
            self.0 = ops::digest_u32(&self.1[..]);
            Ok(())
        }
    }
    
    impl Hasher for Sha256 {

        /// Get the first 8 bytes of the state
        fn finish(&self) -> u64 {
            ((self.0[0] as u64) << 32u64) |
             (self.0[1] as u64)
        }

        /// Write to buffer
        fn write(&mut self, buf: &[u8]) {
            Write::write(self, buf).unwrap();
        }
    }
    
    impl IntoBytes for Sha256 {
        fn into_bytes(mut self) -> Vec<u8> {
            let mut bytes = vec![0u8; 32];
            (&mut self).read(&mut bytes[..]).unwrap();
            bytes
        }
    }
    
    impl ToHex for Sha256 {
        fn to_hex(&self) -> String {
            self.clone().into_bytes().as_slice().to_hex()
        }
    }

    impl Digest for Sha256 {}
    impl DigestExt for Sha256 {
        fn default_len() -> usize {
            32
        }
    }
}

#[cfg(test)]
pub mod tests {
    use test::Bencher;
    use serialize::hex::ToHex;
    use std::default::Default;
    use std::io::prelude::*;
    use super::Sha256;
    use super::consts::{
        BLOCK_LEN,
        STATE_LEN,
    };

    /// Simplify state parameter
    pub fn digest_block(state: &mut [u32; 8], buf: &[u8]) {
        *state = super::ops::digest_block_u32(*state, buf);
    }
    
    /// Simplify state parameter
    pub fn digest(state: &mut [u32; 8], buf: &[u8]) -> Sha256 {
        let mut h = Sha256::new(state);
        Write::write_all(&mut h, buf).unwrap();
        Write::flush(&mut h).unwrap();
        *state = h.0;
        h
    }

    // Common entry point for tests
    pub fn digest_to_bytes(buf: &[u8]) -> Vec<u8> {
        let mut h: Sha256 = Default::default();
        Write::write_all(&mut h, buf).unwrap();
        Write::flush(&mut h).unwrap();

        // Serialize
        let mut bytes = vec![0u8; STATE_LEN*4];
        h.read(&mut bytes[..]).unwrap();
        bytes
    }

    // Common entry point for tests
    pub fn digest_to_hex(msg: &str) -> String {
        digest_to_bytes(&msg.as_bytes()).as_slice().to_hex()
    }

    //
    // Tests for `hash`
    //

    #[test]
    fn sha256_empty_hash() {
        use std::default::Default;
        use std::hash::{Hash, Hasher};
        use std::io::Write;

        let mut hasher: Sha256 = Default::default();
        let msg: &[u8] = "".as_bytes();
        <u8 as Hash>::hash_slice::<Sha256>(msg, &mut hasher);
        hasher.flush().unwrap(); // important!
        let digest: u64 = hasher.finish();

        assert_eq!(0xe3b0c44298fc1c14u64, digest);
    }

    #[test]
    fn sha256_hello_hash() {
        use std::default::Default;
        use std::hash::{Hash, Hasher};
        use std::io::Write;

        let mut hasher: Sha256 = Default::default();
        let msg: &[u8] = "hello world".as_bytes();
        <u8 as Hash>::hash_slice::<Sha256>(msg, &mut hasher);
        hasher.flush().unwrap(); // important!
        let digest: u64 = hasher.finish();

        assert_eq!(0xb94d27b9934d3e08u64, digest);
    }

    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn sha256_hello() {

        assert_eq!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
                   digest_to_hex("hello world").as_slice());

        assert_eq!("7509e5bda0c762d2bac7f90d758b5b2263fa01ccbc542ab5e3df163be08e6ca9",
                   digest_to_hex("hello world!").as_slice());

        assert_eq!("db4067cec62c58bf8b2f8982071e77c082da9e00924bf3631f3b024fa54e7d7e",
                   digest_to_hex("hello World").as_slice());

        assert_eq!("e4ad0102dc2523443333d808b91a989b71c2439d7362aca6538d49f76baaa5ca",
                   digest_to_hex("hello World!").as_slice());

        assert_eq!("64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c",
                   digest_to_hex("Hello world").as_slice());

        assert_eq!("c0535e4be2b79ffd93291305436bf889314e4a3faec05ecffcbb7df31ad9e51a",
                   digest_to_hex("Hello world!").as_slice());

        assert_eq!("a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e",
                   digest_to_hex("Hello World").as_slice());

        assert_eq!("7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069",
                   digest_to_hex("Hello World!").as_slice());

        assert_eq!("09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b",
                   digest_to_hex("hello, world").as_slice());

        assert_eq!("68e656b251e67e8358bef8483ab0d51c6619f3e7a1a9f0e75838d41ff368f728",
                   digest_to_hex("hello, world!").as_slice());

        assert_eq!("211f927b277d1e8feeae2d929912b87ecdfbb3b6155833ccb438710d1694682d",
                   digest_to_hex("hello, World").as_slice());

        assert_eq!("04aa5d2533987c34839e8dbc8d8fcac86f0137e31c1c6ea4349ade4fcaf87ed8",
                   digest_to_hex("hello, World!").as_slice());

        assert_eq!("4ae7c3b6ac0beff671efa8cf57386151c06e58ca53a78d83f36107316cec125f",
                   digest_to_hex("Hello, world").as_slice());

        assert_eq!("315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3",
                   digest_to_hex("Hello, world!").as_slice());

        assert_eq!("03675ac53ff9cd1535ccc7dfcdfa2c458c5218371f418dc136f2d19ac1fbe8a5",
                   digest_to_hex("Hello, World").as_slice());

        assert_eq!("dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f",
                   digest_to_hex("Hello, World!").as_slice());
    }

    #[test]
    fn sha256_empty() {

        assert_eq!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                   digest_to_hex("").as_slice());
    }

    #[test]
    fn sha256_multi_block() {
        let s = "GNU LESSER GENERAL PUBLIC LICENSE Version 3, 29 June 2007 Copyright (C) 2007 Free Software Foundation, Inc. <http://fsf.org/>";
        assert_eq!("d33d14f2ea60beb394082598e05375cdd6ff8966315322c34b6faea80e7d5a7c", digest_to_hex(s).as_slice());
    }

    #[test]
    fn sha256_is_hex_ok_1k() {
        let s = "                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        ";
        assert_eq!("08c9b52f61fadf1eff6fb89169f1735fbae7bb583b23cb119d0e1a0151bac952", digest_to_hex(s).as_slice());
    }

    //
    // Tests for `digest_to_bytes`
    //

    #[test]
    fn sha256_hello_bytes() {
        let bytes = digest_to_bytes("hello world".as_bytes());

        assert_eq!(b"\xb9\x4d\x27\xb9\x93\x4d\x3e\x08\xa5\x2e\x52\xd7\xda\x7d\xab\xfa\xc4\x84\xef\xe3\x7a\x53\x80\xee\x90\x88\xf7\xac\xe2\xef\xcd\xe9",
                   bytes.as_slice());
    }

    //
    // Tests for `digest`
    //

    #[test]
    fn sha256_hello_digest() {
        let mut words = [0u32; STATE_LEN];
        digest(&mut words, "hello world".as_bytes());

        assert_eq!(words[0], 0xb94d27b9u32);
        assert_eq!(words[1], 0x934d3e08u32);
        assert_eq!(words[2], 0xa52e52d7u32);
        assert_eq!(words[3], 0xda7dabfau32);
        assert_eq!(words[4], 0xc484efe3u32);
        assert_eq!(words[5], 0x7a5380eeu32);
        assert_eq!(words[6], 0x9088f7acu32);
        assert_eq!(words[7], 0xe2efcde9u32);
    }

    //
    // Tests for `digest_block`
    //

    fn make_empty_block() -> Vec<u8> {
        let mut block = vec![0u8; BLOCK_LEN*4];
        assert_eq!(block.len(), BLOCK_LEN*4);
        block[0] = 0x80u8;
        block
    }

    fn make_hello_block() -> Vec<u8> {

        // this could use a concat_bytes!
        static HELLO_BLOCK: &'static [u8] = b"hello world\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x58";

        let mut block: Vec<u8> = Vec::with_capacity(BLOCK_LEN*4);
        unsafe { block.set_len(BLOCK_LEN*4) };
        (&mut block[..]).clone_from_slice(HELLO_BLOCK);
        assert_eq!(block.len(), BLOCK_LEN*4);
        block
    }

    #[test]
    fn sha256_empty_block() {
        use serialize::hex::ToHex;
        use endian::ReadEndian;
        let mut state = [0u32; STATE_LEN];
        (&mut state[..]).clone_from_slice(&super::consts::H[..]);
        let block_vec = make_empty_block();
        let block = &block_vec[..];
        digest_block(&mut state, &block[..]);
        let mut bytes = vec![0u8; STATE_LEN*4];
        (&state[..]).read_be(&mut bytes[..]).unwrap();

        assert_eq!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                   bytes.as_slice().to_hex());
    }

    #[test]
    fn sha256_hello_block() {
        use serialize::hex::ToHex;
        use endian::ReadEndian;
        let mut state = [0u32; STATE_LEN];
        (&mut state[..]).clone_from_slice(&super::consts::H[..]);
        let block_vec = make_hello_block();
        let block = &block_vec[..];
        digest_block(&mut state, &block[..]);
        let mut bytes = vec![0u8; STATE_LEN*4];
        (&state[..]).read_be(&mut bytes[..]).unwrap();

        assert_eq!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
                   bytes.as_slice().to_hex());
    }

    //
    // Benchmarks for `digest_bytes`
    //

    #[bench]
    fn sha256_hello_blocks(b: & mut Bencher) {
        let mut state = [0u32; STATE_LEN];
        (&mut state[..]).clone_from_slice(&super::consts::H[..]);
        let block_vec = make_hello_block();
        let block = &block_vec[..];
        b.iter( || { digest_block(&mut state, block) });
        b.bytes = 64u64;
    }

    #[bench]
    fn sha256_empty_blocks(b: & mut Bencher) {
        let mut state = [0u32; STATE_LEN];
        (&mut state[..]).clone_from_slice(&super::consts::H[..]);
        let block_vec = make_empty_block();
        let block = &block_vec[..];
        b.iter( || { digest_block(&mut state, block) });
        b.bytes = 64u64;
    }

    //
    // Benchmarks for `digest`
    //

    #[bench]
    fn sha256_10(b: & mut Bencher) {
        let mut state = [0u32; STATE_LEN];
        let buf = [0x20u8; 10];
        let msg = &buf[..];
        b.iter( || { digest(&mut state, msg); });
        b.bytes = msg.len() as u64;
    }
    #[bench]
    fn sha256_1k(b: & mut Bencher) {
        let mut state = [0u32; STATE_LEN];
        let buf = [0x20u8; 1024];
        let msg = &buf[..];
        b.iter( || { digest(&mut state, msg); });
        b.bytes = msg.len() as u64;
    }
    #[bench]
    fn sha256_64k(b: & mut Bencher) {
        let mut state = [0u32; STATE_LEN];
        let buf = [0x20u8; 65536];
        let msg = &buf[..];
        b.iter( || { digest(&mut state, msg); });
        b.bytes = msg.len() as u64;
    }

    //
    // Benchmarks for `digest_to_bytes`
    //

    #[bench]
    fn sha256_to_bytes_10(b: & mut Bencher) {
        let buf = [0x20u8; 10];
        let msg = &buf[..];
        b.iter( || { digest_to_bytes(msg); });
        b.bytes = msg.len() as u64;
    }
    //#[bench]
    //fn sha256_to_bytes_1k(b: & mut Bencher) {
    //    let buf = [0x20u8; 1024];
    //    let msg = &buf[..];
    //    b.iter( || { digest_to_bytes(msg); });
    //    b.bytes = msg.len() as u64;
    //}
    //#[bench]
    //fn sha256_to_bytes_64k(b: & mut Bencher) {
    //    let buf = [0x20u8; 65536];
    //    let msg = &buf[..];
    //    b.iter( || { digest_to_bytes(msg); });
    //    b.bytes = msg.len() as u64;
    //}

    //
    // Benchmarks for `digest_to_hex`
    //

    #[bench]
    fn sha256_to_hex_10(b: & mut Bencher) {
        let buf = [0x20u8; 10];
        let msg = ::std::str::from_utf8(&buf[..]).unwrap();
        b.iter( || { digest_to_hex(msg); });
        b.bytes = msg.len() as u64;
    }
    //#[bench]
    //fn sha256_to_hex_1k(b: & mut Bencher) {
    //    let buf = [0x20u8; 1024];
    //    let msg = ::std::str::from_utf8(&buf[..]).unwrap();
    //    b.iter( || { digest_to_hex(msg); });
    //    b.bytes = msg.len() as u64;
    //}
    //#[bench]
    //fn sha256_to_hex_64k(b: & mut Bencher) {
    //    let buf = [0x20u8; 65536];
    //    let msg = ::std::str::from_utf8(&buf[..]).unwrap();
    //    b.iter( || { digest_to_hex(msg); });
    //    b.bytes = msg.len() as u64;
    //}
}
