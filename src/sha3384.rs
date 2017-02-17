/// TODO: docs
#[derive(Clone)]
pub struct Sha3384([u64; 25], Vec<u8>);

mod impls {
    use std::borrow::Borrow;
    use std::default::Default;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::leu64;
    use super::Sha3384;
    use utils::{Reset,
                Digest,
                DigestExt,
                ReadPadBlocksExt,
                StdPad};

    impl Default for Sha3384 {

        /// Construct a default `Sha3384` object.
        fn default() -> Sha3384 {
            Sha3384([0u64; 25], Vec::new())
        }
    }

    impl Reset for Sha3384 {

        /// Reset the state
        fn reset(&mut self) {
            self.0 = [0u64; 25];
            self.1.clear();
        }
    }

    impl Write for Sha3384 {

        /// Write to buffer
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Write::write(&mut self.1, buf)
        }

        /// Digest buffer
        fn flush(&mut self) -> io::Result<()> {
            let ref buf = self.1;
            for block in buf.pad_blocks(104, |_: usize| {
                StdPad::with_prefix(0x06u8, vec![0x80u8], 104)
                }) {
                super::super::keccak::ops::digest_block(&mut self.0, block.borrow());
            }
            Ok(())
        }
    }

    impl Read for Sha3384 {

        /// Read state as big-endian
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let state_buf = &self.0[..buf.len()/8];
            leu64::encode_slice(buf, state_buf);
            Ok(buf.len())
        }
    }

    impl Hasher for Sha3384 {

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

    impl Digest for Sha3384 {}

    impl DigestExt for Sha3384 {
        fn default_len() -> usize {
            return 48;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use utils::{Digest, DigestExt};
    use super::Sha3384;

    fn digest_to_hex(msg: &str) -> String {
        Sha3384::default().digest(msg.as_bytes()).to_hex()
    }

    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn test_sha3384_empty() {

        assert_eq!(digest_to_hex(""),
                   concat!("0c63a75b845e4f7d01107d852e4c2485",
                           "c51a50aaaa94fc61995e71bbee983a2a",
                           "c3713831264adb47fb6bd1e058d5f004"));
    }
}
