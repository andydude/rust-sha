//use serialize::hex::ToHex;
//use std::simd::u32x4;
//use std::old_io::IoError;
//use super::stdish::io::{Hasher, Reset, HashRead, HashWrite};

//#[unstable(feature = "default", reason = "1.0.0")]
//pub mod hw;
//
//#[unstable(feature = "default", reason = "1.0.0")]
//pub mod sw;
//
//#[unstable(feature = "default", reason = "1.0.0")]
//pub mod sw_nettle;
//
//#[unstable(feature = "default", reason = "1.0.0")]
//pub mod sw_openssl;

#[stable(feature = "default", since = "1.0.0")]
#[derive(Clone, Debug)]
pub struct Sha1 {

    #[stable(feature = "default", since = "1.0.0")]
    pub finished: bool,

    #[stable(feature = "default", since = "1.0.0")]
    pub length: usize,

    #[stable(feature = "default", since = "1.0.0")]
    pub state: Vec<u8>,
        
    #[stable(feature = "default", since = "1.0.0")]
    pub block: Vec<u8>,
}

#[stable(feature = "default", since = "1.0.0")]
impl Sha1 {

    #[stable(feature = "default", since = "1.0.0")]
    pub fn new() -> Sha1 {
        Sha1 {
            finished: false,
            length: 0us,
            state: Vec::with_capacity(20),  // (20 bytes/hash)
            block: Vec::with_capacity(128), // (64 bytes/block)*(2 blocks)
        }
    }
}

//impl HashRead for Sha1 {
//
//    /// Read a 20-byte message digest
//    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
//    fn read(&mut self, buf: &mut [u8]) {
//        use std::slice::bytes::copy_memory;
//        
//        if buf.len() < 20us {
//            panic!("Digest read buf must be exactly 20 bytes");
//        }
//        if !self.finished {
//            panic!("Digest read requires a finished state");
//        }
//
//        // dst <- src
//        copy_memory(buf, &self.state);
//        
//        //Ok(20us)
//    }
//
//    /// Read the message digest
//    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
//    fn read_to_end(&mut self, buf: &mut Vec<u8>) {
//        buf.resize(20, 0u8);
//        self.read(buf.as_mut_slice());
//    }
//}

//use std::io::{Write, Result};
//
//impl Write for Sha1 {
//    /// Write a 64-byte message block
//    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
//    fn write(&mut self, buf: &[u8]) -> Result<usize> {
//    
//        if buf.len() < 64us {
//            panic!("Digest write buf must be exactly 64 bytes");
//        }
//        if self.finished {
//            panic!("Digest write requires an unfinished state");
//        }
//    
//        sw::digest_block(&mut self.state, buf);
//    
//        // the length of the last block doesn't count
//        self.length +=  64us;
//        
//        //Ok(64us)
//    }
//
//    /// Write a bytestring message
//    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
//    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
//        sw::write_last(&mut self.state, &mut self.block, buf, self.length);
//
//        // the last block increments the length
//        // so we decrement it to the right value
//        self.length -= 64us;
//        self.finished = true;
//    }
//}

use std::hash::{Hasher, Writer};

impl Writer for Sha1 {

    /// Write a bytestring message
    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
    fn write(&mut self, buf: &[u8]) {
        sw::write_last(&mut self.state, &mut self.block, buf, self.length);

        // the last block increments the length
        // so we decrement it to the right value
        self.length -= 64us;
        self.finished = true;
    }
}

impl Hasher for Sha1 {
    type Output = Vec<u8>;

    fn reset(&mut self) {
        unsafe {
            self.state.set_len(20);
            self.block.set_len(0);
        }

        // initialize hash state
        sw::reset(&mut self.state);

        // prepare for next time
        self.finished = false;
        self.length = 0;
    }
    
    fn finish(&self) -> Vec<u8> {
        self.state.to_vec()
    }
}

//impl Hash<Sha1> for [u8] {
//    fn hash(&self, state: &mut Sha1) {
//        state.write(&self);
//    }
//}

