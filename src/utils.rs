#![macro_use]
use std::num::{Int, NumCast};
//use serialize::hex::ToHex;
use endian::ReadEndian;
use std::io;
use std::io::Write;
use std::marker::MarkerTrait;

#[macro_export]
macro_rules! as_array {
	($x:expr, $telem:ident, $tsize:expr) => ({
	    let felemsize = ::std::mem::size_of_val(&$x[0]);
	    let telemsize = ::std::mem::size_of::<$telem>();
		assert_eq!(felemsize*$x.len(), telemsize*$tsize);
		let (y, _): (&[$telem; $tsize], usize) = unsafe { 
		    ::std::mem::transmute($x) 
		};
		y
	})
}

#[macro_export]
macro_rules! as_mut_array {
	($x:expr, $telem:ident, $tsize:expr) => ({
	    let felemsize = ::std::mem::size_of_val(&$x[0]);
	    let telemsize = ::std::mem::size_of::<$telem>();
		assert_eq!(felemsize*$x.len(), telemsize*$tsize);
		let (mut y, _): (&mut [$telem; $tsize], usize) = unsafe { 
		    ::std::mem::transmute($x) 
		};
		y
	})
}

///// Read the value of a vector of bytes as a u32 value in big-endian format.
//pub fn read_be_u32(src: &[u8]) -> u32 {
//    return
//        (src[0] as u32) << 24 |
//        (src[1] as u32) << 16 |
//        (src[2] as u32) << 8 |
//        (src[3] as u32);
//}
//
///// Read a vector of bytes into a vector of u32s. The values are read in big-endian format.
//pub fn read_be_u32v(dst: &mut[u32], src: &[u8]) {
//    assert_eq!(dst.len() * 4, src.len());
//
//    for (d, s) in dst.iter_mut().zip(src.chunks(4)) {
//        *d = read_be_u32(s);
//    }
//}
//
///// Read the value of a vector of bytes as a u32 value in big-endian format.
//pub fn read_le_u32(src: &[u8]) -> u32 {
//    return
//        (src[3] as u32) << 24 |
//        (src[2] as u32) << 16 |
//        (src[1] as u32) << 8 |
//        (src[0] as u32);
//}
//
///// Read a vector of bytes into a vector of u32s. The values are read in big-endian format.
//pub fn read_le_u32v(dst: &mut[u32], src: &[u8]) {
//    assert_eq!(dst.len() * 4, src.len());
//
//    for (d, s) in dst.iter_mut().zip(src.chunks(4)) {
//        *d = read_le_u32(s);
//    }
//}
//
///// Write a u32 into a vector, which must be 4 bytes long. The value is written in big-endian
///// format.
//pub fn write_be_u32(dst: &mut[u8], src: u32) {
//    dst[0] = (src >> 24) as u8;
//    dst[1] = (src >> 16) as u8;
//    dst[2] = (src >> 8) as u8;
//    dst[3] = src as u8;
//}
//
///// Read a vector of bytes into a vector of u32s. The values are read in big-endian format.
//pub fn write_be_u32v(dst: &mut[u8], src: &[u32]) {
//    assert_eq!(dst.len(), src.len() * 4);
//
//    for (d, s) in dst.chunks_mut(4).zip(src.iter()) {
//        write_be_u32(d, *s);
//    }
//}
//
///// Write a u8 into a vector, which must be 2 bytes long. The value is written in big-endian
///// format.
//#[inline]
//pub fn write_hex_u8(dst: &mut[u8], src: u8) {
//    macro_rules! hex_digit {
//        ($a:expr) => (
//            ::std::char::from_digit(0x0f & ($a) as u32, 16).unwrap() as u8
//        )
//    }
//    dst[0] = hex_digit!(src >> 4);
//    dst[1] = hex_digit!(src);
//}
//
///// Write a vector of n bytes into a vector of 2*n bytes.
//#[inline]
//pub fn write_hex_u8v(dst: &mut[u8], src: &[u8]) {
//    for (d, s) in dst.chunks_mut(2).zip(src.iter()) {
//        write_hex_u8(d, *s);
//    }
//}
//
///// Write a u32 into a vector, which must be 8 bytes long. The value is written in big-endian
///// format.
//#[inline]
//pub fn write_hex_u32(dst: &mut[u8], src: u32) {
//    macro_rules! hex_digit {
//        ($a:expr) => (
//            ::std::char::from_digit(0x0f & ($a) as u32, 16).unwrap() as u8
//        )
//    }
//    dst[0] = hex_digit!(src >> 28);
//    dst[1] = hex_digit!(src >> 24);
//    dst[2] = hex_digit!(src >> 20);
//    dst[3] = hex_digit!(src >> 16);
//    dst[4] = hex_digit!(src >> 12);
//    dst[5] = hex_digit!(src >> 8);
//    dst[6] = hex_digit!(src >> 4);
//    dst[7] = hex_digit!(src);
//}
//
///// Write a u32 into a vector, which must be 8 bytes long. The value is written in big-endian
///// format.
//#[inline]
//pub fn write_hex_u32v(dst: &mut[u8], src: &[u32]) {
//    for (dch, sch) in dst.chunks_mut(8).zip(src.iter()) {
//        write_hex_u32(dch, *sch);
//    }
//}

