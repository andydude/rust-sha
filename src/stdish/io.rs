use serialize::hex::ToHex;
//use std::old_io::{IoError, IoErrorKind};
//use std::result::Result;

//#[unstable(feature = "cryptoil_internals", reason = "std::hash::Hasher is unstable")]
//pub fn io_error(s: &'static str) -> IoError {
//    IoError{
//        kind: IoErrorKind::InvalidInput,
//        desc: s,
//        detail: None,
//    }
//}

//#[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
//pub trait Read<E: Sized + Error> {
//    // required
//    fn read(&mut self, buf: &mut [u8]) -> Result<usize, E>;
//    
//    // optional
//    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
//    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<(), E> {
//        try!(self.read(buf));
//        Ok(())
//    }
//    
//    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
//    fn read_to_string(&mut self, buf: &mut String) -> Result<(), E> {
//        let mut hash: Vec<u8> = Vec::with_capacity(20);
//        try!(self.read_to_end(&mut hash));
//        buf.clone_from(&hash.to_hex());
//        Ok(())
//
//    }
//}
//
//#[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
//pub trait Write<E: Sized + Error>  {
//    // required
//    fn write(&mut self, buf: &[u8]) -> Result<usize, E>;
//    fn write_all(&mut self, buf: &[u8]) -> Result<(), E>;
//}

/// Compare to std::hash::Hasher
#[unstable(feature = "default", reason = "std::hash::Hasher is unstable")]
pub trait Reset {
    fn reset(&mut self);
}

pub trait HashRead {
    // required
    fn read(&mut self, buf: &mut [u8]);
    
    // optional
    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) {
        self.read(buf);
    }
    
    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
    fn read_to_string(&mut self, buf: &mut String) {
        let mut hash: Vec<u8> = Vec::with_capacity(20);
        self.read_to_end(&mut hash);
        buf.clone_from(&hash.to_hex());
    }
}

pub trait HashWrite {
    // required
    fn write(&mut self, buf: &[u8]);
    fn write_all(&mut self, buf: &[u8]);
}

#[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
pub trait Hasher: Reset + HashRead + HashWrite {
    
    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
    fn digest(&mut self, msg: &[u8], hash: &mut [u8]) {
        self.reset();
        self.write_all(msg);
        self.read(hash);
    }
    
    #[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
    fn hex_digest(&mut self, msg: &str) -> String {
        let mut hash = [0u8; 20];
        self.digest(msg.as_bytes(), &mut hash[0..20]);
        hash.to_hex()
    }
}
