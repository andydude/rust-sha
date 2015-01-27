#![feature(simd, simd_ffi, link_llvm_intrinsics)]
#![allow(non_snake_case)]

use serialize::hex::ToHex;
use std::simd::u32x4;
use rotate;

const SHA1_H0: u32 = 0x67452301u32; /// digits are (34*n + 1) where n = 3, 2, 1, 0
const SHA1_H1: u32 = 0xefcdab89u32; /// digits are (34*n + 1) where n = 7, 6, 5, 4
const SHA1_H2: u32 = 0x98badcfeu32; /// digits are (34*n + 16) where n = 4, 5, 6, 7
const SHA1_H3: u32 = 0x10325476u32; /// digits are (34*n + 16) where n = 0, 1, 2, 3
const SHA1_H4: u32 = 0xc3d2e1f0u32; /// digits are (15*n) where n = 13, 14, 15, 16

const SHA1_K0: u32 = 0x5a827999u32; /// digits of floor(sqrt(2)*2 ^ 30)
const SHA1_K1: u32 = 0x6ed9eba1u32; /// digits of floor(sqrt(3)*2 ^ 30)
const SHA1_K2: u32 = 0x8f1bbcdcu32; /// digits of floor(sqrt(5)*2 ^ 30)
const SHA1_K3: u32 = 0xca62c1d6u32; /// digits of floor(sqrt(10)*2 ^ 30)

const SHA1_K0V: u32x4 = u32x4(SHA1_K0, SHA1_K0, SHA1_K0, SHA1_K0);
const SHA1_K1V: u32x4 = u32x4(SHA1_K1, SHA1_K1, SHA1_K1, SHA1_K1);
const SHA1_K2V: u32x4 = u32x4(SHA1_K2, SHA1_K2, SHA1_K2, SHA1_K2);
const SHA1_K3V: u32x4 = u32x4(SHA1_K3, SHA1_K3, SHA1_K3, SHA1_K3);

const SHA1_INITIAL_HASH: [u32; 5] = [SHA1_H0, SHA1_H1, SHA1_H2, SHA1_H3, SHA1_H4];
const SHA1_CONSTANT_POOL: [u32; 4] = [SHA1_K0, SHA1_K1, SHA1_K2, SHA1_K3];

mod arm {
    use std::simd::u32x4;
    
    #[link_name = "llvm.arm.neon.sha1h"]
    pub fn sha1h(a: u32) -> u32;

    #[link_name = "llvm.arm.neon.sha1c"]
    pub fn sha1c(a: u32x4, b: u32, c: u32x4) -> u32x4;

    #[link_name = "llvm.arm.neon.sha1m"]
    pub fn sha1m(a: u32x4, b: u32, c: u32x4) -> u32x4;

    #[link_name = "llvm.arm.neon.sha1p"]
    pub fn sha1p(a: u32x4, b: u32, c: u32x4) -> u32x4;

    #[link_name = "llvm.arm.neon.sha1su0"]
    pub fn sha1su0(a: u32x4, b: u32x4, c: u32x4) -> u32x4;

    #[link_name = "llvm.arm.neon.sha1su1"]
    pub fn sha1su1(a: u32x4, b: u32x4) -> u32x4;
}

mod x86 {
    use std::simd::u32x4;
    
    #[link_name = "llvm.x86.sha1rnds4"]
    pub fn sha1rnds4(a: u32x4, b: u32x4, c: i8) -> u32x4;

    #[link_name = "llvm.x86.sha1nexte"]
    pub fn sha1nexte(a: u32x4, b: u32x4) -> u32x4;

    #[link_name = "llvm.x86.sha1msg1"]
    pub fn sha1msg1(a: u32x4, b: u32x4) -> u32x4;

    #[link_name = "llvm.x86.sha1msg2"]
    pub fn sha1msg2(a: u32x4, b: u32x4) -> u32x4;
}

mod emu {
    use std::simd::u32x4;
    use rotate;
    use sha1;
    
    pub fn addfirst(hash: u32x4, e: u32) -> u32x4 {
        let u32x4(a, b, c, d) = hash;
        u32x4(e + a, b, c, d)
    }
    
    pub fn first(hash: u32x4) -> u32 {
        let u32x4(a, _, _, _) = hash;
        a
    }

    // emulates llvmint::arm::sha1h
    // the letter 'H' might stand for half, maybe?
    pub fn sha1h(a: u32) -> u32 {
        rotate::left_u32(a, 30)
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
        let w16 = rotate::left_u32((x0 ^ w13), 1);
        let d = a ^ u32x4(w13, w14, w15, w16);

        rotate::left_u32x4(d, 1)
    }
    
    // emulates llvmint::arm::sha1c
    pub fn sha1c(hash: u32x4, h4: u32, msg: u32x4) -> u32x4 {
        let u32x4(mut a, mut b, mut c, mut d) = hash;
        let u32x4(w0, w1, w2, w3) = msg;
        let mut e = h4; let mut f: u32;

        //println!("{:08x}{:08x}{:08x}{:08x}, {:08x}, {:08x}, {:08x}{:08x}{:08x}{:08x}", a, b, c, d, e, rotate::left_u32(a, 30), w0, w1, w2, w3);

        f = rotate::left_u32(a, 5) + w0 + e + bool3ary_202!(b, c, d);
        e = d; d = c; c = rotate::left_u32(b, 30); b = a; a = f;
        f = rotate::left_u32(a, 5) + w1 + e + bool3ary_202!(b, c, d);
        e = d; d = c; c = rotate::left_u32(b, 30); b = a; a = f;
        f = rotate::left_u32(a, 5) + w2 + e + bool3ary_202!(b, c, d);
        e = d; d = c; c = rotate::left_u32(b, 30); b = a; a = f;
        f = rotate::left_u32(a, 5) + w3 + e + bool3ary_202!(b, c, d);
               d = c; c = rotate::left_u32(b, 30); b = a; a = f;

        u32x4(a, b, c, d)
    }
    