//#[inline]
//pub fn padding<P: Int, S: Int>(
//    buf: &mut [u8],
//    msg: &[u8],
//    prefix: P,
//    suffix: S,
//    blocklen: usize) -> usize
//{
//    let prefixlen: usize = ::std::mem::size_of::<P>();
//    let suffixlen: usize = ::std::mem::size_of::<S>();
//    let big = (msg.len() % blocklen) > blocklen - prefixlen - suffixlen;
//    let padlen = if big {blocklen*2} else {blocklen};
//    let zerolen: usize = (padlen - suffixlen) - (msg.len() + prefixlen);
//    assert!(buf.len() >= padlen);
//    println!("padding::buf == {}", buf.to_hex().as_slice());
//    println!("padding::msg == {}", msg.to_hex().as_slice());
//    println!("padding::prefix == {}", prefix.to_u64().unwrap());
//    println!("padding::suffix == {}", suffix.to_u64().unwrap());
//    
//    unsafe {
//        use std::ptr::{copy_memory, set_memory};
//        let prefixptr: *const u8 = &prefix as *const _ as *const u8;
//        let suffixptr: *const u8 = &suffix as *const _ as *const u8;
//        let mut padptr: *mut u8 = buf.as_mut_ptr();
//
//        copy_memory(padptr, msg.as_ptr(), msg.len());
//        padptr = padptr.offset(msg.len() as isize);
//        copy_memory(padptr, prefixptr, prefixlen);
//        padptr = padptr.offset(prefixlen as isize);
//        set_memory(padptr, 0u8, zerolen);
//        padptr = padptr.offset(zerolen as isize);
//        copy_memory(padptr, suffixptr, suffixlen);
//    }
//    
//    padlen
//}
//
//#[inline]
//pub fn move_padding<P: Sized, S: Sized>(
//    blocklen: usize,
//    msg: &mut [u8],
//    prefix: P,
//    suffix: S) -> usize
//{
//    let prefixlen: usize = ::std::mem::size_of::<P>();
//    let suffixlen: usize = ::std::mem::size_of::<S>();
//    let big = (msg.len() % blocklen) > blocklen - prefixlen - suffixlen;
//    let padlen = if big {blocklen*2} else {blocklen};
//    let zerolen: usize = (padlen - suffixlen) - (msg.len() + prefixlen);
//    
//    unsafe {
//        use std::ptr::{copy_memory, set_memory};
//        let prefixptr: *const u8 = &prefix as *const _ as *const u8;
//        let suffixptr: *const u8 = &suffix as *const _ as *const u8;
//        let mut padptr: *mut u8 = msg.as_mut_ptr();
//
//        //copy_memory(padptr, msg.as_ptr(), msg.len());
//        padptr = padptr.offset(msg.len() as isize);
//        copy_memory(padptr, prefixptr, prefixlen);
//        padptr = padptr.offset(prefixlen as isize);
//        set_memory(padptr, 0u8, zerolen);
//        padptr = padptr.offset(zerolen as isize);
//        copy_memory(padptr, suffixptr, suffixlen);
//    }
//    
//    padlen
//}

//#[inline]
//pub fn flush<T: Sized, F: Copy + Fn(&mut T, &[u8])>(
//    blocker: F,
//    blocklen: usize,
//    state: &mut T,
//    msg: &[u8]) -> usize
//{
//    let len = msg.len() - (msg.len() % blocklen);
//    for block in (&msg[0..len]).chunks(blocklen) {
//        blocker(state, block);
//    }
//    
//    len
//}
//
///// Finish
/////
///// * `state`
//#[inline]
//pub fn finish<T: Sized, F: Copy + Fn(&mut T, &[u8]), P: Sized, S: Sized>(
//    blocker: F,
//    blocklen: usize,
//    buf: &mut [u8], 
//    state: &mut T,
//    msg: &[u8], 
//    prefix: P,
//    suffix: S) -> usize
//{
//    let mostlen = flush::<T, F>(blocker, blocklen, state, msg);
//    let padlen = copy_padding::<P, S>(blocklen, buf, &msg[mostlen..], prefix, suffix);
//    let lastlen = flush::<T, F>(blocker, blocklen, state, &buf[..padlen]);
//    
//    (mostlen + lastlen)
//}













