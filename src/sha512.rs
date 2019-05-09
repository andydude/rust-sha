/// TODO: docs
#[derive(Clone)]
pub struct Sha512([u64; 8], Vec<u8>);

mod impls {
    use std::borrow::Borrow;
    use std::default::Default;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::beu64;
    use super::Sha512;
    use utils::{Reset,
                Digest,
                DigestExt,
                ReadPadBlocksExt,
                StdPad};

    impl Default for Sha512 {

        /// Construct a default `Sha512` object.
        fn default() -> Sha512 {
            Sha512(super::consts::H, Vec::new())
        }
    }

    impl Reset for Sha512 {

        /// Reset the state
        fn reset(&mut self) {
            self.0 = super::consts::H;
            self.1.clear();
        }
    }

    impl Write for Sha512 {

        /// Write to buffer
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Write::write(&mut self.1, buf)
        }

        /// Digest buffer
        fn flush(&mut self) -> io::Result<()> {
            let mut state = self.0;
            let ref buf = self.1;

            fn pad(len: usize) -> StdPad {
                let mut suffix = vec![0u8; 16];
                beu64::encode(&mut suffix[8..], 8*len as u64);
                StdPad::new(suffix, 128)
            }

            for block in buf.pad_blocks(128, |len: usize| pad(len)) {
                super::ops::digest_block(&mut state, block.borrow());
            }

            self.0 = state;
            Ok(())
        }
    }

    impl Read for Sha512 {

        /// Read state as big-endian
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let state_buf = &self.0[..buf.len()/8];
            beu64::encode_slice(buf, state_buf);
            Ok(buf.len())
        }
    }

    impl Hasher for Sha512 {

        /// Get the first 8 bytes of the state
        fn finish(&self) -> u64 {
            let mut h = self.clone();
            h.flush().unwrap();
            h.0[0]
        }

        /// Write to buffer
        fn write(&mut self, buf: &[u8]) {
            Write::write(self, buf).unwrap();
        }
    }

    impl Digest for Sha512 {}

    impl DigestExt for Sha512 {
        fn default_len() -> usize {
            return 64;
        }
    }
}

/// TODO
//#[unstable(feature="default", reason="TODO")]
pub mod consts {

    /// TODO
    pub const H: [u64; 8] = [
        0x6a09e667f3bcc908,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ];

    /// TODO
    pub const K: [u64; 80] = [
        0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
        0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
        0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
        0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
        0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
        0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
        0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
        0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
        0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
        0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
        0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
        0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
        0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
        0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
        0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
        0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
        0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
        0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
        0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
        0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
    ];
}

/// TODO: docs
pub mod ops {
    use bswap::beu64;

    macro_rules! rotate_right {
        ($a:expr, $b:expr) => (($a >> $b) ^ ($a << (64 - $b)))
    }
    macro_rules! sigma0 {
        ($a:expr) => (rotate_right!($a, 1) ^ rotate_right!($a, 8) ^ ($a >> 7))
    }
    macro_rules! sigma1 {
        ($a:expr) => (rotate_right!($a, 19) ^ rotate_right!($a, 61) ^ ($a >> 6))
    }
    macro_rules! big_sigma0 {
        ($a:expr) => (rotate_right!($a, 28) ^ rotate_right!($a, 34) ^ rotate_right!($a, 39))
    }
    macro_rules! big_sigma1 {
        ($a:expr) => (rotate_right!($a, 14) ^ rotate_right!($a, 18) ^ rotate_right!($a, 41))
    }
    macro_rules! bool3ary_202 {
        ($a:expr, $b:expr, $c:expr) => ($c ^ ($a & ($b ^ $c)))
    }
    macro_rules! bool3ary_232 {
        ($a:expr, $b:expr, $c:expr) => (($a & $b) ^ ($a & $c) ^ ($b & $c))
    }

