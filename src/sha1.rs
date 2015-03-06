/// TODO: docs
#[derive(Clone)]
pub struct Sha1([u32; 5], Vec<u8>);

pub mod impls {
    use std::default::Default;
    use std::ffi::IntoBytes;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::beu32;
    use serialize::hex::ToHex;
    use utils::{Reset, Digest, DigestExt};
    use super::Sha1;
    
    impl Default for Sha1 {

        /// Construct a default `Sha1` object.
        fn default() -> Sha1 {
            Sha1(super::consts::H, Vec::new())
        }
    }

    impl Reset for Sha1 {

        /// Reset the state
        fn reset(&mut self) {
            let state = super::consts::H;
            self.0 = state;
        }
    }

    impl Read for Sha1 {

        /// Read state as big-endian
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            beu32::encode_slice(buf, &self.0[..]);
            Ok(buf.len())
        }
    }

    impl Write for Sha1 {
        
        /// Write to buffer
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Write::write(&mut self.1, buf)
        }
        
        /// Digest buffer
        fn flush(&mut self) -> io::Result<()> {
            self.0 = super::ops::digest(&self.1[..]);
            Ok(())
        }
    }
    
    impl Hasher for Sha1 {

        /// Get the first 8 bytes of the state
        fn finish(&self) -> u64 {
            let state = self.0;
            ((state[0] as u64) << 32u64) |
             (state[1] as u64)
        }

        /// Write to buffer
        fn write(&mut self, buf: &[u8]) {
            Write::write(self, buf).unwrap();
        }
    }
    
    impl IntoBytes for Sha1 {
        fn into_bytes(mut self) -> Vec<u8> {
            let mut bytes = vec![0u8; 20];
            (&mut self).read(&mut bytes[..]).unwrap();
            bytes
        }
    }
    
    impl ToHex for Sha1 {
        fn to_hex(&self) -> String {
            self.clone().into_bytes().as_slice().to_hex()
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
#[unstable(feature="default", reason="1.0.0")]
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

pub mod ops {
    use std::io::prelude::*;
    use bswap::beu32;
    use utils::StdPad;
    use super::consts::{H, K};
    
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
    macro_rules! expand_round {
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
    macro_rules! digest_round {
        ($a:ident, $b:ident, $c:ident, $d:ident,
         $e:ident, $k:expr, $w:expr, $i:expr) => {
            {
                $e = $e
                    .wrapping_add($k)
                    .wrapping_add($w)
                    .wrapping_add(rotate_left!($a, 5))
                    .wrapping_add(round_func!($b, $c, $d, $i));
                
                $b = rotate_left!($b, 30);
            }
        }
    }
    macro_rules! expand_round_x4 {
        ($work:expr, $t:expr) => {
            {
                println!("{:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x}",
                         $work[0], $work[1], $work[2], $work[3], $work[4],
                         $work[5], $work[6], $work[7], $work[8], $work[9],
                         $work[10], $work[11], $work[12], $work[13], $work[14],
                         $work[15], $work[16], $work[17], $work[18], $work[19]);
                
                expand_round!($work, $t);
                expand_round!($work, $t + 1);
                expand_round!($work, $t + 2);
                expand_round!($work, $t + 3);
            }
        }
    }
    macro_rules! digest_round_x5 {
        ($a:ident, $b:ident, $c:ident, $d:ident,
         $e:ident, $k:expr, $w:expr, $t:expr, $i:expr) => {
            {
                println!("{:08x} {:08x} {:08x} {:08x} {:08x}",
                         $a, $b, $c, $d, $e);
                
                digest_round!($a, $b, $c, $d, $e, $k, $w[$t + 0], $i);
                digest_round!($e, $a, $b, $c, $d, $k, $w[$t + 1], $i);
                digest_round!($d, $e, $a, $b, $c, $k, $w[$t + 2], $i);
                digest_round!($c, $d, $e, $a, $b, $k, $w[$t + 3], $i);
                digest_round!($b, $c, $d, $e, $a, $k, $w[$t + 4], $i);
            }
        }
    }
    
    #[inline]
    pub fn expand_round_x20(w: &mut [u32]) {
        expand_round_x4!(w, 0);
        expand_round_x4!(w, 4);
        expand_round_x4!(w, 8);
        expand_round_x4!(w, 12);
        expand_round_x4!(w, 16);
    }
    
    #[inline]
    pub fn digest_round_x20(state: &mut [u32; 5], k: u32, w: &[u32], i: u8) {
        let [mut a, mut b, mut c, mut d, mut e] = *state;
        digest_round_x5!(a, b, c, d, e, k, w, 0, i);
        digest_round_x5!(a, b, c, d, e, k, w, 5, i);
        digest_round_x5!(a, b, c, d, e, k, w, 10, i);
        digest_round_x5!(a, b, c, d, e, k, w, 15, i);
        *state = [a, b, c, d, e];
    }

    pub fn digest_block(state: &mut [u32; 5], buf: &[u8]) {
        let state2 = *state;
        let mut w: [u32; 20] = [0; 20];
        
        beu32::decode_slice(&mut w[..16], buf);
        expand_round_x4!(w, 16);
        digest_round_x20(state, K[0], &w, 0);
        expand_round_x20(&mut w);
        digest_round_x20(state, K[1], &w, 1);
        expand_round_x20(&mut w);
        digest_round_x20(state, K[2], &w, 2);
        expand_round_x20(&mut w);
        digest_round_x20(state, K[3], &w, 3);

        for i in 0..5 {
            state[i] = state[i]
                .wrapping_add(state2[i]);
        }
    }
    
    pub fn digest(buf: &[u8]) -> [u32; 5] {
        let mut pad_buf = [0u8; 128];
        let pad_len = StdPad::new(buf.len())
            .read(&mut pad_buf[..]).unwrap();
        
        // Pad the message to a multiple of 64 bytes
        let blocks = buf.iter().cloned()
            .chain((&pad_buf[..pad_len]).iter().cloned())
            .collect::<Vec<u8>>();
        
        // Digest these blocks of 64 bytes.
        let mut state = H;
        for block in blocks.chunks(64) {
            digest_block(&mut state, block);
        }
        state
    }
}

#[cfg(test)]
pub mod tests {
    use std::default::Default;
    use std::ffi::IntoBytes;
    use std::io::prelude::*;
    use serialize::hex::ToHex;
    use test::Bencher;
    use bswap::beu32;
    use super::Sha1;
    
    //
    // Helper functions
    //
    
    fn digest_block(state: &mut [u32; 5], buf: &[u8]) {
        super::ops::digest_block(state, buf);
    }
    
    fn digest(buf: &[u8]) -> Sha1 {
        let mut h: Sha1 = Default::default();
        Write::write_all(&mut h, buf).unwrap();
        Write::flush(&mut h).unwrap();
        h
    }

    fn digest_to_bytes(buf: &[u8]) -> Vec<u8> {
        digest(buf).into_bytes()
    }

    fn digest_to_hex(msg: &str) -> String {
        digest(&msg.as_bytes()).to_hex()
    }

    //pub fn super::digest_block(state: &mut [u8], msg: &[u8]) {
    //    //use super::sw_openssl::digest;
    //    //digest(hash, msg);
    //    use std::mem::transmute;
    //    let (mut state2, _): (&mut [u8; 20], usize) = unsafe { transmute(state) };
    //    super::sw::digest_block(state2, msg);
    //}
    //
    //pub fn super::digest(state: &mut [u8], msg: &[u8]) {
    //    //use super::sw_openssl::digest;
    //    //digest(hash, msg);
    //    use super::Sha1;
    //    
    //    super::sw::digest(state, msg);
    //}
    //
    ///// Digest whole message, return hex string
    //#[unstable(feature = "sha_internals", reason = "will be trait method")]
    //pub fn digest_to_hex(msg: &str) -> String {
    //    let mut hash = [0u8; 20];
    //    super::digest(&mut hash[0..20], msg.as_bytes());
    //    hash.to_hex()
    //}
    
    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn sha1_hello() {
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
    fn sha1_empty() {
        assert_eq!("da39a3ee5e6b4b0d3255bfef95601890afd80709",
                   digest_to_hex("").as_slice());
    }
    
    #[test]
    fn test_multi_block() {
        let s = "GNU LESSER GENERAL PUBLIC LICENSE Version 3, 29 June 2007 Copyright (C) 2007 Free Software Foundation, Inc. <http://fsf.org/>";
        assert_eq!("a31e8cb8a139d146a0070fa13795d6766acaccd4", digest_to_hex(s).as_slice());
    }
    
    
    #[bench]
    fn bench_hello_world(b: & mut Bencher) {
        let s = "hello world";
    
        b.iter(|| digest_to_hex(s));
    }
    
    #[bench]
    fn bench_multi_block(b: & mut Bencher) {
        let s = "GNU LESSER GENERAL PUBLIC LICENSE Version 3, 29 June 2007 Copyright (C) 2007 Free Software Foundation, Inc. <http://fsf.org/>";
    
        b.iter(|| digest_to_hex(s));
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
    fn sha1_10(b: & mut Bencher) {
        let buf = [0x20u8; 10];
        b.iter( || { digest(&buf[..]); });
        b.bytes = buf.len() as u64;
    }
    #[bench]
    fn sha1_1k(b: & mut Bencher) {
        let buf = [0x20u8; 1024];
        b.iter( || { digest(&buf[..]); });
        b.bytes = buf.len() as u64;
    }
    #[bench]
    fn sha1_64k(b: & mut Bencher) {
        let buf = [0x20u8; 65536];
        b.iter( || { digest(&buf[..]); });
        b.bytes = buf.len() as u64;
    }  
}
