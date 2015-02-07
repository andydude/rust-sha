use std::simd::u32x4;

#[cfg(target_arch = "arm")]
#[stable(feature="cryptoil_internals", since="1.0.0")]
pub mod arm {
    use std::simd::u32x4;

    #[stable(feature="cryptoil_internals", since="1.0.0")]
    pub mod intrinsics {
        use std::simd::u32x4;

        extern {

            #[link_name = "llvm.arm.neon.sha1h"]
            #[stable(feature="cryptoil_internals", since="1.0.0")]
            pub fn sha1h(a: u32) -> u32;

            #[link_name = "llvm.arm.neon.sha1c"]
            #[stable(feature="cryptoil_internals", since="1.0.0")]
            pub fn sha1c(a: u32x4, b: u32, c: u32x4) -> u32x4;

            #[link_name = "llvm.arm.neon.sha1m"]
            #[stable(feature="cryptoil_internals", since="1.0.0")]
            pub fn sha1m(a: u32x4, b: u32, c: u32x4) -> u32x4;

            #[link_name = "llvm.arm.neon.sha1p"]
            #[stable(feature="cryptoil_internals", since="1.0.0")]
            pub fn sha1p(a: u32x4, b: u32, c: u32x4) -> u32x4;

            #[link_name = "llvm.arm.neon.sha1su0"]
            #[stable(feature="cryptoil_internals", since="1.0.0")]
            pub fn sha1su0(a: u32x4, b: u32x4, c: u32x4) -> u32x4;

            #[link_name = "llvm.arm.neon.sha1su1"]
            #[stable(feature="cryptoil_internals", since="1.0.0")]
            pub fn sha1su1(a: u32x4, b: u32x4) -> u32x4;
        }

        #[stable(feature="cryptoil_internals", since="1.0.0")]
        pub fn has_sha1() -> bool {
            false
        }
    }

    /// Digest message block (arm-specific)
    #[stable(feature="cryptoil_internals", since="1.0.0")]
    pub fn digest_block(hashw: &mut [u32; 5], msgv: &[u32x4; 4]) {
        use super::super::sw::constants::{SHA1_K0, SHA1_K1, SHA1_K2, SHA1_K3};
        use super::super::sw::intrinsics::{add_1st, get_1st};
        use self::intrinsics::{sha1su0, sha1su1, sha1h, sha1c, sha1p, sha1m};

        const SHA1_K0V: u32x4 = u32x4(SHA1_K0, SHA1_K0, SHA1_K0, SHA1_K0);
        const SHA1_K1V: u32x4 = u32x4(SHA1_K1, SHA1_K1, SHA1_K1, SHA1_K1);
        const SHA1_K2V: u32x4 = u32x4(SHA1_K2, SHA1_K2, SHA1_K2, SHA1_K2);
        const SHA1_K3V: u32x4 = u32x4(SHA1_K3, SHA1_K3, SHA1_K3, SHA1_K3);

        unsafe {

            let h0 = u32x4(hashw[0], hashw[1], hashw[2], hashw[3]);
            let w0 = msgv[0];
            let h1 = sha1c(h0, hashw[4], w0 + SHA1_K0V);
            let w1 = msgv[1];
            let h2 = sha1c(h1, sha1h(get_1st(h0)), w1 + SHA1_K0V);
            let w2 = msgv[2];
            let h3 = sha1c(h2, sha1h(get_1st(h1)), w2 + SHA1_K0V);
            let w3 = msgv[3];
            let h4 = sha1c(h3, sha1h(get_1st(h2)), w3 + SHA1_K0V);
            let w4 = sha1su1(sha1su0(w0, w1, w2), w3);
            let h5 = sha1c(h4, sha1h(get_1st(h3)), w4 + SHA1_K0V);
            let w5 = sha1su1(sha1su0(w1, w2, w3), w4);
            let h6 = sha1p(h5, sha1h(get_1st(h4)), w5 + SHA1_K1V);
            let w6 = sha1su1(sha1su0(w2, w3, w4), w5);
            let h7 = sha1p(h6, sha1h(get_1st(h5)), w6 + SHA1_K1V);
            let w7 = sha1su1(sha1su0(w3, w4, w5), w6);
            let h8 = sha1p(h7, sha1h(get_1st(h6)), w7 + SHA1_K1V);
            let w8 = sha1su1(sha1su0(w4, w5, w6), w7);
            let h9 = sha1p(h8, sha1h(get_1st(h7)), w8 + SHA1_K1V);
            let w9 = sha1su1(sha1su0(w5, w6, w7), w8);
            let h10 = sha1p(h9, sha1h(get_1st(h8)), w9 + SHA1_K1V);
            let w10 = sha1su1(sha1su0(w6, w7, w8), w9);
            let h11 = sha1m(h10, sha1h(get_1st(h9)), w10 + SHA1_K2V);
            let w11 = sha1su1(sha1su0(w7, w8, w9), w10);
            let h12 = sha1m(h11, sha1h(get_1st(h10)), w11 + SHA1_K2V);
            let w12 = sha1su1(sha1su0(w8, w9, w10), w11);
            let h13 = sha1m(h12, sha1h(get_1st(h11)), w12 + SHA1_K2V);
            let w13 = sha1su1(sha1su0(w9, w10, w11), w12);
            let h14 = sha1m(h13, sha1h(get_1st(h12)), w13 + SHA1_K2V);
            let w14 = sha1su1(sha1su0(w10, w11, w12), w13);
            let h15 = sha1m(h14, sha1h(get_1st(h13)), w14 + SHA1_K2V);
            let w15 = sha1su1(sha1su0(w11, w12, w13), w14);
            let h16 = sha1p(h15, sha1h(get_1st(h14)), w15 + SHA1_K3V);
            let w16 = sha1su1(sha1su0(w12, w13, w14), w15);
            let h17 = sha1p(h16, sha1h(get_1st(h15)), w16 + SHA1_K3V);
            let w17 = sha1su1(sha1su0(w13, w14, w15), w16);
            let h18 = sha1p(h17, sha1h(get_1st(h16)), w17 + SHA1_K3V);
            let w18 = sha1su1(sha1su0(w14, w15, w16), w17);
            let h19 = sha1p(h18, sha1h(get_1st(h17)), w18 + SHA1_K3V);
            let w19 = sha1su1(sha1su0(w15, w16, w17), w18);
            let h20 = sha1p(h19, sha1h(get_1st(h18)), w19 + SHA1_K3V);
            let e20 = get_1st(h19).rotate_left(30);

            let u32x4(a, b, c, d) = h20;

            hashw[0] += a;
            hashw[1] += b;
            hashw[2] += c;
            hashw[3] += d;
            hashw[4] += e20;
        }
    }
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature="cryptoil_shani"))]
#[stable(feature="cryptoil_internals", since="1.0.0")]
pub mod x86 {
    use std::simd::u32x4;

