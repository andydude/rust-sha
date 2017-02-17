/// TODO: docs
#[derive(Clone)]
pub struct Shake256([u64; 25], Vec<u8>);

mod impls {
    use std::borrow::Borrow;
    use std::default::Default;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::leu64;
    use super::Shake256;
    use utils::{Reset,
                Digest,
                ReadPadBlocksExt,
                StdPad};

    impl Default for Shake256 {

        /// Construct a default `Shake256` object.
        fn default() -> Shake256 {
            Shake256([0u64; 25], Vec::new())
        }
    }

    impl Reset for Shake256 {

        /// Reset the state
        fn reset(&mut self) {
            self.0 = [0u64; 25];
            self.1.clear();
        }
    }

    impl Write for Shake256 {

        /// Write to buffer
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Write::write(&mut self.1, buf)
        }

        /// Digest buffer
        fn flush(&mut self) -> io::Result<()> {
            let ref buf = self.1;
            for block in buf.pad_blocks(136, |_: usize| {
                StdPad::with_prefix(0x1fu8, vec![0x80u8], 136)
                }) {
                super::super::keccak::ops::digest_block(&mut self.0, block.borrow());
            }
            Ok(())
        }
    }

    impl Read for Shake256 {

        /// Read state as big-endian
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            // this requires special handling
            // because the default digest length
            // crosses a u64 boundary.
            for chunk in buf.chunks_mut(136) {
                if chunk.len() == 136 {
                    {
                        let state_buf = &self.0[..chunk.len()/8];
                        leu64::encode_slice(chunk, state_buf);
                        drop(state_buf)
                    }
                    {
                        let block = [0u8; 200];
                        super::super::keccak::ops::digest_block(&mut self.0, &block[..]);
                    }
                } else {
                    // last chunk
                    let mut buf2 = [0u8; 200];
                    let state_buf = &self.0[..];
                    leu64::encode_slice(&mut buf2[..], state_buf);
                    let buf_len = chunk.len();
                    chunk.copy_from_slice(&buf2[..buf_len]);
                }
            }
            Ok(buf.len())
        }
    }

    impl Hasher for Shake256 {

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

    impl Digest for Shake256 {}
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use utils::Digest;
    use super::Shake256;

    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn test_shake256_empty() {

        assert_eq!(Shake256::default().digest("".as_bytes()).to_hex_len(32),
                   concat!("46b9dd2b0ba88d13233b3feb743eeb24",
                           "3fcd52ea62b81b82b50c27646ed5762f"));
    }

    #[test]
    fn test_shake256_empty_long() {

        assert_eq!(Shake256::default().digest("".as_bytes()).to_hex_len(512),
                   concat!("46b9dd2b0ba88d13233b3feb743eeb24",
                           "3fcd52ea62b81b82b50c27646ed5762f",
                           "d75dc4ddd8c0f200cb05019d67b592f6",
                           "fc821c49479ab48640292eacb3b7c4be",
                           "141e96616fb13957692cc7edd0b45ae3",
                           "dc07223c8e92937bef84bc0eab862853",
                           "349ec75546f58fb7c2775c38462c5010",
                           "d846c185c15111e595522a6bcd16cf86",
                           "f3d122109e3b1fdd943b6aec468a2d62",
                           "1a7c06c6a957c62b54dafc3be87567d6",
                           "77231395f6147293b68ceab7a9e0c58d",
                           "864e8efde4e1b9a46cbe854713672f5c",
                           "aaae314ed9083dab4b099f8e300f01b8",
                           "650f1f4b1d8fcf3f3cb53fb8e9eb2ea2",
                           "03bdc970f50ae55428a91f7f53ac266b",
                           "28419c3778a15fd248d339ede785fb7f",
                           "5a1aaa96d313eacc890936c173cdcd0f",
                           "ab882c45755feb3aed96d477ff96390b",
                           "f9a66d1368b208e21f7c10d04a3dbd4e",
                           "360633e5db4b602601c14cea737db3dc",
                           "f722632cc77851cbdde2aaf0a33a07b3",
                           "73445df490cc8fc1e4160ff118378f11",
                           "f0477de055a81a9eda57a4a2cfb0c839",
                           "29d310912f729ec6cfa36c6ac6a75837",
                           "143045d791cc85eff5b21932f23861bc",
                           "f23a52b5da67eaf7baae0f5fb1369db7",
                           "8f3ac45f8c4ac5671d85735cdddb09d2",
                           "b1e34a1fc066ff4a162cb263d6541274",
                           "ae2fcc865f618abe27c124cd8b074ccd",
                           "516301b91875824d09958f341ef274bd",
                           "ab0bae316339894304e35877b0c28a9b",
                           "1fd166c796b9cc258a064a8f57e27f2a"));
    }
}