    // emulates llvmint::arm::sha1p
    pub fn sha1p(hash: u32x4, h4: u32, msg: u32x4) -> u32x4 {
        let u32x4(mut a, mut b, mut c, mut d) = hash;
        let u32x4(w0, w1, w2, w3) = msg;
        let mut e = h4; let mut f: u32;

        //println!("{:08x}{:08x}{:08x}{:08x}, {:08x}, {:08x}, {:08x}{:08x}{:08x}{:08x}", a, b, c, d, e, rotate::left_u32(a, 30), w0, w1, w2, w3);

        f = rotate::left_u32(a, 5) + w0 + e + bool3ary_150!(b, c, d);
        e = d; d = c; c = rotate::left_u32(b, 30); b = a; a = f;
        f = rotate::left_u32(a, 5) + w1 + e + bool3ary_150!(b, c, d);
        e = d; d = c; c = rotate::left_u32(b, 30); b = a; a = f;
        f = rotate::left_u32(a, 5) + w2 + e + bool3ary_150!(b, c, d);
        e = d; d = c; c = rotate::left_u32(b, 30); b = a; a = f;
        f = rotate::left_u32(a, 5) + w3 + e + bool3ary_150!(b, c, d);
               d = c; c = rotate::left_u32(b, 30); b = a; a = f;

        u32x4(a, b, c, d)
    }
    
    // emulates llvmint::arm::sha1m
    pub fn sha1m(hash: u32x4, h4: u32, msg: u32x4) -> u32x4 {
        let u32x4(mut a, mut b, mut c, mut d) = hash;
        let u32x4(w0, w1, w2, w3) = msg;
        let mut e = h4; let mut f: u32;

        //println!("{:08x}{:08x}{:08x}{:08x}, {:08x}, {:08x}, {:08x}{:08x}{:08x}{:08x}", a, b, c, d, e, rotate::left_u32(a, 30), w0, w1, w2, w3);
        
        f = rotate::left_u32(a, 5) + w0 + e + bool3ary_232!(b, c, d);
        e = d; d = c; c = rotate::left_u32(b, 30); b = a; a = f;
        f = rotate::left_u32(a, 5) + w1 + e + bool3ary_232!(b, c, d);
        e = d; d = c; c = rotate::left_u32(b, 30); b = a; a = f;
        f = rotate::left_u32(a, 5) + w2 + e + bool3ary_232!(b, c, d);
        e = d; d = c; c = rotate::left_u32(b, 30); b = a; a = f;
        f = rotate::left_u32(a, 5) + w3 + e + bool3ary_232!(b, c, d);
               d = c; c = rotate::left_u32(b, 30); b = a; a = f;
        
        u32x4(a, b, c, d)
    }

    // emulates llvmint::x86::sha1nexte
    pub fn sha1nexte(hash: u32x4, msg: u32x4) -> u32x4 {
        addfirst(msg, rotate::left_u32(first(hash), 30))
    }

    // emulates llvmint::x86::sha1rnds4
    pub fn sha1rnds4(hash: u32x4, work: u32x4, i: i8) -> u32x4 {
        match i {
            0 => sha1c(hash, 0, work + sha1::SHA1_K0V),
            1 => sha1p(hash, 0, work + sha1::SHA1_K1V),
            2 => sha1m(hash, 0, work + sha1::SHA1_K2V),
            3 => sha1p(hash, 0, work + sha1::SHA1_K3V),
            _ => panic!("unknown icosaround index")
        }
    }

    // constant icosaround index
    
    //pub fn sha1rnds4_0(a: u32x4, b: u32x4) -> u32x4 {
    //    sha1rnds4(a, b, 0)
    //}
    //
    //pub fn sha1rnds4_1(a: u32x4, b: u32x4) -> u32x4 {
    //    sha1rnds4(a, b, 1)
    //}
    //
    //pub fn sha1rnds4_2(a: u32x4, b: u32x4) -> u32x4 {
    //    sha1rnds4(a, b, 2)
    //}
    //
    //pub fn sha1rnds4_3(a: u32x4, b: u32x4) -> u32x4 {
    //    sha1rnds4(a, b, 3)
    //}
}

/*
 * sha1::init_hash(hash: &[u32; 5]);
 * sha1::init_work(work: &[u32; 80]);
 * sha1::rounds4_0a(hash: &mut [u32; 5], subwork: &[u32; 4]);
 * sha1::rounds4_0(hash: &mut [u32; 5], subwork: &[u32; 4]);
 * sha1::rounds4_1(hash: &mut [u32; 5], subwork: &[u32; 4]);
 * sha1::rounds4_2(hash: &mut [u32; 5], subwork: &[u32; 4]);
 * sha1::rounds4_3(hash: &mut [u32; 5], subwork: &[u32; 4]);
 * sha1::rounds4(hash: &mut [u32; 5], subwork: &[u32; 4], qround: u8);
 * sha1::rounds20(hash: &mut [u32; 5], subwork: &[u32; 4], qround: u8);
 * sha1::rounds80(hash: &mut [u32; 5], subwork: &[u32; 4]);

 * sha1::digest_block(hash: &[u32; 5], msg_block: &[u32; 16]);
 * sha1::digest_bytes(hash: &[u32; 5], msg_block: &[u8; 64]);
 * sha1::pad(msg_blocks: &[u32], msg: &[u8], size: u64) -> Vec<u8>;
 * sha1::digest(hash: &[u32; 5], msg: &[u8]);
 * sha1::hex_digest(msg: &str) -> String;
 */

