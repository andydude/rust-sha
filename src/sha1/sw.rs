use std::simd::u32x4;
//use super::super::stdish::num::RotateInt;
use std::num::Int;
use stdish::num::RotateInt;
//use stdish::num::SwapBytesInt;
//use std::simd::u32x4;
//use std::old_io::IoError;
//use std::slice::bytes::copy_memory;
//use stdish::slice::transmute_memory;
//use stdish::io::{Digest, Reset, Read, Write, io_error};


#[unstable(feature="default", reason="1.0.0")]
pub mod constants {

    // needs concat_bytes!()
    #[unstable(feature="default", reason="1.0.0")]
    pub const IV: &'static [u8] = b"\x67\x45\x23\x01\xef\xcd\xab\x89\x98\xba\xdc\xfe\x10\x32\x54\x76\xc3\xd2\xe1\xf0";

    /// digits of floor(sqrt(2)*2 ^ 30)
    #[unstable(feature="default", reason="1.0.0")]
    pub const K0: u32 = 0x5a827999u32;

    /// digits of floor(sqrt(3)*2 ^ 30)
    #[unstable(feature="default", reason="1.0.0")]
    pub const K1: u32 = 0x6ed9eba1u32;

    /// digits of floor(sqrt(5)*2 ^ 30)
    #[unstable(feature="default", reason="1.0.0")]
    pub const K2: u32 = 0x8f1bbcdcu32;

    /// digits of floor(sqrt(10)*2 ^ 30)
    #[unstable(feature="default", reason="1.0.0")]
    pub const K3: u32 = 0xca62c1d6u32;

    //pub const KS: [u32; 4] = [
    //    0x5a827999u32, // digits of floor(sqrt(2) * 2 ^ 30)
    //    0x6ed9eba1u32, // digits of floor(sqrt(3) * 2 ^ 30)
    //    0x8f1bbcdcu32, // digits of floor(sqrt(5) * 2 ^ 30)
    //    0xca62c1d6u32, // digits of floor(sqrt(10) * 2 ^ 30)
    //];

    //pub const H0: u32 = 0x67452301u32; /// digits are (34 * n + 1) where n = 3, 2, 1, 0
    //pub const H1: u32 = 0xefcdab89u32; /// digits are (34 * n + 1) where n = 7, 6, 5, 4
    //pub const H2: u32 = 0x98badcfeu32; /// digits are (34 * n + 16) where n = 4, 5, 6, 7
    //pub const H3: u32 = 0x10325476u32; /// digits are (34 * n + 16) where n = 0, 1, 2, 3
    //pub const H4: u32 = 0xc3d2e1f0u32; /// digits are (15 * n) where n = 13, 14, 15, 16

    //pub static K0V: u32x4 = u32x4(K0, K0, K0, K0);
    //pub static K1V: u32x4 = u32x4(K1, K1, K1, K1);
    //pub static K2V: u32x4 = u32x4(K2, K2, K2, K2);
    //pub static K3V: u32x4 = u32x4(K3, K3, K3, K3);
    //pub static INITIAL_HASH: [u32; 5] = [H0, H1, H2, H3, H4];
    //const CONSTANT_POOL: [u32; 4] = [K0, K1, K2, K3];
}

#[unstable(feature="cryptoil_experimental", reason="this is temporary code for testing")]
pub mod arm {
    use std::simd::u32x4;

    #[unstable(feature="cryptoil_experimental", reason="this is temporary code for testing")]
    pub mod intrinsics {
        use std::simd::u32x4;
        use std::num::Int;

        //// Emulates llvm.arm.neon.sha1h
        // the letter 'H' might stand for half, maybe?
        #[inline]
        pub fn sha1h(a: u32) -> u32 {
            let b = a.rotate_left(30);
            b
        }

        /// Emulates llvm.arm.neon.sha1su0
        #[inline]
        pub fn sha1su0(a: u32x4, b: u32x4, c: u32x4) -> u32x4 {
            let u32x4(_, _, w2, w3) = a;
            let u32x4(w4, w5, _, _) = b;
            a ^ u32x4(w2, w3, w4, w5) ^ c
        }

        /// Emulates llvm.arm.neon.sha1su1
        #[inline]
        pub fn sha1su1(a: u32x4, b: u32x4) -> u32x4 {
            use stdish::num::RotateInt;

            let u32x4(x0, _, _, _) = a;
            let u32x4(_, w13, w14, w15) = b;
            let w16 = (x0 ^ w13).rotate_left(1);
            let d = a ^ u32x4(w13, w14, w15, w16);

            d.rotate_left(1)
        }

        /// Emulates llvm.arm.neon.sha1c
        #[inline]
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

        /// Emulates llvm.arm.neon.sha1p
        #[inline]
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

        /// Emulates llvm.arm.neon.sha1m
        #[inline]
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
    }

    // 337 MB/s
    #[stable(feature="default", since="1.0.0")]
    pub fn digest_block(hashw: &mut [u32; 5], msgv: &[u32x4; 4]) {
        use super::intrinsics::{get_1st};
        use super::constants::{K0, K1, K2, K3};
        use self::intrinsics::{sha1su0, sha1su1, sha1h, sha1c, sha1p, sha1m};
        use std::num::Int;

        const K0V: u32x4 = u32x4(K0, K0, K0, K0);
        const K1V: u32x4 = u32x4(K1, K1, K1, K1);
        const K2V: u32x4 = u32x4(K2, K2, K2, K2);
        const K3V: u32x4 = u32x4(K3, K3, K3, K3);

        let h0 = u32x4(hashw[0], hashw[1], hashw[2], hashw[3]);
        let w0 = msgv[0];
        let h1 = sha1c(h0, hashw[4], w0 + K0V);
        let w1 = msgv[1];
        let h2 = sha1c(h1, sha1h(get_1st(h0)), w1 + K0V);
        let w2 = msgv[2];
        let h3 = sha1c(h2, sha1h(get_1st(h1)), w2 + K0V);
        let w3 = msgv[3];
        let h4 = sha1c(h3, sha1h(get_1st(h2)), w3 + K0V);
        let w4 = sha1su1(sha1su0(w0, w1, w2), w3);
        let h5 = sha1c(h4, sha1h(get_1st(h3)), w4 + K0V);
        let w5 = sha1su1(sha1su0(w1, w2, w3), w4);
        let h6 = sha1p(h5, sha1h(get_1st(h4)), w5 + K1V);
        let w6 = sha1su1(sha1su0(w2, w3, w4), w5);
        let h7 = sha1p(h6, sha1h(get_1st(h5)), w6 + K1V);
        let w7 = sha1su1(sha1su0(w3, w4, w5), w6);
        let h8 = sha1p(h7, sha1h(get_1st(h6)), w7 + K1V);
        let w8 = sha1su1(sha1su0(w4, w5, w6), w7);
        let h9 = sha1p(h8, sha1h(get_1st(h7)), w8 + K1V);
        let w9 = sha1su1(sha1su0(w5, w6, w7), w8);
        let h10 = sha1p(h9, sha1h(get_1st(h8)), w9 + K1V);
        let w10 = sha1su1(sha1su0(w6, w7, w8), w9);
        let h11 = sha1m(h10, sha1h(get_1st(h9)), w10 + K2V);
        let w11 = sha1su1(sha1su0(w7, w8, w9), w10);
        let h12 = sha1m(h11, sha1h(get_1st(h10)), w11 + K2V);
        let w12 = sha1su1(sha1su0(w8, w9, w10), w11);
        let h13 = sha1m(h12, sha1h(get_1st(h11)), w12 + K2V);
        let w13 = sha1su1(sha1su0(w9, w10, w11), w12);
        let h14 = sha1m(h13, sha1h(get_1st(h12)), w13 + K2V);
        let w14 = sha1su1(sha1su0(w10, w11, w12), w13);
        let h15 = sha1m(h14, sha1h(get_1st(h13)), w14 + K2V);
        let w15 = sha1su1(sha1su0(w11, w12, w13), w14);
        let h16 = sha1p(h15, sha1h(get_1st(h14)), w15 + K3V);
        let w16 = sha1su1(sha1su0(w12, w13, w14), w15);
        let h17 = sha1p(h16, sha1h(get_1st(h15)), w16 + K3V);
        let w17 = sha1su1(sha1su0(w13, w14, w15), w16);
        let h18 = sha1p(h17, sha1h(get_1st(h16)), w17 + K3V);
        let w18 = sha1su1(sha1su0(w14, w15, w16), w17);
        let h19 = sha1p(h18, sha1h(get_1st(h17)), w18 + K3V);
        let w19 = sha1su1(sha1su0(w15, w16, w17), w18);
        let h20 = sha1p(h19, sha1h(get_1st(h18)), w19 + K3V);
        let e20 = get_1st(h19).rotate_left(30);

        let u32x4(a, b, c, d) = h20;

        hashw[0] += a;
        hashw[1] += b;
        hashw[2] += c;
        hashw[3] += d;
        hashw[4] += e20;
    }
} // mod cryptoil::sha1::sw::arm