#[inline]
pub fn padding<P: Int + ReadEndian, S: Int + ReadEndian>(
    buf: &mut [u8],
    msg: &[u8],
    prefix: P,
    suffix: S,
    blocklen: usize) -> usize
{
    let prefixlen: usize = ::std::mem::size_of::<P>();
    let suffixlen: usize = ::std::mem::size_of::<S>();
    let big = (msg.len() % blocklen) > blocklen - prefixlen - suffixlen;
    let padlen = if big {blocklen*2} else {blocklen};
    let zerolen: usize = (padlen - suffixlen) - (msg.len() + prefixlen);
    assert!(buf.len() >= padlen);
    //println!("padding::prefix == {:x}", prefix.to_u64().unwrap());
    //println!("padding::suffix == {:x}", suffix.to_u64().unwrap());
    //println!("padding::msg == {}", msg.to_hex().as_slice());
    //println!("padding::buf == {}", buf.to_hex().as_slice());
    buf.clone_from_slice(msg);
    prefix.read_be(&mut buf[msg.len()..(msg.len() + prefixlen)]).unwrap();
    suffix.read_be(&mut buf[(padlen - suffixlen)..padlen]).unwrap();
    //println!("padding::buf == {}", buf.to_hex().as_slice());
    
    padlen
}

// -- GENERAL

pub trait FromArray<A: Sized> {
    fn from_array(&mut self, from: &A);
}

pub trait ToArray<A: Sized> {
    fn to_array(&self) -> &A;
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait Reset {
    fn reset(&mut self);
}

pub trait State: MarkerTrait {
    type State: Copy + Sized;
}

pub trait Block {
    type Prefix: Int + ReadEndian;
    type Suffix: Int + ReadEndian;
    
    fn block_len() -> usize;
    fn write_block(&mut self, buf: &[u8]);
}

//pub trait Pad {
//    fn pad(&'a mut self, buf: &[u8]) -> &'a [u8];
//}

pub struct PadWriter<B: Block> {
    inner: B,
    padbuf: Vec<u8>,
    msgbuf: Vec<u8>,
    pub prefix: B::Prefix,
    pub suffix: B::Suffix,
    length: usize,
}

impl<B: Block> PadWriter<B> {

    pub fn new(inner: B) -> Self {
        PadWriter {
            inner: inner,
            padbuf: Vec::with_capacity(<B as Block>::block_len()*2),
            msgbuf: Vec::with_capacity(<B as Block>::block_len()),
            prefix: NumCast::from(0).unwrap(),
            suffix: NumCast::from(0).unwrap(),
            length: 0,
        }
    }

    pub fn get_ref(&self) -> &B {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut B {
        &mut self.inner
    }
}

//impl<B: Block> Pad for PadWriter<B> {
//    
//    fn pad(&'a mut self, buf: &[u8]) -> &'a [u8] {
//        use serialize::hex::ToHex;
//        
//        println!("PadWriter::buf == {}", buf.as_slice().to_hex());
//        println!("PadWriter::blocklen = {}", self.blocklen);
//        println!("PadWriter::prefix = {}", self.prefix.to_u64().unwrap());
//        println!("PadWriter::suffix = {}", self.suffix.to_u64().unwrap());
//        let len = buf.len();
//        println!("PadWriter::len = {}", len);
//        let prefixlen: usize = ::std::mem::size_of::<P>();
//        println!("PadWriter::prefixlen = {}", prefixlen);
//        let suffixlen: usize = ::std::mem::size_of::<S>();
//        println!("PadWriter::suffixlen = {}", suffixlen);
//        let big = (len % self.blocklen) > self.blocklen - prefixlen - suffixlen;
//        println!("PadWriter::big = {}", big);
//        let padlen = if big { self.blocklen*2 } else { self.blocklen };
//        println!("PadWriter::padlen = {}", padlen);
//        let zerolen: usize = (padlen - suffixlen) - (len + prefixlen);
//        println!("PadWriter::zerolen = {}", zerolen);
//
//        self.buffer.resize(padlen, 0u8);
//        (&mut self.buffer[..]).clone_from_slice(buf);
//        self.buffer[padlen - suffixlen - zerolen - prefixlen] = 0x80u8;
//        self.suffix.read_be(&mut self.buffer[(padlen - suffixlen)..padlen]);
//        &self.buffer[..]
//    }
//
//    // Something is wrong with this
//    //fn pad(&mut self, buf: &[u8]) -> Vec<u8> {
//    //    println!("blocklen = {}", self.blocklen);
//    //    let len = buf.len();
//    //    println!("len = {}", len);
//    //    let prefixlen: usize = ::std::mem::size_of::<P>();
//    //    println!("prefixlen = {}", prefixlen);
//    //    let suffixlen: usize = ::std::mem::size_of::<S>();
//    //    println!("suffixlen = {}", suffixlen);
//    //    let big = (len % self.blocklen) > self.blocklen - prefixlen - suffixlen;
//    //    println!("big = {}", big);
//    //    let padlen = if big { self.blocklen*2 } else { self.blocklen };
//    //    println!("padlen = {}", padlen);
//    //    let zerolen: usize = (padlen - suffixlen) - (len + prefixlen);
//    //    println!("zerolen = {}", zerolen);
//    //    self.buffer.resize(len + prefixlen + zerolen + suffixlen, 0u8);
//    //
//    //    unsafe {
//    //        use std::ptr::{copy_memory, set_memory};
//    //        let prefixptr: *const u8 = &self.prefix as *const _ as *const u8;
//    //        let suffixptr: *const u8 = &self.suffix as *const _ as *const u8;
//    //        let mut padptr: *mut u8 = self.buffer.as_mut_ptr();
//    //
//    //        copy_memory(padptr, buf.as_ptr(), len);
//    //        padptr = padptr.offset(len as isize);
//    //        copy_memory(padptr, prefixptr, prefixlen);
//    //        padptr = padptr.offset(prefixlen as isize);
//    //        set_memory(padptr, 0u8, zerolen);
//    //        padptr = padptr.offset(zerolen as isize);
//    //        copy_memory(padptr, suffixptr, suffixlen);
//    //    }
//    //
//    //    //let buff = ::std::str::from_utf8(&self.buffer[..]).unwrap();
//    //    //println!("self.buffer = {}", buff);
//    //
//    //    self.buffer.clone()
//    //}
//}

impl<B: Block> Reset for PadWriter<B> {
    fn reset(&mut self) {
        self.padbuf.clear();
        self.padbuf.resize(<B as Block>::block_len()*2, 0u8);
        self.padbuf.clear();
    }
}
impl<B: Block>  Write for PadWriter<B> {