    macro_rules! sha512_expand_round {
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

    macro_rules! sha512_digest_round {
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

    /// There are no plans for hardware implementations at this time,
    /// but this function can be easily implemented with some kind of
    /// SIMD assistance.
    ///
    /// ```ignore
    /// {
    ///     // this is the core expression
    ///     let temp = sha512load(work[4], work[5]);
    ///     sha512msg(work[0], work[1], temp, work[7]);
    /// }
    /// ```
    #[inline]
    pub fn expand_round_x4(w: &mut [u64; 16], t: usize) {
        sha512_expand_round!(w, t);
        sha512_expand_round!(w, t + 1);
        sha512_expand_round!(w, t + 2);
        sha512_expand_round!(w, t + 3);
    }

    /// There are no plans for hardware implementations at this time,
    /// but this function can be easily implemented with some kind of
    /// SIMD assistance.
    ///
    /// ```ignore
    /// {
    ///     // this is to illustrate the data order
    ///     let ae = u64x2(a, e);
    ///     let bf = u64x2(b, f);
    ///     let cg = u64x2(c, g);
    ///     let dh = u64x2(d, h);
    ///
    ///     // this is the core expression
    ///     dh = sha512rnd(dh, ae, bf, cg, work[0]);
    ///     cg = sha512rnd(cg, dh, ae, bf, work[1]);
    ///     bf = sha512rnd(bf, cg, dh, ae, work[2]);
    ///     ae = sha512rnd(ae, bf, cg, dh, work[3]);
    ///
    ///     a = ae.0;
    ///     b = bf.0;
    ///     c = cg.0;
    ///     d = dh.0;
    ///     e = ae.1;
    ///     f = bf.1;
    ///     g = cg.1;
    ///     h = dh.1;
    /// }
    /// ```
    #[inline]
    pub fn digest_round_x4(state: &mut [u64; 8], k: [u64; 4], w: [u64; 4]) {
        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];
        let mut e = state[4];
        let mut f = state[5];
        let mut g = state[6];
        let mut h = state[7];
        sha512_digest_round!(a, b, c, d, e, f, g, h, k[0], w[0]);
        sha512_digest_round!(h, a, b, c, d, e, f, g, k[1], w[1]);
        sha512_digest_round!(g, h, a, b, c, d, e, f, k[2], w[2]);
        sha512_digest_round!(f, g, h, a, b, c, d, e, k[3], w[3]);
        *state = [e, f, g, h, a, b, c, d];
    }

    /// TODO
    #[inline]
    pub fn expand_round_x16(w: &mut [u64; 16]) {
        expand_round_x4(w, 0);
        expand_round_x4(w, 4);
        expand_round_x4(w, 8);
        expand_round_x4(w, 12);
    }

    /// TODO
    #[inline]
    pub fn digest_round_x16(state: &mut [u64; 8], k: [u64; 16], w: [u64; 16]) {
        macro_rules! as_simd {
            ($x:expr) => {{let (y, _): (&[u64; 4], usize) = unsafe {::std::mem::transmute($x)}; *y}}
        }

        digest_round_x4(state, as_simd!(&k[0..4]), as_simd!(&w[0..4]));
        digest_round_x4(state, as_simd!(&k[4..8]), as_simd!(&w[4..8]));
        digest_round_x4(state, as_simd!(&k[8..12]), as_simd!(&w[8..12]));
        digest_round_x4(state, as_simd!(&k[12..16]), as_simd!(&w[12..16]));
    }

    /// TODO
    pub fn digest_block(state: &mut [u64; 8], buf: &[u8]) {
        use std::mem::transmute;
        use super::consts::K;
        let state2 = *state;
        let mut w: [u64; 16] = [0; 16];

        macro_rules! as_simd_array {
            ($x:expr) => {{let (y, _): (&[u64; 16], usize) = unsafe {transmute($x)}; *y}}
        }

        beu64::decode_slice(&mut w[..], buf);
        digest_round_x16(state, as_simd_array!(&K[0..16]), w);
        expand_round_x16(&mut w);
        digest_round_x16(state, as_simd_array!(&K[16..32]), w);
        expand_round_x16(&mut w);
        digest_round_x16(state, as_simd_array!(&K[32..48]), w);
        expand_round_x16(&mut w);
        digest_round_x16(state, as_simd_array!(&K[48..64]), w);
        expand_round_x16(&mut w);
        digest_round_x16(state, as_simd_array!(&K[64..80]), w);

        for i in 0..8 {
            state[i] = state[i]
                .wrapping_add(state2[i]);
        }
    }

    /// TODO
    pub fn digest(buf: &[u8]) -> [u64; 8] {
        use std::default::Default;
        use utils::Digest;

        super::Sha512::default().digest(buf).0
    }
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use super::Sha512;
    use utils::{Digest, DigestExt};

    //
    // Helper functions
    //

    //fn digest_block(state: &mut [u64; 8], buf: &[u8]) {
    //    super::ops::digest_block(state, buf);
    //}

    fn digest(buf: &[u8]) -> Sha512 {
        let mut h: Sha512 = Default::default();
        h.digest(buf);
        h
    }

    //fn digest_to_bytes(buf: &[u8]) -> Vec<u8> {
    //    digest(buf).to_bytes()
    //}

    fn digest_to_hex(msg: &str) -> String {
        digest(&msg.as_bytes()).to_hex()
    }

    //
    // Tests for `hash`
    //

    #[test]
    fn test_sha512_empty_hash() {
        use std::hash::{Hash, Hasher};

        let msg: &[u8] = "".as_bytes();
        let mut h: Sha512 = Default::default();
        <u8 as Hash>::hash_slice::<Sha512>(msg, &mut h);
        let digest: u64 = h.finish();
        assert_eq!(0xcf83e1357eefb8bdu64, digest);
    }

    #[test]
    fn test_sha512_hello_hash() {
        use std::hash::{Hash, Hasher};

        let msg: &[u8] = "hello world".as_bytes();
        let mut h: Sha512 = Default::default();
        <u8 as Hash>::hash_slice::<Sha512>(msg, &mut h);
        let digest: u64 = h.finish();
        assert_eq!(0x309ecc489c12d6ebu64, digest);
    }

    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn test_sha512_empty() {

        assert_eq!(digest_to_hex(""),
                   concat!("cf83e1357eefb8bdf1542850d66d8007",
                           "d620e4050b5715dc83f4a921d36ce9ce",
                           "47d0d13c5d85f2b0ff8318d2877eec2f",
                           "63b931bd47417a81a538327af927da3e"));
    }

    #[test]
    fn test_sha512_hello() {

        assert_eq!(digest_to_hex("hello world"),
                   concat!("309ecc489c12d6eb4cc40f50c902f2b4",
                           "d0ed77ee511a7c7a9bcd3ca86d4cd86f",
                           "989dd35bc5ff499670da34255b45b0cf",
                           "d830e81f605dcf7dc5542e93ae9cd76f"));

        assert_eq!(digest_to_hex("hello world!"),
                   concat!("db9b1cd3262dee37756a09b906497358",
                           "9847caa8e53d31a9d142ea2701b1b28a",
                           "bd97838bb9a27068ba305dc8d04a45a1",
                           "fcf079de54d607666996b3cc54f6b67c"));

        assert_eq!(digest_to_hex("hello World"),
                   concat!("1ca107777d9d999bdd8099875438919b",
                           "5dca244104e393685f7d05f4feb5f181",
                           "f1878e1178daf1a8c97c5b290222609c",
                           "9515dd096344b625b37d7a8910076ed2"));

        assert_eq!(digest_to_hex("hello World!"),
                   concat!("6913f7893ca35886a8befb1f9c0875be",
                           "48b6a399e4a6cad42f6178995640045a",
                           "8c50f62552d58a4ec753d41ceb36ed8d",
                           "c79de1c3adfdae7805507060ba6b5b93"));

        assert_eq!(digest_to_hex("Hello world"),
                   concat!("b7f783baed8297f0db917462184ff4f0",
                           "8e69c2d5e5f79a942600f9725f58ce1f",
                           "29c18139bf80b06c0fff2bdd34738452",
                           "ecf40c488c22a7e3d80cdf6f9c1c0d47"));

        assert_eq!(digest_to_hex("Hello world!"),
                   concat!("f6cde2a0f819314cdde55fc227d8d7da",
                           "e3d28cc556222a0a8ad66d91ccad4aad",
                           "6094f517a2182360c9aacf6a3dc32316",
                           "2cb6fd8cdffedb0fe038f55e85ffb5b6"));

        assert_eq!(digest_to_hex("Hello World"),
                   concat!("2c74fd17edafd80e8447b0d46741ee24",
                           "3b7eb74dd2149a0ab1b9246fb30382f2",
                           "7e853d8585719e0e67cbda0daa8f5167",
                           "1064615d645ae27acb15bfb1447f459b"));

        assert_eq!(digest_to_hex("Hello World!"),
                   concat!("861844d6704e8573fec34d967e20bcfe",
                           "f3d424cf48be04e6dc08f2bd58c72974",
                           "3371015ead891cc3cf1c9d34b49264b5",
                           "10751b1ff9e537937bc46b5d6ff4ecc8"));

        assert_eq!(digest_to_hex("hello, world"),
                   concat!("8710339dcb6814d0d9d2290ef422285c",
                           "9322b7163951f9a0ca8f883d3305286f",
                           "44139aa374848e4174f5aada663027e4",
                           "548637b6d19894aec4fb6c46a139fbf9"));

        assert_eq!(digest_to_hex("hello, world!"),
                   concat!("6c2618358da07c830b88c5af8c353508",
                           "0e8e603c88b891028a259ccdb9ac802d",
                           "0fc0170c99d58affcf00786ce188fc5d",
                           "753e8c6628af2071c3270d50445c4b1c"));

        assert_eq!(digest_to_hex("hello, World"),
                   concat!("7c8d44b246cfad2848ac0718d01c7291",
                           "3d2dc7552c5667967b92aefec699849c",
                           "ec8435147e8566d6798b80ecd6bc4554",
                           "35d4feba047a0707b2da86c0088dcfff"));

        assert_eq!(digest_to_hex("hello, World!"),
                   concat!("c0d0df8be7405b0cdb12df4d674d64eb",
                           "ed62207ffe118ee5ee9d33071af4abf3",
                           "83d6efa2b56450e1475971e7e9105629",
                           "c11ad855b08e17e9fbc6584c08403990"));

        assert_eq!(digest_to_hex("Hello, world"),
                   concat!("f986313ffca1a20c61fa2cff5cb597f1",
                           "af10a650aecca497a746e8d11d1b6bf3",
                           "3e9e6a25eb7ba26af2fcfaa70472d825",
                           "0b908419a188a16e17191fc26f423f52"));

        assert_eq!(digest_to_hex("Hello, world!"),
                   concat!("c1527cd893c124773d811911970c8fe6",
                           "e857d6df5dc9226bd8a160614c0cd963",
                           "a4ddea2b94bb7d36021ef9d865d5cea2",
                           "94a82dd49a0bb269f51f6e7a57f79421"));

        assert_eq!(digest_to_hex("Hello, World"),
                   concat!("45546d4d71407e82ecda31eba5bf74b6",
                           "5bc092b0436a2409a6b615c1f78fdb2d",
                           "3da371758f07a65b5d2b3ee8fa9ea0c7",
                           "72dd1eff884c4c77d4290177b002ccdc"));

        assert_eq!(digest_to_hex("Hello, World!"),
                   concat!("374d794a95cdcfd8b35993185fef9ba3",
                           "68f160d8daf432d08ba9f1ed1e5abe6c",
                           "c69291e0fa2fe0006a52570ef18c19de",
                           "f4e617c33ce52ef0a6e5fbe318cb0387"));
    }

    // #[test]
    // fn test_sha512_multi() {
    //     let s = "GNU LESSER GENERAL PUBLIC LICENSE Version 3, 29 June 2007 Copyright (C) 2007 Free Software Foundation, Inc. <http://fsf.org/>";
    //     assert_eq!("d33d14f2ea60beb394082598e05375cdd6ff8966315322c34b6faea80e7d5a7c", digest_to_hex(s));
    // }
    //
    // #[test]
    // fn test_sha512_1k() {
    //     let buf = [0x20; 1000];
    //     let msg = str::from_utf8(&buf[..]).unwrap();
    //     assert_eq!("08c9b52f61fadf1eff6fb89169f1735fbae7bb583b23cb119d0e1a0151bac952", digest_to_hex(msg));
    // }
    //
    // //
    // // Tests for `digest_to_bytes`
    // //
    //
    // #[test]
    // fn test_sha512_hello_bytes() {
    //     let bytes = digest_to_bytes("hello world".as_bytes());
    //
    //     assert_eq!(b"\xb9\x4d\x27\xb9\x93\x4d\x3e\x08\xa5\x2e\x52\xd7\xda\x7d\xab\xfa\xc4\x84\xef\xe3\x7a\x53\x80\xee\x90\x88\xf7\xac\xe2\xef\xcd\xe9",
    //                bytes);
    // }
    //
    // //
    // // Tests for `digest`
    // //
    //
    // #[test]
    // fn test_sha512_hello_digest() {
    //     let words: [u64; 8] = digest("hello world".as_bytes()).0;
    //
    //     assert_eq!(words[0], 0xb94d27b9);
    //     assert_eq!(words[1], 0x934d3e08);
    //     assert_eq!(words[2], 0xa52e52d7);
    //     assert_eq!(words[3], 0xda7dabfa);
    //     assert_eq!(words[4], 0xc484efe3);
    //     assert_eq!(words[5], 0x7a5380ee);
    //     assert_eq!(words[6], 0x9088f7ac);
    //     assert_eq!(words[7], 0xe2efcde9);
    // }
    //
    // //
    // // Tests for `digest_block`
    // //
    //
    // fn make_empty_block() -> Vec<u8> {
    //     let mut block = vec![0u8; 16*4];
    //     assert_eq!(block.len(), 16*4);
    //     block[0] = 0x80u8;
    //     block
    // }
    //
    // fn make_hello_block() -> Vec<u8> {
    //
    //     // this could use a concat_bytes!
    //     static HELLO_BLOCK: &'static [u8] = b"hello world\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x58";
    //
    //     let mut block: Vec<u8> = Vec::with_capacity(16*4);
    //     unsafe { block.set_len(16*4) };
    //     (&mut block[..]).clone_from_slice(HELLO_BLOCK);
    //     assert_eq!(block.len(), 16*4);
    //     block
    // }
    //
    // #[test]
    // fn test_sha512_empty_block() {
    //     use serialize::hex::ToHex;
    //     let mut state: [u64; 8] = [0; 8];
    //     (&mut state[..]).clone_from_slice(&super::consts::H[..]);
    //     let block_vec = make_empty_block();
    //     let block = &block_vec[..];
    //     digest_block(&mut state, &block[..]);
    //     let mut bytes = vec![0u8; 8*4];
    //     beu64::encode_slice(&mut bytes[..], &state[..]);
    //     assert_eq!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    //                bytes.to_hex());
    // }
    //
    // #[test]
    // fn test_sha512_hello_block() {
    //     use serialize::hex::ToHex;
    //     let mut state: [u64; 8] = [0; 8];
    //     (&mut state[..]).clone_from_slice(&super::consts::H[..]);
    //     let block_vec = make_hello_block();
    //     let block = &block_vec[..];
    //     digest_block(&mut state, &block[..]);
    //     let mut bytes = vec![0u8; 8*4];
    //     beu64::encode_slice(&mut bytes[..], &state[..]);
    //     assert_eq!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
    //                bytes.to_hex());
    // }
    //
    // #[bench]
    // pub fn bench_sha512_block(bh: & mut Bencher) {
    //     let mut state: [u64; 8] = [0; 8];
    //     let bytes = [1u8; 16*4];
    //     let block = &bytes[..];
    //     bh.iter( || { digest_block(&mut state, block); });
    //     bh.bytes = 64u64;
    // }
    //
    // //
    // // Benchmarks for `digest_bytes`
    // //
    //
    // #[bench]
    // fn bench_sha512_hello_block(b: & mut Bencher) {
    //     let mut state: [u64; 8] = [0; 8];
    //     (&mut state[..]).clone_from_slice(&super::consts::H[..]);
    //     let block_vec = make_hello_block();
    //     let block = &block_vec[..];
    //     b.iter( || { digest_block(&mut state, block) });
    //     b.bytes = 64u64;
    // }
    //
    // #[bench]
    // fn bench_sha512_empty_block(b: & mut Bencher) {
    //     let mut state: [u64; 8] = [0; 8];
    //     (&mut state[..]).clone_from_slice(&super::consts::H[..]);
    //     let block_vec = make_empty_block();
    //     let block = &block_vec[..];
    //     b.iter( || { digest_block(&mut state, block) });
    //     b.bytes = 64u64;
    // }
    //
    // //
    // // Benchmarks for `digest`
    // //
    //
    // #[bench]
    // fn bench_sha512_10(b: & mut Bencher) {
    //     let buf = [0x20u8; 10];
    //     b.iter( || { digest(&buf[..]); });
    //     b.bytes = buf.len() as u64;
    // }
    // #[bench]
    // fn bench_sha512_1k(b: & mut Bencher) {
    //     let buf = [0x20u8; 1024];
    //     b.iter( || { digest(&buf[..]); });
    //     b.bytes = buf.len() as u64;
    // }
    // #[bench]
    // fn bench_sha512_64k(b: & mut Bencher) {
    //     let buf = [0x20u8; 65536];
    //     b.iter( || { digest(&buf[..]); });
    //     b.bytes = buf.len() as u64;
    // }
    //
    // //
    // // Benchmarks for `digest_to_bytes`
    // //
    //
    // #[bench]
    // fn bench_sha512_to_bytes_10(b: & mut Bencher) {
    //     let buf = [0x20u8; 10];
    //     b.iter( || { digest_to_bytes(&buf[..]); });
    //     b.bytes = buf.len() as u64;
    // }
    // #[bench]
    // fn bench_sha512_to_bytes_1k(b: & mut Bencher) {
    //     let buf = [0x20u8; 1024];
    //     b.iter( || { digest_to_bytes(&buf[..]); });
    //     b.bytes = buf.len() as u64;
    // }
    // #[bench]
    // fn bench_sha512_to_bytes_64k(b: & mut Bencher) {
    //     let buf = [0x20u8; 65536];
    //     b.iter( || { digest_to_bytes(&buf[..]); });
    //     b.bytes = buf.len() as u64;
    // }
    //
    // //
    // // Benchmarks for `digest_to_hex`
    // //
    //
    // #[bench]
    // fn bench_sha512_to_hex_10(b: & mut Bencher) {
    //     let buf = [0x20u8; 10];
    //     let msg = ::std::str::from_utf8(&buf[..]).unwrap();
    //     b.iter( || { digest_to_hex(msg); });
    //     b.bytes = msg.len() as u64;
    // }
    // #[bench]
    // fn bench_sha512_to_hex_1k(b: & mut Bencher) {
    //     let buf = [0x20u8; 1024];
    //     let msg = ::std::str::from_utf8(&buf[..]).unwrap();
    //     b.iter( || { digest_to_hex(msg); });
    //     b.bytes = msg.len() as u64;
    // }
    // #[bench]
    // fn bench_sha512_to_hex_64k(b: & mut Bencher) {
    //     let buf = [0x20u8; 65536];
    //     let msg = ::std::str::from_utf8(&buf[..]).unwrap();
    //     b.iter( || { digest_to_hex(msg); });
    //     b.bytes = msg.len() as u64;
    // }
    //
    // //
    // // Test Structure
    // //
    //
    // struct Test {
    //     input: &'static str,
    //     output_str: &'static str,
    // }
    //
    // fn make_test_list() -> Vec<Test> {
    //     // Examples from wikipedia
    //     let wikipedia_tests = vec![
    //         Test {
    //             input: "",
    //             output_str: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    //         },
    //         Test {
    //             input: "The quick brown fox jumps over the lazy dog",
    //             output_str: "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"
    //         },
    //         Test {
    //             input: "The quick brown fox jumps over the lazy dog.",
    //             output_str: "ef537f25c895bfa782526529a9b63d97aa631564d5d789c2b765448c8635fb6c"
    //         },
    //     ];
    //     wikipedia_tests
    // }
    //
    // #[test]
    // fn test_sha512_wikipedia() {
    //
    //     let tests = make_test_list();
    //     let mut h: Sha512 = Default::default();
    //
    //     // Test that it works when accepting the message all at once
    //     for t in tests.iter() {
    //         let out_str = h.digest(t.input.as_bytes()).to_hex();
    //         assert_eq!(&out_str[..], t.output_str);
    //         h.reset();
    //     }
    //
    //     // Test that it works when accepting the message in pieces
    //     for t in tests.iter() {
    //         let len = t.input.len();
    //         let mut left = len;
    //
    //         while left > 0 {
    //             let take = (left + 1) / 2;
    //             h.write((&t.input[len - left..take + len - left]).as_bytes()).unwrap();
    //             left = left - take;
    //         }
    //         h.flush().unwrap();
    //
    //         let out_str = h.to_hex();
    //         assert_eq!(&out_str[..], t.output_str);
    //
    //         h.reset();
    //     }
    // }
}

//#[cfg(test)]
//mod tests {
//    use digest::Digest;
//    use sha2::{Sha512, Sha384, Sha512Trunc512, Sha512Trunc224, Sha512, Sha224};
//
//}