#[unstable(feature="cryptoil_experimental", reason="this is temporary code for testing")]
pub mod x86 {
    use std::simd::u32x4;
    pub mod intrinsics {
        use std::simd::u32x4;
        use std::num::Int;

        /// Emulates `llvm.x86.sha1msg1` intrinsic.
        #[inline]
        pub fn sha1msg1(a: u32x4, b: u32x4) -> u32x4 {
            use super::super::arm::intrinsics::sha1su0;
            sha1su0(a, b, u32x4(0, 0, 0, 0))
        }

        /// Emulates `llvm.x86.sha1msg2` intrinsic.
        #[inline]
        pub fn sha1msg2(a: u32x4, b: u32x4) -> u32x4 {
            use super::super::arm::intrinsics::sha1su1;
            sha1su1(a, b)
        }

        /// Emulates `llvm.x86.sha1nexte` intrinsic.
        #[inline]
        pub fn sha1nexte(hash: u32x4, msg: u32x4) -> u32x4 {
            use super::super::intrinsics::{add_1st, get_1st};
            add_1st(get_1st(hash).rotate_left(30), msg)
        }

        /// Emulates `llvm.x86.sha1rnds4` intrinsic.
        #[inline]
        pub fn sha1rnds4(hash: u32x4, work: u32x4, i: i8) -> u32x4 {
            use super::super::arm::intrinsics::{sha1c, sha1p, sha1m};
            use super::super::constants::{K0, K1, K2, K3};
            const K0V: u32x4 = u32x4(K0, K0, K0, K0);
            const K1V: u32x4 = u32x4(K1, K1, K1, K1);
            const K2V: u32x4 = u32x4(K2, K2, K2, K2);
            const K3V: u32x4 = u32x4(K3, K3, K3, K3);

            match i {
                0 => sha1c(hash, 0, work + K0V),
                1 => sha1p(hash, 0, work + K1V),
                2 => sha1m(hash, 0, work + K2V),
                3 => sha1p(hash, 0, work + K3V),
                _ => panic!("unknown icosaround index")
            }
        }
    } // mod cryptoil::sha1::emu::x86::intrinsics

    // 297 MB/s
    pub fn digest_block(hashw: &mut [u32; 5], msgv: &[u32x4; 4]) {
        use super::intrinsics::{add_1st, get_1st};
        use self::intrinsics::{sha1msg1, sha1msg2, sha1nexte, sha1rnds4};
        use std::num::Int;

        let h0 = u32x4(hashw[0], hashw[1], hashw[2], hashw[3]);
        let w0 = msgv[0];
        let h1 = sha1rnds4(h0, add_1st(hashw[4], w0), 0);
        let w1 = msgv[1];
        let h2 = sha1rnds4(h1, sha1nexte(h0, w1), 0);
        let w2 = msgv[2];
        let h3 = sha1rnds4(h2, sha1nexte(h1, w2), 0);
        let w3 = msgv[3];
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
        let e20 = get_1st(h19).rotate_left(30);

        let u32x4(a, b, c, d) = h20;

        hashw[0] += a;
        hashw[1] += b;
        hashw[2] += c;
        hashw[3] += d;
        hashw[4] += e20;
    }
} // mod cryptoil::sha1::sw::x86

#[unstable(feature="default", reason="1.0.0")]
pub mod intrinsics {
    use std::simd::u32x4;

    #[unstable(feature="cryptoil_internals", reason="1.0.0")]
    #[inline]
    pub fn add_1st(e: u32, w0: u32x4) -> u32x4 {
        let u32x4(a, b, c, d) = w0;
        u32x4(e + a, b, c, d)
    }

    #[unstable(feature="cryptoil_internals", reason="1.0.0")]
    #[inline]
    pub fn get_1st(w0: u32x4) -> u32 {
        let u32x4(a, _, _, _) = w0;
        a
    }

    #[inline]
    #[stable(feature="cryptoil_internals", since="1.0.0")]
    pub fn schedule(w0: u32x4, w1: u32x4, w2: u32x4, w3: u32x4) -> u32x4 {
        use super::x86::intrinsics::{sha1msg1, sha1msg2};
        // this uses emulated x86 intrinsics
        sha1msg2(sha1msg1(w0, w1) ^ w2, w3)
    }

    // constant icosaround index

    #[inline]
    #[stable(feature="cryptoil_internals", since="1.0.0")]
    pub fn rounds40(hash: u32x4, oldh: u32x4, work: u32x4) -> u32x4 {
        use super::constants::K0;
        use super::arm::intrinsics::{sha1h, sha1c};
        // this uses emulated ARM intrinsics

        const K0V: u32x4 = u32x4(K0, K0, K0, K0);
        sha1c(hash, sha1h(get_1st(oldh)), work + K0V)
    }

    #[inline]
    #[stable(feature="cryptoil_internals", since="1.0.0")]
    pub fn rounds41(hash: u32x4, oldh: u32x4, work: u32x4) -> u32x4 {
        use super::constants::K1;
        use super::arm::intrinsics::{sha1h, sha1p};
        // this uses emulated ARM intrinsics
        const K1V: u32x4 = u32x4(K1, K1, K1, K1);
        sha1p(hash, sha1h(get_1st(oldh)), work + K1V)
    }

    #[inline]
    #[stable(feature="cryptoil_internals", since="1.0.0")]
    pub fn rounds42(hash: u32x4, oldh: u32x4, work: u32x4) -> u32x4 {
        use super::constants::K2;
        use super::arm::intrinsics::{sha1h, sha1m};
        // this uses emulated ARM intrinsics
        const K2V: u32x4 = u32x4(K2, K2, K2, K2);
        sha1m(hash, sha1h(get_1st(oldh)), work + K2V)
    }

    #[inline]
    #[stable(feature="cryptoil_internals", since="1.0.0")]
    pub fn rounds43(hash: u32x4, oldh: u32x4, work: u32x4) -> u32x4 {
        use super::constants::K3;
        use super::arm::intrinsics::{sha1h, sha1p};
        // this uses emulated ARM intrinsics
        const K3V: u32x4 = u32x4(K3, K3, K3, K3);
        sha1p(hash, sha1h(get_1st(oldh)), work + K3V)
    }
} // mod cryptoil::sha1::sw::intrinsics

