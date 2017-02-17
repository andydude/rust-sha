/// TODO: docs
#[derive(Clone)]
pub struct Sha256([u32; 8], Vec<u8>);

mod impls {
    use std::borrow::Borrow;
    use std::default::Default;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::{beu32, beu64};
    use super::Sha256;
    use utils::{Reset,
                Digest,
                DigestExt,
                ReadPadBlocksExt,
                StdPad};

    impl Default for Sha256 {

        /// Construct a default `Sha256` object.
        fn default() -> Sha256 {
            Sha256(super::consts::H, Vec::new())
        }
    }

    impl Reset for Sha256 {

        /// Reset the state
        fn reset(&mut self) {
            self.0 = super::consts::H;
            self.1.clear();
        }
    }

    impl Write for Sha256 {

        /// Write to buffer
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Write::write(&mut self.1, buf)
        }

        /// Digest buffer
        fn flush(&mut self) -> io::Result<()> {
            let mut state = self.0;
            let ref buf = self.1;

            fn pad(len: usize) -> StdPad {
                let mut suffix = vec![0u8; 8];
                beu64::encode(&mut suffix[..], 8*len as u64);
                StdPad::new(suffix, 64)
            }

            for block in buf.pad_blocks(64, |len: usize| pad(len)) {
                super::ops::digest_block(&mut state, block.borrow());
            }

            self.0 = state;
            Ok(())
        }
    }

    impl Read for Sha256 {

        /// Read state as big-endian
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let state_buf = &self.0[..buf.len()/4];
            beu32::encode_slice(buf, state_buf);
            Ok(buf.len())
        }
    }

    impl Hasher for Sha256 {

        /// Get the first 8 bytes of the state
        fn finish(&self) -> u64 {
            let mut h = self.clone();
            h.flush().unwrap();
            let state = h.0;

            ((state[0] as u64) << 32u64) |
             (state[1] as u64)
        }

        /// Write to buffer
        fn write(&mut self, buf: &[u8]) {
            Write::write(self, buf).unwrap();
        }
    }

    impl Digest for Sha256 {}

    impl DigestExt for Sha256 {
        fn default_len() -> usize {
            return 32;
        }
    }
}

/// TODO
//#[unstable(feature="default", reason="TODO")]
pub mod consts {

    /// TODO
    pub const H: [u32; 8] = [
        0x6a09e667,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19
    ];

    /// TODO
    pub const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
        0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
        0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
        0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
    ];
}

/// TODO: docs
pub mod ops {
    use bswap::beu32;

    macro_rules! rotate_right {
        ($a:expr, $b:expr) => ((($a >> $b) ^ ($a << (32 - $b))))
    }
    macro_rules! sigma0 {
        ($a:expr) => ((rotate_right!($a, 7) ^ rotate_right!($a, 18) ^ ($a >> 3)))
    }
    macro_rules! sigma1 {
        ($a:expr) => ((rotate_right!($a, 17) ^ rotate_right!($a, 19) ^ ($a >> 10)))
    }
    macro_rules! big_sigma0 {
        ($a:expr) => ((rotate_right!($a, 2) ^ rotate_right!($a, 13) ^ rotate_right!($a, 22)))
    }
    macro_rules! big_sigma1 {
        ($a:expr) => ((rotate_right!($a, 6) ^ rotate_right!($a, 11) ^ rotate_right!($a, 25)))
    }
    macro_rules! bool3ary_202 {
        ($a:expr, $b:expr, $c:expr) => (($c ^ ($a & ($b ^ $c))))
    }
    macro_rules! bool3ary_232 {
        ($a:expr, $b:expr, $c:expr) => (($a & $b) ^ ($a & $c) ^ ($b & $c))
    }

    macro_rules! sha256_expand_round {
        ($work:expr, $t:expr) => {
            {
                let w = $work[($t + 0) & 15]
                    .wrapping_add(sigma1!($work[($t + 14) & 15]))
                    .wrapping_add(sigma0!($work[($t + 1) & 15]))
                    .wrapping_add($work[($t + 9) & 15]);
                $work[($t + 0) & 15] = w;
                w
            }
        }
    }

