use std::simd::u32x4;
//use super::super::stdish::num::RotateInt;
use std::num::Int;
use stdish::num::RotateInt;

pub const SHA1_H: &'static [u8] = b"\x67\x45\x23\x01\xef\xcd\xab\x89\x98\xba\xdc\xfe\x10\x32\x54\x76\xc3\xd2\xe1\xf0";

//pub const SHA1_H0: u32 = 0x67452301u32; /// digits are (34*n + 1) where n = 3, 2, 1, 0
//pub const SHA1_H1: u32 = 0xefcdab89u32; /// digits are (34*n + 1) where n = 7, 6, 5, 4
//pub const SHA1_H2: u32 = 0x98badcfeu32; /// digits are (34*n + 16) where n = 4, 5, 6, 7
//pub const SHA1_H3: u32 = 0x10325476u32; /// digits are (34*n + 16) where n = 0, 1, 2, 3
//pub const SHA1_H4: u32 = 0xc3d2e1f0u32; /// digits are (15*n) where n = 13, 14, 15, 16

pub const SHA1_K0: u32 = 0x5a827999u32; /// digits of floor(sqrt(2)*2 ^ 30)
pub const SHA1_K1: u32 = 0x6ed9eba1u32; /// digits of floor(sqrt(3)*2 ^ 30)
pub const SHA1_K2: u32 = 0x8f1bbcdcu32; /// digits of floor(sqrt(5)*2 ^ 30)
pub const SHA1_K3: u32 = 0xca62c1d6u32; /// digits of floor(sqrt(10)*2 ^ 30)

pub static SHA1_K0V: u32x4 = u32x4(SHA1_K0, SHA1_K0, SHA1_K0, SHA1_K0);
pub static SHA1_K1V: u32x4 = u32x4(SHA1_K1, SHA1_K1, SHA1_K1, SHA1_K1);
pub static SHA1_K2V: u32x4 = u32x4(SHA1_K2, SHA1_K2, SHA1_K2, SHA1_K2);
pub static SHA1_K3V: u32x4 = u32x4(SHA1_K3, SHA1_K3, SHA1_K3, SHA1_K3);

//pub static SHA1_INITIAL_HASH: [u32; 5] = [SHA1_H0, SHA1_H1, SHA1_H2, SHA1_H3, SHA1_H4];
//const SHA1_CONSTANT_POOL: [u32; 4] = [SHA1_K0, SHA1_K1, SHA1_K2, SHA1_K3];


pub fn add_1st(e: u32, w0: u32x4) -> u32x4 {
    let u32x4(a, b, c, d) = w0;
    u32x4(e + a, b, c, d)
}

pub fn get_1st(w0: u32x4) -> u32 {
    let u32x4(a, _, _, _) = w0;
    a
}

// emulates llvmint::arm::sha1h
// the letter 'H' might stand for half, maybe?
pub fn sha1h(a: u32) -> u32 {
    let b = a.rotate_left(30);
    b
}

// emulates llvmint::arm::sha1su0
pub fn sha1su0(a: u32x4, b: u32x4, c: u32x4) -> u32x4 {
    sha1msg1(a, b) ^ c
}

// emulates llvmint::arm::sha1su1
pub fn sha1su1(a: u32x4, b: u32x4) -> u32x4 {
    sha1msg2(a, b)
}

// emulates llvmint::x86::sha1msg1
pub fn sha1msg1(a: u32x4, b: u32x4) -> u32x4 {
    let u32x4(_, _, w2, w3) = a;
    let u32x4(w4, w5, _, _) = b;
    a ^ u32x4(w2, w3, w4, w5)
}

// emulates llvmint::x86::sha1msg2
pub fn sha1msg2(a: u32x4, b: u32x4) -> u32x4 {
    let u32x4(x0, _, _, _) = a;
    let u32x4(_, w13, w14, w15) = b;
    let w16 = (x0 ^ w13).rotate_left(1);
    let d = a ^ u32x4(w13, w14, w15, w16);

    d.rotate_left(1)
}