//#[stable(feature="default", since="1.0.0")]
//pub fn digest_block_old(hashw: &mut [u32; 5], msgv: &[u32x4; 4]) {
//    use self::intrinsics::{add_1st, get_1st, schedule};
//    use self::intrinsics::{rounds40, rounds41, rounds42, rounds43};
//
//    let h0 = u32x4(hashw[0], hashw[1], hashw[2], hashw[3]);
//    let w0 = msgv[0];
//    let h1 = rounds40(h0, u32x4(0, 0, 0, 0), add_1st(hashw[4], w0));
//    let w1 = msgv[1];
//    let h2 = rounds40(h1, h0, w1);
//    let w2 = msgv[2];
//    let h3 = rounds40(h2, h1, w2);
//    let w3 = msgv[3];
//    let h4 = rounds40(h3, h2, w3);
//    let w4 = schedule(w0, w1, w2, w3);
//    let h5 = rounds40(h4, h3, w4);
//    let w5 = schedule(w1, w2, w3, w4);
//    let h6 = rounds41(h5, h4, w5);
//    let w6 = schedule(w2, w3, w4, w5);
//    let h7 = rounds41(h6, h5, w6);
//    let w7 = schedule(w3, w4, w5, w6);
//    let h8 = rounds41(h7, h6, w7);
//    let w8 = schedule(w4, w5, w6, w7);
//    let h9 = rounds41(h8, h7, w8);
//    let w9 = schedule(w5, w6, w7, w8);
//    let h10 = rounds41(h9, h8, w9);
//    let w10 = schedule(w6, w7, w8, w9);
//    let h11 = rounds42(h10, h9, w10);
//    let w11 = schedule(w7, w8, w9, w10);
//    let h12 = rounds42(h11, h10, w11);
//    let w12 = schedule(w8, w9, w10, w11);
//    let h13 = rounds42(h12, h11, w12);
//    let w13 = schedule(w9, w10, w11, w12);
//    let h14 = rounds42(h13, h12, w13);
//    let w14 = schedule(w10, w11, w12, w13);
//    let h15 = rounds42(h14, h13, w14);
//    let w15 = schedule(w11, w12, w13, w14);
//    let h16 = rounds43(h15, h14, w15);
//    let w16 = schedule(w12, w13, w14, w15);
//    let h17 = rounds43(h16, h15, w16);
//    let w17 = schedule(w13, w14, w15, w16);
//    let h18 = rounds43(h17, h16, w17);
//    let w18 = schedule(w14, w15, w16, w17);
//    let h19 = rounds43(h18, h17, w18);
//    let w19 = schedule(w15, w16, w17, w18);
//    let h20 = rounds43(h19, h18, w19);
//    let e20 = get_1st(h19).rotate_left(30);
//
//    let u32x4(a, b, c, d) = h20;
//
//    hashw[0] += a;
//    hashw[1] += b;
//    hashw[2] += c;
//    hashw[3] += d;
//    hashw[4] += e20;
//}

macro_rules! sha1_schedule {
    ($w0:expr, $w1:expr, $w2:expr, $w3:expr) => {
        {
            use std::num::Int;
            use stdish::num::RotateInt;
            let u32x4(w0, _, w2, w3) = $w0;
            let u32x4(w4, w5, _, _) = $w1;
            let w17 = $w0 ^ u32x4(w2, w3, w4, w5) ^ $w2;
            let u32x4(w8, _, _, _) = $w2;
            let u32x4(_, w13, w14, w15) = $w3;
            let w16 = (w0 ^ w2 ^ w8 ^ w13).rotate_left(1);
            (w17 ^ u32x4(w13, w14, w15, w16)).rotate_left(1)
        }
    }
}

//macro_rules! sha1_round {
//    ($f:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $g:expr) => {
//        {
//            use std::num::Int;
//            $f = $a.rotate_left(5) + $e + $g;
//            $e = $d; $d = $c; $c = $b.rotate_left(30); $b = $a; $a = $f;
//        }
//    }
//}

macro_rules! sha1_round {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $rest:expr) => {
        {
            use std::num::Int;
            $e += $a.rotate_left(5) + $rest;
            $b = $b.rotate_left(30);
        }
    }
}

macro_rules! sha1_debug {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        println!("{:08x}{:08x}{:08x}{:08x} {:08x}", $a, $b, $c, $d, $e);
    }
}
//sha1_debug!(a, b, c, d, e);

