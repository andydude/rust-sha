/// TODO: docs
#[derive(Clone)]
pub struct Sha3512([u64; 25], Vec<u8>);

mod impls {
    use std::borrow::Borrow;
    use std::default::Default;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::leu64;
    use super::Sha3512;
    use utils::{Reset,
                Digest,
                DigestExt,
                ReadPadBlocksExt,
                StdPad};

    impl Default for Sha3512 {

        /// Construct a default `Sha3512` object.
        fn default() -> Sha3512 {
            Sha3512([0u64; 25], Vec::new())
        }
    }

    impl Reset for Sha3512 {

        /// Reset the state
        fn reset(&mut self) {
            self.0 = [0u64; 25];
            self.1.clear();
        }
    }

    impl Write for Sha3512 {

        /// Write to buffer
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Write::write(&mut self.1, buf)
        }

        /// Digest buffer
        fn flush(&mut self) -> io::Result<()> {
            let ref buf = self.1;
            for block in buf.pad_blocks(72, |_: usize| {
                StdPad::with_prefix(0x06u8, vec![0x80u8], 72)
                }) {
                super::super::keccak::ops::digest_block(&mut self.0, block.borrow());
            }
            Ok(())
        }
    }

    impl Read for Sha3512 {

        /// Read state as big-endian
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let state_buf = &self.0[..buf.len()/8];
            leu64::encode_slice(buf, state_buf);
            Ok(buf.len())
        }
    }

    impl Hasher for Sha3512 {

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

    impl Digest for Sha3512 {}

    impl DigestExt for Sha3512 {
        fn default_len() -> usize {
            return 64;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use utils::{Digest, DigestExt};
    use super::Sha3512;

    fn digest_to_hex(msg: &str) -> String {
        Sha3512::default().digest(msg.as_bytes()).to_hex()
    }

    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn test_sha3512_empty() {

        assert_eq!(digest_to_hex(""),
                   concat!("a69f73cca23a9ac5c8b567dc185a756e",
                           "97c982164fe25859e0d1dcc1475c80a6",
                           "15b2123af1f5f94c11e3e9402c3ac558",
                           "f500199d95b6d3e301758586281dcd26"));
    }
}