// emulates llvmint::arm::sha1c
pub fn sha1c(hash: u32x4, h4: u32, msg: u32x4) -> u32x4 {
    let u32x4(mut a, mut b, mut c, mut d) = hash;
    let u32x4(w0, w1, w2, w3) = msg;
    let mut e = h4; let mut f: u32;

    //println!("{:08x}{:08x}{:08x}{:08x}, {:08x}, {:08x}, {:08x}{:08x}{:08x}{:08x}", a, b, c, d, e, a.rotate_left(30), w0, w1, w2, w3);

    f = a.rotate_left(5) + w0 + e + bool3ary_202!(b, c, d);
    e = d; d = c; c = b.rotate_left(30); b = a; a = f;
    f = a.rotate_left(5) + w1 + e + bool3ary_202!(b, c, d);
    e = d; d = c; c = b.rotate_left(30); b = a; a = f;
    f = a.rotate_left(5) + w2 + e + bool3ary_202!(b, c, d);
    e = d; d = c; c = b.rotate_left(30); b = a; a = f;
    f = a.rotate_left(5) + w3 + e + bool3ary_202!(b, c, d);
           d = c; c = b.rotate_left(30); b = a; a = f;

    u32x4(a, b, c, d)
}

// emulates llvmint::arm::sha1p
pub fn sha1p(hash: u32x4, h4: u32, msg: u32x4) -> u32x4 {
    let u32x4(mut a, mut b, mut c, mut d) = hash;
    let u32x4(w0, w1, w2, w3) = msg;
    let mut e = h4; let mut f: u32;

    //println!("{:08x}{:08x}{:08x}{:08x}, {:08x}, {:08x}, {:08x}{:08x}{:08x}{:08x}", a, b, c, d, e, a.rotate_left(30), w0, w1, w2, w3);

    f = a.rotate_left(5) + w0 + e + bool3ary_150!(b, c, d);
    e = d; d = c; c = b.rotate_left(30); b = a; a = f;
    f = a.rotate_left(5) + w1 + e + bool3ary_150!(b, c, d);
    e = d; d = c; c = b.rotate_left(30); b = a; a = f;
    f = a.rotate_left(5) + w2 + e + bool3ary_150!(b, c, d);
    e = d; d = c; c = b.rotate_left(30); b = a; a = f;
    f = a.rotate_left(5) + w3 + e + bool3ary_150!(b, c, d);
           d = c; c = b.rotate_left(30); b = a; a = f;

    u32x4(a, b, c, d)
}

// emulates llvmint::arm::sha1m
pub fn sha1m(hash: u32x4, h4: u32, msg: u32x4) -> u32x4 {
    let u32x4(mut a, mut b, mut c, mut d) = hash;
    let u32x4(w0, w1, w2, w3) = msg;
    let mut e = h4; let mut f: u32;

    //println!("{:08x}{:08x}{:08x}{:08x}, {:08x}, {:08x}, {:08x}{:08x}{:08x}{:08x}", a, b, c, d, e, a.rotate_left(30), w0, w1, w2, w3);
    
    f = a.rotate_left(5) + w0 + e + bool3ary_232!(b, c, d);
    e = d; d = c; c = b.rotate_left(30); b = a; a = f;
    f = a.rotate_left(5) + w1 + e + bool3ary_232!(b, c, d);
    e = d; d = c; c = b.rotate_left(30); b = a; a = f;
    f = a.rotate_left(5) + w2 + e + bool3ary_232!(b, c, d);
    e = d; d = c; c = b.rotate_left(30); b = a; a = f;
    f = a.rotate_left(5) + w3 + e + bool3ary_232!(b, c, d);
           d = c; c = b.rotate_left(30); b = a; a = f;
    
    u32x4(a, b, c, d)
}

// emulates llvmint::x86::sha1nexte
pub fn sha1nexte(hash: u32x4, msg: u32x4) -> u32x4 {
    add_1st(get_1st(hash).rotate_left(30), msg)
}

// emulates llvmint::x86::sha1rnds4
pub fn sha1rnds4(hash: u32x4, work: u32x4, i: i8) -> u32x4 {
    match i {
        0 => sha1c(hash, 0, work + SHA1_K0V),
        1 => sha1p(hash, 0, work + SHA1_K1V),
        2 => sha1m(hash, 0, work + SHA1_K2V),
        3 => sha1p(hash, 0, work + SHA1_K3V),
        _ => panic!("unknown icosaround index")
    }
}