#[allow(unused_variables)]
#[stable(feature="default", since="1.0.0")]
pub fn digest_block_fast(hashw: &mut [u32; 5], msgv: &[u32x4; 4]) {
    use self::constants::{K0, K1, K2, K3};
    let (k0, k1, k2, k3) = (K0, K1, K2, K3);
    
    let mut a = hashw[0];
    let mut b = hashw[1];
    let mut c = hashw[2];
    let mut d = hashw[3];
    let mut e = hashw[4];

    let w0 = msgv[0]; let u32x4(t, u, v, w) = w0;
    e += a.rotate_left(5) + bool3ary_202!(b, c, d) + t + k0; b = b.rotate_left(30);
    d += e.rotate_left(5) + bool3ary_202!(a, b, c) + u + k0; a = a.rotate_left(30);
    c += d.rotate_left(5) + bool3ary_202!(e, a, b) + v + k0; e = e.rotate_left(30);
    b += c.rotate_left(5) + bool3ary_202!(d, e, a) + w + k0; d = d.rotate_left(30);
    let w1 = msgv[1]; let u32x4(t, u, v, w) = w1;
    a += b.rotate_left(5) + bool3ary_202!(c, d, e) + t + k0; c = c.rotate_left(30);
    e += a.rotate_left(5) + bool3ary_202!(b, c, d) + u + k0; b = b.rotate_left(30);
    d += e.rotate_left(5) + bool3ary_202!(a, b, c) + v + k0; a = a.rotate_left(30);
    c += d.rotate_left(5) + bool3ary_202!(e, a, b) + w + k0; e = e.rotate_left(30);
    let w2 = msgv[2]; let u32x4(t, u, v, w) = w2;
    b += c.rotate_left(5) + bool3ary_202!(d, e, a) + t + k0; d = d.rotate_left(30);
    a += b.rotate_left(5) + bool3ary_202!(c, d, e) + u + k0; c = c.rotate_left(30);
    e += a.rotate_left(5) + bool3ary_202!(b, c, d) + v + k0; b = b.rotate_left(30);
    d += e.rotate_left(5) + bool3ary_202!(a, b, c) + w + k0; a = a.rotate_left(30);
    let w3 = msgv[3]; let u32x4(t, u, v, w) = w3;
    c += d.rotate_left(5) + bool3ary_202!(e, a, b) + t + k0; e = e.rotate_left(30);
    b += c.rotate_left(5) + bool3ary_202!(d, e, a) + u + k0; d = d.rotate_left(30);
    a += b.rotate_left(5) + bool3ary_202!(c, d, e) + v + k0; c = c.rotate_left(30);
    e += a.rotate_left(5) + bool3ary_202!(b, c, d) + w + k0; b = b.rotate_left(30);
    let w4 = sha1_schedule!(w0, w1, w2, w3); let u32x4(t, u, v, w) = w4;
    d += e.rotate_left(5) + bool3ary_202!(a, b, c) + t + k0; a = a.rotate_left(30);
    c += d.rotate_left(5) + bool3ary_202!(e, a, b) + u + k0; e = e.rotate_left(30);
    b += c.rotate_left(5) + bool3ary_202!(d, e, a) + v + k0; d = d.rotate_left(30);
    a += b.rotate_left(5) + bool3ary_202!(c, d, e) + w + k0; c = c.rotate_left(30);

    let w5 = sha1_schedule!(w1, w2, w3, w4); let u32x4(t, u, v, w) = w5;
    e += a.rotate_left(5) + bool3ary_150!(b, c, d) + t + k1; b = b.rotate_left(30);
    d += e.rotate_left(5) + bool3ary_150!(a, b, c) + u + k1; a = a.rotate_left(30);
    c += d.rotate_left(5) + bool3ary_150!(e, a, b) + v + k1; e = e.rotate_left(30);
    b += c.rotate_left(5) + bool3ary_150!(d, e, a) + w + k1; d = d.rotate_left(30);
    let w6 = sha1_schedule!(w2, w3, w4, w5); let u32x4(t, u, v, w) = w6;
    a += b.rotate_left(5) + bool3ary_150!(c, d, e) + t + k1; c = c.rotate_left(30);
    e += a.rotate_left(5) + bool3ary_150!(b, c, d) + u + k1; b = b.rotate_left(30);
    d += e.rotate_left(5) + bool3ary_150!(a, b, c) + v + k1; a = a.rotate_left(30);
    c += d.rotate_left(5) + bool3ary_150!(e, a, b) + w + k1; e = e.rotate_left(30);
    let w7 = sha1_schedule!(w3, w4, w5, w6); let u32x4(t, u, v, w) = w7;
    b += c.rotate_left(5) + bool3ary_150!(d, e, a) + t + k1; d = d.rotate_left(30);
    a += b.rotate_left(5) + bool3ary_150!(c, d, e) + u + k1; c = c.rotate_left(30);
    e += a.rotate_left(5) + bool3ary_150!(b, c, d) + v + k1; b = b.rotate_left(30);
    d += e.rotate_left(5) + bool3ary_150!(a, b, c) + w + k1; a = a.rotate_left(30);
    let w8 = sha1_schedule!(w4, w5, w6, w7); let u32x4(t, u, v, w) = w8;
    c += d.rotate_left(5) + bool3ary_150!(e, a, b) + t + k1; e = e.rotate_left(30);
    b += c.rotate_left(5) + bool3ary_150!(d, e, a) + u + k1; d = d.rotate_left(30);
    a += b.rotate_left(5) + bool3ary_150!(c, d, e) + v + k1; c = c.rotate_left(30);
    e += a.rotate_left(5) + bool3ary_150!(b, c, d) + w + k1; b = b.rotate_left(30);
    let w9 = sha1_schedule!(w5, w6, w7, w8); let u32x4(t, u, v, w) = w9;
    d += e.rotate_left(5) + bool3ary_150!(a, b, c) + t + k1; a = a.rotate_left(30);
    c += d.rotate_left(5) + bool3ary_150!(e, a, b) + u + k1; e = e.rotate_left(30);
    b += c.rotate_left(5) + bool3ary_150!(d, e, a) + v + k1; d = d.rotate_left(30);
    a += b.rotate_left(5) + bool3ary_150!(c, d, e) + w + k1; c = c.rotate_left(30);

    let w10 = sha1_schedule!(w6, w7, w8, w9); let u32x4(t, u, v, w) = w10;
    e += a.rotate_left(5) + bool3ary_232!(b, c, d) + t + k2; b = b.rotate_left(30);
    d += e.rotate_left(5) + bool3ary_232!(a, b, c) + u + k2; a = a.rotate_left(30);
    c += d.rotate_left(5) + bool3ary_232!(e, a, b) + v + k2; e = e.rotate_left(30);
    b += c.rotate_left(5) + bool3ary_232!(d, e, a) + w + k2; d = d.rotate_left(30);
    let w11 = sha1_schedule!(w7, w8, w9, w10); let u32x4(t, u, v, w) = w11;
    a += b.rotate_left(5) + bool3ary_232!(c, d, e) + t + k2; c = c.rotate_left(30);
    e += a.rotate_left(5) + bool3ary_232!(b, c, d) + u + k2; b = b.rotate_left(30);
    d += e.rotate_left(5) + bool3ary_232!(a, b, c) + v + k2; a = a.rotate_left(30);
    c += d.rotate_left(5) + bool3ary_232!(e, a, b) + w + k2; e = e.rotate_left(30);
    let w12 = sha1_schedule!(w8, w9, w10, w11); let u32x4(t, u, v, w) = w12;
    b += c.rotate_left(5) + bool3ary_232!(d, e, a) + t + k2; d = d.rotate_left(30);
    a += b.rotate_left(5) + bool3ary_232!(c, d, e) + u + k2; c = c.rotate_left(30);
    e += a.rotate_left(5) + bool3ary_232!(b, c, d) + v + k2; b = b.rotate_left(30);
    d += e.rotate_left(5) + bool3ary_232!(a, b, c) + w + k2; a = a.rotate_left(30);
    let w13 = sha1_schedule!(w9, w10, w11, w12); let u32x4(t, u, v, w) = w13;
    c += d.rotate_left(5) + bool3ary_232!(e, a, b) + t + k2; e = e.rotate_left(30);
    b += c.rotate_left(5) + bool3ary_232!(d, e, a) + u + k2; d = d.rotate_left(30);
    a += b.rotate_left(5) + bool3ary_232!(c, d, e) + v + k2; c = c.rotate_left(30);
    e += a.rotate_left(5) + bool3ary_232!(b, c, d) + w + k2; b = b.rotate_left(30);
    let w14 = sha1_schedule!(w10, w11, w12, w13); let u32x4(t, u, v, w) = w14;
    d += e.rotate_left(5) + bool3ary_232!(a, b, c) + t + k2; a = a.rotate_left(30);
    c += d.rotate_left(5) + bool3ary_232!(e, a, b) + u + k2; e = e.rotate_left(30);
    b += c.rotate_left(5) + bool3ary_232!(d, e, a) + v + k2; d = d.rotate_left(30);
    a += b.rotate_left(5) + bool3ary_232!(c, d, e) + w + k2; c = c.rotate_left(30);

    let w15 = sha1_schedule!(w11, w12, w13, w14); let u32x4(t, u, v, w) = w15;
    e += a.rotate_left(5) + bool3ary_150!(b, c, d) + t + k3; b = b.rotate_left(30);
    d += e.rotate_left(5) + bool3ary_150!(a, b, c) + u + k3; a = a.rotate_left(30);
    c += d.rotate_left(5) + bool3ary_150!(e, a, b) + v + k3; e = e.rotate_left(30);
    b += c.rotate_left(5) + bool3ary_150!(d, e, a) + w + k3; d = d.rotate_left(30);
    let w16 = sha1_schedule!(w12, w13, w14, w15); let u32x4(t, u, v, w) = w16;
    a += b.rotate_left(5) + bool3ary_150!(c, d, e) + t + k3; c = c.rotate_left(30);
    e += a.rotate_left(5) + bool3ary_150!(b, c, d) + u + k3; b = b.rotate_left(30);
    d += e.rotate_left(5) + bool3ary_150!(a, b, c) + v + k3; a = a.rotate_left(30);
    c += d.rotate_left(5) + bool3ary_150!(e, a, b) + w + k3; e = e.rotate_left(30);
    let w17 = sha1_schedule!(w13, w14, w15, w16); let u32x4(t, u, v, w) = w17;
    b += c.rotate_left(5) + bool3ary_150!(d, e, a) + t + k3; d = d.rotate_left(30);
    a += b.rotate_left(5) + bool3ary_150!(c, d, e) + u + k3; c = c.rotate_left(30);
    e += a.rotate_left(5) + bool3ary_150!(b, c, d) + v + k3; b = b.rotate_left(30);
    d += e.rotate_left(5) + bool3ary_150!(a, b, c) + w + k3; a = a.rotate_left(30);
    let w18 = sha1_schedule!(w14, w15, w16, w17); let u32x4(t, u, v, w) = w18;
    c += d.rotate_left(5) + bool3ary_150!(e, a, b) + t + k3; e = e.rotate_left(30);
    b += c.rotate_left(5) + bool3ary_150!(d, e, a) + u + k3; d = d.rotate_left(30);
    a += b.rotate_left(5) + bool3ary_150!(c, d, e) + v + k3; c = c.rotate_left(30);
    e += a.rotate_left(5) + bool3ary_150!(b, c, d) + w + k3; b = b.rotate_left(30);
    let w19 = sha1_schedule!(w15, w16, w17, w18); let u32x4(t, u, v, w) = w19;
    d += e.rotate_left(5) + bool3ary_150!(a, b, c) + t + k3; a = a.rotate_left(30);
    c += d.rotate_left(5) + bool3ary_150!(e, a, b) + u + k3; e = e.rotate_left(30);
    b += c.rotate_left(5) + bool3ary_150!(d, e, a) + v + k3; d = d.rotate_left(30);
    a += b.rotate_left(5) + bool3ary_150!(c, d, e) + w + k3; c = c.rotate_left(30);

    hashw[0] += a;
    hashw[1] += b;
    hashw[2] += c;
    hashw[3] += d;
    hashw[4] += e;
}

