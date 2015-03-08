/// TODO: docs
#[derive(Clone)]
pub struct Sha384([u64; 8], Vec<u8>);

mod impls {
    use std::default::Default;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::beu64;
    use super::Sha384;
    use utils::{Reset,
                Digest,
                DigestExt,
                ReadPadBlocksExt,
                StdPad};

    impl Default for Sha384 {

        /// Construct a default `Sha384` object.
        fn default() -> Sha384 {
            Sha384(super::consts::H, Vec::new())
        }
    }

    impl Reset for Sha384 {

        /// Reset the state
        fn reset(&mut self) {
            self.0 = super::consts::H;
            self.1.clear();
        }
    }

    impl Write for Sha384 {

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

    impl Read for Sha384 {

        /// Read state as big-endian
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let state_buf = &self.0[..buf.len()/8];
            beu64::encode_slice(buf, state_buf);
            Ok(buf.len())
        }
    }

    impl Hasher for Sha384 {

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

    impl Digest for Sha384 {}

    impl DigestExt for Sha384 {
        fn default_len() -> usize {
            return 48;
        }
    }
}

/// TODO
pub mod consts {

    /// TODO
    pub const H: [u64; 8] = [
        0xcbbb9d5dc1059ed8,
        0x629a292a367cd507,
        0x9159015a3070dd17,
        0x152fecd8f70e5939,
        0x67332667ffc00b31,
        0x8eb44a8768581511,
        0xdb0c2e0d64f98fa7,
        0x47b5481dbefa4fa4,
    ];
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use serialize::hex::ToHex;
    use utils::{Digest, DigestExt};
    use super::Sha384;

    fn digest_to_hex(msg: &str) -> String {
        Sha384::default().digest(msg.as_bytes()).to_hex()
    }

    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn test_sha384_empty() {

        assert_eq!(digest_to_hex("").as_slice(),
                   concat!("38b060a751ac96384cd9327eb1b1e36a",
                           "21fdb71114be07434c0cc7bf63f6e1da",
                           "274edebfe76f65fbd51ad2f14898b95b"));
    }
}