    macro_rules! sha256_digest_round {
        ($a:ident, $b:ident, $c:ident, $d:ident,
         $e:ident, $f:ident, $g:ident, $h:ident,
         $k:expr, $w:expr) => {
            {
                $h = $h
                    .wrapping_add($k)
                    .wrapping_add($w)
                    .wrapping_add(big_sigma1!($e))
                    .wrapping_add(bool3ary_202!($e, $f, $g));
                $d = $d
                    .wrapping_add($h);
                $h = $h
                    .wrapping_add(big_sigma0!($a))
                    .wrapping_add(bool3ary_232!($a, $b, $c));
            }
        }
    }

    /// This function can be easily implemented with Intel SHA intruction set extensions.
    ///
    /// ```ignore
    /// {
    ///     let temp = sha256load(work[2], work[3]);
    ///     sha256msg2(sha256msg1(work[0], work[1]) + temp, work[3])
    /// }
    /// ```
    //#[inline]
    //pub fn expand_round_x4(w: &mut [u32; 16], t: usize) {
    //    use std::simd::u32x4;
    //
    //    // Not an intrinsic, but works like an unaligned load.
    //    #[inline]
    //    fn sha256load(v2: u32x4, v3: u32x4) -> u32x4 {
    //        u32x4(v3.3, v2.0, v2.1, v2.2)
    //    }
    //    // Not an intrinsic, but useful for swapping vectors.
    //    #[inline]
    //    fn sha256swap(v0: u32x4) -> u32x4 {
    //        u32x4(v0.2, v0.3, v0.0, v0.1)
    //    }
    //
    //    /// Emulates `llvm.x86.sha256msg1` intrinsic.
    //    #[inline]
    //    fn sha256msg1(v0: u32x4, v1: u32x4) -> u32x4 {
    //        // sigma 0 on vectors
    //        #[inline]
    //        fn sigma0x4(a: u32x4) -> u32x4 {
    //            ((a >> u32x4( 7,  7,  7,  7)) | (a << u32x4(25, 25, 25, 25))) ^
    //            ((a >> u32x4(18, 18, 18, 18)) | (a << u32x4(14, 14, 14, 14))) ^
    //             (a >> u32x4( 3,  3,  3,  3))
    //        }
    //        v0 + sigma0x4(sha256load(v0, v1))
    //    }
    //
    //    /// Emulates `llvm.x86.sha256msg2` intrinsic.
    //    #[inline]
    //    fn sha256msg2(v4: u32x4, v3: u32x4) -> u32x4 {
    //        macro_rules! sigma1 {
    //            ($a:expr) => ((rotate_right!($a, 17) ^ rotate_right!($a, 19) ^ ($a >> 10)))
    //        }
    //        macro_rules! sigma1 {
    //            ($a:expr) => ((rotate_right!($a, 17) ^ rotate_right!($a, 19) ^ ($a >> 10)))
    //        }
    //        #[inline]
    //        fn sigma1x4(x: u32x4) -> u32x4 {
    //            ((a >> u32x4(17, 17, 17, 17)) | (a << u32x4(15, 15, 15, 15))) ^
    //            ((a >> u32x4(19, 19, 19, 19)) | (a << u32x4(13, 13, 13, 13))) ^
    //             (a >> u32x4(10, 10, 10, 10))
    //        }
    //
    //        let u32x4(w15, w14, _, _) = v3;
    //        let u32x4(x3, x2, x1, x0) = v4;
    //        let v5 = u32x4(x1, x0, w15, w14);
    //        let v6 = u32x4(w15, w14, 0, 0);
    //        
    //        v4 + sigma1x4(v5 + sigma1x4(v6))
    //    }
    //
    //    let w0 = u32x4(w[(t + 3) & 15], w[(t + 2) & 15], w[(t + 1) & 15], w[(t + 0) & 15]);
    //    let w4 = u32x4(w[(t + 7) & 15], w[(t + 6) & 15], w[(t + 5) & 15], w[(t + 4) & 15]);
    //    let w8 = u32x4(w[(t + 11) & 15], w[(t + 10) & 15], w[(t + 9) & 15], w[(t + 8) & 15]);
    //    let w12 = u32x4(w[(t + 15) & 15], w[(t + 14) & 15], w[(t + 13) & 15], w[(t + 12) & 15]);
    //
    //    let ret = sha256msg2(sha256msg1(w0, w4) + sha256load(w8, w12), w12);
    //
    //
    //}
    #[inline]
    pub fn expand_round_x4(w: &mut [u32; 16], t: usize) {
        sha256_expand_round!(w, t);
        sha256_expand_round!(w, t + 1);
        sha256_expand_round!(w, t + 2);
        sha256_expand_round!(w, t + 3);
    }