//#[allow(unused_variables)]
//#[stable(feature="default", since="1.0.0")]
//pub fn digest_block_superfast(hashw: &mut [u32; 5], msgv: &[u32x4; 4]) {
//    use std::num::Int;
//    use stdish::num::RotateInt;
//    use self::constants::{K0, K1, K2, K3};
//    let (k0, k1, k2, k3) = (K0, K1, K2, K3);
//    let mut a = hashw[0];
//    let mut b = hashw[1];
//    let mut c = hashw[2];
//    let mut d = hashw[3];
//    let mut e = hashw[4];
//    let mut f: u32;
//
//    //sha1_dump!(a, b, c, d, e);
//
//    let w0 = msgv[0]; let u32x4(t, u, v, w) = w0;
//    e += rotl5!(a) + bool3ary_202!(b, c, d) + t + k0; b = rotl30!(b);
//    d += rotl5!(e) + bool3ary_202!(a, b, c) + u + k0; a = rotl30!(a);
//    c += rotl5!(d) + bool3ary_202!(e, a, b) + v + k0; e = rotl30!(e);
//    b += rotl5!(c) + bool3ary_202!(d, e, a) + w + k0; d = rotl30!(d);
//    let w1 = msgv[1]; let u32x4(t, u, v, w) = w1;
//    a += rotl5!(b) + bool3ary_202!(c, d, e) + t + k0; c = rotl30!(c);
//    e += rotl5!(a) + bool3ary_202!(b, c, d) + u + k0; b = rotl30!(b);
//    d += rotl5!(e) + bool3ary_202!(a, b, c) + v + k0; a = rotl30!(a);
//    c += rotl5!(d) + bool3ary_202!(e, a, b) + w + k0; e = rotl30!(e);
//    let w2 = msgv[2]; let u32x4(t, u, v, w) = w2;
//    b += rotl5!(c) + bool3ary_202!(d, e, a) + t + k0; d = rotl30!(d);
//    a += rotl5!(b) + bool3ary_202!(c, d, e) + u + k0; c = rotl30!(c);
//    e += rotl5!(a) + bool3ary_202!(b, c, d) + v + k0; b = rotl30!(b);
//    d += rotl5!(e) + bool3ary_202!(a, b, c) + w + k0; a = rotl30!(a);
//    let w3 = msgv[3]; let u32x4(t, u, v, w) = w3;
//    c += rotl5!(d) + bool3ary_202!(e, a, b) + t + k0; e = rotl30!(e);
//    b += rotl5!(c) + bool3ary_202!(d, e, a) + u + k0; d = rotl30!(d);
//    a += rotl5!(b) + bool3ary_202!(c, d, e) + v + k0; c = rotl30!(c);
//    e += rotl5!(a) + bool3ary_202!(b, c, d) + w + k0; b = rotl30!(b);
//    let w4 = sha1_schedule!(w0, w1, w2, w3); let u32x4(t, u, v, w) = w4;
//    d += rotl5!(e) + bool3ary_202!(a, b, c) + t + k0; a = rotl30!(a);
//    c += rotl5!(d) + bool3ary_202!(e, a, b) + u + k0; e = rotl30!(e);
//    b += rotl5!(c) + bool3ary_202!(d, e, a) + v + k0; d = rotl30!(d);
//    a += rotl5!(b) + bool3ary_202!(c, d, e) + w + k0; c = rotl30!(c);
//    
//    let w5 = sha1_schedule!(w1, w2, w3, w4); let u32x4(t, u, v, w) = w5;
//    e += rotl5!(a) + bool3ary_150!(b, c, d) + t + k1; b = rotl30!(b);
//    d += rotl5!(e) + bool3ary_150!(a, b, c) + u + k1; a = rotl30!(a);
//    c += rotl5!(d) + bool3ary_150!(e, a, b) + v + k1; e = rotl30!(e);
//    b += rotl5!(c) + bool3ary_150!(d, e, a) + w + k1; d = rotl30!(d);
//    let w6 = sha1_schedule!(w2, w3, w4, w5); let u32x4(t, u, v, w) = w6;
//    a += rotl5!(b) + bool3ary_150!(c, d, e) + t + k1; c = rotl30!(c);
//    e += rotl5!(a) + bool3ary_150!(b, c, d) + u + k1; b = rotl30!(b);
//    d += rotl5!(e) + bool3ary_150!(a, b, c) + v + k1; a = rotl30!(a);
//    c += rotl5!(d) + bool3ary_150!(e, a, b) + w + k1; e = rotl30!(e);
//    let w7 = sha1_schedule!(w3, w4, w5, w6); let u32x4(t, u, v, w) = w7;
//    b += rotl5!(c) + bool3ary_150!(d, e, a) + t + k1; d = rotl30!(d);
//    a += rotl5!(b) + bool3ary_150!(c, d, e) + u + k1; c = rotl30!(c);
//    e += rotl5!(a) + bool3ary_150!(b, c, d) + v + k1; b = rotl30!(b);
//    d += rotl5!(e) + bool3ary_150!(a, b, c) + w + k1; a = rotl30!(a);
//    let w8 = sha1_schedule!(w4, w5, w6, w7); let u32x4(t, u, v, w) = w8;
//    c += rotl5!(d) + bool3ary_150!(e, a, b) + t + k1; e = rotl30!(e);
//    b += rotl5!(c) + bool3ary_150!(d, e, a) + u + k1; d = rotl30!(d);
//    a += rotl5!(b) + bool3ary_150!(c, d, e) + v + k1; c = rotl30!(c);
//    e += rotl5!(a) + bool3ary_150!(b, c, d) + w + k1; b = rotl30!(b);
//    let w9 = sha1_schedule!(w5, w6, w7, w8); let u32x4(t, u, v, w) = w9;
//    d += rotl5!(e) + bool3ary_150!(a, b, c) + t + k1; a = rotl30!(a);
//    c += rotl5!(d) + bool3ary_150!(e, a, b) + u + k1; e = rotl30!(e);
//    b += rotl5!(c) + bool3ary_150!(d, e, a) + v + k1; d = rotl30!(d);
//    a += rotl5!(b) + bool3ary_150!(c, d, e) + w + k1; c = rotl30!(c);
//
//    let w10 = sha1_schedule!(w6, w7, w8, w9); let u32x4(t, u, v, w) = w10;
//    e += rotl5!(a) + bool3ary_232!(b, c, d) + t + k2; b = rotl30!(b);
//    d += rotl5!(e) + bool3ary_232!(a, b, c) + u + k2; a = rotl30!(a);
//    c += rotl5!(d) + bool3ary_232!(e, a, b) + v + k2; e = rotl30!(e);
//    b += rotl5!(c) + bool3ary_232!(d, e, a) + w + k2; d = rotl30!(d);
//    let w11 = sha1_schedule!(w7, w8, w9, w10); let u32x4(t, u, v, w) = w11;
//    a += rotl5!(b) + bool3ary_232!(c, d, e) + t + k2; c = rotl30!(c);
//    e += rotl5!(a) + bool3ary_232!(b, c, d) + u + k2; b = rotl30!(b);
//    d += rotl5!(e) + bool3ary_232!(a, b, c) + v + k2; a = rotl30!(a);
//    c += rotl5!(d) + bool3ary_232!(e, a, b) + w + k2; e = rotl30!(e);
//    let w12 = sha1_schedule!(w8, w9, w10, w11); let u32x4(t, u, v, w) = w12;
//    b += rotl5!(c) + bool3ary_232!(d, e, a) + t + k2; d = rotl30!(d);
//    a += rotl5!(b) + bool3ary_232!(c, d, e) + u + k2; c = rotl30!(c);
//    e += rotl5!(a) + bool3ary_232!(b, c, d) + v + k2; b = rotl30!(b);
//    d += rotl5!(e) + bool3ary_232!(a, b, c) + w + k2; a = rotl30!(a);
//    let w13 = sha1_schedule!(w9, w10, w11, w12); let u32x4(t, u, v, w) = w13;
//    c += rotl5!(d) + bool3ary_232!(e, a, b) + t + k2; e = rotl30!(e);
//    b += rotl5!(c) + bool3ary_232!(d, e, a) + u + k2; d = rotl30!(d);
//    a += rotl5!(b) + bool3ary_232!(c, d, e) + v + k2; c = rotl30!(c);
//    e += rotl5!(a) + bool3ary_232!(b, c, d) + w + k2; b = rotl30!(b);
//    let w14 = sha1_schedule!(w10, w11, w12, w13); let u32x4(t, u, v, w) = w14;
//    d += rotl5!(e) + bool3ary_232!(a, b, c) + t + k2; a = rotl30!(a);
//    c += rotl5!(d) + bool3ary_232!(e, a, b) + u + k2; e = rotl30!(e);
//    b += rotl5!(c) + bool3ary_232!(d, e, a) + v + k2; d = rotl30!(d);
//    a += rotl5!(b) + bool3ary_232!(c, d, e) + w + k2; c = rotl30!(c);
//
//    let w15 = sha1_schedule!(w11, w12, w13, w14); let u32x4(t, u, v, w) = w15;
//    e += rotl5!(a) + bool3ary_150!(b, c, d) + t + k3; b = rotl30!(b);
//    d += rotl5!(e) + bool3ary_150!(a, b, c) + u + k3; a = rotl30!(a);
//    c += rotl5!(d) + bool3ary_150!(e, a, b) + v + k3; e = rotl30!(e);
//    b += rotl5!(c) + bool3ary_150!(d, e, a) + w + k3; d = rotl30!(d);
//    let w16 = sha1_schedule!(w12, w13, w14, w15); let u32x4(t, u, v, w) = w16;
//    a += rotl5!(b) + bool3ary_150!(c, d, e) + t + k3; c = rotl30!(c);
//    e += rotl5!(a) + bool3ary_150!(b, c, d) + u + k3; b = rotl30!(b);
//    d += rotl5!(e) + bool3ary_150!(a, b, c) + v + k3; a = rotl30!(a);
//    c += rotl5!(d) + bool3ary_150!(e, a, b) + w + k3; e = rotl30!(e);
//    let w17 = sha1_schedule!(w13, w14, w15, w16); let u32x4(t, u, v, w) = w17;
//    b += rotl5!(c) + bool3ary_150!(d, e, a) + t + k3; d = rotl30!(d);
//    a += rotl5!(b) + bool3ary_150!(c, d, e) + u + k3; c = rotl30!(c);
//    e += rotl5!(a) + bool3ary_150!(b, c, d) + v + k3; b = rotl30!(b);
//    d += rotl5!(e) + bool3ary_150!(a, b, c) + w + k3; a = rotl30!(a);
//    let w18 = sha1_schedule!(w14, w15, w16, w17); let u32x4(t, u, v, w) = w18;
//    c += rotl5!(d) + bool3ary_150!(e, a, b) + t + k3; e = rotl30!(e);
//    b += rotl5!(c) + bool3ary_150!(d, e, a) + u + k3; d = rotl30!(d);
//    a += rotl5!(b) + bool3ary_150!(c, d, e) + v + k3; c = rotl30!(c);
//    e += rotl5!(a) + bool3ary_150!(b, c, d) + w + k3; b = rotl30!(b);
//    let w19 = sha1_schedule!(w15, w16, w17, w18); let u32x4(t, u, v, w) = w19;
//    d += rotl5!(e) + bool3ary_150!(a, b, c) + t + k3; a = rotl30!(a);
//    c += rotl5!(d) + bool3ary_150!(e, a, b) + u + k3; e = rotl30!(e);
//    b += rotl5!(c) + bool3ary_150!(d, e, a) + v + k3; d = rotl30!(d);
//    a += rotl5!(b) + bool3ary_150!(c, d, e) + w + k3; c = rotl30!(c);
//
//    hashw[0] += a;
//    hashw[1] += b;
//    hashw[2] += c;
//    hashw[3] += d;
//    hashw[4] += e;
//}

