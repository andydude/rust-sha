/// TODO: docs
#[derive(Clone)]
pub struct Shake128([u64; 25], Vec<u8>);

mod impls {
    use std::default::Default;
    use std::hash::Hasher;
    use std::io::prelude::*;
    use std::io;
    use bswap::leu64;
    use super::Shake128;
    use utils::{Reset,
                Digest,
                ReadPadBlocksExt,
                StdPad};

    impl Default for Shake128 {

        /// Construct a default `Shake128` object.
        fn default() -> Shake128 {
            Shake128([0u64; 25], Vec::new())
        }
    }

    impl Reset for Shake128 {

        /// Reset the state
        fn reset(&mut self) {
            self.0 = [0u64; 25];
            self.1.clear();
        }
    }

    impl Write for Shake128 {

        /// Write to buffer
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Write::write(&mut self.1, buf)
        }

        /// Digest buffer
        fn flush(&mut self) -> io::Result<()> {
            let ref buf = self.1;
            for block in buf.pad_blocks(168, |_: usize| {
                StdPad::with_prefix(0x1fu8, vec![0x80u8], 168)
                }) {
                super::super::keccak::ops::digest_block(&mut self.0, block);
            }
            Ok(())
        }
    }

    impl Read for Shake128 {

        /// Read state as big-endian
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            // this requires special handling
            // because the default digest length
            // crosses a u64 boundary.
            use std::slice::bytes::copy_memory;
            for chunk in buf.chunks_mut(168) {
                if chunk.len() == 168 {
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
                    copy_memory(chunk, &buf2[..buf_len]);
                }
            }
            Ok(buf.len())
        }
    }

    impl Hasher for Shake128 {

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

    impl Digest for Shake128 {}
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use utils::Digest;
    use super::Shake128;

    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn test_shake128_empty() {

        assert_eq!(Shake128::default().digest("".as_bytes()).to_hex_len(16),
                   "7f9c2ba4e88f827d616045507605853e");
    }

    #[test]
    fn test_shake128_empty_long() {
        assert_eq!(Shake128::default().digest("".as_bytes()).to_hex_len(512),
                   concat!("7f9c2ba4e88f827d616045507605853e",
                           "d73b8093f6efbc88eb1a6eacfa66ef26",
                           "3cb1eea988004b93103cfb0aeefd2a68",
                           "6e01fa4a58e8a3639ca8a1e3f9ae57e2",
                           "35b8cc873c23dc62b8d260169afa2f75",
                           "ab916a58d974918835d25e6a435085b2",
                           "badfd6dfaac359a5efbb7bcc4b59d538",
                           "df9a04302e10c8bc1cbf1a0b3a5120ea",
                           "17cda7cfad765f5623474d368ccca8af",
                           "0007cd9f5e4c849f167a580b14aabdef",
                           "aee7eef47cb0fca9767be1fda69419df",
                           "b927e9df07348b196691abaeb580b32d",
                           "ef58538b8d23f87732ea63b02b4fa0f4",
                           "873360e2841928cd60dd4cee8cc0d4c9",
                           "22a96188d032675c8ac850933c7aff15",
                           "33b94c834adbb69c6115bad4692d8619",
                           "f90b0cdf8a7b9c264029ac185b70b83f",
                           "2801f2f4b3f70c593ea3aeeb613a7f1b",
                           "1de33fd75081f592305f2e4526edc096",
                           "31b10958f464d889f31ba010250fda7f",
                           "1368ec2967fc84ef2ae9aff268e0b170",
                           "0affc6820b523a3d917135f2dff2ee06",
                           "bfe72b3124721d4a26c04e53a75e30e7",
                           "3a7a9c4a95d91c55d495e9f51dd0b5e9",
                           "d83c6d5e8ce803aa62b8d654db53d09b",
                           "8dcff273cdfeb573fad8bcd45578bec2",
                           "e770d01efde86e721a3f7c6cce275dab",
                           "e6e2143f1af18da7efddc4c7b70b5e34",
                           "5db93cc936bea323491ccb38a388f546",
                           "a9ff00dd4e1300b9b2153d2041d205b4",
                           "43e41b45a653f2a5c4492c1add544512",
                           "dda2529833462b71a41a45be97290b6f"));

    }
}
