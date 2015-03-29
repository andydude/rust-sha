/// TODO: docs
#[derive(Clone)]
pub struct Sha512224([u64; 8], Vec<u8>);

mod impls {
    use std::default::Default;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::beu64;
    use super::Sha512224;
    use utils::{Reset,
                Digest,
                DigestExt,
                ReadPadBlocksExt,
                StdPad};

    impl Default for Sha512224 {

        /// Construct a default `Sha512224` object.
        fn default() -> Sha512224 {
            Sha512224(super::consts::H, Vec::new())
        }
    }

    impl Reset for Sha512224 {

        /// Reset the state
        fn reset(&mut self) {
            self.0 = super::consts::H;
            self.1.clear();
        }
    }

    impl Write for Sha512224 {

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
                super::super::sha512::ops::digest_block(&mut state, &block);
            }

            self.0 = state;
            Ok(())
        }
    }

    impl Read for Sha512224 {

        /// Read state as big-endian
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            // this requires special handling
            // because the default digest length
            // crosses a u64 boundary.
            use std::slice::bytes::copy_memory;
            let mut buf2 = [0u8; 64];
            let state_buf = &self.0[..];
            beu64::encode_slice(&mut buf2[..], state_buf);
            let buf_len = buf.len();
            copy_memory(buf, &buf2[..buf_len]);
            Ok(buf.len())
        }
    }

    impl Hasher for Sha512224 {

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

    impl Digest for Sha512224 {}

    impl DigestExt for Sha512224 {
        fn default_len() -> usize {
            return 28;
        }
    }
}

/// TODO
pub mod consts {

    /// TODO
    pub const H: [u64; 8] = [
        0x8c3d37c819544da2,
        0x73e1996689dcd4d6,
        0x1dfab7ae32ff9c82,
        0x679dd514582f9fcf,
        0x0f6d2b697bd44da8,
        0x77e36f7304c48942,
        0x3f9d85a86a1d36c8,
        0x1112e6ad91d692a1,
    ];
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use utils::{Digest, DigestExt};
    use super::Sha512224;

    fn digest_to_hex(msg: &str) -> String {
        Sha512224::default().digest(msg.as_bytes()).to_hex()
    }

    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn test_sha512224_empty() {

        assert_eq!(digest_to_hex(""),
                   concat!("6ed0dd02806fa89e",
                           "25de060c19d3ac86",
                           "cabb87d6a0ddd05c",
                           "333b84f4"));
    }
}