// constant icosaround index

#[inline]
pub fn rounds40(hash: u32x4, oldh: u32x4, work: u32x4) -> u32x4 {
    // this uses emulated ARM intrinsics
    let k0v = u32x4(SHA1_K0, SHA1_K0, SHA1_K0, SHA1_K0);
    sha1c(hash, sha1h(get_1st(oldh)), work + k0v)
}
#[inline]
pub fn rounds41(hash: u32x4, oldh: u32x4, work: u32x4) -> u32x4 {
    // this uses emulated ARM intrinsics
    let k1v = u32x4(SHA1_K1, SHA1_K1, SHA1_K1, SHA1_K1);
    sha1p(hash, sha1h(get_1st(oldh)), work + k1v)
}
#[inline]
pub fn rounds42(hash: u32x4, oldh: u32x4, work: u32x4) -> u32x4 {
    // this uses emulated ARM intrinsics
    let k2v = u32x4(SHA1_K2, SHA1_K2, SHA1_K2, SHA1_K2);
    sha1m(hash, sha1h(get_1st(oldh)), work + k2v)
}
#[inline]
pub fn rounds43(hash: u32x4, oldh: u32x4, work: u32x4) -> u32x4 {
    // this uses emulated ARM intrinsics
    let k3v = u32x4(SHA1_K3, SHA1_K3, SHA1_K3, SHA1_K3);
    sha1p(hash, sha1h(get_1st(oldh)), work + k3v)
}

#[inline]
pub fn schedule(w0: u32x4, w1: u32x4, w2: u32x4, w3: u32x4) -> u32x4 {
    // this uses emulated x86 intrinsics
    sha1msg2(sha1msg1(w0, w1) ^ w2, w3)
}

