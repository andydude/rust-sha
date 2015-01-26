
#[inline]
pub fn rotl(x: u32, y: usize) -> u32 {
    return (x << y) | (x >> (32 - y));
}

#[inline]
pub fn rotr(x: u32, y: usize) -> u32 {
    return (x >> y) | (x << (32 - y));
}

#[inline]
pub fn shr(x: u32, y: usize) -> u32 {
    return x >> y;
}

#[inline]
pub fn shl(x: u32, y: usize) -> u32 {
    return x << y;
}

#[inline]
pub fn rotl_5(x: u32) -> u32 {
    return (x << 5) | (x >> 27);
}

#[inline]
pub fn rotl_30(x: u32) -> u32 {
    return (x << 30) | (x >> 2);
}






// std::io::Reader::read_le_u32
#[inline]
pub fn from_le(v: &[u8]) -> u32 {
    return (v[3] as u32) << 24 
         | (v[2] as u32) << 16 
         | (v[1] as u32) << 8 
         | (v[0] as u32);
}

// std::io::Reader::read_be_u32
#[inline]
pub fn from_be(v: &[u8]) -> u32 {
    return (v[0] as u32) << 24 
         | (v[1] as u32) << 16 
         | (v[2] as u32) << 8 
         | (v[3] as u32);
}

// std::io::Writer::write_be_u32
#[inline]
pub fn to_be(x: u32) -> Vec<u8> {
    return vec![((x >> 24)&0xff) as u8,
                ((x >> 16)&0xff) as u8,
                ((x >> 8)&0xff) as u8,
                (x&0xff) as u8];
}

// std::io::Writer::write_le_u32
#[inline]
pub fn to_le(x: u32) -> Vec<u8> {
    return vec![((x)&0xff) as u8,
                ((x >> 8)&0xff) as u8,
                ((x >> 16)&0xff) as u8,
                ((x >> 24)&0xff) as u8];
}

#[inline]
pub fn from_le_v(v: &[u8]) -> Vec<u32> {
    let mut ret = Vec::new();
    for bytes in v.chunks(4) {
        let word = from_le(bytes);
        ret.push(word);
    }
    ret
}

#[inline]
pub fn from_be_v(v: &[u8]) -> Vec<u32> {
    let mut ret = Vec::new();
    for bytes in v.chunks(4) {
        let word = from_be(bytes);
        ret.push(word);
    }
    ret
}

#[inline]
pub fn to_be_v(words: &[u32]) -> Vec<u8> {
    let mut ret = Vec::new();
    for word in words.iter() {
        let bytes = to_be(*word);
        for byte in bytes.iter() {
            ret.push(*byte);
        }
    }
    ret
}

#[inline]
pub fn to_le_v(words: &[u32]) -> Vec<u8> {
    let mut ret = Vec::new();
    for word in words.iter() {
        let bytes = to_le(*word);
        for byte in bytes.iter() {
            ret.push(*byte);
        }
    }
    ret
}


//pub fn bytes_as_le_u32(v: Vec<u8>) -> u32 {
//    return v[3] as u32 << 24 
//         | v[2] as u32 << 16 
//         | v[1] as u32 << 8 
//         | v[0] as u32;
//}
//
//pub fn le_u32_as_bytes(x: u32) -> Vec<u8> {
//    return &[(x&0xff) as u8,
//             ((x >> 8)&0xff) as u8,
//             ((x >> 16)&0xff) as u8,
//             ((x >> 24)&0xff) as u8];
//}

pub fn new_zero_vec(size: usize) -> Vec<u32> {
    let mut v = vec![];
    for _ in 0..(size) {
        v.push(0u32);
    }
    return v;
}
