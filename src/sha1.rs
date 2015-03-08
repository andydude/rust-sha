/// TODO: docs
#[derive(Clone)]
pub struct Sha1(pub [u32; 5], Vec<u8>);

mod impls {
    use std::default::Default;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::{beu32, beu64};
    use super::Sha1;
    use utils::{Reset,
                Digest,
                DigestExt,
                ReadPadBlocksExt,
                StdPad};

    impl Default for Sha1 {

        /// Construct a default `Sha1` object.
        fn default() -> Sha1 {
            Sha1(super::consts::H, Vec::new())
        }
    }

    impl Reset for Sha1 {

        /// (Step 0) Reset the state
        fn reset(&mut self) {
            self.0 = super::consts::H;
            self.1.clear();
        }
    }

    impl Write for Sha1 {

        /// (Step 1) Write to buffer
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
                super::ops::digest_block(&mut state, &block);
            }
            
            self.0 = state;
            Ok(())
        }
    }

    impl Read for Sha1 {

        /// (Step 4) Read state as big-endian
        ///
        /// The buffer length must be a multiple of 4.
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let state_buf = &self.0[..buf.len()/4];
            beu32::encode_slice(buf, state_buf);
            Ok(buf.len())
        }
    }

    impl Hasher for Sha1 {
    
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

    impl Digest for Sha1 {}

    impl DigestExt for Sha1 {
            fn default_len() -> usize {
            return 20;
        }
    }
}

/// TODO
#[unstable(feature="default", reason="TODO")]
pub mod consts {

    /// TODO
    pub const H: [u32; 5] = [
        0x67452301,
        0xefcdab89,
        0x98badcfe,
        0x10325476,
        0xc3d2e1f0];

    /// TODO
    pub const K: [u32; 4] = [
        0x5a827999,
        0x6ed9eba1,
        0x8f1bbcdc,
        0xca62c1d6];
}

/// TODO: docs
pub mod ops {
    use bswap::beu32;

    macro_rules! rotate_left {
        ($a:expr, $b:expr) => (($a << $b) ^ ($a >> (32 - $b)))
    }
    macro_rules! bool3ary_150 {
        ($a:expr, $b:expr, $c:expr) => (($a ^ $b ^ $c))
    }
    macro_rules! bool3ary_202 {
        ($a:expr, $b:expr, $c:expr) => (($c ^ ($a & ($b ^ $c))))
    }
    macro_rules! bool3ary_232 {
        ($a:expr, $b:expr, $c:expr) => (($a & $b) ^ ($a & $c) ^ ($b & $c))
    }
    
    macro_rules! round_func {
        ($a:expr, $b:expr, $c:expr, $i:expr) => {
            match $i {
                0 => bool3ary_202!($a, $b, $c),
                1 => bool3ary_150!($a, $b, $c),
                2 => bool3ary_232!($a, $b, $c),
                3 => bool3ary_150!($a, $b, $c),
                _ => panic!("unknown icosaround index")
            }
        }
    }
    
    macro_rules! sha1_expand_round {
        ($work:expr, $t:expr) => {
            {
                let temp = $work[($t + 4 + 0) % 20]
                    ^ $work[($t + 4 + 2) % 20]
                    ^ $work[($t + 4 + 8) % 20]
                    ^ $work[($t + 4 + 13) % 20];

                $work[($t + 0) % 20] = rotate_left!(temp, 1);
            }
        }
    }

    macro_rules! sha1_digest_round {
        ($a:ident, $b:ident, $c:ident, $d:ident,
         $e:ident, $w:expr, $i:expr) => {
            {
                use super::consts::K;
                
                $e = $e
                    .wrapping_add(K[$i])
                    .wrapping_add($w)
                    .wrapping_add(rotate_left!($a, 5))
                    .wrapping_add(round_func!($b, $c, $d, $i));

                $b = rotate_left!($b, 30);
            }
        }
    }

    /// This function can be easily implemented with Intel SHA intruction set extensions.
    ///
    /// ```ignore
    /// {
    ///     sha1msg2(sha1msg1(work[0], work[1]) ^ work[2], work[3])
    /// }
    /// ```
    #[inline]
    pub fn expand_round_x4(w: &mut [u32; 20], t: usize) {
        sha1_expand_round!(w, t);
        sha1_expand_round!(w, t + 1);
        sha1_expand_round!(w, t + 2);
        sha1_expand_round!(w, t + 3);
    }

