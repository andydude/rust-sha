use std::default::Default;
use std::io::prelude::*;
use std::io;
use serialize::hex::ToHex;

pub trait Reset {
    fn reset(&mut self);
}

pub trait Digest: Default + Reset + Read + Write {

    fn digest(&mut self, msg: &[u8]) -> &mut Self {
        self.reset();
        self.write_all(msg).unwrap();
        self.flush().unwrap();
        self
    }

    fn to_bytes_len(&mut self, len: usize) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(len);
        unsafe { bytes.set_len(len); };
        self.read(&mut bytes[..]).unwrap();
        bytes
    }
    
    fn to_hex_len(&mut self, len: usize) -> String {
        self.to_bytes_len(len).as_slice().to_hex()
    }
}

pub trait DigestExt: Digest {
    // required
    fn default_len() -> usize;

    fn to_bytes(&mut self) -> Vec<u8> {
        self.to_bytes_len(<Self as DigestExt>::default_len())
    }
    
    fn to_hex(&mut self) -> String {
        self.to_bytes().as_slice().to_hex()
    }
}

//
// Library
//
//
//
///// A Pad can return a ReadPad.
//pub trait Pad {
//    type Output: ReadPad;
//
//    fn block_len(&self) -> usize;
//    fn pad(&self, len: usize) -> <Self as Pad>::Output;
//}

/// A ReadPad can read to a buffer.
pub trait ReadPad {
    fn read_pad(&self, buf: &mut Vec<u8>) -> io::Result<usize>;
}

pub trait ReadPadBlocksExt: Read + Sized {
	fn pad_blocks<P: ReadPad, F: Fn(usize) -> P>(
        self, block_len: usize, padder: F) -> PadBlocks<Self, P, F> {
        let mut buf = Vec::with_capacity(2*block_len);
        buf.resize(block_len, 0);
        
        PadBlocks {
            inner: self,
            padder: padder,
            block_len: block_len,
            buf: buf,
            len: 0,
            finished: false,
        }
    }
}
impl<T: Read> ReadPadBlocksExt for T {}

pub struct PadBlocks<I: Read, P: ReadPad, F: Fn(usize) -> P> {
    inner: I,
    padder: F,
    block_len: usize,
    buf: Vec<u8>,
    len: usize,
    finished: bool,
}

impl<I: Read, P: ReadPad, F: Fn(usize) -> P> PadBlocks<I, P, F> {

    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.finished {
            if self.buf.len() > self.block_len && self.len > 0 {
                self.len = 0;
                return Ok(&self.buf[self.block_len..self.block_len*2]);
            }
            return Ok(&self.buf[0..0]);
        }
        
        self.buf.clear();
        unsafe { self.buf.set_len(self.block_len); };
        let mut len = try!(self.inner.read(&mut self.buf[..]));
        self.len += len;
            
        if len < self.block_len {
            unsafe { self.buf.set_len(len); };
            let reader: P = (self.padder)(self.len);
            len += try!(reader.read_pad(&mut self.buf));
            //assert_eq!(self.buf.len(), self.block_len);
            self.finished = true;
        }
        
        if len < self.block_len && len > 0 {
            return Err(io::Error::new(
                    io::ErrorKind::ResourceUnavailable,
                    "unable to read block to end", None));
        }
        
        Ok(&self.buf[..self.block_len])
    }
}

impl<'a, I: Read, P: ReadPad, F: Fn(usize) -> P> Iterator for PadBlocks<I, P, F> {
    type Item = &'a [u8];
    
    fn next<'b>(&'b mut self) -> Option<&'b [u8]> {
        let buf = self.fill_buf().unwrap();
        if buf.len() == 0 { return None };
        Some(buf)
    }
}

pub struct StdPad {
    prefix: u8,
    suffix: Vec<u8>,
    block_len: usize,
}

impl StdPad {
    pub fn new(suffix: Vec<u8>, block_len: usize) -> StdPad {
        StdPad {
            prefix: 0x80,
            suffix: suffix,
            block_len: block_len,
        }
    }
}

impl ReadPad for StdPad {
    
    fn read_pad(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let len = buf.len() + 1 + self.suffix.len();
        let big = if len > self.block_len {2} else {1};
        let written = self.block_len*big - buf.len();

        buf.push(self.prefix);
        buf.resize(self.block_len*big - self.suffix.len(), 0);
        try!(Write::write(buf, &self.suffix[..]));

        Ok(written)
    }
}