//#[allow(unused_variables)]
//#[stable(feature="default", since="1.0.0")]
//pub fn digest_block_safe(hashw: &mut [u32; 5], msgv: &[u32x4; 4]) {
//    use std::num::Int;
//    use stdish::num::RotateInt;
//    use self::constants::{K0, K1, K2, K3};
//    let (k0, k1, k2, k3) = (K0, K1, K2, K3);
//    let mut a = hashw[0];
//    let mut b = hashw[1];
//    let mut c = hashw[2];
//    let mut d = hashw[3];
//    let mut e = hashw[4];
//    let mut f: u32;
//
//    //sha1_dump!(a, b, c, d, e);
//
//    let w0 = msgv[0]; let u32x4(t, u, v, w) = w0;
//    sha1_round!(f, a, b, c, d, e, t + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k0 + bool3ary_202!(b, c, d));
//    let w1 = msgv[1]; let u32x4(t, u, v, w) = w1;
//    sha1_round!(f, a, b, c, d, e, t + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k0 + bool3ary_202!(b, c, d));
//    let w2 = msgv[2]; let u32x4(t, u, v, w) = w2;
//    sha1_round!(f, a, b, c, d, e, t + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k0 + bool3ary_202!(b, c, d));
//    let w3 = msgv[3]; let u32x4(t, u, v, w) = w3;
//    sha1_round!(f, a, b, c, d, e, t + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k0 + bool3ary_202!(b, c, d));
//    let w4 = sha1_schedule!(w0, w1, w2, w3); let u32x4(t, u, v, w) = w4;
//    sha1_round!(f, a, b, c, d, e, t + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k0 + bool3ary_202!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k0 + bool3ary_202!(b, c, d));
//
//    let w5 = sha1_schedule!(w1, w2, w3, w4); let u32x4(t, u, v, w) = w5;
//    sha1_round!(f, a, b, c, d, e, t + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k1 + bool3ary_150!(b, c, d));
//    let w6 = sha1_schedule!(w2, w3, w4, w5); let u32x4(t, u, v, w) = w6;
//    sha1_round!(f, a, b, c, d, e, t + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k1 + bool3ary_150!(b, c, d));
//    let w7 = sha1_schedule!(w3, w4, w5, w6); let u32x4(t, u, v, w) = w7;
//    sha1_round!(f, a, b, c, d, e, t + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k1 + bool3ary_150!(b, c, d));
//    let w8 = sha1_schedule!(w4, w5, w6, w7); let u32x4(t, u, v, w) = w8;
//    sha1_round!(f, a, b, c, d, e, t + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k1 + bool3ary_150!(b, c, d));
//    let w9 = sha1_schedule!(w5, w6, w7, w8); let u32x4(t, u, v, w) = w9;
//    sha1_round!(f, a, b, c, d, e, t + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k1 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k1 + bool3ary_150!(b, c, d));
//
//    let w10 = sha1_schedule!(w6, w7, w8, w9); let u32x4(t, u, v, w) = w10;
//    sha1_round!(f, a, b, c, d, e, t + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k2 + bool3ary_232!(b, c, d));
//    let w11 = sha1_schedule!(w7, w8, w9, w10); let u32x4(t, u, v, w) = w11;
//    sha1_round!(f, a, b, c, d, e, t + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k2 + bool3ary_232!(b, c, d));
//    let w12 = sha1_schedule!(w8, w9, w10, w11); let u32x4(t, u, v, w) = w12;
//    sha1_round!(f, a, b, c, d, e, t + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k2 + bool3ary_232!(b, c, d));
//    let w13 = sha1_schedule!(w9, w10, w11, w12); let u32x4(t, u, v, w) = w13;
//    sha1_round!(f, a, b, c, d, e, t + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k2 + bool3ary_232!(b, c, d));
//    let w14 = sha1_schedule!(w10, w11, w12, w13); let u32x4(t, u, v, w) = w14;
//    sha1_round!(f, a, b, c, d, e, t + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k2 + bool3ary_232!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k2 + bool3ary_232!(b, c, d));
//
//    let w15 = sha1_schedule!(w11, w12, w13, w14); let u32x4(t, u, v, w) = w15;
//    sha1_round!(f, a, b, c, d, e, t + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k3 + bool3ary_150!(b, c, d));
//    let w16 = sha1_schedule!(w12, w13, w14, w15); let u32x4(t, u, v, w) = w16;
//    sha1_round!(f, a, b, c, d, e, t + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k3 + bool3ary_150!(b, c, d));
//    let w17 = sha1_schedule!(w13, w14, w15, w16); let u32x4(t, u, v, w) = w17;
//    sha1_round!(f, a, b, c, d, e, t + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k3 + bool3ary_150!(b, c, d));
//    let w18 = sha1_schedule!(w14, w15, w16, w17); let u32x4(t, u, v, w) = w18;
//    sha1_round!(f, a, b, c, d, e, t + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k3 + bool3ary_150!(b, c, d));
//    let w19 = sha1_schedule!(w15, w16, w17, w18); let u32x4(t, u, v, w) = w19;
//    sha1_round!(f, a, b, c, d, e, t + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, u + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, v + k3 + bool3ary_150!(b, c, d));
//    sha1_round!(f, a, b, c, d, e, w + k3 + bool3ary_150!(b, c, d));
//
//    hashw[0] += a;
//    hashw[1] += b;
//    hashw[2] += c;
//    hashw[3] += d;
//    hashw[4] += e;
//}

