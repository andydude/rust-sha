/// TODO: docs
#[derive(Clone)]
pub struct Sha512256([u64; 8], Vec<u8>);

mod impls {
    use std::borrow::Borrow;
    use std::default::Default;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::beu64;
    use super::Sha512256;
    use utils::{Reset,
                Digest,
                DigestExt,
                ReadPadBlocksExt,
                StdPad};

    impl Default for Sha512256 {

        /// Construct a default `Sha512256` object.
        fn default() -> Sha512256 {
            Sha512256(super::consts::H, Vec::new())
        }
    }

    impl Reset for Sha512256 {

        /// Reset the state
        fn reset(&mut self) {
            self.0 = super::consts::H;
            self.1.clear();
        }
    }

    impl Write for Sha512256 {

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
                super::super::sha512::ops::digest_block(&mut state, block.borrow());
            }

            self.0 = state;
            Ok(())
        }
    }

    impl Read for Sha512256 {

        /// Read state as big-endian
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let state_buf = &self.0[..buf.len()/8];
            beu64::encode_slice(buf, state_buf);
            Ok(buf.len())
        }
    }

    impl Hasher for Sha512256 {

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

    impl Digest for Sha512256 {}

    impl DigestExt for Sha512256 {
        fn default_len() -> usize {
            return 32;
        }
    }
}

/// TODO
pub mod consts {

    /// TODO
    pub const H: [u64; 8] = [
        0x22312194fc2bf72c,
        0x9f555fa3c84c64c2,
        0x2393b86b6f53b151,
        0x963877195940eabd,
        0x96283ee2a88effe3,
        0xbe5e1e2553863992,
        0x2b0199fc2c85b8aa,
        0x0eb72ddc81c52ca2,
    ];
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use utils::{Digest, DigestExt};
    use super::Sha512256;

    fn digest_to_hex(msg: &str) -> String {
        Sha512256::default().digest(msg.as_bytes()).to_hex()
    }

    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn test_sha512256_empty() {

        assert_eq!(digest_to_hex(""),
                   concat!("c672b8d1ef56ed28ab87c3622c511406",
                           "9bdd3ad7b8f9737498d0c01ecef0967a"));
    }
}
