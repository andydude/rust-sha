#![macro_use]

//use endian::ReadEndian;
//use std::iter::{Iterator, Extend};
//use std::marker::MarkerTrait;
//use std::mem;
//use std::num::Int;
//use std::ptr;
//use std::iter::Chain;
//use std::iter::IntoIterator;
//use std::vec::IntoIter;
use std::default::Default;
use std::io::prelude::*;
use std::io;
use bswap::beu64;


pub trait Reset {
    fn reset(&mut self);
}

pub trait ReadBlock {
    type Block: Copy + Clone + Sized;
    
    fn read_block(&mut self, block: &mut <Self as ReadBlock>::Block);
}

pub trait WriteBlock {
    type Block: Copy + Clone + Sized;
    
    fn write_block(&mut self, block: &<Self as WriteBlock>::Block);
}

pub trait Digest: Default + Reset + Read + Write {
    
    fn finish_len(&mut self, len: usize) -> Vec<u8> {
        self.flush().unwrap();
        let mut bytes: Vec<u8> = Vec::with_capacity(len);
        unsafe { bytes.set_len(len); };
        self.read_to_end(&mut bytes).unwrap();
        bytes
    }
    
    fn digest_len(&mut self, msg: &[u8], len: usize) -> Vec<u8> {
        self.reset();
        self.write_all(msg).unwrap();
        self.finish_len(len)
    }
}

pub trait DigestExt: Digest {
    // required
    fn default_len() -> usize;
    
    fn finish(&mut self) -> Vec<u8> {
        self.flush().unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.read_to_end(&mut bytes).unwrap();
        bytes
    }
    
    fn digest(&mut self, msg: &[u8]) -> Vec<u8> {
        self.reset();
        self.write_all(msg).unwrap();
        self.finish()
    }
}


//pub fn std_pad(len: usize) -> Vec<u8> {
//    use bswap::beu64;
//    //use serialize::hex::ToHex;
//    let mut v: Vec<u8> = Vec::new();
//    v.push(0x80);
//    v.resize(std_pad_len(len, 9) + 1, 0);
//    let mut suffix = [0u8; 8];
//    beu64::encode(&mut suffix[..], (8*len) as u64);
//    Write::write(&mut v, &suffix[..]).unwrap();
//    //println!("{}", v.as_slice().to_hex());
//    //assert_eq!((len + v.len()) % 64, 0);
//    v
//}

pub struct StdPad {
    block_len: usize,
    len: usize,
    pos: usize,
}

impl StdPad {
    pub fn new(len: usize) -> StdPad {
        StdPad{block_len: 64, len: len, pos: 0}
    }

}

impl Read for StdPad {
    
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        use std::slice::bytes::{MutableByteVector, copy_memory};
            
        fn std_pad_len(len: usize, req_len: usize, block_len: usize) -> usize {
            let last_len = len % block_len;
            let big = if last_len + req_len > block_len {2} else {1};
            (big*block_len - last_len)
        }

        // prefix
        buf[0] = 0x80;

        // padding
        let pad_len = std_pad_len(self.len, 9, self.block_len);
        (&mut buf[1 .. pad_len - 8]).set_memory(0);

        // suffix
        let mut suffix = [0u8; 8];
        beu64::encode(&mut suffix[..], (8*self.len) as u64);
        copy_memory(&mut buf[pad_len - 8 .. pad_len], &suffix[..]);

        Ok(pad_len)
    }
}


//pub trait RecurrenceExt<T> {
//    fn recurrence_map<F: FnMut(&[T]) -> T>(&self, f: F)
//        -> RecurrenceMap<T, F>;
//    fn chain_recurrence_map<F: FnMut(&[T]) -> T>(&self, f: F)
//        -> Chain<IntoIter<T>, RecurrenceMap<T, F>>;
//}
//
//impl <T: Copy + Clone + Sized> RecurrenceExt<T> for [T] {
//    fn recurrence_map<F: FnMut(&[T]) -> T>(&self, f: F)
//        -> RecurrenceMap<T, F> {
//        RecurrenceMap{v: self.to_vec(), f: f, size: self.len()}
//    }
//    fn chain_recurrence_map<F: FnMut(&[T]) -> T>(&self, f: F)
//        -> Chain<IntoIter<T>, RecurrenceMap<T, F>> {
//        self.to_vec().into_iter().chain(self.recurrence_map(f))
//    }
//}
//
//impl <T: Copy + Clone + Sized> RecurrenceExt<T> for Vec<T> {
//    fn recurrence_map<F: FnMut(&[T]) -> T>(&self, f: F)
//        -> RecurrenceMap<T, F> {
//        RecurrenceMap{v: self.to_vec(), f: f, size: self.len()}
//    }
//    fn chain_recurrence_map<F: FnMut(&[T]) -> T>(&self, f: F)
//        -> Chain<IntoIter<T>, RecurrenceMap<T, F>> {
//        self.to_vec().into_iter().chain(self.recurrence_map(f))
//    }
//}
//
//pub struct RecurrenceMap<T: Copy + Sized, F: FnMut(&[T]) -> T> {
//    v: Vec<T>,
//    f: F,
//    size: usize,
//}
//
//impl <T: Copy + Sized, F: FnMut(&[T]) -> T> Iterator for
// RecurrenceMap<T, F> {type
//    Item = T;
//    
//    #[inline]
//    fn next(&mut self) -> Option<T> {
//        let pos = self.v.len();
//        let item = (self.f)(&self.v[pos - self.size..pos]);
//        self.v.push(item);
//        Some(item)
//    }
//    #[inline]
//    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
//}

