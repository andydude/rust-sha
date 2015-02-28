use std::io;
use std::io::prelude::*;
use std::simd::u32x4;

/// The `Sha256State` private backend to a SHA-1 message digest algorithm.
type Sha256State = (u32x4, u32x4);

/// The `Sha256` public frontend to a SHA-1 message digest algorithm.
pub struct Sha256(Sha256State, Vec<u8>);

    
    //impl Write for Sha256State {
    //    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    //        if buf.len() != BLOCK_LEN*4 {
    //            return Err(
    //                io::Error::new(
    //                    io::ErrorKind::InvalidInput,
    //                    "input buffer must be a block",
    //                    None));
    //        }
    //
    //        // Digest Block
    //        ops::digest_block(self.0, buf);
    //        Ok(BLOCK_LEN*4)
    //    }
    //    fn flush(&mut self) -> io::Result<()> {
    //        Ok(())
    //    }
    //}

//
///// The `Sha256State` private backend to a SHA-1 message digest algorithm.
//pub struct Sha256State([u32; STATE_LEN]);
//
///// The `Sha256` public frontend to a SHA-1 message digest algorithm.
//pub struct Sha256(Sha256State, Vec<u8>);
//
//mod impls {
//    use endian::{ReadEndian, WriteEndian};
//    use std::default::Default;
//    use std::hash::Hasher;
//    use std::io::{Cursor, Read, Write};
//    use std::io;
//    use utils::{Reset, State, WriteBlock};
//    use super::{
//        H, STATE_LEN, BLOCK_LEN,
//        Sha256, Sha256State,
//    };
//
//    //
//    // Sha256State implementations
//    //
//
//    impl State for Sha256State {
//        type Inner = [u32; STATE_LEN];
//
//        /// Set state
//        fn with_state(state: &[u32; STATE_LEN]) -> Self {
//            Sha256State(*state)
//        }
//
//        /// Get state
//        fn get_ref(&self) -> &[u32; STATE_LEN] {
//            &self.0
//        }
//
//        /// Get state
//        fn get_mut(&mut self) -> &mut [u32; STATE_LEN] {
//            &mut self.0
//        }
//    }
//
//    impl WriteBlock for Sha256State {
//        type Block = [u8; 64];
//
//        //type Prefix = u8;
//        //type Suffix = u64;
//
//        /// Return the block size in bytes.
//        //fn block_len() -> usize { BLOCK_LEN*4 }
//
//        /// Reset this to the original state.
//        //fn write_block(&mut self, buf: &[u8]) {
//        //    assert_eq!(buf.len(), <Self as Block>::block_len());
//        //    let mut block = [0u32; BLOCK_LEN];
//        //    (&mut block[..]).write_be(buf).unwrap();
//        //    self.write_block_u32
//        //}
//
//        fn write_block(&mut self, buf: &<Self as WriteBlock>::Block) {
//            use std::mem;
//            use super::good::digest_block;
//            
//            //use super::good::digest_block_u32;
//            //use super::slow::digest_block_u32;
//            //use super::fast::digest_block_u32;
//            //use super::x86emu::digest_block_u32;
//            //use super::x86ffi::digest_block_u32;
//            //use super::x86asm::digest_block_u32;
//            //use super::nettle::digest_block_u32;
//            //use super::openssl::digest_block_u32;
//            //use serialize::hex::ToHex;
//            //println!("Sha256State::Block::write_block()");
//            //digest_block_u32(&mut self.0, &buf);
//            
//            let buf2: &[u8; BLOCK_LEN*4] = unsafe {
//                mem::transmute(buf)
//            };
//            //self.write_bytes(buf2);
//            digest_block(&mut self.0, buf);
//        }
//    }
//
//    impl Default for Sha256State {
//
//        /// Construct a default `Sha256State` object.
//        fn default() -> Sha256State {
//            <Self as State>::with_state(&H)
//        }
//    }
//
//    impl Reset for Sha256State {
//
//        /// TODO
//        fn reset(&mut self) {
//            self.0 = H;
//        }
//    }
//
//    impl Hasher for Sha256State {
//
//        /// TODO
//        fn finish(&self) -> u64 {
//            println!("Sha256State::Hasher::finish()");
//            ((self.0[0] as u64) << 32u64) | (self.0[1] as u64)
//        }
//
//        /// TODO
//        fn write(&mut self, buf: &[u8]) {
//            println!("Sha256State::Hasher::write()");
//            self.write_bytes(buf);
//        }
//    }
//
//    impl AsSlice<u32> for Sha256State {
//
//        /// Get slice
//        fn as_slice<'a>(&'a self) -> &'a [u32] {
//            &self.0[..]
//        }
//    }
//
//    //
//    // Sha256 implementations
//    //
//
//    impl State for Sha256 {
//        type Inner = [u32; STATE_LEN];
//
//        /// Construct with state
//        fn with_state(state: &[u32; STATE_LEN]) -> Self {
//            Sha256(Sha256State(*state), Vec::new())
//        }
//
//        /// Set hasher state
//        //fn set_state(&mut self, from: &[u32; STATE_LEN]) {
//        //    (self.0).0 = *from;
//        //}
//
//        /// Get hasher state
//        fn get_ref(&self) -> &[u32; STATE_LEN] {
//            &(self.0).0
//        }
//
//        /// Get hasher state
//        fn get_mut(&mut self) -> &mut [u32; STATE_LEN] {
//            &mut (self.0).0
//        }
//    }
//
//
//    impl Reset for Sha256 {
//
//        /// TODO
//        fn reset(&mut self) {
//
//            // Reset padder
//            //self.0.reset();
//
//            // Reset hasher
//            //self.0.get_mut().reset();
//        }
//    }
//
//
//    impl AsSlice<u32> for Sha256 {
//
//        /// Get hasher as slice
//        fn as_slice<'a>(&'a self) -> &'a [u32] {
//            self.get_ref().as_slice()
//        }
//    }
//
//    impl Read for Sha256 {
//
//        /// Read state as big-endian
//        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//            self.as_slice().read_be(buf)
//        }
//    }
//
//    impl Write for Sha256 {
//
//        /// TODO
//        fn flush(&mut self) -> io::Result<()> {
//            {
//                use std::iter::repeat;
//                let prefix = 0x80u8;
//                let suffix = (self.1.len()*8) as u64;
//
//                // prefix
//                let blocklen: usize = 64;
//                let buflen = self.1.len() % blocklen;
//                let prefixlen: usize = 1;
//                let suffixlen: usize = 8;
//                let mut written: usize = 0;
//                Write::write(&mut self.1, &[0x80u8]).unwrap();
//
//                // void
//                let big = buflen > blocklen - prefixlen - suffixlen;
//                let padlen = if big {blocklen*2} else {blocklen};
//                let zerolen = (padlen - suffixlen) - (buflen + prefixlen);
//                Extend::extend(&mut self.1, repeat(0u8).take(zerolen));
//
//                // suffix
//                let mut suffixbuf = [0u8; 8];
//                suffix.read_be(&mut suffixbuf).unwrap();
//                Write::write(&mut self.1, &suffixbuf).unwrap();
//            }
//
//            for block in self.1.chunks(64) {
//                self.0.write_bytes(&block);
//            }
//
//            Ok(())
//        }
//
//        /// TODO
//        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
//            Write::write_all(&mut self.1, buf).unwrap();
//            Ok(buf.len())
//        }
//    }
//}
//