//pub fn digest_block_arm_emu(
//    m0: u32x4, m4: u32x4, m8: u32x4, m12: u32x4,
//    h0: u32x4, e: u32) -> (u32x4, u32) {
//    
//    let w0 = m0;
//    let h1 = sha1c(h0, e, w0 + SHA1_K0V);
//    let w1 = m4;
//    let h2 = sha1c(h1, sha1h(get_1st(h0)), w1 + SHA1_K0V);
//    let w2 = m8;
//    let h3 = sha1c(h2, sha1h(get_1st(h1)), w2 + SHA1_K0V);
//    let w3 = m12;
//    let h4 = sha1c(h3, sha1h(get_1st(h2)), w3 + SHA1_K0V);
//    let w4 = sha1su1(sha1su0(w0, w1, w2), w3);
//    let h5 = sha1c(h4, sha1h(get_1st(h3)), w4 + SHA1_K0V);
//    let w5 = sha1su1(sha1su0(w1, w2, w3), w4);
//    let h6 = sha1p(h5, sha1h(get_1st(h4)), w5 + SHA1_K1V);
//    let w6 = sha1su1(sha1su0(w2, w3, w4), w5);
//    let h7 = sha1p(h6, sha1h(get_1st(h5)), w6 + SHA1_K1V);
//    let w7 = sha1su1(sha1su0(w3, w4, w5), w6);
//    let h8 = sha1p(h7, sha1h(get_1st(h6)), w7 + SHA1_K1V);
//    let w8 = sha1su1(sha1su0(w4, w5, w6), w7);
//    let h9 = sha1p(h8, sha1h(get_1st(h7)), w8 + SHA1_K1V);
//    let w9 = sha1su1(sha1su0(w5, w6, w7), w8);
//    let h10 = sha1p(h9, sha1h(get_1st(h8)), w9 + SHA1_K1V);
//    let w10 = sha1su1(sha1su0(w6, w7, w8), w9);
//    let h11 = sha1m(h10, sha1h(get_1st(h9)), w10 + SHA1_K2V);
//    let w11 = sha1su1(sha1su0(w7, w8, w9), w10);
//    let h12 = sha1m(h11, sha1h(get_1st(h10)), w11 + SHA1_K2V);
//    let w12 = sha1su1(sha1su0(w8, w9, w10), w11);
//    let h13 = sha1m(h12, sha1h(get_1st(h11)), w12 + SHA1_K2V);
//    let w13 = sha1su1(sha1su0(w9, w10, w11), w12);
//    let h14 = sha1m(h13, sha1h(get_1st(h12)), w13 + SHA1_K2V);
//    let w14 = sha1su1(sha1su0(w10, w11, w12), w13);
//    let h15 = sha1m(h14, sha1h(get_1st(h13)), w14 + SHA1_K2V);
//    let w15 = sha1su1(sha1su0(w11, w12, w13), w14);
//    let h16 = sha1p(h15, sha1h(get_1st(h14)), w15 + SHA1_K3V);
//    let w16 = sha1su1(sha1su0(w12, w13, w14), w15);
//    let h17 = sha1p(h16, sha1h(get_1st(h15)), w16 + SHA1_K3V);
//    let w17 = sha1su1(sha1su0(w13, w14, w15), w16);
//    let h18 = sha1p(h17, sha1h(get_1st(h16)), w17 + SHA1_K3V);
//    let w18 = sha1su1(sha1su0(w14, w15, w16), w17);
//    let h19 = sha1p(h18, sha1h(get_1st(h17)), w18 + SHA1_K3V);
//    let w19 = sha1su1(sha1su0(w15, w16, w17), w18);
//    let h20 = sha1p(h19, sha1h(get_1st(h18)), w19 + SHA1_K3V);
//    let e20 = get_1st(h19).rotate_left(30);
//
//    (h0 + h20, e + e20)
//}
//
//// 1220 ns/iter
//pub fn digest_block_x86_emu(
//    m0: u32x4, m4: u32x4, m8: u32x4, m12: u32x4,
//    h0: u32x4, e: u32) -> (u32x4, u32) {
//    
//    let w0 = m0;
//    let h1 = sha1rnds4(h0, add_1st(e, w0), 0);
//    let w1 = m4;
//    let h2 = sha1rnds4(h1, sha1nexte(h0, w1), 0);
//    let w2 = m8;
//    let h3 = sha1rnds4(h2, sha1nexte(h1, w2), 0);
//    let w3 = m12;
//    let h4 = sha1rnds4(h3, sha1nexte(h2, w3), 0);
//    let w4 = sha1msg2(sha1msg1(w0, w1) ^ w2, w3);
//    let h5 = sha1rnds4(h4, sha1nexte(h3, w4), 0);
//    let w5 = sha1msg2(sha1msg1(w1, w2) ^ w3, w4);
//    let h6 = sha1rnds4(h5, sha1nexte(h4, w5), 1);
//    let w6 = sha1msg2(sha1msg1(w2, w3) ^ w4, w5);
//    let h7 = sha1rnds4(h6, sha1nexte(h5, w6), 1);
//    let w7 = sha1msg2(sha1msg1(w3, w4) ^ w5, w6);
//    let h8 = sha1rnds4(h7, sha1nexte(h6, w7), 1);
//    let w8 = sha1msg2(sha1msg1(w4, w5) ^ w6, w7);
//    let h9 = sha1rnds4(h8, sha1nexte(h7, w8), 1);
//    let w9 = sha1msg2(sha1msg1(w5, w6) ^ w7, w8);
//    let h10 = sha1rnds4(h9, sha1nexte(h8, w9), 1);
//    let w10 = sha1msg2(sha1msg1(w6, w7) ^ w8, w9);
//    let h11 = sha1rnds4(h10, sha1nexte(h9, w10), 2);
//    let w11 = sha1msg2(sha1msg1(w7, w8) ^ w9, w10);
//    let h12 = sha1rnds4(h11, sha1nexte(h10, w11), 2);
//    let w12 = sha1msg2(sha1msg1(w8, w9) ^ w10, w11);
//    let h13 = sha1rnds4(h12, sha1nexte(h11, w12), 2);
//    let w13 = sha1msg2(sha1msg1(w9, w10) ^ w11, w12);
//    let h14 = sha1rnds4(h13, sha1nexte(h12, w13), 2);
//    let w14 = sha1msg2(sha1msg1(w10, w11) ^ w12, w13);
//    let h15 = sha1rnds4(h14, sha1nexte(h13, w14), 2);
//    let w15 = sha1msg2(sha1msg1(w11, w12) ^ w13, w14);
//    let h16 = sha1rnds4(h15, sha1nexte(h14, w15), 3);
//    let w16 = sha1msg2(sha1msg1(w12, w13) ^ w14, w15);
//    let h17 = sha1rnds4(h16, sha1nexte(h15, w16), 3);
//    let w17 = sha1msg2(sha1msg1(w13, w14) ^ w15, w16);
//    let h18 = sha1rnds4(h17, sha1nexte(h16, w17), 3);
//    let w18 = sha1msg2(sha1msg1(w14, w15) ^ w16, w17);
//    let h19 = sha1rnds4(h18, sha1nexte(h17, w18), 3);
//    let w19 = sha1msg2(sha1msg1(w15, w16) ^ w17, w18);
//    let h20 = sha1rnds4(h19, sha1nexte(h18, w19), 3);
//    let e20 = get_1st(h19).rotate_left(30);
//
//    (h0 + h20, e + e20)
//}