#[inline]
fn init_hash(hash: &mut [u32]) {
    assert_eq!(hash.len(), 5);
    for t in 0us..5us {
        hash[t] = SHA1_INITIAL_HASH[t];
    }
}

// x86::SHA1MSG1 x, y
// x86::SHA1MSG2 x, y
#[inline]
pub fn init_work(work: &mut [u32]) {
    assert_eq!(work.len(), 80);
    for t in 16us..80us {
        work[t] = rotate::left_u32(work[t-3]^work[t-8]^work[t-14]^work[t-16], 1);
    }
}

//macro_rules! init_work_asm {
//}


//macro_rules! rounds4_0_asm {
//    ($h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr,
//     $w0:expr, $w1:expr, $w2:expr, $w3:expr) => {
//        {
//            let a: u32 = $h0;
//            let b: u32 = $h1;
//            let c: u32 = $h2;
//            let d: u32 = $h3;
//            let e: u32 = $h4;
//
//            asm!("
//                 sha1nexte %xmm0, %xmm1
//                 sha1rnds4 %xmm0, %xmm1, 0
//                 ");
//
//        }
//    }
//}

//macro_rules! rounds4 {
//    ($h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr,
//     $w0:expr, $w1:expr, $w2:expr, $w3:expr, $q:expr) => {
//        {
//            let a: u32 = $h0;
//            let b: u32 = $h1;
//            let c: u32 = $h2;
//            let d: u32 = $h3;
//            let e: u32 = $h4;
//
//            let j = arith_u32::rotl_30(b);
//            let k = SHA1_CONSTANT_POOL[$q] + arith_u32::rotl_5(a);
//            let d3 = func!($q, b, c, d) + k + e + $w0;
//            let c3 = func!($q, a, arith_u32::rotl_5(b), c) + k + d + $w1;
//            let e4 = arith_u32::rotl_30(a);
//            let d4 = arith_u32::rotl_30(d3);
//
//            $h4 = e4;
//            $h3 = d4;
//            $h2 = arith_u32::rotl_30(c3);
//            $h1 = func!($q, d3, e4, j) + k + c + $w2;
//            $h0 = func!($q, c3, d4, e4) + k + j + $w3;
//        }
//    }
//}


macro_rules! func {
    ($kon:expr, $a:expr, $b:expr, $c:expr) => {
        if $kon == 0 {
            bool3ary_202!($a, $b, $c)
        } else if $kon == 2 {
            bool3ary_232!($a, $b, $c)
        } else {
            bool3ary_150!($a, $b, $c)
        }
    }
}

macro_rules! rounds4 {
    ($h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr,
     $w0:expr, $w1:expr, $w2:expr, $w3:expr, $q:expr) => {
        {
            let mut a: u32 = $h0;
            let mut b: u32 = $h1;
            let mut c: u32 = $h2;
            let mut d: u32 = $h3;
            let mut e: u32 = $h4;
            let mut temp: u32;
            let mut k: u32;

            k = rotate::left_u32(a, 5) + SHA1_CONSTANT_POOL[$q];
            temp = e + $w0 + k + func!($q, b, c, d);
            e = d; d = c; c = rotate::left_u32(b, 30); b = a; a = temp;
            k = rotate::left_u32(a, 5) + SHA1_CONSTANT_POOL[$q];
            temp = e + $w1 + k + func!($q, b, c, d);
            e = d; d = c; c = rotate::left_u32(b, 30); b = a; a = temp;
            k = rotate::left_u32(a, 5) + SHA1_CONSTANT_POOL[$q];
            temp = e + $w2 + k + func!($q, b, c, d);
            e = d; d = c; c = rotate::left_u32(b, 30); b = a; a = temp;
            k = rotate::left_u32(a, 5) + SHA1_CONSTANT_POOL[$q];
            temp = e + $w3 + k + func!($q, b, c, d);
            e = d; d = c; c = rotate::left_u32(b, 30); b = a; a = temp;

            $h4 = e;
            $h3 = d;
            $h2 = c;
            $h1 = b;
            $h0 = a;
        }
    }
}


// x86::SHA1RNDS4 x, y, 0
// x86::SHA1NEXTE
#[inline]
pub fn rounds4_0(hash: &mut [u32], work: &[u32]) {
    assert_eq!(work.len(), 4);
    rounds4!(hash[0], hash[1], hash[2], hash[3], hash[4], 
             work[0], work[1], work[2], work[3], 0);
}

// x86::SHA1RNDS4 x, y, 1
// x86::SHA1NEXTE
#[inline]
pub fn rounds4_1(hash: &mut [u32], work: &[u32]) {
    assert_eq!(work.len(), 4);
    rounds4!(hash[0], hash[1], hash[2], hash[3], hash[4], 
             work[0], work[1], work[2], work[3], 1);
}

// x86::SHA1RNDS4 x, y, 2
// x86::SHA1NEXTE
#[inline]
pub fn rounds4_2(hash: &mut [u32], work: &[u32]) {
    assert_eq!(work.len(), 4);
    rounds4!(hash[0], hash[1], hash[2], hash[3], hash[4], 
             work[0], work[1], work[2], work[3], 2);
}