    /// This function can be easily implemented with Intel SHA intruction set extensions.
    ///
    /// ```ignore
    /// {
    ///     let abcd = u32x4(a, b, c, d);
    ///     let e000 = u32x4(e, 0, 0, 0);
    ///
    ///     abcd = sha1rnds4(abcd, e000 + work, i);
    ///
    ///     e = a.rotate_left(30);
    ///     a = abcd.0;
    ///     b = abcd.1;
    ///     c = abcd.2;
    ///     d = abcd.3;
    /// }
    /// ```
    #[inline]
    pub fn digest_round_x4(state: &mut [u32; 5], w: [u32; 4], i: usize) {
        let [mut a, mut b, mut c, mut d, mut e] = *state;
        sha1_digest_round!(a, b, c, d, e, w[0], i);
        sha1_digest_round!(e, a, b, c, d, w[1], i);
        sha1_digest_round!(d, e, a, b, c, w[2], i);
        sha1_digest_round!(c, d, e, a, b, w[3], i);
        *state = [b, c, d, e, a];
    }

    #[inline]
    pub fn expand_round_x20(w: &mut [u32; 20]) {
        expand_round_x4(w, 0);
        expand_round_x4(w, 4);
        expand_round_x4(w, 8);
        expand_round_x4(w, 12);
        expand_round_x4(w, 16);
    }

    #[inline]
    pub fn digest_round_x20(state: &mut [u32; 5], w: [u32; 20], i: usize) {
        macro_rules! as_simd {
            ($x:expr) => {
                {
                    let (y, _): (&[u32; 4], usize) =
                        unsafe {::std::mem::transmute($x)}; *y
                }
            }
        }
        
        digest_round_x4(state, as_simd!(&w[0..4]), i);
        digest_round_x4(state, as_simd!(&w[4..8]), i);
        digest_round_x4(state, as_simd!(&w[8..12]), i);
        digest_round_x4(state, as_simd!(&w[12..16]), i);
        digest_round_x4(state, as_simd!(&w[16..20]), i);
    }

    /// TODO
    pub fn digest_block(state: &mut [u32; 5], buf: &[u8]) {
        let state2 = *state;
        let mut w: [u32; 20] = [0; 20];

        beu32::decode_slice(&mut w[..16], buf);
        expand_round_x4(&mut w, 16);
        digest_round_x20(state, w, 0);
        expand_round_x20(&mut w);
        digest_round_x20(state, w, 1);
        expand_round_x20(&mut w);
        digest_round_x20(state, w, 2);
        expand_round_x20(&mut w);
        digest_round_x20(state, w, 3);

        for i in 0..5 {
            state[i] = state[i]
                .wrapping_add(state2[i]);
        }
    }

    /// TODO
    pub fn digest(buf: &[u8]) -> [u32; 5] {
        use std::default::Default;
        use utils::Digest;
        
        super::Sha1::default().digest(buf).0
    }
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use std::io::prelude::*;
    use serialize::hex::ToHex;
    use test::Bencher;
    use super::Sha1;
    use utils::{Digest, DigestExt};
    
    //
    // Helper functions
    //

    fn digest_block(state: &mut [u32; 5], buf: &[u8]) {
        super::ops::digest_block(state, buf);
    }

    fn digest(buf: &[u8]) -> Sha1 {
        let mut h: Sha1 = Default::default();
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
    fn test_sha1_empty_hash() {
        use std::hash::{Hash, Hasher};

        let msg: &[u8] = "".as_bytes();
        let mut h: Sha1 = Default::default();
        <u8 as Hash>::hash_slice(msg, &mut h);
        let digest: u64 = h.finish();
        assert_eq!(0xda39a3ee5e6b4b0du64, digest);
    }

    #[test]
    fn test_sha1_hello_hash() {
        use std::hash::{Hash, Hasher};

        let msg: &[u8] = "hello world".as_bytes();
        let mut h: Sha1 = Default::default();
        <u8 as Hash>::hash_slice(msg, &mut h);
        let digest: u64 = h.finish();
        assert_eq!(0x2aae6c35c94fcfb4u64, digest);
    }

    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn test_sha1_empty() {
        assert_eq!("da39a3ee5e6b4b0d3255bfef95601890afd80709",
                   digest_to_hex("").as_slice());
    }

    #[test]
    fn test_sha1_hello() {
        assert_eq!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed",
                   digest_to_hex("hello world").as_slice());
        assert_eq!("430ce34d020724ed75a196dfc2ad67c77772d169",
                   digest_to_hex("hello world!").as_slice());
        assert_eq!("22c219648f00c61e5b3b1bd81ffa8e7767e2e3c5",
                   digest_to_hex("hello World").as_slice());
        assert_eq!("788245b4dad73c1e5a630c126c484c7a2464f280",
                   digest_to_hex("hello World!").as_slice());
        assert_eq!("7b502c3a1f48c8609ae212cdfb639dee39673f5e",
                   digest_to_hex("Hello world").as_slice());
        assert_eq!("d3486ae9136e7856bc42212385ea797094475802",
                   digest_to_hex("Hello world!").as_slice());
        assert_eq!("0a4d55a8d778e5022fab701977c5d840bbc486d0",
                   digest_to_hex("Hello World").as_slice());
        assert_eq!("2ef7bde608ce5404e97d5f042f95f89f1c232871",
                   digest_to_hex("Hello World!").as_slice());
        assert_eq!("b7e23ec29af22b0b4e41da31e868d57226121c84",
                   digest_to_hex("hello, world").as_slice());
        assert_eq!("1f09d30c707d53f3d16c530dd73d70a6ce7596a9",
                   digest_to_hex("hello, world!").as_slice());
        assert_eq!("ca3c58516ddef44b25693df5a915206e1bd094da",
                   digest_to_hex("hello, World").as_slice());
        assert_eq!("dd0588c172986c32636ffdd8cc690de7b41bf253",
                   digest_to_hex("hello, World!").as_slice());
        assert_eq!("e02aa1b106d5c7c6a98def2b13005d5b84fd8dc8",
                   digest_to_hex("Hello, world").as_slice());
        assert_eq!("943a702d06f34599aee1f8da8ef9f7296031d699",
                   digest_to_hex("Hello, world!").as_slice());
        assert_eq!("907d14fb3af2b0d4f18c2d46abe8aedce17367bd",
                   digest_to_hex("Hello, World").as_slice());
        assert_eq!("0a0a9f2a6772942557ab5355d76af442f8f65e01",
                   digest_to_hex("Hello, World!").as_slice());
    }