pub fn digest_block_simd(hashw: &mut [u32; 5], msgv: &[u32x4; 4]) {
    let h0 = u32x4(hashw[0], hashw[1], hashw[2], hashw[3]);
    let w0 = msgv[0];
    let h1 = rounds40(h0, u32x4(0, 0, 0, 0), add_1st(hashw[4], w0));
    let w1 = msgv[1];
    let h2 = rounds40(h1, h0, w1);
    let w2 = msgv[2];
    let h3 = rounds40(h2, h1, w2);
    let w3 = msgv[3];
    let h4 = rounds40(h3, h2, w3);
    let w4 = schedule(w0, w1, w2, w3);
    let h5 = rounds40(h4, h3, w4);
    let w5 = schedule(w1, w2, w3, w4);
    let h6 = rounds41(h5, h4, w5);
    let w6 = schedule(w2, w3, w4, w5);
    let h7 = rounds41(h6, h5, w6);
    let w7 = schedule(w3, w4, w5, w6);
    let h8 = rounds41(h7, h6, w7);
    let w8 = schedule(w4, w5, w6, w7);
    let h9 = rounds41(h8, h7, w8);
    let w9 = schedule(w5, w6, w7, w8);
    let h10 = rounds41(h9, h8, w9);
    let w10 = schedule(w6, w7, w8, w9);
    let h11 = rounds42(h10, h9, w10);
    let w11 = schedule(w7, w8, w9, w10);
    let h12 = rounds42(h11, h10, w11);
    let w12 = schedule(w8, w9, w10, w11);
    let h13 = rounds42(h12, h11, w12);
    let w13 = schedule(w9, w10, w11, w12);
    let h14 = rounds42(h13, h12, w13);
    let w14 = schedule(w10, w11, w12, w13);
    let h15 = rounds42(h14, h13, w14);
    let w15 = schedule(w11, w12, w13, w14);
    let h16 = rounds43(h15, h14, w15);
    let w16 = schedule(w12, w13, w14, w15);
    let h17 = rounds43(h16, h15, w16);
    let w17 = schedule(w13, w14, w15, w16);
    let h18 = rounds43(h17, h16, w17);
    let w18 = schedule(w14, w15, w16, w17);
    let h19 = rounds43(h18, h17, w18);
    let w19 = schedule(w15, w16, w17, w18);
    let h20 = rounds43(h19, h18, w19);
    let e20 = get_1st(h19).rotate_left(30);

    let u32x4(a, b, c, d) = h20;

    hashw[0] += a;
    hashw[1] += b;
    hashw[2] += c;
    hashw[3] += d;
    hashw[4] += e20;
}

//#[inline]
//pub fn init_hash(h: &mut [u32]) {
//    assert_eq!(h.len(), 5);
//    for t in 0us..5us {
//        h[t] = SHA1_INITIAL_HASH[t];
//    }
//}
//
//#[inline]
//pub fn init_work(w: &mut [u32]) {
//    assert_eq!(w.len(), 80);
//    for t in 16us..80us {
//        w[t] = (w[t-3]^w[t-8]^w[t-14]^w[t-16]).rotate_left(1);
//    }
//}

/// Pad message
pub fn pad(msg: &[u8], length: usize) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.push_all(msg);
    bytes.push(0x80u8);
    for _ in 0us..((55 - length) % 64) {
        bytes.push(0u8);
    }
    bytes.write_be_u64(8*length as u64).unwrap();
    bytes
}