// x86::SHA1RNDS4 x, y, 3
// x86::SHA1NEXTE
#[inline]
pub fn rounds4_3(hash: &mut [u32], work: &[u32]) {
    assert_eq!(work.len(), 4);
    rounds4!(hash[0], hash[1], hash[2], hash[3], hash[4], 
             work[0], work[1], work[2], work[3], 3);
}

pub fn rounds80(hash: &mut [u32], work: &[u32]) {
    assert_eq!(hash.len(), 5);
    assert_eq!(work.len(), 80);

    rounds4_0(hash, &work[0..4]);
    rounds4_0(hash, &work[4..8]);
    rounds4_0(hash, &work[8..12]);
    rounds4_0(hash, &work[12..16]);
    rounds4_0(hash, &work[16..20]);
    rounds4_1(hash, &work[20..24]);
    rounds4_1(hash, &work[24..28]);
    rounds4_1(hash, &work[28..32]);
    rounds4_1(hash, &work[32..36]);
    rounds4_1(hash, &work[36..40]);
    rounds4_2(hash, &work[40..44]);
    rounds4_2(hash, &work[44..48]);
    rounds4_2(hash, &work[48..52]);
    rounds4_2(hash, &work[52..56]);
    rounds4_2(hash, &work[56..60]);
    rounds4_3(hash, &work[60..64]);
    rounds4_3(hash, &work[64..68]);
    rounds4_3(hash, &work[68..72]);
    rounds4_3(hash, &work[72..76]);
    rounds4_3(hash, &work[76..80]);
}

#[allow(unstable)]
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

/// Digest message block in words
pub fn digest_block(hash: &mut [u32], msg_block: &[u32]) {
    digest_block_naive(hash, msg_block);    
    //digest_block_arm_emu(hash, msg_block);    
    //digest_block_x86_emu(hash, msg_block);    
    //digest_block_arm(hash, msg_block);    
    //digest_block_x86(hash, msg_block);    
}

/// Digest message block in words (ARM-specific code)
pub fn digest_block_arm(hash: &mut [u32], msg: &[u32]) {
    assert_eq!(hash.len(), 5);
    assert_eq!(msg.len(), 16);

    let hash0 = u32x4(hash[0] as u32,
                      hash[1] as u32,
                      hash[2] as u32,
                      hash[3] as u32);

    let work0 = u32x4(msg[0] as u32,
                      msg[1] as u32,
                      msg[2] as u32,
                      msg[3] as u32);

    let hash1 = arm::sha1c(hash0, hash[4] as u32, work0 + SHA1_K0V);

    let work1 = u32x4(msg[4] as u32,
                      msg[5] as u32,
                      msg[6] as u32,
                      msg[7] as u32);

    let hash2 = arm::sha1c(hash1, arm::sha1h(hash[0] as u32), work1 + SHA1_K0V);

    let work2 = u32x4(msg[8] as u32,
                      msg[9] as u32,
                      msg[10] as u32,
                      msg[11] as u32);

    let hash3 = arm::sha1c(hash2, arm::sha1h(emu::first(hash1)), work2 + SHA1_K0V);
    
    let work3 = u32x4(msg[12] as u32,
                      msg[13] as u32,
                      msg[14] as u32,
                      msg[15] as u32);
    
    let hash4 = arm::sha1c(hash3, arm::sha1h(emu::first(hash2)), work3 + SHA1_K0V);
    let work4 = arm::sha1su1(arm::sha1su0(work0, work1, work2), work3);
    let hash5 = arm::sha1c(hash4, arm::sha1h(emu::first(hash3)), work4 + SHA1_K0V);
    let work5 = arm::sha1su1(arm::sha1su0(work1, work2, work3), work4);
    let hash6 = arm::sha1p(hash5, arm::sha1h(emu::first(hash4)), work5 + SHA1_K1V);
    let work6 = arm::sha1su1(arm::sha1su0(work2, work3, work4), work5);
    let hash7 = arm::sha1p(hash6, arm::sha1h(emu::first(hash5)), work6 + SHA1_K1V);
    let work7 = arm::sha1su1(arm::sha1su0(work3, work4, work5), work6);
    let hash8 = arm::sha1p(hash7, arm::sha1h(emu::first(hash6)), work7 + SHA1_K1V);
    let work8 = arm::sha1su1(arm::sha1su0(work4, work5, work6), work7);
    let hash9 = arm::sha1p(hash8, arm::sha1h(emu::first(hash7)), work8 + SHA1_K1V);
    let work9 = arm::sha1su1(arm::sha1su0(work5, work6, work7), work8);
    let hash10 = arm::sha1p(hash9, arm::sha1h(emu::first(hash8)), work9 + SHA1_K1V);
    let work10 = arm::sha1su1(arm::sha1su0(work6, work7, work8), work9);
    let hash11 = arm::sha1m(hash10, arm::sha1h(emu::first(hash9)), work10 + SHA1_K2V);
    let work11 = arm::sha1su1(arm::sha1su0(work7, work8, work9), work10);
    let hash12 = arm::sha1m(hash11, arm::sha1h(emu::first(hash10)), work11 + SHA1_K2V);
    let work12 = arm::sha1su1(arm::sha1su0(work8, work9, work10), work11);
    let hash13 = arm::sha1m(hash12, arm::sha1h(emu::first(hash11)), work12 + SHA1_K2V);
    let work13 = arm::sha1su1(arm::sha1su0(work9, work10, work11), work12);
    let hash14 = arm::sha1m(hash13, arm::sha1h(emu::first(hash12)), work13 + SHA1_K2V);
    let work14 = arm::sha1su1(arm::sha1su0(work10, work11, work12), work13);
    let hash15 = arm::sha1m(hash14, arm::sha1h(emu::first(hash13)), work14 + SHA1_K2V);
    let work15 = arm::sha1su1(arm::sha1su0(work11, work12, work13), work14);
    let hash16 = arm::sha1p(hash15, arm::sha1h(emu::first(hash14)), work15 + SHA1_K3V);
    let work16 = arm::sha1su1(arm::sha1su0(work12, work13, work14), work15);
    let hash17 = arm::sha1p(hash16, arm::sha1h(emu::first(hash15)), work16 + SHA1_K3V);
    let work17 = arm::sha1su1(arm::sha1su0(work13, work14, work15), work16);
    let hash18 = arm::sha1p(hash17, arm::sha1h(emu::first(hash16)), work17 + SHA1_K3V);
    let work18 = arm::sha1su1(arm::sha1su0(work14, work15, work16), work17);
    let hash19 = arm::sha1p(hash18, arm::sha1h(emu::first(hash17)), work18 + SHA1_K3V);
    let work19 = arm::sha1su1(arm::sha1su0(work15, work16, work17), work18);
    let hash20 = arm::sha1p(hash19, arm::sha1h(emu::first(hash18)), work19 + SHA1_K3V);

    let u32x4(a, b, c, d) = hash20;
    hash[0] += a as u32;
    hash[1] += b as u32;
    hash[2] += c as u32;
    hash[3] += d as u32;
    hash[4] += rotate::left_u32(emu::first(hash19), 30) as u32;
}

