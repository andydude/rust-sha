/// TODO: docs
#[derive(Clone)]
pub struct Sha3256([u64; 25], Vec<u8>);

mod impls {
    use std::default::Default;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::leu64;
    use super::Sha3256;
    use utils::{Reset,
                Digest,
                DigestExt,
                ReadPadBlocksExt,
                StdPad};

    impl Default for Sha3256 {

        /// Construct a default `Sha3256` object.
        fn default() -> Sha3256 {
            Sha3256([0u64; 25], Vec::new())
        }
    }

    impl Reset for Sha3256 {

        /// Reset the state
        fn reset(&mut self) {
            self.0 = [0u64; 25];
            self.1.clear();
        }
    }

    impl Write for Sha3256 {

        /// Write to buffer
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Write::write(&mut self.1, buf)
        }

        /// Digest buffer
        fn flush(&mut self) -> io::Result<()> {
            let ref buf = self.1;
            for block in buf.pad_blocks(136, |_: usize| {
                StdPad::with_prefix(0x06u8, vec![0x80u8], 136)
                }) {
                super::super::keccak::ops::digest_block(&mut self.0, block);
            }
            Ok(())
        }
    }

    impl Read for Sha3256 {

        /// Read state as big-endian
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let state_buf = &self.0[..buf.len()/8];
            leu64::encode_slice(buf, state_buf);
            Ok(buf.len())
        }
    }

    impl Hasher for Sha3256 {

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

    impl Digest for Sha3256 {}

    impl DigestExt for Sha3256 {
        fn default_len() -> usize {
            return 32;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use test::Bencher;
    use utils::{Digest, DigestExt};
    use super::Sha3256;

    //
    // Helper functions
    //

    fn digest(buf: &[u8]) -> Sha3256 {
        let mut h: Sha3256 = Default::default();
        h.digest(buf);
        h
    }
    
    fn digest_to_bytes(buf: &[u8]) -> Vec<u8> {
        digest(buf).to_bytes()
    }
    
    fn digest_to_hex(msg: &str) -> String {
        Sha3256::default().digest(msg.as_bytes()).to_hex()
    }

    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn test_sha3256_empty() {

        assert_eq!(digest_to_hex(""),
                   concat!("a7ffc6f8bf1ed766",
                           "51c14756a061d662",
                           "f580ff4de43b49fa",
                           "82d80a4b80f8434a"));
    }


    #[bench]
    pub fn bench_sha256_block(bh: & mut Bencher) {
        use super::super::keccak::ops::digest_block;
        let mut state: [u64; 25] = [0; 25];
        let bytes = [1u8; 136];
        let block = &bytes[..];
        bh.iter( || { digest_block(&mut state, block); });
        bh.bytes = 64u64;
    }

    //#[bench]
    //pub fn bench_sha256_block_simd(bh: & mut Bencher) {
    //    use super::super::keccak_simd::ops::digest_block;
    //    let mut state: [u64; 25] = [0; 25];
    //    let bytes = [1u8; 136];
    //    let block = &bytes[..];
    //    bh.iter( || { digest_block(&mut state, block); });
    //    bh.bytes = 64u64;
    //}
    //
    //#[bench]
    //pub fn bench_sha256_block_simd2(bh: & mut Bencher) {
    //    use super::super::keccak_simd::ops::digest_block;
    //    let mut state: [u64; 25] = [0; 25];
    //    let bytes = [1u8; 136];
    //    let block = &bytes[..];
    //    bh.iter( || { digest_block(&mut state, block); });
    //    bh.bytes = 64u64;
    //}


    #[bench]
    fn bench_sha3256_10(b: & mut Bencher) {
        let buf = [0x20u8; 10];
        b.iter( || { digest(&buf[..]); });
        b.bytes = buf.len() as u64;
    }
    #[bench]
    fn bench_sha3256_1k(b: & mut Bencher) {
        let buf = [0x20u8; 1000];
        b.iter( || { digest(&buf[..]); });
        b.bytes = buf.len() as u64;
    }
    #[bench]
    fn bench_sha3256_64k(b: & mut Bencher) {
        let buf = [0x20u8; 65536];
        b.iter( || { digest(&buf[..]); });
        b.bytes = buf.len() as u64;
    }
    #[bench]
    fn bench_sha3256_to_bytes_10(b: & mut Bencher) {
        let buf = [0x20u8; 10];
        b.iter( || { digest_to_bytes(&buf[..]); });
        b.bytes = buf.len() as u64;
    }
    #[bench]
    fn bench_sha3256_to_hex_10(b: & mut Bencher) {
        let buf = [0x20u8; 10];
        let msg = ::std::str::from_utf8(&buf[..]).unwrap();
        b.iter( || { digest_to_hex(msg); });
        b.bytes = msg.len() as u64;
    }


}
