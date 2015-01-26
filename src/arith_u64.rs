
#[inline]
pub fn rotl(x: u64, y: usize) -> u64 {
    return (x << y) | (x >> (64 - y));
}

#[inline]
pub fn rotr(x: u64, y: usize) -> u64 {
    return (x >> y) | (x << (64 - y));
}

#[inline]
pub fn shl(x: u64, y: usize) -> u64 {
    return x << y;
}

#[inline]
pub fn shr(x: u64, y: usize) -> u64 {
    return x >> y;
}

#[inline]
pub fn from_be(v: &[u8]) -> u64 {
    return (v[0] as u64) << 56 
         | (v[1] as u64) << 48 
         | (v[2] as u64) << 40
         | (v[3] as u64) << 32
         | (v[4] as u64) << 24 
         | (v[5] as u64) << 16 
         | (v[6] as u64) << 8 
         | (v[7] as u64);
}

#[inline]
pub fn from_le(v: &[u8]) -> u64 {
    return (v[7] as u64) << 56 
         | (v[6] as u64) << 48 
         | (v[5] as u64) << 40
         | (v[4] as u64) << 32
         | (v[3] as u64) << 24 
         | (v[2] as u64) << 16 
         | (v[1] as u64) << 8 
         | (v[0] as u64);
}

#[inline]
pub fn to_be(x: u64) -> Vec<u8> {
    return vec![((x >> 56)&0xff) as u8,
                ((x >> 48)&0xff) as u8,
                ((x >> 40)&0xff) as u8,
                ((x >> 32)&0xff) as u8,
                ((x >> 24)&0xff) as u8,
                ((x >> 16)&0xff) as u8,
                ((x >> 8)&0xff) as u8,
                (x&0xff) as u8];
}

#[inline]
pub fn to_le(x: u64) -> Vec<u8> {
    return vec![((x)&0xff) as u8,
                ((x >> 8)&0xff) as u8,
                ((x >> 16)&0xff) as u8,
                ((x >> 24)&0xff) as u8,
                ((x >> 32)&0xff) as u8,
                ((x >> 40)&0xff) as u8,
                ((x >> 48)&0xff) as u8,
                ((x >> 56)&0xff) as u8];
}

#[inline]
pub fn from_be_v(v: &[u8]) -> Vec<u64> {
    let mut ret = Vec::new();
    for bytes in v.chunks(4) {
        let word = from_be(bytes);
        ret.push(word);
    }
    ret
}

#[inline]
pub fn from_le_v(v: &[u8]) -> Vec<u64> {
    let mut ret = Vec::new();
    for bytes in v.chunks(4) {
        let word = from_le(bytes);
        ret.push(word);
    }
    ret
}

#[inline]
pub fn to_be_v(x: &[u64]) -> Vec<u8> {
    let mut ret = Vec::new();
    for word in x.iter() {
        let bytes = to_be(*word);
        for byte in bytes.iter() {
            ret.push(*byte);
        }
    }
    ret
}

#[inline]
pub fn to_le_v(x: &[u64]) -> Vec<u8> {
    let mut ret = Vec::new();
    for word in x.iter() {
        let bytes = to_le(*word);
        for byte in bytes.iter() {
            ret.push(*byte);
        }
    }
    ret
}

pub fn new_zero_vec(size: usize) -> Vec<u64> {
    let mut v = vec![];
    for _ in 0..(size) {
        v.push(0u64);
    }
    return v;
}

pub fn pad_le_64(msg: Vec<u8>, bit: u8, length: usize) -> Vec<u8> {
    // FIPS-180-4 SS 6.1.1.2 message is padded
    let mut ret = Vec::new();
    for i in 0..msg.len() {
        ret.push(msg[i]);
    }

    // primarily for implementing MD5
    ret.push(bit);
    for _ in 0..((55 - length) % 64) {
        ret.push(0u8);
    }

    // big-endian u64 size
    let pad = to_le((length as u64)*8);
    for i in 0..pad.len() {
        ret.push(pad[i]);
    }
    ret
}

pub fn pad_be_64(msg: Vec<u8>, bit: u8, length: usize) -> Vec<u8> {
    // FIPS-180-4 SS 6.1.1.2 message is padded
    let mut ret = Vec::new();
    for i in 0..msg.len() {
        ret.push(msg[i]);
    }

    // primarily for implementing SHA1, SHA224, SHA256
    ret.push(bit);
    for _ in 0..((55 - length) % 64) {
        ret.push(0u8);
    }

    // big-endian u64 size
    let pad = to_be((length as u64)*8);
    for i in 0..pad.len() {
        ret.push(pad[i]);
    }
    ret
}

pub fn pad_be_128(msg: Vec<u8>, bit: u8, length: usize) -> Vec<u8> {
    // FIPS-180-4 SS 6.1.1.2 message is padded
    let mut ret = Vec::new();
    for i in 0..msg.len() {
        ret.push(msg[i]);
    }

    // primarily for implementing SHA384, SHA512, SHA512224, SHA512256
    ret.push(bit);
    for _ in 0..((111 - length) % 128) {
        ret.push(0u8);
    }

    // most significant u64 of the u128 size
    for _ in 0..8 {
        ret.push(0u8);
    }

    // big-endian u64 size
    let pad = to_be((length as u64)*8);
    for i in 0..pad.len() {
        ret.push(pad[i]);
    }
    ret
}