    #[test]
    fn test_sha1_multi() {
        let s = "GNU LESSER GENERAL PUBLIC LICENSE Version 3, 29 June 2007 Copyright (C) 2007 Free Software Foundation, Inc. <http://fsf.org/>";
        assert_eq!("a31e8cb8a139d146a0070fa13795d6766acaccd4", digest_to_hex(s).as_slice());
    }


    #[bench]
    fn bench_sha1_hello(b: & mut Bencher) {
        let s = "hello world";

        b.iter(|| digest_to_hex(s));
        b.bytes = s.len() as u64;
    }

    #[bench]
    fn bench_sha1_multi(b: & mut Bencher) {
        let s = "GNU LESSER GENERAL PUBLIC LICENSE Version 3, 29 June 2007 Copyright (C) 2007 Free Software Foundation, Inc. <http://fsf.org/>";

        b.iter(|| digest_to_hex(s));
        b.bytes = s.len() as u64;
    }

    //#[bench]
    //pub fn block_64(bh: & mut Bencher) {
    //    let mut state = [0u8; 20];
    //    let bytes = [1u8; 64];
    //    bh.iter( || {
    //            digest_block(&mut state, &bytes[..]);
    //        });
    //    bh.bytes = bytes.len() as u64;
    //}

    //
    // Benchmarks for `digest`
    //

    #[bench]
    fn bench_sha1_10(b: & mut Bencher) {
        let buf = [0x20u8; 10];
        b.iter( || { digest(&buf[..]); });
        b.bytes = buf.len() as u64;
    }
    #[bench]
    fn bench_sha1_1k(b: & mut Bencher) {
        let buf = [0x20u8; 1024];
        b.iter( || { digest(&buf[..]); });
        b.bytes = buf.len() as u64;
    }
    #[bench]
    fn bench_sha1_64k(b: & mut Bencher) {
        let buf = [0x20u8; 65536];
        b.iter( || { digest(&buf[..]); });
        b.bytes = buf.len() as u64;
    }

    //
    // Benchmarks for `digest_to_bytes`
    //

    #[bench]
    fn bench_sha1_to_bytes_10(b: & mut Bencher) {
        let buf = [0x20u8; 10];
        b.iter( || { digest_to_bytes(&buf[..]); });
        b.bytes = buf.len() as u64;
    }
    #[bench]
    fn bench_sha1_to_bytes_1k(b: & mut Bencher) {
        let buf = [0x20u8; 1024];
        b.iter( || { digest_to_bytes(&buf[..]); });
        b.bytes = buf.len() as u64;
    }
    #[bench]
    fn bench_sha1_to_bytes_64k(b: & mut Bencher) {
        let buf = [0x20u8; 65536];
        b.iter( || { digest_to_bytes(&buf[..]); });
        b.bytes = buf.len() as u64;
    }

    //
    // Benchmarks for `digest_to_hex`
    //

    #[bench]
    fn bench_sha1_to_hex_10(b: & mut Bencher) {
        let buf = [0x20u8; 10];
        let msg = ::std::str::from_utf8(&buf[..]).unwrap();
        b.iter( || { digest_to_hex(msg); });
        b.bytes = msg.len() as u64;
    }
    #[bench]
    fn bench_sha1_to_hex_1k(b: & mut Bencher) {
        let buf = [0x20u8; 1024];
        let msg = ::std::str::from_utf8(&buf[..]).unwrap();
        b.iter( || { digest_to_hex(msg); });
        b.bytes = msg.len() as u64;
    }
    #[bench]
    fn bench_sha1_to_hex_64k(b: & mut Bencher) {
        let buf = [0x20u8; 65536];
        let msg = ::std::str::from_utf8(&buf[..]).unwrap();
        b.iter( || { digest_to_hex(msg); });
        b.bytes = msg.len() as u64;
    }
}
