use serialize::hex::ToHex;
use std::error::Error;
use std::hash::Hasher;
use std::old_io::{IoError, IoErrorKind};
use std::result::Result;

pub fn io_error(s: &'static str) -> IoError {
    IoError{
        kind: IoErrorKind::InvalidInput,
        desc: s,
        detail: None,
    }
}

pub trait Read<E: Sized + Error> {
    // required
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, E>;
    
    // optional
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<(), E> {
        try!(self.read(buf));
        Ok(())
    }
    
    fn read_to_string(&mut self, buf: &mut String) -> Result<(), E> {
        let mut hash: Vec<u8> = Vec::with_capacity(20);
        try!(self.read_to_end(&mut hash));
        buf.clone_from(&hash.to_hex());
        Ok(())

    }
}

pub trait Write<E: Sized + Error>  {
    // required
    fn write(&mut self, buf: &[u8]) -> Result<usize, E>;
    fn write_all(&mut self, buf: &[u8]) -> Result<(), E>;
}

/// Compare to std::hash::Hasher
pub trait Reset {
    fn reset(&mut self);
}

pub trait Digest<E: Sized + Error>: Sized + Reset + Read<E> + Write<E> {
    
    fn digest(&mut self, msg: &[u8], hash: &mut [u8]) -> Result<(), E> {
        self.reset();
        try!(self.write_all(msg));
        try!(self.read(hash));
        Ok(())
    }
    
    fn hex_digest(&mut self, msg: &str) -> Result<String, E> {
        let mut hash = [0u8; 20];
        try!(self.digest(msg.as_bytes(), &mut hash[0..20]));
        Ok(hash.to_hex())
    }
}