//#[stable(feature="default", since="1.0.0")]
//pub fn digest_block(hashw: &mut [u32; 5], msgv: &[u32x4; 4]) {
//    //digest_block_fast(hashw, msgv);
//    //digest_block_good(hashw, msgv);
//    //arm::digest_block(hashw, msgv);
//    //x86::digest_block(hashw, msgv);
//    //x86asm::digest_block(hashw, msgv);
//}

/// Digest message block in vectors
///
/// This function
///
//#[stable(feature = "default", since = "1.0.0")]
//pub fn digest_block(hash: &mut [u32; 5], msg: &[u32x4; 4]) {
//    //hw::digest_block(hash, msg);
//}
//
///// Digest message block in bytes
/////
///// This function
//#[stable(feature = "default", since = "1.0.0")]
//pub fn digest_block_with_u32(state: &mut [u8/*; 20*/], block: &[u8/*; 64*/]) {
//    //use std::slice::bytes::copy_memory;
//    //use super::stdish::slice::transmute_memory;
//    //assert_eq!(msg.len(), 64);
//    //let mut msgb = [0u8; 64];
//    //copy_memory(&mut msgb, msg);
//    //let mut msgv = [u32x4(0, 0, 0, 0); 4];
//    //transmute_memory::<[u32x4; 4], [u8; 64]>(&mut msgv, &mut msgb);
//    //for m in msgv.iter_mut() { *m = m.to_be() }
//    //let mut hashw = &mut [0u32; 5];
//    //transmute_memory::<[u32; 5], [u8; 20]>(&mut hashw, hash);
//    //for h in hashw.iter_mut() { *h = h.to_be() }
//
//    use std::mem::transmute;
//    use std::num::Int;
//    use super::stdish::num::SwapBytesInt;
//    assert_eq!(state.len(), 20);
//    assert_eq!(block.len(), 64);
//    let (mut hw, _): (&mut [u32; 5], usize) = unsafe { transmute(state) };
//    let (mut mv, _): (&mut [u32x4; 4], usize) = unsafe { transmute(block) } ;
//    for m in mv.iter_mut() { *m = m.to_be() }
//    for h in hw.iter_mut() { *h = h.to_be() }
//    
//    // the crux of the algorithm
//	digest_block(&mut hw, &mv);
//
//    for h in hw.iter_mut() { *h = h.to_be() }
//}
//
//pub fn digest_block_bytes(hash: &mut [u8; 20], msg: &[u8]) {
//    //let hashs: &mut [u8] = hash;
//    //sw_openssl::digest_block_bytes(hashs, msg);
//
//    digest_block_bytes_sw(hash, msg);
//}

/// Pad message, 
pub fn padding(buf: &mut Vec<u8>, msglen: u64) {
    let buflen = buf.len();

    // `newlen` is always a multiple of the block size (64)
    let newlen: usize = buflen + 9 + ((55 - buflen) % 64);
    
    // standard padding
    buf.push(0x80u8);
    buf.resize(newlen - 8, 0u8);
    buf.write_be_u64(8*msglen).unwrap();
}

/// Reset hash to initial value
#[unstable(feature = "default", reason = "1.0.0")]
pub fn reset(hash: &mut [u8/*; 20*/]) {
    use std::slice::bytes::copy_memory;
    copy_memory(hash, constants::IV);
}

#[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
pub fn digest_block_u32_u32x4(state: &mut [u32/*; 5*/], block: &[u32x4/*; 4*/]) {
    use std::mem::transmute;
    let (state2, _): (&mut [u32; 5], usize) = unsafe { transmute(state) };
    let (block2, _): (&mut [u32x4; 4], usize) = unsafe { transmute(block) };
    digest_block_fast(state2, block2);
}

#[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
pub fn digest_block_u32(state: &mut [u32/*; 5*/], block: &[u32/*; 16*/]) {
    use std::mem::transmute;
    let block2: &mut [u32x4] = unsafe { transmute(block) };
    digest_block_u32_u32x4(state, block2);
}

/// Digest block where the state is 5 words and the block is 64 bytes
///
/// Compare with other APIs:
///   `void _nettle_sha1_compress(uint32_t *state, const uint8_t *input);`
#[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
pub fn digest_block_u32_u8(state: &mut [u32/*; 5*/], block: &[u8/*; 64*/]) {
    use std::mem::transmute;
    let (block2, _): (&mut [u32; 16], usize) = unsafe { transmute(block) };
    digest_block_u32(state, block2);
}

#[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
pub fn digest_block(state: &mut [u8/*; 20*/], block: &[u8/*; 64*/]) {
    use std::mem::transmute;
    use std::num::Int;
    assert_eq!(state.len(), 20);
    assert_eq!(block.len(), 64);
    
    let (mut hw, _): (&mut [u32; 5], usize) = unsafe { transmute(state) };
    let (mut mv, _): (&mut [u32; 16], usize) = unsafe { transmute(block) };
    
    for m in mv.iter_mut() { *m = m.to_be() }
    for h in hw.iter_mut() { *h = h.to_be() }
    
    // the crux of the algorithm
	digest_block_u32(hw, mv);

    for h in hw.iter_mut() { *h = h.to_be() }
}