/// Digest message block in words (x86-specific code)
pub fn digest_block_x86(hash: &mut [u32], msg: &[u32]) {
    assert_eq!(hash.len(), 5);
    assert_eq!(msg.len(), 16);
    
    let hash0 = u32x4(hash[0] as u32,
                      hash[1] as u32,
                      hash[2] as u32,
                      hash[3] as u32);
    
    let work0 = u32x4(msg[0] as u32,
                      msg[1] as u32,
                      msg[2] as u32,
                      msg[3] as u32);

    let hash1 = x86::sha1rnds4(hash0, emu::addfirst(work0, hash[4] as u32), 0);
    
    let work1 = u32x4(msg[4] as u32,
                      msg[5] as u32,
                      msg[6] as u32,
                      msg[7] as u32);
    
    let hash2 = x86::sha1rnds4(hash1, x86::sha1nexte(hash0, work1), 0);

    let work2 = u32x4(msg[8] as u32,
                      msg[9] as u32,
                      msg[10] as u32,
                      msg[11] as u32);

    let hash3 = x86::sha1rnds4(hash2, x86::sha1nexte(hash1, work2), 0);
    
    let work3 = u32x4(msg[12] as u32,
                      msg[13] as u32,
                      msg[14] as u32,
                      msg[15] as u32);
    
    let hash4 = x86::sha1rnds4(hash3, x86::sha1nexte(hash2, work3), 0);
    let work4 = x86::sha1msg2(x86::sha1msg1(work0, work1) ^ work2, work3);
    let hash5 = x86::sha1rnds4(hash4, x86::sha1nexte(hash3, work4), 0);
    let work5 = x86::sha1msg2(x86::sha1msg1(work1, work2) ^ work3, work4);
    let hash6 = x86::sha1rnds4(hash5, x86::sha1nexte(hash4, work5), 1);
    let work6 = x86::sha1msg2(x86::sha1msg1(work2, work3) ^ work4, work5);
    let hash7 = x86::sha1rnds4(hash6, x86::sha1nexte(hash5, work6), 1);
    let work7 = x86::sha1msg2(x86::sha1msg1(work3, work4) ^ work5, work6);
    let hash8 = x86::sha1rnds4(hash7, x86::sha1nexte(hash6, work7), 1);
    let work8 = x86::sha1msg2(x86::sha1msg1(work4, work5) ^ work6, work7);
    let hash9 = x86::sha1rnds4(hash8, x86::sha1nexte(hash7, work8), 1);
    let work9 = x86::sha1msg2(x86::sha1msg1(work5, work6) ^ work7, work8);
    let hash10 = x86::sha1rnds4(hash9, x86::sha1nexte(hash8, work9), 1);
    let work10 = x86::sha1msg2(x86::sha1msg1(work6, work7) ^ work8, work9);
    let hash11 = x86::sha1rnds4(hash10, x86::sha1nexte(hash9, work10), 2);
    let work11 = x86::sha1msg2(x86::sha1msg1(work7, work8) ^ work9, work10);
    let hash12 = x86::sha1rnds4(hash11, x86::sha1nexte(hash10, work11), 2);
    let work12 = x86::sha1msg2(x86::sha1msg1(work8, work9) ^ work10, work11);
    let hash13 = x86::sha1rnds4(hash12, x86::sha1nexte(hash11, work12), 2);
    let work13 = x86::sha1msg2(x86::sha1msg1(work9, work10) ^ work11, work12);
    let hash14 = x86::sha1rnds4(hash13, x86::sha1nexte(hash12, work13), 2);
    let work14 = x86::sha1msg2(x86::sha1msg1(work10, work11) ^ work12, work13);
    let hash15 = x86::sha1rnds4(hash14, x86::sha1nexte(hash13, work14), 2);
    let work15 = x86::sha1msg2(x86::sha1msg1(work11, work12) ^ work13, work14);
    let hash16 = x86::sha1rnds4(hash15, x86::sha1nexte(hash14, work15), 3);
    let work16 = x86::sha1msg2(x86::sha1msg1(work12, work13) ^ work14, work15);
    let hash17 = x86::sha1rnds4(hash16, x86::sha1nexte(hash15, work16), 3);
    let work17 = x86::sha1msg2(x86::sha1msg1(work13, work14) ^ work15, work16);
    let hash18 = x86::sha1rnds4(hash17, x86::sha1nexte(hash16, work17), 3);
    let work18 = x86::sha1msg2(x86::sha1msg1(work14, work15) ^ work16, work17);
    let hash19 = x86::sha1rnds4(hash18, x86::sha1nexte(hash17, work18), 3);
    let work19 = x86::sha1msg2(x86::sha1msg1(work15, work16) ^ work17, work18);
    let hash20 = x86::sha1rnds4(hash19, x86::sha1nexte(hash18, work19), 3);

    let u32x4(a, b, c, d) = hash20;
    hash[0] += a as u32;
    hash[1] += b as u32;
    hash[2] += c as u32;
    hash[3] += d as u32;
    hash[4] += rotate::left_u32(emu::first(hash19), 30) as u32;
}