    /// This function can be easily implemented with Intel SHA intruction set extensions.
    ///
    /// ```ignore
    /// {
    ///     let abef = u32x4(a, b, e, f);
    ///     let cdgh = u32x4(c, d, g, h);
    ///
    ///     cdgh = sha256rnds2(cdgh, abef, work);
    ///     abef = sha256rnds2(abef, cdgh, sha256swap(work));
    ///
    ///     a = abef.0;
    ///     b = abef.1;
    ///     c = cdgh.0;
    ///     d = cdgh.1;
    ///     e = abef.2;
    ///     f = abef.3;
    ///     g = cdgh.2;
    ///     h = cdgh.3;
    /// }
    /// ```
    #[inline]
    pub fn digest_round_x4(state: &mut [u32; 8], k: [u32; 4], w: [u32; 4]) {
        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];
        let mut e = state[4];
        let mut f = state[5];
        let mut g = state[6];
        let mut h = state[7];
        sha256_digest_round!(a, b, c, d, e, f, g, h, k[0], w[0]);
        sha256_digest_round!(h, a, b, c, d, e, f, g, k[1], w[1]);
        sha256_digest_round!(g, h, a, b, c, d, e, f, k[2], w[2]);
        sha256_digest_round!(f, g, h, a, b, c, d, e, k[3], w[3]);
        *state = [e, f, g, h, a, b, c, d];
    }

    #[inline]
    pub fn expand_round_x16(w: &mut [u32; 16]) {
        expand_round_x4(w, 0);
        expand_round_x4(w, 4);
        expand_round_x4(w, 8);
        expand_round_x4(w, 12);
    }


    #[inline]
    pub fn digest_round_x16(state: &mut [u32; 8], k: [u32; 16], w: [u32; 16]) {
        macro_rules! as_simd {
            ($x:expr) => {
                {
                    let (y, _): (&[u32; 4], usize) =
                        unsafe {::std::mem::transmute($x)}; *y
                }
            }
        }

        digest_round_x4(state, as_simd!(&k[0..4]), as_simd!(&w[0..4]));
        digest_round_x4(state, as_simd!(&k[4..8]), as_simd!(&w[4..8]));
        digest_round_x4(state, as_simd!(&k[8..12]), as_simd!(&w[8..12]));
        digest_round_x4(state, as_simd!(&k[12..16]), as_simd!(&w[12..16]));
    }

    /// TODO
    pub fn digest_block(state: &mut [u32; 8], buf: &[u8]) {
        use std::mem::transmute;
        use super::consts::K;
        let state2 = *state;
        let mut w: [u32; 16] = [0; 16];

        macro_rules! as_simd_array {
            ($x:expr) => {{let (y, _): (&[u32; 16], usize) = unsafe {transmute($x)}; *y}}
        }

        beu32::decode_slice(&mut w[..], buf);
        digest_round_x16(state, as_simd_array!(&K[0..16]), w);
        expand_round_x16(&mut w);
        digest_round_x16(state, as_simd_array!(&K[16..32]), w);
        expand_round_x16(&mut w);
        digest_round_x16(state, as_simd_array!(&K[32..48]), w);
        expand_round_x16(&mut w);
        digest_round_x16(state, as_simd_array!(&K[48..64]), w);

        //digest_round_x4(state, as_simd!(&K[0..4]), as_simd!(&w[0..4]));
        //digest_round_x4(state, as_simd!(&K[4..8]), as_simd!(&w[4..8]));
        //digest_round_x4(state, as_simd!(&K[8..12]), as_simd!(&w[8..12]));
        //digest_round_x4(state, as_simd!(&K[12..16]), as_simd!(&w[12..16]));
        //
        //for r in 1..4 {
        //    expand_round_x4(&mut w, 0);
        //    digest_round_x4(state, as_simd!(&K[16*r..16*r + 4]), as_simd!(&w[0..4]));
        //    expand_round_x4(&mut w, 4);
        //    digest_round_x4(state, as_simd!(&K[16*r + 4..16*r + 8]), as_simd!(&w[4..8]));
        //    expand_round_x4(&mut w, 8);
        //    digest_round_x4(state, as_simd!(&K[16*r + 8..16*r + 12]), as_simd!(&w[8..12]));
        //    expand_round_x4(&mut w, 12);
        //    digest_round_x4(state, as_simd!(&K[16*r + 12..16*r + 16]), as_simd!(&w[12..16]));
        //}

        for i in 0..8 {
            state[i] = state[i]
                .wrapping_add(state2[i]);
        }
    }

    /// TODO
    pub fn digest(buf: &[u8]) -> [u32; 8] {
        use std::default::Default;
        use utils::Digest;

        super::Sha256::default().digest(buf).0
    }
}