//
//
//
//
//
//
//
//
//
//
//pub trait Pad {
//    type Output: ReadPad;
//
//    /// A Pad can return a ReadPad.
//    fn pad(&self, len: usize) -> <Self as Pad>::Output;
//}
//
//pub trait ReadPad {
//
//    /// A ReadPad can read to a buffer.
//    fn read_pad(&self, buf: &mut Vec<u8>) -> io::Result<usize>;
//}
//
//pub trait ReadPadBlocksExt: Read + Sized {
//	fn pad_blocks<'a, P: Pad>(&'a mut self, block_len: usize, padder: &'a P) -> PadBlocks<'a, Self, P> {
//        let mut buf = Vec::with_capacity(2*block_len);
//        buf.resize(block_len, 0);
//        
//        PadBlocks {
//            inner: self,
//            padder: padder,
//            block_len: block_len,
//            buf: buf,
//            len: 0,
//        }
//    }
//}
//
//impl<T: Read> ReadPadBlocksExt for T {}
//
//pub struct PadBlocks<'a, I: 'a + Read, P: 'a + Pad> {
//    inner: &'a mut I,
//    padder: P,
//    block_len: usize,
//    buf: Vec<u8>,
//    len: usize,
//}
//
//macro_rules! try_or_none {
//    ($expr:expr) => (match $expr {
//        Ok(val) => val,
//        Err(err) => {
//            return None
//        }
//    })
//}
//
//impl<'a, I: Read, P: Pad> Iterator for PadBlocks<'a, I, P> {
//    type Item = &'a [u8];
//    
//    fn next(&mut self) -> Option<&'a [u8]> {
//        let len = try_or_none!(self.inner.read(&mut self.buf[..]));
//        self.len += len;
//            
//        if len < self.block_len {
//            unsafe { self.buf.set_len(len); };
//            let reader: <P as Pad>::Output = self.padder.pad(self.len);
//            self.len += try_or_none!(reader.read_pad(&mut self.buf));
//            assert_eq!(self.buf.len(), self.block_len);
//        }
//        
//        if len < self.block_len && len > 0 {
//            return None;
//            //return Err(io::Error::new(
//            //        io::ErrorKind::ResourceUnavailable,
//            //        "unable to read block to end", None));
//        }
//        
//        Some(&self.buf[..])
//    }
//}
//    //impl<'a> ReadBlock for io::Chain<io::Cursor<&'a [u8]>, StdPad> {
//    //    type Block = [u8; 64];
//    //    
//    //    fn read_block(&mut self, block: &mut <Self as ReadBlock>::Block) -> io::Result<usize> {
//    //        Ok(size)
//    //    }
//    //}
//
//pub struct StdPad {
//    prefix: u8,
//    suffix: Vec<u8>,
//    block_len: usize,
//}
//
//impl StdPad {
//    pub fn new(suffix: Vec<u8>, block_len: usize) -> StdPad {
//        StdPad {
//            prefix: 0x80,
//            suffix: suffix,
//            block_len: block_len,
//        }
//    }
//}
//
//impl ReadPad for StdPad {
//    fn read_pad(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
//        let len = buf.len() + 1 + self.suffix.len();
//        let big = if len > self.block_len {2} else {1};
//
//        buf.push(self.prefix);
//        buf.resize(self.block_len*big - self.suffix.len(), 0);
//        Write::write(buf, &self.suffix[..]);
//    }
//}

//impl<'a> Read for StdPad<'a> {
//
//    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//        use std::slice::bytes::{MutableByteVector, copy_memory};
//
//        if self.pos == 0 {
//            self.block_len = buf.len();
//        }
//        
//        let rem_len = self.len % self.block_len;
//        let nblocks = if rem_len + 9 > self.block_len {2} else {1};
//        let mut pad_len = nblocks*self.block_len - rem_len;
//
//        if pad_len <= self.pos {
//            return Ok(0);
//        }
//
//        if pad_len > buf.len() {
//            if self.pos == 0 {
//                pad_len -= self.block_len;
//
//                // prefix
//                buf[0] = 0x80;
//            
//                // padding
//                (&mut buf[1 .. pad_len]).set_memory(0);
//            } else {
//                pad_len = self.block_len;
//
//                // padding
//                (&mut buf[0 .. pad_len - 8]).set_memory(0);
//
//                // suffix
//                let mut suffix = [0u8; 8];
//                beu64::encode(&mut suffix[..], (8*self.len) as u64);
//                copy_memory(&mut buf[pad_len - 8 .. pad_len], &suffix[..]);
//            }
//        } else {
//        
//            // prefix
//            buf[0] = 0x80;
//            
//            // padding
//            (&mut buf[1 .. pad_len - 8]).set_memory(0);
//            
//            // suffix
//            let mut suffix = [0u8; 8];
//            beu64::encode(&mut suffix[..], (8*self.len) as u64);
//            copy_memory(&mut buf[pad_len - 8 .. pad_len], &suffix[..]);
//        }
//
//        self.pos += pad_len;
//        Ok(pad_len)
//    }
//}
