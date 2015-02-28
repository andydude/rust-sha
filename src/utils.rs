#![macro_use]

//use endian::ReadEndian;
//use std::iter::{Iterator, Extend};
//use std::marker::MarkerTrait;
//use std::mem;
//use std::num::Int;
//use std::ptr;
use std::default::Default;
use std::io::{Read, Write};
use std::iter::Chain;
use std::iter::IntoIterator;
use std::vec::IntoIter;

pub trait Reset {
    fn reset(&mut self);
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

pub fn std_pad_len(len: usize, reqlen: usize) -> usize {
    let blocklen: usize = 64;
    let buflen = len % blocklen;
    let big = buflen > blocklen - reqlen;
    let padlen = if big {blocklen*2} else {blocklen};
    let zerolen = padlen - buflen - reqlen;
    zerolen
}

pub fn std_pad(len: usize) -> Vec<u8> {
    use bswap::beu64;
    use serialize::hex::ToHex;
    let mut v: Vec<u8> = Vec::new();
    v.push(0x80);
    v.resize(std_pad_len(len, 9) + 1, 0);
    let mut suffix = [0u8; 8];
    beu64::encode(&mut suffix[..], (8*len) as u64);
    Write::write(&mut v, &suffix[..]).unwrap();
    //println!("{}", v.as_slice().to_hex());
    //assert_eq!((len + v.len()) % 64, 0);
    v
}

pub struct StdPad(IntoIter<u8>);

impl StdPad {
    pub fn new(len: usize) -> StdPad {
        let v = std_pad(len);
        StdPad(v.into_iter())
    }
}

impl Iterator for StdPad {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        self.0.next()
    }
}


pub trait RecurrenceExt<T> {
    fn recurrence_map<F: FnMut(&[T]) -> T>(&self, f: F)
        -> RecurrenceMap<T, F>;
    fn chain_recurrence_map<F: FnMut(&[T]) -> T>(&self, f: F)
        -> Chain<IntoIter<T>, RecurrenceMap<T, F>>;
}

impl <T: Copy + Clone + Sized> RecurrenceExt<T> for [T] {
    fn recurrence_map<F: FnMut(&[T]) -> T>(&self, f: F)
        -> RecurrenceMap<T, F> {
        RecurrenceMap{v: self.to_vec(), f: f, size: self.len()}
    }
    fn chain_recurrence_map<F: FnMut(&[T]) -> T>(&self, f: F)
        -> Chain<IntoIter<T>, RecurrenceMap<T, F>> {
        self.to_vec().into_iter().chain(self.recurrence_map(f))
    }
}

impl <T: Copy + Clone + Sized> RecurrenceExt<T> for Vec<T> {
    fn recurrence_map<F: FnMut(&[T]) -> T>(&self, f: F)
        -> RecurrenceMap<T, F> {
        RecurrenceMap{v: self.to_vec(), f: f, size: self.len()}
    }
    fn chain_recurrence_map<F: FnMut(&[T]) -> T>(&self, f: F)
        -> Chain<IntoIter<T>, RecurrenceMap<T, F>> {
        self.to_vec().into_iter().chain(self.recurrence_map(f))
    }
}

pub struct RecurrenceMap<T: Copy + Sized, F: FnMut(&[T]) -> T> {
    v: Vec<T>,
    f: F,
    size: usize,
}

impl <T: Copy + Sized, F: FnMut(&[T]) -> T> Iterator for
 RecurrenceMap<T, F> {type
    Item = T;
    
    #[inline]
    fn next(&mut self) -> Option<T> {
        let pos = self.v.len();
        let item = (self.f)(&self.v[pos - self.size..pos]);
        self.v.push(item);
        Some(item)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
