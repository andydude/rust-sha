/// TODO: docs
#[derive(Clone)]
pub struct Sha224([u32; 8], Vec<u8>);

mod impls {
    use std::default::Default;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::{beu32, beu64};
    use super::Sha224;
    use utils::{Reset,
                Digest,
                DigestExt,
                ReadPadBlocksExt,
                StdPad};

    impl Default for Sha224 {

        /// Construct a default `Sha224` object.
        fn default() -> Sha224 {
            Sha224(super::consts::H, Vec::new())
        }
    }

    impl Reset for Sha224 {

        /// Reset the state
        fn reset(&mut self) {
            self.0 = super::consts::H;
            self.1.clear();
        }
    }

    impl Write for Sha224 {

        /// Write to buffer
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
                super::super::sha256::ops::digest_block(&mut state, &block);
            }

            self.0 = state;
            Ok(())
        }
    }

    impl Read for Sha224 {

        /// Read state as big-endian
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let state_buf = &self.0[..buf.len()/4];
            beu32::encode_slice(buf, state_buf);
            Ok(buf.len())
        }
    }

    impl Hasher for Sha224 {

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

    impl Digest for Sha224 {}

    impl DigestExt for Sha224 {
        fn default_len() -> usize {
            return 28;
        }
    }
}

/// TODO
pub mod consts {

    /// TODO
    pub const H: [u32; 8] = [
        0xc1059ed8,
        0x367cd507,
        0x3070dd17,
        0xf70e5939,
        0xffc00b31,
        0x68581511,
        0x64f98fa7,
        0xbefa4fa4,
    ];
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use serialize::hex::ToHex;
    use utils::{Digest, DigestExt};
    use super::Sha224;

    fn digest_to_hex(msg: &str) -> String {
        Sha224::default().digest(msg.as_bytes()).to_hex()
    }

    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn test_sha224_empty() {

        assert_eq!(digest_to_hex("").as_slice(),
                   concat!("d14a028c2a3a2bc9",
                           "476102bb288234c4",
                           "15a2b01f828ea62a",
                           "c5b3e42f"));
                   
    }
}