#[cfg(test)]
mod tests {
    use std::str;
    use std::default::Default;
    use std::io::prelude::*;
    use bswap::beu32;
    use test::Bencher;
    use super::Sha256;
    use utils::{Reset, Digest, DigestExt};

    //
    // Helper functions
    //

    fn digest_block(state: &mut [u32; 8], buf: &[u8]) {
        super::ops::digest_block(state, buf);
    }

    fn digest(buf: &[u8]) -> Sha256 {
        let mut h: Sha256 = Default::default();
        h.digest(buf);
        h
    }

    fn digest_to_bytes(buf: &[u8]) -> Vec<u8> {
        digest(buf).to_bytes()
    }

    fn digest_to_hex(msg: &str) -> String {
        digest(&msg.as_bytes()).to_hex()
    }

    //
    // Tests for `hash`
    //

    #[test]
    fn test_sha256_empty_hash() {
        use std::hash::{Hash, Hasher};

        let msg: &[u8] = "".as_bytes();
        let mut h: Sha256 = Default::default();
        <u8 as Hash>::hash_slice::<Sha256>(msg, &mut h);
        let digest: u64 = h.finish();
        assert_eq!(0xe3b0c44298fc1c14u64, digest);
    }

    #[test]
    fn test_sha256_hello_hash() {
        use std::hash::{Hash, Hasher};

        let msg: &[u8] = "hello world".as_bytes();
        let mut h: Sha256 = Default::default();
        <u8 as Hash>::hash_slice::<Sha256>(msg, &mut h);
        let digest: u64 = h.finish();
        assert_eq!(0xb94d27b9934d3e08u64, digest);
    }

    //
    // Tests for `digest_to_hex`
    //

    //#[test]
    //fn test_sha256_empty() {
    //
    //    assert_eq!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    //               digest_to_hex("") as &str);
    //}

    #[test]
    fn test_sha256_hello() {

        assert_eq!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
                   digest_to_hex("hello world"));

        assert_eq!("7509e5bda0c762d2bac7f90d758b5b2263fa01ccbc542ab5e3df163be08e6ca9",
                   digest_to_hex("hello world!"));

        assert_eq!("db4067cec62c58bf8b2f8982071e77c082da9e00924bf3631f3b024fa54e7d7e",
                   digest_to_hex("hello World"));

        assert_eq!("e4ad0102dc2523443333d808b91a989b71c2439d7362aca6538d49f76baaa5ca",
                   digest_to_hex("hello World!"));

        assert_eq!("64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c",
                   digest_to_hex("Hello world"));

        assert_eq!("c0535e4be2b79ffd93291305436bf889314e4a3faec05ecffcbb7df31ad9e51a",
                   digest_to_hex("Hello world!"));

        assert_eq!("a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e",
                   digest_to_hex("Hello World"));

        assert_eq!("7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069",
                   digest_to_hex("Hello World!"));

        assert_eq!("09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b",
                   digest_to_hex("hello, world"));

        assert_eq!("68e656b251e67e8358bef8483ab0d51c6619f3e7a1a9f0e75838d41ff368f728",
                   digest_to_hex("hello, world!"));

        assert_eq!("211f927b277d1e8feeae2d929912b87ecdfbb3b6155833ccb438710d1694682d",
                   digest_to_hex("hello, World"));

        assert_eq!("04aa5d2533987c34839e8dbc8d8fcac86f0137e31c1c6ea4349ade4fcaf87ed8",
                   digest_to_hex("hello, World!"));

        assert_eq!("4ae7c3b6ac0beff671efa8cf57386151c06e58ca53a78d83f36107316cec125f",
                   digest_to_hex("Hello, world"));

        assert_eq!("315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3",
                   digest_to_hex("Hello, world!"));