//1181 ns/iter
pub fn digest_block_arm_emu(hash: &mut [u32], msg: &[u32]) {
    assert_eq!(hash.len(), 5);
    assert_eq!(msg.len(), 16);

    let hash0 = u32x4(hash[0] as u32,
                      hash[1] as u32,
                      hash[2] as u32,
                      hash[3] as u32);

    let work0 = u32x4(msg[0] as u32,
                      msg[1] as u32,
                      msg[2] as u32,
                      msg[3] as u32);

    let hash1 = emu::sha1c(hash0, hash[4] as u32, work0 + SHA1_K0V);

    let work1 = u32x4(msg[4] as u32,
                      msg[5] as u32,
                      msg[6] as u32,
                      msg[7] as u32);

    let hash2 = emu::sha1c(hash1, emu::sha1h(hash[0] as u32), work1 + SHA1_K0V);

    let work2 = u32x4(msg[8] as u32,
                      msg[9] as u32,
                      msg[10] as u32,
                      msg[11] as u32);

    let hash3 = emu::sha1c(hash2, emu::sha1h(emu::first(hash1)), work2 + SHA1_K0V);
    
    let work3 = u32x4(msg[12] as u32,
                      msg[13] as u32,
                      msg[14] as u32,
                      msg[15] as u32);
    
    let hash4 = emu::sha1c(hash3, emu::sha1h(emu::first(hash2)), work3 + SHA1_K0V);
    let work4 = emu::sha1su1(emu::sha1su0(work0, work1, work2), work3);
    let hash5 = emu::sha1c(hash4, emu::sha1h(emu::first(hash3)), work4 + SHA1_K0V);
    let work5 = emu::sha1su1(emu::sha1su0(work1, work2, work3), work4);
    let hash6 = emu::sha1p(hash5, emu::sha1h(emu::first(hash4)), work5 + SHA1_K1V);
    let work6 = emu::sha1su1(emu::sha1su0(work2, work3, work4), work5);
    let hash7 = emu::sha1p(hash6, emu::sha1h(emu::first(hash5)), work6 + SHA1_K1V);
    let work7 = emu::sha1su1(emu::sha1su0(work3, work4, work5), work6);
    let hash8 = emu::sha1p(hash7, emu::sha1h(emu::first(hash6)), work7 + SHA1_K1V);
    let work8 = emu::sha1su1(emu::sha1su0(work4, work5, work6), work7);
    let hash9 = emu::sha1p(hash8, emu::sha1h(emu::first(hash7)), work8 + SHA1_K1V);
    let work9 = emu::sha1su1(emu::sha1su0(work5, work6, work7), work8);
    let hash10 = emu::sha1p(hash9, emu::sha1h(emu::first(hash8)), work9 + SHA1_K1V);
    let work10 = emu::sha1su1(emu::sha1su0(work6, work7, work8), work9);
    let hash11 = emu::sha1m(hash10, emu::sha1h(emu::first(hash9)), work10 + SHA1_K2V);
    let work11 = emu::sha1su1(emu::sha1su0(work7, work8, work9), work10);
    let hash12 = emu::sha1m(hash11, emu::sha1h(emu::first(hash10)), work11 + SHA1_K2V);
    let work12 = emu::sha1su1(emu::sha1su0(work8, work9, work10), work11);
    let hash13 = emu::sha1m(hash12, emu::sha1h(emu::first(hash11)), work12 + SHA1_K2V);
    let work13 = emu::sha1su1(emu::sha1su0(work9, work10, work11), work12);
    let hash14 = emu::sha1m(hash13, emu::sha1h(emu::first(hash12)), work13 + SHA1_K2V);
    let work14 = emu::sha1su1(emu::sha1su0(work10, work11, work12), work13);
    let hash15 = emu::sha1m(hash14, emu::sha1h(emu::first(hash13)), work14 + SHA1_K2V);
    let work15 = emu::sha1su1(emu::sha1su0(work11, work12, work13), work14);
    let hash16 = emu::sha1p(hash15, emu::sha1h(emu::first(hash14)), work15 + SHA1_K3V);
    let work16 = emu::sha1su1(emu::sha1su0(work12, work13, work14), work15);
    let hash17 = emu::sha1p(hash16, emu::sha1h(emu::first(hash15)), work16 + SHA1_K3V);
    let work17 = emu::sha1su1(emu::sha1su0(work13, work14, work15), work16);
    let hash18 = emu::sha1p(hash17, emu::sha1h(emu::first(hash16)), work17 + SHA1_K3V);
    let work18 = emu::sha1su1(emu::sha1su0(work14, work15, work16), work17);
    let hash19 = emu::sha1p(hash18, emu::sha1h(emu::first(hash17)), work18 + SHA1_K3V);
    let work19 = emu::sha1su1(emu::sha1su0(work15, work16, work17), work18);
    let hash20 = emu::sha1p(hash19, emu::sha1h(emu::first(hash18)), work19 + SHA1_K3V);

    let u32x4(a, b, c, d) = hash20;
    hash[0] += a as u32;
    hash[1] += b as u32;
    hash[2] += c as u32;
    hash[3] += d as u32;
    hash[4] += rotate::left_u32(emu::first(hash19), 30) as u32;
}

