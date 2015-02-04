use std::simd::u32x4;
use super::super::stdish::num::{PartialInt, RotateInt};
use super::emu;

extern {

    #[link_name = "llvm.x86.sha1rnds4"]
    pub fn sha1rnds4(a: u32x4, b: u32x4, c: i8) -> u32x4;

    #[link_name = "llvm.x86.sha1nexte"]
    pub fn sha1nexte(a: u32x4, b: u32x4) -> u32x4;

    #[link_name = "llvm.x86.sha1msg1"]
    pub fn sha1msg1(a: u32x4, b: u32x4) -> u32x4;

    #[link_name = "llvm.x86.sha1msg2"]
    pub fn sha1msg2(a: u32x4, b: u32x4) -> u32x4;
}

/// Checks CPUID.7.0.EBX[29]
pub fn has_sha() -> bool {
    let mut b: u32;
    
    unsafe {
        asm!("mov $$7, %eax
              mov $$0, %ecx
              cpuid
              mov %ebx, $0"
             : "=r"(b)
             :: "eax", "ebx", "ecx", "edx")
    }

    ((b >> 29) & 1) != 0
}

/// Digest message block (x86-specific)
pub fn digest_block_simd(msg_0: u32x4,
    msg_16: u32x4, msg_32: u32x4, msg_48: u32x4,
    hash_abcd: u32x4, hash_e: u32) -> (u32x4, u32) {
    unsafe {
        
        let w0 = msg_0;
        let h1 = sha1rnds4(hash_abcd, emu::add_1st(hash_e, w0), 0);
        let w1 = msg_16;
        let h2 = sha1rnds4(h1, sha1nexte(hash_abcd, w1), 0);
        let w2 = msg_32;
        let h3 = sha1rnds4(h2, sha1nexte(h1, w2), 0);
        let w3 = msg_48;
        let h4 = sha1rnds4(h3, sha1nexte(h2, w3), 0);
        let w4 = sha1msg2(sha1msg1(w0, w1) ^ w2, w3);
        let h5 = sha1rnds4(h4, sha1nexte(h3, w4), 0);
        let w5 = sha1msg2(sha1msg1(w1, w2) ^ w3, w4);
        let h6 = sha1rnds4(h5, sha1nexte(h4, w5), 1);
        let w6 = sha1msg2(sha1msg1(w2, w3) ^ w4, w5);
        let h7 = sha1rnds4(h6, sha1nexte(h5, w6), 1);
        let w7 = sha1msg2(sha1msg1(w3, w4) ^ w5, w6);
        let h8 = sha1rnds4(h7, sha1nexte(h6, w7), 1);
        let w8 = sha1msg2(sha1msg1(w4, w5) ^ w6, w7);
        let h9 = sha1rnds4(h8, sha1nexte(h7, w8), 1);
        let w9 = sha1msg2(sha1msg1(w5, w6) ^ w7, w8);
        let h10 = sha1rnds4(h9, sha1nexte(h8, w9), 1);
        let w10 = sha1msg2(sha1msg1(w6, w7) ^ w8, w9);
        let h11 = sha1rnds4(h10, sha1nexte(h9, w10), 2);
        let w11 = sha1msg2(sha1msg1(w7, w8) ^ w9, w10);
        let h12 = sha1rnds4(h11, sha1nexte(h10, w11), 2);
        let w12 = sha1msg2(sha1msg1(w8, w9) ^ w10, w11);
        let h13 = sha1rnds4(h12, sha1nexte(h11, w12), 2);
        let w13 = sha1msg2(sha1msg1(w9, w10) ^ w11, w12);
        let h14 = sha1rnds4(h13, sha1nexte(h12, w13), 2);
        let w14 = sha1msg2(sha1msg1(w10, w11) ^ w12, w13);
        let h15 = sha1rnds4(h14, sha1nexte(h13, w14), 2);
        let w15 = sha1msg2(sha1msg1(w11, w12) ^ w13, w14);
        let h16 = sha1rnds4(h15, sha1nexte(h14, w15), 3);
        let w16 = sha1msg2(sha1msg1(w12, w13) ^ w14, w15);
        let h17 = sha1rnds4(h16, sha1nexte(h15, w16), 3);
        let w17 = sha1msg2(sha1msg1(w13, w14) ^ w15, w16);
        let h18 = sha1rnds4(h17, sha1nexte(h16, w17), 3);
        let w18 = sha1msg2(sha1msg1(w14, w15) ^ w16, w17);
        let h19 = sha1rnds4(h18, sha1nexte(h17, w18), 3);
        let w19 = sha1msg2(sha1msg1(w15, w16) ^ w17, w18);
        let h20 = sha1rnds4(h19, sha1nexte(h18, w19), 3);
        let e20 = emu::get_1st(h19).rotate_left(30);
        
        (hash_abcd + h20, hash_e + e20)
    }
}