        assert_eq!("03675ac53ff9cd1535ccc7dfcdfa2c458c5218371f418dc136f2d19ac1fbe8a5",
                   digest_to_hex("Hello, World"));

        assert_eq!("dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f",
                   digest_to_hex("Hello, World!"));
    }

    #[test]
    fn test_sha256_multi() {
        let s = "GNU LESSER GENERAL PUBLIC LICENSE Version 3, 29 June 2007 Copyright (C) 2007 Free Software Foundation, Inc. <http://fsf.org/>";
        assert_eq!("d33d14f2ea60beb394082598e05375cdd6ff8966315322c34b6faea80e7d5a7c", digest_to_hex(s));
    }

    #[test]
    fn test_sha256_1k() {
        let buf = [0x20; 1000];
        let msg = str::from_utf8(&buf[..]).unwrap();
        assert_eq!("08c9b52f61fadf1eff6fb89169f1735fbae7bb583b23cb119d0e1a0151bac952", digest_to_hex(msg));
    }

    //
    // Tests for `digest_to_bytes`
    //

    #[test]
    fn test_sha256_hello_bytes() {
        let bytes = digest_to_bytes("hello world".as_bytes());

        assert_eq!(b"\xb9\x4d\x27\xb9\x93\x4d\x3e\x08\xa5\x2e\x52\xd7\xda\x7d\xab\xfa\xc4\x84\xef\xe3\x7a\x53\x80\xee\x90\x88\xf7\xac\xe2\xef\xcd\xe9",
                   bytes);
    }

    //
    // Tests for `digest`
    //

    #[test]
    fn test_sha256_hello_digest() {
        let words: [u32; 8] = digest("hello world".as_bytes()).0;

        assert_eq!(words[0], 0xb94d27b9);
        assert_eq!(words[1], 0x934d3e08);
        assert_eq!(words[2], 0xa52e52d7);
        assert_eq!(words[3], 0xda7dabfa);
        assert_eq!(words[4], 0xc484efe3);
        assert_eq!(words[5], 0x7a5380ee);
        assert_eq!(words[6], 0x9088f7ac);
        assert_eq!(words[7], 0xe2efcde9);
    }

    //
    // Tests for `digest_block`
    //

    fn make_empty_block() -> Vec<u8> {
        let mut block = vec![0u8; 16*4];
        assert_eq!(block.len(), 16*4);
        block[0] = 0x80u8;
        block
    }

    fn make_hello_block() -> Vec<u8> {

        // this could use a concat_bytes!
        static HELLO_BLOCK: &'static [u8] = b"hello world\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x58";

        let mut block: Vec<u8> = Vec::with_capacity(16*4);
        unsafe { block.set_len(16*4) };
        (&mut block[..]).clone_from_slice(HELLO_BLOCK);
        assert_eq!(block.len(), 16*4);
        block
    }

    #[test]
    fn test_sha256_empty_block() {
        use bswap::u8::encode_hex;
        let mut state: [u32; 8] = [0; 8];
        (&mut state[..]).clone_from_slice(&super::consts::H[..]);
        let block_vec = make_empty_block();
        let block = &block_vec[..];
        digest_block(&mut state, &block[..]);
        let mut bytes = vec![0u8; 8*4];
        beu32::encode_slice(&mut bytes[..], &state[..]);
        assert_eq!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                   encode_hex(bytes.as_ref()));
    }

    #[test]
    fn test_sha256_hello_block() {
        use bswap::u8::encode_hex;
        let mut state: [u32; 8] = [0; 8];
        (&mut state[..]).clone_from_slice(&super::consts::H[..]);
        let block_vec = make_hello_block();
        let block = &block_vec[..];
        digest_block(&mut state, &block[..]);
        let mut bytes = vec![0u8; 8*4];
        beu32::encode_slice(&mut bytes[..], &state[..]);

        assert_eq!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
                   encode_hex(bytes.as_ref()));
    }

    #[bench]
    pub fn bench_sha256_block(bh: & mut Bencher) {
        let mut state: [u32; 8] = [0; 8];
        let bytes = [1u8; 16*4];
        let block = &bytes[..];
        bh.iter( || { digest_block(&mut state, block); });
        bh.bytes = 64u64;
    }

    //
    // Benchmarks for `digest_bytes`
    //

    #[bench]
    fn bench_sha256_hello_block(b: & mut Bencher) {
        let mut state: [u32; 8] = [0; 8];
        (&mut state[..]).clone_from_slice(&super::consts::H[..]);
        let block_vec = make_hello_block();
        let block = &block_vec[..];
        b.iter( || { digest_block(&mut state, block) });
        b.bytes = 64u64;
    }

    #[bench]
    fn bench_sha256_empty_block(b: & mut Bencher) {
        let mut state: [u32; 8] = [0; 8];
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
    fn bench_sha256_10(b: & mut Bencher) {
        let buf = [0x20u8; 10];
        b.iter( || { digest(&buf[..]); });
        b.bytes = buf.len() as u64;
    }
    #[bench]
    fn bench_sha256_1k(b: & mut Bencher) {
        let buf = [0x20u8; 1024];
        b.iter( || { digest(&buf[..]); });
        b.bytes = buf.len() as u64;
    }
    #[bench]
    fn bench_sha256_64k(b: & mut Bencher) {
        let buf = [0x20u8; 65536];
        b.iter( || { digest(&buf[..]); });
        b.bytes = buf.len() as u64;
    }

    //
    // Benchmarks for `digest_to_bytes`
    //

    #[bench]
    fn bench_sha256_to_bytes_10(b: & mut Bencher) {
        let buf = [0x20u8; 10];
        b.iter( || { digest_to_bytes(&buf[..]); });
        b.bytes = buf.len() as u64;
    }
    #[bench]
    fn bench_sha256_to_bytes_1k(b: & mut Bencher) {
        let buf = [0x20u8; 1024];
        b.iter( || { digest_to_bytes(&buf[..]); });
        b.bytes = buf.len() as u64;
    }
    #[bench]
    fn bench_sha256_to_bytes_64k(b: & mut Bencher) {
        let buf = [0x20u8; 65536];
        b.iter( || { digest_to_bytes(&buf[..]); });
        b.bytes = buf.len() as u64;
    }

    //
    // Benchmarks for `digest_to_hex`
    //

    #[bench]
    fn bench_sha256_to_hex_10(b: & mut Bencher) {
        let buf = [0x20u8; 10];
        let msg = ::std::str::from_utf8(&buf[..]).unwrap();
        b.iter( || { digest_to_hex(msg); });
        b.bytes = msg.len() as u64;
    }
    #[bench]
    fn bench_sha256_to_hex_1k(b: & mut Bencher) {
        let buf = [0x20u8; 1024];
        let msg = ::std::str::from_utf8(&buf[..]).unwrap();
        b.iter( || { digest_to_hex(msg); });
        b.bytes = msg.len() as u64;
    }
    #[bench]
    fn bench_sha256_to_hex_64k(b: & mut Bencher) {
        let buf = [0x20u8; 65536];
        let msg = ::std::str::from_utf8(&buf[..]).unwrap();
        b.iter( || { digest_to_hex(msg); });
        b.bytes = msg.len() as u64;
    }

    //
    // Test Structure
    //

    struct Test {
        input: &'static str,
        output_str: &'static str,
    }

    fn make_test_list() -> Vec<Test> {
        // Examples from wikipedia
        let wikipedia_tests = vec![
            Test {
                input: "",
                output_str: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            },
            Test {
                input: "The quick brown fox jumps over the lazy dog",
                output_str: "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"
            },
            Test {
                input: "The quick brown fox jumps over the lazy dog.",
                output_str: "ef537f25c895bfa782526529a9b63d97aa631564d5d789c2b765448c8635fb6c"
            },
        ];
        wikipedia_tests
    }

    #[test]
    fn test_sha256_wikipedia() {

        let tests = make_test_list();
        let mut h: Sha256 = Default::default();

        // Test that it works when accepting the message all at once
        for t in tests.iter() {
            let out_str = h.digest(t.input.as_bytes()).to_hex();
            assert_eq!(&out_str[..], t.output_str);
            h.reset();
        }

        // Test that it works when accepting the message in pieces
        for t in tests.iter() {
            let len = t.input.len();
            let mut left = len;

            while left > 0 {
                let take = (left + 1) / 2;
                h.write((&t.input[len - left..take + len - left]).as_bytes()).unwrap();
                left = left - take;
            }
            h.flush().unwrap();

            let out_str = h.to_hex();
            assert_eq!(&out_str[..], t.output_str);

            h.reset();
        }
    }

}