#[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
pub fn digest(state: &mut [u8/*; 20*/], data: &[u8]) {
    let mut buf: Vec<u8> = Vec::with_capacity(data.len() + 128);
    reset(state);
    write_last(state, &mut buf, data, data.len());
}

#[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
pub fn hex_digest(message: &str) -> String {
    use serialize::hex::ToHex;
    let mut state = [0u8; 20];
    digest(&mut state[0..20], message.as_bytes());
    state.to_hex()
}

//pub fn write_block(state: &mut [u8/*; 20*/], block: &[u8]) -> usize {
//    //if buf.len() < 64us {
//    //    panic!("Digest write buf must be exactly 64 bytes");
//    //}
//    //if self.finished {
//    //    panic!("Digest write requires an unfinished state");
//    //}
//    // TODO: partial writes
//    //sw::digest_block(state, buf);
//}
//
//pub fn write(state: &mut [u8/*; 20*/], buf: &mut Vec<u8>, data: &[u8]) -> usize {
//    unimplemented!();
//}

pub fn write_last(state: &mut [u8/*; 20*/], buf: &mut Vec<u8>, data: &[u8], length: usize) {
    use std::slice::bytes::copy_memory;
    let buflen = buf.len();
    buf.resize(buflen + data.len(), 0u8);
    copy_memory(&mut buf[buflen..(buflen + data.len())], data);
    padding(buf, length as u64);
    let buflen = buf.len();

    // write remaining blocks
    for block in (&buf[0..(buflen - 64)]).chunks(64) {
        digest_block(state, block);
    }
    digest_block(state, &buf[(buflen - 64)..buflen]);
}
                       
//use self::sw::write_block;
//// TODO fix .to_vec()
////padbuf.clone_from_slice()
//
//// calculate length before pad
//self.length += buf.len();
//let mut padbuf: Vec<u8> = buf.to_vec();
//sw::padding(&mut padbuf, self.length as u64);
//let padbuflen = padbuf.len();
//
//// write remaining blocks
//for block in (&mut self.state, &padbuf[0..(padbuflen - 64)]).chunks(64) {
//    write_block(block);
//}
//write_block(&mut self.state, &padbuf[(padbuflen - 64)..padbuflen]);



#[cfg(test)]
pub mod tests {
    use test::Bencher;
    
    //pub fn super::digest_block(state: &mut [u8], msg: &[u8]) {
    //    //use super::sw_openssl::digest;
    //    //digest(hash, msg);
    //    use std::mem::transmute;
    //    let (mut state2, _): (&mut [u8; 20], usize) = unsafe { transmute(state) };
    //    super::sw::digest_block(state2, msg);
    //}
    //
    //pub fn super::digest(state: &mut [u8], msg: &[u8]) {
    //    //use super::sw_openssl::digest;
    //    //digest(hash, msg);
    //    use super::Sha1;
    //    
    //    super::sw::digest(state, msg);
    //}
    //
    ///// Digest whole message, return hex string
    //#[unstable(feature = "cryptoil_internal", reason = "will be trait method")]
    //pub fn super::hex_digest(msg: &str) -> String {
    //    let mut hash = [0u8; 20];
    //    super::digest(&mut hash[0..20], msg.as_bytes());
    //    hash.to_hex()
    //}
    
    #[test]
    fn test_empty_string() {
        assert_eq!("da39a3ee5e6b4b0d3255bfef95601890afd80709", super::hex_digest("").as_slice());
    }
    
    #[test]
    fn test_hello_world() {
        assert_eq!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed", super::hex_digest("hello world").as_slice());
        assert_eq!("430ce34d020724ed75a196dfc2ad67c77772d169", super::hex_digest("hello world!").as_slice());
        assert_eq!("22c219648f00c61e5b3b1bd81ffa8e7767e2e3c5", super::hex_digest("hello World").as_slice());
        assert_eq!("788245b4dad73c1e5a630c126c484c7a2464f280", super::hex_digest("hello World!").as_slice());
        assert_eq!("7b502c3a1f48c8609ae212cdfb639dee39673f5e", super::hex_digest("Hello world").as_slice());
        assert_eq!("d3486ae9136e7856bc42212385ea797094475802", super::hex_digest("Hello world!").as_slice());
        assert_eq!("0a4d55a8d778e5022fab701977c5d840bbc486d0", super::hex_digest("Hello World").as_slice());
        assert_eq!("2ef7bde608ce5404e97d5f042f95f89f1c232871", super::hex_digest("Hello World!").as_slice());
        assert_eq!("b7e23ec29af22b0b4e41da31e868d57226121c84", super::hex_digest("hello, world").as_slice());
        assert_eq!("1f09d30c707d53f3d16c530dd73d70a6ce7596a9", super::hex_digest("hello, world!").as_slice());
        assert_eq!("ca3c58516ddef44b25693df5a915206e1bd094da", super::hex_digest("hello, World").as_slice());
        assert_eq!("dd0588c172986c32636ffdd8cc690de7b41bf253", super::hex_digest("hello, World!").as_slice());
        assert_eq!("e02aa1b106d5c7c6a98def2b13005d5b84fd8dc8", super::hex_digest("Hello, world").as_slice());
        assert_eq!("943a702d06f34599aee1f8da8ef9f7296031d699", super::hex_digest("Hello, world!").as_slice());
        assert_eq!("907d14fb3af2b0d4f18c2d46abe8aedce17367bd", super::hex_digest("Hello, World").as_slice());
        assert_eq!("0a0a9f2a6772942557ab5355d76af442f8f65e01", super::hex_digest("Hello, World!").as_slice());
    }
    
    #[test]
    fn test_multi_block() {
        let s = "GNU LESSER GENERAL PUBLIC LICENSE Version 3, 29 June 2007 Copyright (C) 2007 Free Software Foundation, Inc. <http://fsf.org/>";
        assert_eq!("a31e8cb8a139d146a0070fa13795d6766acaccd4", super::hex_digest(s).as_slice());
    }
    
    
    #[bench]
    fn bench_hello_world(b: & mut Bencher) {
        let s = "hello world";
    
        b.iter(|| super::hex_digest(s));
    }
    
    #[bench]
    fn bench_multi_block(b: & mut Bencher) {
        let s = "GNU LESSER GENERAL PUBLIC LICENSE Version 3, 29 June 2007 Copyright (C) 2007 Free Software Foundation, Inc. <http://fsf.org/>";
    
        b.iter(|| super::hex_digest(s));
    }
    
    
    #[bench]
    pub fn bench_padding(bh: & mut Bencher) {
        use super::padding;
        let mut bytes = vec![1u8; 25];
        bh.iter( || {
                padding(&mut bytes, 25u64);
            });
        bh.bytes = 64 as u64;
    }
    
    #[bench]
    pub fn block_64(bh: & mut Bencher) {
        use super::digest_block;
        let mut result = [0u8; 20];
        let bytes = [1u8; 64];
        bh.iter( || {
                super::digest_block(&mut result, &bytes[]);
            });
        bh.bytes = bytes.len() as u64;
    }
    
    #[bench]
    pub fn bench_message_10(bh: & mut Bencher) {
        let mut result = [0u8; 20];
        let bytes = [1u8; 10];
        bh.iter( || {
                super::digest(&mut result, &bytes[]);
            });
        bh.bytes = bytes.len() as u64;
    }
    #[bench]
    pub fn bench_message_1k(bh: & mut Bencher) {
        let mut result = [0u8; 20];
        let bytes = [1u8; 1024];
        bh.iter( || {
                super::digest(&mut result, &bytes[]);
            });
        bh.bytes = bytes.len() as u64;
    }
    #[bench]
    pub fn bench_message_64k(bh: & mut Bencher) {
        let bytes = [1u8; 65536];
        let mut result = [0u8; 20];
        bh.iter( || {
                super::digest(&mut result, &bytes);
            });
        bh.bytes = bytes.len() as u64;
    }
    
}