// 1220 ns/iter
pub fn digest_block_x86_emu(hash: &mut [u32], msg: &[u32]) {
    assert_eq!(hash.len(), 5);
    assert_eq!(msg.len(), 16);
    
    let hash0 = u32x4(hash[0] as u32,
                      hash[1] as u32,
                      hash[2] as u32,
                      hash[3] as u32);
    
    let work0 = u32x4(msg[0] as u32,
                      msg[1] as u32,
                      msg[2] as u32,
                      msg[3] as u32);

    let hash1 = emu::sha1rnds4(hash0, emu::addfirst(work0, hash[4] as u32), 0);
    
    let work1 = u32x4(msg[4] as u32,
                      msg[5] as u32,
                      msg[6] as u32,
                      msg[7] as u32);
    
    let hash2 = emu::sha1rnds4(hash1, emu::sha1nexte(hash0, work1), 0);

    let work2 = u32x4(msg[8] as u32,
                      msg[9] as u32,
                      msg[10] as u32,
                      msg[11] as u32);

    let hash3 = emu::sha1rnds4(hash2, emu::sha1nexte(hash1, work2), 0);
    
    let work3 = u32x4(msg[12] as u32,
                      msg[13] as u32,
                      msg[14] as u32,
                      msg[15] as u32);
    
    let hash4 = emu::sha1rnds4(hash3, emu::sha1nexte(hash2, work3), 0);
    let work4 = emu::sha1msg2(emu::sha1msg1(work0, work1) ^ work2, work3);
    let hash5 = emu::sha1rnds4(hash4, emu::sha1nexte(hash3, work4), 0);
    let work5 = emu::sha1msg2(emu::sha1msg1(work1, work2) ^ work3, work4);
    let hash6 = emu::sha1rnds4(hash5, emu::sha1nexte(hash4, work5), 1);
    let work6 = emu::sha1msg2(emu::sha1msg1(work2, work3) ^ work4, work5);
    let hash7 = emu::sha1rnds4(hash6, emu::sha1nexte(hash5, work6), 1);
    let work7 = emu::sha1msg2(emu::sha1msg1(work3, work4) ^ work5, work6);
    let hash8 = emu::sha1rnds4(hash7, emu::sha1nexte(hash6, work7), 1);
    let work8 = emu::sha1msg2(emu::sha1msg1(work4, work5) ^ work6, work7);
    let hash9 = emu::sha1rnds4(hash8, emu::sha1nexte(hash7, work8), 1);
    let work9 = emu::sha1msg2(emu::sha1msg1(work5, work6) ^ work7, work8);
    let hash10 = emu::sha1rnds4(hash9, emu::sha1nexte(hash8, work9), 1);
    let work10 = emu::sha1msg2(emu::sha1msg1(work6, work7) ^ work8, work9);
    let hash11 = emu::sha1rnds4(hash10, emu::sha1nexte(hash9, work10), 2);
    let work11 = emu::sha1msg2(emu::sha1msg1(work7, work8) ^ work9, work10);
    let hash12 = emu::sha1rnds4(hash11, emu::sha1nexte(hash10, work11), 2);
    let work12 = emu::sha1msg2(emu::sha1msg1(work8, work9) ^ work10, work11);
    let hash13 = emu::sha1rnds4(hash12, emu::sha1nexte(hash11, work12), 2);
    let work13 = emu::sha1msg2(emu::sha1msg1(work9, work10) ^ work11, work12);
    let hash14 = emu::sha1rnds4(hash13, emu::sha1nexte(hash12, work13), 2);
    let work14 = emu::sha1msg2(emu::sha1msg1(work10, work11) ^ work12, work13);
    let hash15 = emu::sha1rnds4(hash14, emu::sha1nexte(hash13, work14), 2);
    let work15 = emu::sha1msg2(emu::sha1msg1(work11, work12) ^ work13, work14);
    let hash16 = emu::sha1rnds4(hash15, emu::sha1nexte(hash14, work15), 3);
    let work16 = emu::sha1msg2(emu::sha1msg1(work12, work13) ^ work14, work15);
    let hash17 = emu::sha1rnds4(hash16, emu::sha1nexte(hash15, work16), 3);
    let work17 = emu::sha1msg2(emu::sha1msg1(work13, work14) ^ work15, work16);
    let hash18 = emu::sha1rnds4(hash17, emu::sha1nexte(hash16, work17), 3);
    let work18 = emu::sha1msg2(emu::sha1msg1(work14, work15) ^ work16, work17);
    let hash19 = emu::sha1rnds4(hash18, emu::sha1nexte(hash17, work18), 3);
    let work19 = emu::sha1msg2(emu::sha1msg1(work15, work16) ^ work17, work18);
    let hash20 = emu::sha1rnds4(hash19, emu::sha1nexte(hash18, work19), 3);

    let u32x4(a, b, c, d) = hash20;
    hash[0] += a as u32;
    hash[1] += b as u32;
    hash[2] += c as u32;
    hash[3] += d as u32;
    hash[4] += rotate::left_u32(emu::first(hash19), 30) as u32;
}