    ///
    fn flush(&mut self) -> io::Result<()> {
        self.padbuf.clear();
        self.padbuf.resize(<B as Block>::block_len()*2, 0u8);
        self.padbuf.clear();
        
        let blocklen = <B as Block>::block_len();
        let length = self.length + self.msgbuf.len();
        self.suffix = NumCast::from(length*8).unwrap();
        //self.suffix = self.suffix.to_be();
        //println!("PadWriter::length == {}", length);
        //println!("PadWriter::suffix == {}", self.suffix.to_u64().unwrap());
        unsafe { self.padbuf.set_len(blocklen*2); };
        let padlen = padding(&mut self.padbuf[..], &self.msgbuf[..], self.prefix, self.suffix, blocklen);
        unsafe { self.padbuf.set_len(padlen); };
        //println!("PadWriter::padlen == {}", padlen);
        //println!("PadWriter::padbuf == {}", self.padbuf.as_slice().to_hex());
        for block in (&self.padbuf[..]).chunks(blocklen) {
            self.inner.write_block(block);
        }

        self.msgbuf.clear();
        
        self.padbuf.clear();
        self.padbuf.resize(<B as Block>::block_len()*2, 0u8);
        self.padbuf.clear();
        Ok(())
    }

    ///
    // blocklen
    // fn write_block()
    // length: &mut usize 
    // padder: Pad
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        use std::num::ToPrimitive;
        use serialize::hex::ToHex;
        
        //println!("PadWriter::buf == {}", buf.to_hex());
        //println!("PadWriter::write(buf[0..{}])", buf.len());
        //println!("Sha256State::buf = '{}'", ::std::str::from_utf8(buf).unwrap());
        let len = buf.len();
        if len == 0 {
            //println!("PadWriter::len == 0");
            return Ok(0);
        }
        let blocklen = <B as Block>::block_len();
        let written = len - (len % blocklen);
        for block in (&buf[..written]).chunks(blocklen) {
            self.inner.write_block(block);
        }
        
        self.length += written;
        self.msgbuf.resize(buf.len(), 0u8);
        self.msgbuf.clone_from_slice(&buf[written..]);
        Ok(buf.len())
    }
}



//pub struct BufBlockr<W: Block>(BufWriter, W);
//
//impl<W: Block> Write for BufBlockr {
//    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
//        Ok(())
//    }
//}
//pub struct PadWriter<P: Pad, B: Block> {
//    padder: P,
//    blocker: B,
//}
//
//impl PadWriter<P: Pad, B: Block> {
//    fn new(padder: P, blocker: B) -> Self {
//        PadWriter<P, B> {
//            padder: padder,
//            blocker: blocker,
//        }
//    }
//}
//{
//    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
//    fn write_all(&mut self, buf: &[u8]) -> io::Result<()>;
//    fn flush(&mut self) -> Result<()>
//}
