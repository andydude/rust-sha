/// TODO: docs
#[derive(Clone)]
pub struct Sha3224([u64; 25], Vec<u8>);

mod impls {
    use std::borrow::Borrow;
    use std::default::Default;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::leu64;
    use super::Sha3224;
    use utils::{Reset,
                Digest,
                DigestExt,
                ReadPadBlocksExt,
                StdPad};

    impl Default for Sha3224 {

        /// Construct a default `Sha3224` object.
        fn default() -> Sha3224 {
            Sha3224([0u64; 25], Vec::new())
        }
    }

    impl Reset for Sha3224 {

        /// Reset the state
        fn reset(&mut self) {
            self.0 = [0u64; 25];
            self.1.clear();
        }
    }

    impl Write for Sha3224 {

        /// Write to buffer
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Write::write(&mut self.1, buf)
        }

        /// Digest buffer
        fn flush(&mut self) -> io::Result<()> {
            let ref buf = self.1;
            for block in buf.pad_blocks(144, |_: usize| {
                StdPad::with_prefix(0x06u8, vec![0x80u8], 144)
                }) {
                super::super::keccak::ops::digest_block(&mut self.0, block.borrow());
            }
            Ok(())
        }
    }

    impl Read for Sha3224 {

        /// Read state as big-endian
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            // this requires special handling
            // because the default digest length
            // crosses a u64 boundary.
            let mut buf2 = [0u8; 200];
            let state_buf = &self.0[..];
            leu64::encode_slice(&mut buf2[..], state_buf);
            let buf_len = buf.len();
            buf.copy_from_slice(&buf2[..buf_len]);
            Ok(buf.len())
        }
    }

    impl Hasher for Sha3224 {

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

    impl Digest for Sha3224 {}

    impl DigestExt for Sha3224 {
        fn default_len() -> usize {
            return 28;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use utils::{Digest, DigestExt};
    use super::Sha3224;

    fn digest_to_hex(msg: &str) -> String {
        Sha3224::default().digest(msg.as_bytes()).to_hex()
    }

    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn test_sha3224_empty() {

        assert_eq!(digest_to_hex(""),
                   concat!("6b4e03423667dbb7",
                           "3b6e15454f0eb1ab",
                           "d4597f9a1b078e3f",
                           "5b5a6bc7"));
    }
}