    #[stable(feature="cryptoil_internals", since="1.0.0")]
    pub mod intrinsics {
        use std::simd::u32x4;

        extern {

            /// intrinsic %llvm.x86.sha1rnds4
            #[link_name = "llvm.x86.sha1rnds4"]
            #[stable(feature="cryptoil_internals", since="1.0.0")]
            pub fn sha1rnds4(a: u32x4, b: u32x4, c: i8) -> u32x4;

            /// intrinsic %llvm.x86.sha1nexte
            #[link_name = "llvm.x86.sha1nexte"]
            #[stable(feature="cryptoil_internals", since="1.0.0")]
            pub fn sha1nexte(a: u32x4, b: u32x4) -> u32x4;

            /// intrinsic %llvm.x86.sha1msg1
            #[link_name = "llvm.x86.sha1msg1"]
            #[stable(feature="cryptoil_internals", since="1.0.0")]
            pub fn sha1msg1(a: u32x4, b: u32x4) -> u32x4;

            /// intrinsic %llvm.x86.sha1msg2
            #[link_name = "llvm.x86.sha1msg2"]
            #[stable(feature="cryptoil_internals", since="1.0.0")]
            pub fn sha1msg2(a: u32x4, b: u32x4) -> u32x4;
        }

        /// Checks CPUID.7.0.EBX[29]
        #[stable(feature="cryptoil_internals", since="1.0.0")]
        pub fn has_sha1() -> bool {
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
    }

    /// Digest message block (x86-specific)
    #[stable(feature="cryptoil_internals", since="1.0.0")]
    pub fn digest_block(hashw: &mut [u32; 5], msgv: &[u32x4; 4]) {
        use super::super::sw::intrinsics::{add_1st, get_1st};
        use self::intrinsics::{sha1msg1, sha1msg2, sha1nexte, sha1rnds4};
        use std::num::Int;

        unsafe {

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
    }
} // mod cryptoil::sha1::hw::x86


/// Digest message block in vectors (arm-specific)
#[cfg(target_arch = "arm")]
#[unstable(feature = "cryptoil_internals", reason = "std::simd is unstable")]
pub fn digest_block(hash: &mut [u32; 5], msg: &[u32x4; 4]) {
    if hw::arm::intrinsics::has_sha1() {
        hw::arm::digest_block(hash, msg)
    } else {
        unreachable!();
    }
}


/// Digest message block in vectors (x86-specific)
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature="cryptoil_shani"))]
#[unstable(feature = "cryptoil_internals", reason = "std::simd is unstable")]
pub fn digest_block(hash: &mut [u32; 5], msg: &[u32x4; 4]) {
    if x86::intrinsics::has_sha1() {
        x86::digest_block(hash, msg)
    } else {
        unreachable!();
    }
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(feature="cryptoil_shani")))]
#[unstable(feature = "cryptoil_internals", reason = "std::simd is unstable")]
pub fn digest_block(_: &mut [u32; 5], _: &[u32x4; 4]) {
    unreachable!();
}

#[cfg(not(any(target_arch = "arm", target_arch = "x86", target_arch = "x86_64")))]
#[stable(feature = "cryptoil_internals")]
pub fn digest_block(_: &mut [u32; 5], _: &[u32x4; 4]) {
    unreachable!();
}