// 1464 ns/iter
pub fn digest_block_naive(hash: &mut [u32], msg_block: &[u32]) {
    assert_eq!(hash.len(), 5);
    assert_eq!(msg_block.len(), 16);
    
    let mut hash2 = [0u32; 5];
    for t in 0us..5us {
        hash2[t] = hash[t];
    }
    
    let mut work = [0u32; 80];
    for t in 0us..16us {
        work[t] = msg_block[t];
    }
    
    init_work(&mut work);
    rounds80(&mut hash2, &work);

    for t in 0us..5us {
        hash[t] += hash2[t];
    }
}

/// Digest message block in bytes
#[allow(unstable)]
pub fn digest_bytes(hash: &mut [u32], msg_block: &[u8]) {
    assert_eq!(hash.len(), 5);
    assert_eq!(msg_block.len(), 64);

    // read_u32v_be()
    let mut words: Vec<u32> = Vec::new();
    for mut word in msg_block.chunks(4) {
        words.push(word.read_be_u32().unwrap());
    }
    
    digest_block(hash, words.as_slice());
}

/// Digest whole message
pub fn digest(hash: &mut [u32], msg: &[u8]) {
    init_hash(hash);
    let bytes = pad(msg, msg.len());
    for msg_block in bytes.chunks(64) {
        digest_bytes(hash, msg_block);
    }
}

#[allow(unstable)]
pub fn hex_digest(message: &str) -> String {
    let msg = message.as_bytes();
    let mut hash = [0u32; 5];
    digest(&mut hash, msg);
    
    // write_u32v_be()
    let mut bytes: Vec<u8> = Vec::new();
    for t in 0us..5us {
        bytes.write_be_u32(hash[t]).unwrap();
    }
    
    bytes.to_hex()
}


#[cfg(test)]
mod tests {
    use test::Bencher;
    use sha1;
        
    #[test]
    fn test_empty_string() {
        assert_eq!("da39a3ee5e6b4b0d3255bfef95601890afd80709", sha1::hex_digest("").as_slice());
    }
    
    #[test]
    fn test_hello_world() {
        assert_eq!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed", sha1::hex_digest("hello world").as_slice());
        assert_eq!("430ce34d020724ed75a196dfc2ad67c77772d169", sha1::hex_digest("hello world!").as_slice());
        assert_eq!("22c219648f00c61e5b3b1bd81ffa8e7767e2e3c5", sha1::hex_digest("hello World").as_slice());
        assert_eq!("788245b4dad73c1e5a630c126c484c7a2464f280", sha1::hex_digest("hello World!").as_slice());
        assert_eq!("7b502c3a1f48c8609ae212cdfb639dee39673f5e", sha1::hex_digest("Hello world").as_slice());
        assert_eq!("d3486ae9136e7856bc42212385ea797094475802", sha1::hex_digest("Hello world!").as_slice());
        assert_eq!("0a4d55a8d778e5022fab701977c5d840bbc486d0", sha1::hex_digest("Hello World").as_slice());
        assert_eq!("2ef7bde608ce5404e97d5f042f95f89f1c232871", sha1::hex_digest("Hello World!").as_slice());
        assert_eq!("b7e23ec29af22b0b4e41da31e868d57226121c84", sha1::hex_digest("hello, world").as_slice());
        assert_eq!("1f09d30c707d53f3d16c530dd73d70a6ce7596a9", sha1::hex_digest("hello, world!").as_slice());
        assert_eq!("ca3c58516ddef44b25693df5a915206e1bd094da", sha1::hex_digest("hello, World").as_slice());
        assert_eq!("dd0588c172986c32636ffdd8cc690de7b41bf253", sha1::hex_digest("hello, World!").as_slice());
        assert_eq!("e02aa1b106d5c7c6a98def2b13005d5b84fd8dc8", sha1::hex_digest("Hello, world").as_slice());
        assert_eq!("943a702d06f34599aee1f8da8ef9f7296031d699", sha1::hex_digest("Hello, world!").as_slice());
        assert_eq!("907d14fb3af2b0d4f18c2d46abe8aedce17367bd", sha1::hex_digest("Hello, World").as_slice());
        assert_eq!("0a0a9f2a6772942557ab5355d76af442f8f65e01", sha1::hex_digest("Hello, World!").as_slice());
    }
    
    #[test]
    fn test_multi_block() {
        let s = "GNU LESSER GENERAL PUBLIC LICENSE Version 3, 29 June 2007 Copyright (C) 2007 Free Software Foundation, Inc. <http://fsf.org/>";
        assert_eq!("a31e8cb8a139d146a0070fa13795d6766acaccd4", sha1::hex_digest(s).as_slice());
    }

    
    #[bench]
    fn bench_hello_world(b: & mut Bencher) {
        let s = "hello world";

        b.iter(|| sha1::hex_digest(s));
        
        //let mut sh = Sha1::new();
        //let bytes = [1u8; 65536];
        //bh.iter( || {
        //    sh.input(&bytes);
        //});
        //bh.bytes = bytes.len() as u64;
    }
    
    #[bench]
    fn bench_multi_block(b: & mut Bencher) {
        let s = "GNU LESSER GENERAL PUBLIC LICENSE Version 3, 29 June 2007 Copyright (C) 2007 Free Software Foundation, Inc. <http://fsf.org/>";

        b.iter(|| sha1::hex_digest(s));
        
        //let mut sh = Sha1::new();
        //let bytes = [1u8; 65536];
        //bh.iter( || {
        //    sh.input(&bytes);
        //});
        //bh.bytes = bytes.len() as u64;
    }
}
