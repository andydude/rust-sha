use std::simd::u32x4;
use super::super::stdish::num::{PartialInt, RotateInt};
use super::emu;

extern {
    
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

pub fn has_sha() -> bool {
    false
}

/// Digest message block (arm-specific)
pub fn digest_block(msg_0: u32x4,
    msg_16: u32x4, msg_32: u32x4, msg_48: u32x4,
    hash_abcd: u32x4, hash_e: u32) -> (u32x4, u32) {
    unsafe {
        
        let w0 = msg_0;
        let h1 = sha1c(hash_abcd, hash_e, w0 + emu::SHA1_K0V);
        let w1 = msg_16;
        let h2 = sha1c(h1, sha1h(emu::get_1st(hash_abcd)), w1 + emu::SHA1_K0V);
        let w2 = msg_32;
        let h3 = sha1c(h2, sha1h(emu::get_1st(h1)), w2 + emu::SHA1_K0V);
        let w3 = msg_48;
        let h4 = sha1c(h3, sha1h(emu::get_1st(h2)), w3 + emu::SHA1_K0V);
        let w4 = sha1su1(sha1su0(w0, w1, w2), w3);
        let h5 = sha1c(h4, sha1h(emu::get_1st(h3)), w4 + emu::SHA1_K0V);
        let w5 = sha1su1(sha1su0(w1, w2, w3), w4);
        let h6 = sha1p(h5, sha1h(emu::get_1st(h4)), w5 + emu::SHA1_K1V);
        let w6 = sha1su1(sha1su0(w2, w3, w4), w5);
        let h7 = sha1p(h6, sha1h(emu::get_1st(h5)), w6 + emu::SHA1_K1V);
        let w7 = sha1su1(sha1su0(w3, w4, w5), w6);
        let h8 = sha1p(h7, sha1h(emu::get_1st(h6)), w7 + emu::SHA1_K1V);
        let w8 = sha1su1(sha1su0(w4, w5, w6), w7);
        let h9 = sha1p(h8, sha1h(emu::get_1st(h7)), w8 + emu::SHA1_K1V);
        let w9 = sha1su1(sha1su0(w5, w6, w7), w8);
        let h10 = sha1p(h9, sha1h(emu::get_1st(h8)), w9 + emu::SHA1_K1V);
        let w10 = sha1su1(sha1su0(w6, w7, w8), w9);
        let h11 = sha1m(h10, sha1h(emu::get_1st(h9)), w10 + emu::SHA1_K2V);
        let w11 = sha1su1(sha1su0(w7, w8, w9), w10);
        let h12 = sha1m(h11, sha1h(emu::get_1st(h10)), w11 + emu::SHA1_K2V);
        let w12 = sha1su1(sha1su0(w8, w9, w10), w11);
        let h13 = sha1m(h12, sha1h(emu::get_1st(h11)), w12 + emu::SHA1_K2V);
        let w13 = sha1su1(sha1su0(w9, w10, w11), w12);
        let h14 = sha1m(h13, sha1h(emu::get_1st(h12)), w13 + emu::SHA1_K2V);
        let w14 = sha1su1(sha1su0(w10, w11, w12), w13);
        let h15 = sha1m(h14, sha1h(emu::get_1st(h13)), w14 + emu::SHA1_K2V);
        let w15 = sha1su1(sha1su0(w11, w12, w13), w14);
        let h16 = sha1p(h15, sha1h(emu::get_1st(h14)), w15 + emu::SHA1_K3V);
        let w16 = sha1su1(sha1su0(w12, w13, w14), w15);
        let h17 = sha1p(h16, sha1h(emu::get_1st(h15)), w16 + emu::SHA1_K3V);
        let w17 = sha1su1(sha1su0(w13, w14, w15), w16);
        let h18 = sha1p(h17, sha1h(emu::get_1st(h16)), w17 + emu::SHA1_K3V);
        let w18 = sha1su1(sha1su0(w14, w15, w16), w17);
        let h19 = sha1p(h18, sha1h(emu::get_1st(h17)), w18 + emu::SHA1_K3V);
        let w19 = sha1su1(sha1su0(w15, w16, w17), w18);
        let h20 = sha1p(h19, sha1h(emu::get_1st(h18)), w19 + emu::SHA1_K3V);
        let e20 = emu::get_1st(h19).rotate_left(30);

        (hash_abcd + h20, hash_e + e20)
    }
}
