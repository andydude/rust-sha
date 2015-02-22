use std::default::Default;
use serialize::hex::ToHex;
use std::hash::{Hash, Hasher};
//use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::io::{BufWriter, Read, Write};
use std::simd::u32x4;
use utils::{PadWriter, Reset, State, Block, ToArray, FromArray};
//use endian::WriteEndian;

const BLOCK_LEN: usize = 16; // words, i.e. [u32; 16]
const STATE_LEN: usize = 8;  // words, i.e. [u32; 8]

pub const H: [u32; STATE_LEN] = [
    0x6a09e667u32, // floor(mod(sqrt(2), 1)*2**32)
    0xbb67ae85u32, // floor(mod(sqrt(3), 1)*2**32)
    0x3c6ef372u32, // floor(mod(sqrt(5), 1)*2**32)
    0xa54ff53au32, // floor(mod(sqrt(7), 1)*2**32)
    0x510e527fu32, // floor(mod(sqrt(11), 1)*2**32)
    0x9b05688cu32, // floor(mod(sqrt(13), 1)*2**32)
    0x1f83d9abu32, // floor(mod(sqrt(17), 1)*2**32)
    0x5be0cd19u32, // floor(mod(sqrt(19), 1)*2**32)
];

pub const K: [u32; 64] = [
    0x428a2f98u32, // floor(mod(cbrt(2), 1)*2**32)
    0x71374491u32, // floor(mod(cbrt(3), 1)*2**32)
    0xb5c0fbcfu32, // floor(mod(cbrt(5), 1)*2**32)
    0xe9b5dba5u32, // floor(mod(cbrt(7), 1)*2**32)
    0x3956c25bu32, // floor(mod(cbrt(11), 1)*2**32)
    0x59f111f1u32, // floor(mod(cbrt(13), 1)*2**32)
    0x923f82a4u32, // floor(mod(cbrt(17), 1)*2**32)
    0xab1c5ed5u32, // floor(mod(cbrt(19), 1)*2**32)
    0xd807aa98u32, // floor(mod(cbrt(23), 1)*2**32)
    0x12835b01u32, // floor(mod(cbrt(29), 1)*2**32)
    0x243185beu32, // floor(mod(cbrt(31), 1)*2**32)
    0x550c7dc3u32, // floor(mod(cbrt(37), 1)*2**32)
    0x72be5d74u32, // floor(mod(cbrt(41), 1)*2**32)
    0x80deb1feu32, // floor(mod(cbrt(43), 1)*2**32)
    0x9bdc06a7u32, // floor(mod(cbrt(47), 1)*2**32)
    0xc19bf174u32, // floor(mod(cbrt(53), 1)*2**32)
    0xe49b69c1u32, // floor(mod(cbrt(59), 1)*2**32)
    0xefbe4786u32, // floor(mod(cbrt(61), 1)*2**32)
    0x0fc19dc6u32, // floor(mod(cbrt(67), 1)*2**32)
    0x240ca1ccu32, // floor(mod(cbrt(71), 1)*2**32)
    0x2de92c6fu32, // floor(mod(cbrt(73), 1)*2**32)
    0x4a7484aau32, // floor(mod(cbrt(79), 1)*2**32)
    0x5cb0a9dcu32, // floor(mod(cbrt(83), 1)*2**32)
    0x76f988dau32, // floor(mod(cbrt(89), 1)*2**32)
    0x983e5152u32, // floor(mod(cbrt(97), 1)*2**32)
    0xa831c66du32, // floor(mod(cbrt(101), 1)*2**32)
    0xb00327c8u32, // floor(mod(cbrt(103), 1)*2**32)
    0xbf597fc7u32, // floor(mod(cbrt(107), 1)*2**32)
    0xc6e00bf3u32, // floor(mod(cbrt(109), 1)*2**32)
    0xd5a79147u32, // floor(mod(cbrt(113), 1)*2**32)
    0x06ca6351u32, // floor(mod(cbrt(127), 1)*2**32)
    0x14292967u32, // floor(mod(cbrt(131), 1)*2**32)
    0x27b70a85u32, // floor(mod(cbrt(137), 1)*2**32)
    0x2e1b2138u32, // floor(mod(cbrt(139), 1)*2**32)
    0x4d2c6dfcu32, // floor(mod(cbrt(149), 1)*2**32)
    0x53380d13u32, // floor(mod(cbrt(151), 1)*2**32)
    0x650a7354u32, // floor(mod(cbrt(157), 1)*2**32)
    0x766a0abbu32, // floor(mod(cbrt(163), 1)*2**32)
    0x81c2c92eu32, // floor(mod(cbrt(167), 1)*2**32)
    0x92722c85u32, // floor(mod(cbrt(173), 1)*2**32)
    0xa2bfe8a1u32, // floor(mod(cbrt(179), 1)*2**32)
    0xa81a664bu32, // floor(mod(cbrt(181), 1)*2**32)
    0xc24b8b70u32, // floor(mod(cbrt(191), 1)*2**32)
    0xc76c51a3u32, // floor(mod(cbrt(193), 1)*2**32)
    0xd192e819u32, // floor(mod(cbrt(197), 1)*2**32)
    0xd6990624u32, // floor(mod(cbrt(199), 1)*2**32)
    0xf40e3585u32, // floor(mod(cbrt(211), 1)*2**32)
    0x106aa070u32, // floor(mod(cbrt(223), 1)*2**32)
    0x19a4c116u32, // floor(mod(cbrt(227), 1)*2**32)
    0x1e376c08u32, // floor(mod(cbrt(229), 1)*2**32)
    0x2748774cu32, // floor(mod(cbrt(233), 1)*2**32)
    0x34b0bcb5u32, // floor(mod(cbrt(239), 1)*2**32)
    0x391c0cb3u32, // floor(mod(cbrt(241), 1)*2**32)
    0x4ed8aa4au32, // floor(mod(cbrt(251), 1)*2**32)
    0x5b9cca4fu32, // floor(mod(cbrt(257), 1)*2**32)
    0x682e6ff3u32, // floor(mod(cbrt(263), 1)*2**32)
    0x748f82eeu32, // floor(mod(cbrt(269), 1)*2**32)
    0x78a5636fu32, // floor(mod(cbrt(271), 1)*2**32)
    0x84c87814u32, // floor(mod(cbrt(277), 1)*2**32)
    0x8cc70208u32, // floor(mod(cbrt(281), 1)*2**32)
    0x90befffau32, // floor(mod(cbrt(283), 1)*2**32)
    0xa4506cebu32, // floor(mod(cbrt(293), 1)*2**32)
    0xbef9a3f7u32, // floor(mod(cbrt(307), 1)*2**32)
    0xc67178f2u32, // floor(mod(cbrt(311), 1)*2**32)
];

pub const K_X4: [u32x4; 16] = [
    u32x4(K[3], K[2], K[1], K[0]),
    u32x4(K[7], K[6], K[5], K[4]),
    u32x4(K[11], K[10], K[9], K[8]),
    u32x4(K[15], K[14], K[13], K[12]),
    u32x4(K[19], K[18], K[17], K[16]),
    u32x4(K[23], K[22], K[21], K[20]),
    u32x4(K[27], K[26], K[25], K[24]),
    u32x4(K[31], K[30], K[29], K[28]),
    u32x4(K[35], K[34], K[33], K[32]),
    u32x4(K[39], K[38], K[37], K[36]),
    u32x4(K[43], K[42], K[41], K[40]),
    u32x4(K[47], K[46], K[45], K[44]),
    u32x4(K[51], K[50], K[49], K[48]),
    u32x4(K[55], K[54], K[53], K[52]),
    u32x4(K[59], K[58], K[57], K[56]),
    u32x4(K[63], K[62], K[61], K[60]),
];

macro_rules! sigma0 {
    ($a:expr) => (($a.rotate_right(7) ^ $a.rotate_right(18) ^ ($a >> 3)))
}

macro_rules! sigma1 {
    ($a:expr) => (($a.rotate_right(17) ^ $a.rotate_right(19) ^ ($a >> 10)))
}

macro_rules! big_sigma0 {
    ($a:expr) => (($a.rotate_right(2) ^ $a.rotate_right(13) ^ $a.rotate_right(22)))
}

macro_rules! big_sigma1 {
    ($a:expr) => (($a.rotate_right(6) ^ $a.rotate_right(11) ^ $a.rotate_right(25)))
}

// /// Choose, MD5F, SHA1C
//macro_rules! bool3ary_202 {
//    ($a:expr, $b:expr, $c:expr) => (($c ^ ($a & ($b ^ $c))))
//}
//
// /// Majority, SHA1M
//macro_rules! bool3ary_232 {
//    ($a:expr, $b:expr, $c:expr) => (($a & $b) ^ ($a & $c) ^ ($b & $c))
//}

//mod good {
//    //use std::num::Int;
//
//    pub fn digest_block(state: &mut [u32; 8], block: &[u32; 16]) {
//        //use super::K;
//        //
//        //let mut w: Vec<u32> = Vec::with_capacity(64);
//        //unsafe { w.set_len(64); }
//        //
//        //// FIPS-180-4 SS 6.1.2.1 prepare message schedule
//        //for t in range(0us, 16us) {
//        //    w[t] = block[t];
//        //    //{
//        //    //    println!("w[{:02}] = {:08x}", t, w[t]);
//        //    //}
//        //}
//        //for t in range(16us, 64us) {
//        //    w[t] = sigma1!(w[t-2]) + w[t-7] + sigma0!(w[t-15]) + w[t-16];
//        //    //{
//        //    //    println!("w[{:02}] = {:08x}", t, w[t]);
//        //    //}
//        //}
//        //
//        //// FIPS-180-4 SS 6.1.2.2 initialize working variables
//        //let mut a: u32 = state[0];
//        //let mut b: u32 = state[1];
//        //let mut c: u32 = state[2];
//        //let mut d: u32 = state[3];
//        //let mut e: u32 = state[4];
//        //let mut f: u32 = state[5];
//        //let mut g: u32 = state[6];
//        //let mut h: u32 = state[7];
//        //let mut temp1: u32;
//        //let mut temp2: u32;
//        //
//        //// FIPS-180-4 SS 4.1.1 functions
//        //// FIPS-180-4 SS 4.2.2 constants
//        //// FIPS-180-4 SS 6.1.2.3
//        //for t in range(0us, 64us) {
//        //    //{
//        //    //    println!("abef[{:02}] = ({:08x} {:08x} {:08x} {:08x})", t, a, b, e, f);
//        //    //    println!("cdgh[{:02}] = ({:08x} {:08x} {:08x} {:08x})", t, c, d, g, h);
//        //    //}
//        //    temp1 = h + big_sigma1!(e) + bool3ary_202!(e, f, g) + K[t] + w[t];
//        //    temp2 = big_sigma0!(a) + bool3ary_232!(a, b, c);
//        //    h = g; g = f; f = e; e = d + temp1; d = c; c = b; b = a;
//        //    a = temp1 + temp2;
//        //}
//        //
//        //// FIPS-180-4 SS 6.1.2.4
//        //state[0] += a;
//        //state[1] += b;
//        //state[2] += c;
//        //state[3] += d;
//        //state[4] += e;
//        //state[5] += f;
//        //state[6] += g;
//        //state[7] += h;
//    }
//}

//mod slow {
//    use std::num::Int;
//    use super::{BLOCK_LEN, STATE_LEN};
//
//    pub fn digest_block(state: &mut [u32; STATE_LEN], block: &[u32; BLOCK_LEN]) {
//        use super::K;
//
//        let mut w: Vec<u32> = vec![0u32; 64];
//
//        // FIPS-180-4 SS 6.1.2.1 prepare message schedule
//        for t in 0..16 {
//            w[t] = block[t];
//            //{
//            //    println!("w[{:02}] = {:08x}", t, w[t]);
//            //}
//        }
//        for t in 16..64 {
//            w[t] = sigma1!(w[t-2]) + w[t-7] + sigma0!(w[t-15]) + w[t-16];
//            //{
//            //    println!("w[{:02}] = {:08x}", t, w[t]);
//            //}
//        }
//
//        // FIPS-180-4 SS 6.1.2.2 initialize working variables
//        let mut a: u32 = state[0];
//        let mut b: u32 = state[1];
//        let mut c: u32 = state[2];
//        let mut d: u32 = state[3];
//        let mut e: u32 = state[4];
//        let mut f: u32 = state[5];
//        let mut g: u32 = state[6];
//        let mut h: u32 = state[7];
//        let mut temp1: u32;
//        let mut temp2: u32;
//
//        // FIPS-180-4 SS 4.1.1 functions
//        // FIPS-180-4 SS 4.2.2 constants
//        // FIPS-180-4 SS 6.1.2.3
//        for t in 0..64 {
//            //{
//            //    println!("abef[{:02}] = ({:08x} {:08x} {:08x} {:08x})", t, a, b, e, f);
//            //    println!("cdgh[{:02}] = ({:08x} {:08x} {:08x} {:08x})", t, c, d, g, h);
//            //}
//            temp1 = h + big_sigma1!(e) + bool3ary_202!(e, f, g) + K[t] + w[t];
//            temp2 = big_sigma0!(a) + bool3ary_232!(a, b, c);
//            h = g; g = f; f = e; e = d + temp1; d = c; c = b; b = a;
//            a = temp1 + temp2;
//        }
//
//        // FIPS-180-4 SS 6.1.2.4
//        state[0] += a;
//        state[1] += b;
//        state[2] += c;
//        state[3] += d;
//        state[4] += e;
//        state[5] += f;
//        state[6] += g;
//        state[7] += h;
//    }
//}
//
//mod fast {
//    use std::simd::u32x4;
//    use std::num::Int;
//    use super::{BLOCK_LEN, STATE_LEN};
//
//    pub fn digest_block(state: &mut [u32; STATE_LEN], block: &[u32; BLOCK_LEN]) {
//        use super::K_X4;
//        let k = &K_X4;
//
//        let mut a = state[0];
//        let mut b = state[1];
//        let mut c = state[2];
//        let mut d = state[3];
//        let mut e = state[4];
//        let mut f = state[5];
//        let mut g = state[6];
//        let mut h = state[7];
//
//        macro_rules! schedule {
//            ($v0:expr, $v1:expr, $v2:expr, $v3:expr) => {
//                {
//                    let u32x4(w3, w2, w1, w0) = $v0;
//                    let u32x4(_, _, _, w4) = $v1;
//                    let u32x4(w11, w10, w9, _) = $v2;
//                    let u32x4(w15, w14, _, w12) = $v3;
//
//                    let w16 = sigma1!(w14) + w9  + sigma0!(w1) + w0;
//                    let w17 = sigma1!(w15) + w10 + sigma0!(w2) + w1;
//                    let w18 = sigma1!(w16) + w11 + sigma0!(w3) + w2;
//                    let w19 = sigma1!(w17) + w12 + sigma0!(w4) + w3;
//
//                    u32x4(w19, w18, w17, w16)
//                }
//            }
//        }
//
//        let v0 = u32x4(block[3], block[2], block[1], block[0]); let u32x4(w, v, u, t) = k[0] + v0;
//        h += big_sigma1!(e) + bool3ary_202!(e, f, g) + t; d += h;
//        h += big_sigma0!(a) + bool3ary_232!(a, b, c);
//        g += big_sigma1!(d) + bool3ary_202!(d, e, f) + u; c += g;
//        g += big_sigma0!(h) + bool3ary_232!(h, a, b);
//        f += big_sigma1!(c) + bool3ary_202!(c, d, e) + v; b += f;
//        f += big_sigma0!(g) + bool3ary_232!(g, h, a);
//        e += big_sigma1!(b) + bool3ary_202!(b, c, d) + w; a += e;
//        e += big_sigma0!(f) + bool3ary_232!(f, g, h);
//        let v1 = u32x4(block[7], block[6], block[5], block[4]); let u32x4(w, v, u, t) = k[1] + v1;
//        d += big_sigma1!(a) + bool3ary_202!(a, b, c) + t; h += d;
//        d += big_sigma0!(e) + bool3ary_232!(e, f, g);
//        c += big_sigma1!(h) + bool3ary_202!(h, a, b) + u; g += c;
//        c += big_sigma0!(d) + bool3ary_232!(d, e, f);
//        b += big_sigma1!(g) + bool3ary_202!(g, h, a) + v; f += b;
//        b += big_sigma0!(c) + bool3ary_232!(c, d, e);
//        a += big_sigma1!(f) + bool3ary_202!(f, g, h) + w; e += a;
//        a += big_sigma0!(b) + bool3ary_232!(b, c, d);
//
//        let v2 = u32x4(block[11], block[10], block[9], block[8]); let u32x4(w, v, u, t) = k[2] + v2;
//        h += big_sigma1!(e) + bool3ary_202!(e, f, g) + t; d += h;
//        h += big_sigma0!(a) + bool3ary_232!(a, b, c);
//        g += big_sigma1!(d) + bool3ary_202!(d, e, f) + u; c += g;
//        g += big_sigma0!(h) + bool3ary_232!(h, a, b);
//        f += big_sigma1!(c) + bool3ary_202!(c, d, e) + v; b += f;
//        f += big_sigma0!(g) + bool3ary_232!(g, h, a);
//        e += big_sigma1!(b) + bool3ary_202!(b, c, d) + w; a += e;
//        e += big_sigma0!(f) + bool3ary_232!(f, g, h);
//        let v3 = u32x4(block[15], block[14], block[13], block[12]); let u32x4(w, v, u, t) = k[3] + v3;
//        d += big_sigma1!(a) + bool3ary_202!(a, b, c) + t; h += d;
//        d += big_sigma0!(e) + bool3ary_232!(e, f, g);
//        c += big_sigma1!(h) + bool3ary_202!(h, a, b) + u; g += c;
//        c += big_sigma0!(d) + bool3ary_232!(d, e, f);
//        b += big_sigma1!(g) + bool3ary_202!(g, h, a) + v; f += b;
//        b += big_sigma0!(c) + bool3ary_232!(c, d, e);
//        a += big_sigma1!(f) + bool3ary_202!(f, g, h) + w; e += a;
//        a += big_sigma0!(b) + bool3ary_232!(b, c, d);
//
//        let v4 = schedule!(v0, v1, v2, v3); let u32x4(w, v, u, t) = k[4] + v4;
//        h += big_sigma1!(e) + bool3ary_202!(e, f, g) + t; d += h;
//        h += big_sigma0!(a) + bool3ary_232!(a, b, c);
//        g += big_sigma1!(d) + bool3ary_202!(d, e, f) + u; c += g;
//        g += big_sigma0!(h) + bool3ary_232!(h, a, b);
//        f += big_sigma1!(c) + bool3ary_202!(c, d, e) + v; b += f;
//        f += big_sigma0!(g) + bool3ary_232!(g, h, a);
//        e += big_sigma1!(b) + bool3ary_202!(b, c, d) + w; a += e;
//        e += big_sigma0!(f) + bool3ary_232!(f, g, h);
//        let v5 = schedule!(v1, v2, v3, v4); let u32x4(w, v, u, t) = k[5] + v5;
//        d += big_sigma1!(a) + bool3ary_202!(a, b, c) + t; h += d;
//        d += big_sigma0!(e) + bool3ary_232!(e, f, g);
//        c += big_sigma1!(h) + bool3ary_202!(h, a, b) + u; g += c;
//        c += big_sigma0!(d) + bool3ary_232!(d, e, f);
//        b += big_sigma1!(g) + bool3ary_202!(g, h, a) + v; f += b;
//        b += big_sigma0!(c) + bool3ary_232!(c, d, e);
//        a += big_sigma1!(f) + bool3ary_202!(f, g, h) + w; e += a;
//        a += big_sigma0!(b) + bool3ary_232!(b, c, d);
//
//        let v6 = schedule!(v2, v3, v4, v5); let u32x4(w, v, u, t) = k[6] + v6;
//        h += big_sigma1!(e) + bool3ary_202!(e, f, g) + t; d += h;
//        h += big_sigma0!(a) + bool3ary_232!(a, b, c);
//        g += big_sigma1!(d) + bool3ary_202!(d, e, f) + u; c += g;
//        g += big_sigma0!(h) + bool3ary_232!(h, a, b);
//        f += big_sigma1!(c) + bool3ary_202!(c, d, e) + v; b += f;
//        f += big_sigma0!(g) + bool3ary_232!(g, h, a);
//        e += big_sigma1!(b) + bool3ary_202!(b, c, d) + w; a += e;
//        e += big_sigma0!(f) + bool3ary_232!(f, g, h);
//        let v7 = schedule!(v3, v4, v5, v6); let u32x4(w, v, u, t) = k[7] + v7;
//        d += big_sigma1!(a) + bool3ary_202!(a, b, c) + t; h += d;
//        d += big_sigma0!(e) + bool3ary_232!(e, f, g);
//        c += big_sigma1!(h) + bool3ary_202!(h, a, b) + u; g += c;
//        c += big_sigma0!(d) + bool3ary_232!(d, e, f);
//        b += big_sigma1!(g) + bool3ary_202!(g, h, a) + v; f += b;
//        b += big_sigma0!(c) + bool3ary_232!(c, d, e);
//        a += big_sigma1!(f) + bool3ary_202!(f, g, h) + w; e += a;
//        a += big_sigma0!(b) + bool3ary_232!(b, c, d);
//
//        let v8 = schedule!(v4, v5, v6, v7); let u32x4(w, v, u, t) = k[8] + v8;
//        h += big_sigma1!(e) + bool3ary_202!(e, f, g) + t; d += h;
//        h += big_sigma0!(a) + bool3ary_232!(a, b, c);
//        g += big_sigma1!(d) + bool3ary_202!(d, e, f) + u; c += g;
//        g += big_sigma0!(h) + bool3ary_232!(h, a, b);
//        f += big_sigma1!(c) + bool3ary_202!(c, d, e) + v; b += f;
//        f += big_sigma0!(g) + bool3ary_232!(g, h, a);
//        e += big_sigma1!(b) + bool3ary_202!(b, c, d) + w; a += e;
//        e += big_sigma0!(f) + bool3ary_232!(f, g, h);
//        let v9 = schedule!(v5, v6, v7, v8); let u32x4(w, v, u, t) = k[9] + v9;
//        d += big_sigma1!(a) + bool3ary_202!(a, b, c) + t; h += d;
//        d += big_sigma0!(e) + bool3ary_232!(e, f, g);
//        c += big_sigma1!(h) + bool3ary_202!(h, a, b) + u; g += c;
//        c += big_sigma0!(d) + bool3ary_232!(d, e, f);
//        b += big_sigma1!(g) + bool3ary_202!(g, h, a) + v; f += b;
//        b += big_sigma0!(c) + bool3ary_232!(c, d, e);
//        a += big_sigma1!(f) + bool3ary_202!(f, g, h) + w; e += a;
//        a += big_sigma0!(b) + bool3ary_232!(b, c, d);
//
//        let v10 = schedule!(v6, v7, v8, v9); let u32x4(w, v, u, t) = k[10] + v10;
//        h += big_sigma1!(e) + bool3ary_202!(e, f, g) + t; d += h;
//        h += big_sigma0!(a) + bool3ary_232!(a, b, c);
//        g += big_sigma1!(d) + bool3ary_202!(d, e, f) + u; c += g;
//        g += big_sigma0!(h) + bool3ary_232!(h, a, b);
//        f += big_sigma1!(c) + bool3ary_202!(c, d, e) + v; b += f;
//        f += big_sigma0!(g) + bool3ary_232!(g, h, a);
//        e += big_sigma1!(b) + bool3ary_202!(b, c, d) + w; a += e;
//        e += big_sigma0!(f) + bool3ary_232!(f, g, h);
//        let v11 = schedule!(v7, v8, v9, v10); let u32x4(w, v, u, t) = k[11] + v11;
//        d += big_sigma1!(a) + bool3ary_202!(a, b, c) + t; h += d;
//        d += big_sigma0!(e) + bool3ary_232!(e, f, g);
//        c += big_sigma1!(h) + bool3ary_202!(h, a, b) + u; g += c;
//        c += big_sigma0!(d) + bool3ary_232!(d, e, f);
//        b += big_sigma1!(g) + bool3ary_202!(g, h, a) + v; f += b;
//        b += big_sigma0!(c) + bool3ary_232!(c, d, e);
//        a += big_sigma1!(f) + bool3ary_202!(f, g, h) + w; e += a;
//        a += big_sigma0!(b) + bool3ary_232!(b, c, d);
//
//        let v12 = schedule!(v8, v9, v10, v11); let u32x4(w, v, u, t) = k[12] + v12;
//        h += big_sigma1!(e) + bool3ary_202!(e, f, g) + t; d += h;
//        h += big_sigma0!(a) + bool3ary_232!(a, b, c);
//        g += big_sigma1!(d) + bool3ary_202!(d, e, f) + u; c += g;
//        g += big_sigma0!(h) + bool3ary_232!(h, a, b);
//        f += big_sigma1!(c) + bool3ary_202!(c, d, e) + v; b += f;
//        f += big_sigma0!(g) + bool3ary_232!(g, h, a);
//        e += big_sigma1!(b) + bool3ary_202!(b, c, d) + w; a += e;
//        e += big_sigma0!(f) + bool3ary_232!(f, g, h);
//        let v13 = schedule!(v9, v10, v11, v12); let u32x4(w, v, u, t) = k[13] + v13;
//        d += big_sigma1!(a) + bool3ary_202!(a, b, c) + t; h += d;
//        d += big_sigma0!(e) + bool3ary_232!(e, f, g);
//        c += big_sigma1!(h) + bool3ary_202!(h, a, b) + u; g += c;
//        c += big_sigma0!(d) + bool3ary_232!(d, e, f);
//        b += big_sigma1!(g) + bool3ary_202!(g, h, a) + v; f += b;
//        b += big_sigma0!(c) + bool3ary_232!(c, d, e);
//        a += big_sigma1!(f) + bool3ary_202!(f, g, h) + w; e += a;
//        a += big_sigma0!(b) + bool3ary_232!(b, c, d);
//
//        let v14 = schedule!(v10, v11, v12, v13); let u32x4(w, v, u, t) = k[14] + v14;
//        h += big_sigma1!(e) + bool3ary_202!(e, f, g) + t; d += h;
//        h += big_sigma0!(a) + bool3ary_232!(a, b, c);
//        g += big_sigma1!(d) + bool3ary_202!(d, e, f) + u; c += g;
//        g += big_sigma0!(h) + bool3ary_232!(h, a, b);
//        f += big_sigma1!(c) + bool3ary_202!(c, d, e) + v; b += f;
//        f += big_sigma0!(g) + bool3ary_232!(g, h, a);
//        e += big_sigma1!(b) + bool3ary_202!(b, c, d) + w; a += e;
//        e += big_sigma0!(f) + bool3ary_232!(f, g, h);
//        let v15 = schedule!(v11, v12, v13, v14); let u32x4(w, v, u, t) = k[15] + v15;
//        d += big_sigma1!(a) + bool3ary_202!(a, b, c) + t; h += d;
//        d += big_sigma0!(e) + bool3ary_232!(e, f, g);
//        c += big_sigma1!(h) + bool3ary_202!(h, a, b) + u; g += c;
//        c += big_sigma0!(d) + bool3ary_232!(d, e, f);
//        b += big_sigma1!(g) + bool3ary_202!(g, h, a) + v; f += b;
//        b += big_sigma0!(c) + bool3ary_232!(c, d, e);
//        a += big_sigma1!(f) + bool3ary_202!(f, g, h) + w; e += a;
//        a += big_sigma0!(b) + bool3ary_232!(b, c, d);
//
//        state[0] += a;
//        state[1] += b;
//        state[2] += c;
//        state[3] += d;
//        state[4] += e;
//        state[5] += f;
//        state[6] += g;
//        state[7] += h;
//    }
//}

mod x86emu {
    use std::simd::u32x4;
    use std::num::Int;
    use super::{BLOCK_LEN, STATE_LEN};

    /// Not an intrinsic, but works like an unaligned load.
    #[inline]
    fn sha256load(v2: u32x4, v3: u32x4) -> u32x4 {
        // Emulates `pblendd` intrinsic.
        #[inline]
        fn pblendd_0x7(a: u32x4, b: u32x4) -> u32x4 {
            let u32x4(a3, _, _, _) = a;
            let u32x4(_, b2, b1, b0) = b;
            u32x4(a3, b2, b1, b0)
        }
        // Emulates `pshufd` intrinsic.
        #[inline]
        fn pshufd_0x39(a: u32x4) -> u32x4 {
            let u32x4(a3, a2, a1, a0) = a;
            u32x4(a0, a3, a2, a1)
        }
        pblendd_0x7(pshufd_0x39(v3), pshufd_0x39(v2))
    }

    /// Not an intrinsic, but useful for swapping vectors.
    #[inline]
    fn sha256swap(v0: u32x4) -> u32x4 {
        // Emulates `pshufd` intrinsic.
        #[inline]
        fn pshufd_0x4e(a: u32x4) -> u32x4 {
            let u32x4(a3, a2, a1, a0) = a;
            u32x4(a1, a0, a3, a2)
        }
        pshufd_0x4e(v0)
    }

    /// Emulates `llvm.x86.sha256msg1` intrinsic.
    #[inline]
    pub fn sha256msg1(v0: u32x4, v1: u32x4) -> u32x4 {
        // sigma 0 on vectors
        #[inline]
        fn sigma0x4(x: u32x4) -> u32x4 {
            ((x >> u32x4( 7,  7,  7,  7)) | (x << u32x4(25, 25, 25, 25))) ^
            ((x >> u32x4(18, 18, 18, 18)) | (x << u32x4(14, 14, 14, 14))) ^
             (x >> u32x4( 3,  3,  3,  3))
        }
        v0 + sigma0x4(sha256load(v0, v1))
    }

    /// Emulates `llvm.x86.sha256msg2` intrinsic.
    #[inline]
    pub fn sha256msg2(v4: u32x4, v3: u32x4) -> u32x4 {
        let u32x4(x3, x2, x1, x0) = v4;
        let u32x4(w15, w14, _, _) = v3;
        let w16 = x0 + sigma1!(w14);
        let w17 = x1 + sigma1!(w15);
        let w18 = x2 + sigma1!(w16);
        let w19 = x3 + sigma1!(w17);
        u32x4(w19, w18, w17, w16)
    }

    /// Emulates `llvm.x86.sha256rnds2` intrinsic.
    #[inline]
    pub fn sha256rnds2(cdgh: u32x4, abef: u32x4, wk: u32x4) -> u32x4 {
        let u32x4(_, _, wk1, wk0) = wk;
        let u32x4(a0, b0, e0, f0) = abef;
        let u32x4(c0, d0, g0, h0) = cdgh;

        // a round
        let x0 = big_sigma1!(e0) + bool3ary_202!(e0, f0, g0) + wk0 + h0;
        let y0 = big_sigma0!(a0) + bool3ary_232!(a0, b0, c0);
        let (a1, b1, c1, d1, e1, f1, g1, h1) = (
            x0 + y0, a0, b0, c0,
            x0 + d0, e0, f0, g0);

        // a round
        let x1 = big_sigma1!(e1) + bool3ary_202!(e1, f1, g1) + wk1 + h1;
        let y1 = big_sigma0!(a1) + bool3ary_232!(a1, b1, c1);
        let (a2, b2, _, _, e2, f2, _, _) = (
            x1 + y1, a1, b1, c1,
            x1 + d1, e1, f1, g1);

        u32x4(a2, b2, e2, f2)
    }

    /// Process a block with the SHA-2 SHA-256 algorithm.
    ///
    /// Internally, this uses functions which resemble the new Intel SHA instruction sets,
    /// and so it's data locality properties may improve performance. However, to benefit
    /// the most from this implementation, replace these functions with x86 intrinsics to
    /// get a possible speed boost.
    #[inline]
    pub fn digest_block(state: &mut [u32; STATE_LEN], block: &[u32; BLOCK_LEN]) {
        use super::K_X4;
        let k = &K_X4;

        let mut abef = u32x4(state[0],
                             state[1],
                             state[4],
                             state[5]);
        let mut cdgh = u32x4(state[2],
                             state[3],
                             state[6],
                             state[7]);

        macro_rules! schedule {
            ($v0:expr, $v1:expr, $v2:expr, $v3:expr) => {
                sha256msg2(sha256msg1($v0, $v1) + sha256load($v2, $v3), $v3)
            }
        }
        macro_rules! rounds4 {
            ($abef:ident, $cdgh:ident, $rest:expr) => {
                {
                    $cdgh = sha256rnds2($cdgh, $abef, $rest);
                    $abef = sha256rnds2($abef, $cdgh, sha256swap($rest));
                }
            }
        }

        // Rounds 0..64
        let mut w0 = u32x4(block[3],
                           block[2],
                           block[1],
                           block[0]);
        rounds4!(abef, cdgh, k[0] + w0);
        let mut w1 = u32x4(block[7],
                           block[6],
                           block[5],
                           block[4]);
        rounds4!(abef, cdgh, k[1] + w1);
        let mut w2 = u32x4(block[11],
                           block[10],
                           block[9],
                           block[8]);
        rounds4!(abef, cdgh, k[2] + w2);
        let mut w3 = u32x4(block[15],
                           block[14],
                           block[13],
                           block[12]);
        rounds4!(abef, cdgh, k[3] + w3);
        let mut w4 = schedule!(w0, w1, w2, w3);
        rounds4!(abef, cdgh, k[4] + w4);
        w0 = schedule!(w1, w2, w3, w4);
        rounds4!(abef, cdgh, k[5] + w0);
        w1 = schedule!(w2, w3, w4, w0);
        rounds4!(abef, cdgh, k[6] + w1);
        w2 = schedule!(w3, w4, w0, w1);
        rounds4!(abef, cdgh, k[7] + w2);
        w3 = schedule!(w4, w0, w1, w2);
        rounds4!(abef, cdgh, k[8] + w3);
        w4 = schedule!(w0, w1, w2, w3);
        rounds4!(abef, cdgh, k[9] + w4);
        w0 = schedule!(w1, w2, w3, w4);
        rounds4!(abef, cdgh, k[10] + w0);
        w1 = schedule!(w2, w3, w4, w0);
        rounds4!(abef, cdgh, k[11] + w1);
        w2 = schedule!(w3, w4, w0, w1);
        rounds4!(abef, cdgh, k[12] + w2);
        w3 = schedule!(w4, w0, w1, w2);
        rounds4!(abef, cdgh, k[13] + w3);
        w4 = schedule!(w0, w1, w2, w3);
        rounds4!(abef, cdgh, k[14] + w4);
        w0 = schedule!(w1, w2, w3, w4);
        rounds4!(abef, cdgh, k[15] + w0);
        let u32x4(a, b, e, f) = abef;
        let u32x4(c, d, g, h) = cdgh;

        state[0] += a;
        state[1] += b;
        state[2] += c;
        state[3] += d;
        state[4] += e;
        state[5] += f;
        state[6] += g;
        state[7] += h;
    }
}

//pub mod x86asm {
//    use std::simd::u32x4;
//
//    #[allow(unused_variables)]
//    pub unsafe fn digest_block(state: &mut [u32; STATE_LEN], block: &[u32; BLOCK_LEN]) {
//        const K: &'static [u32x4; 16] = &[u32x4(0,0,0,0); 16];
//
//        let state0 = u32x4(state[3], state[2], state[1], state[0]);
//        let state1 = u32x4(state[7], state[6], state[5], state[4]);
//
//        let state2 = state0; // copy
//        let state3 = state1; // copy
//
//        asm!("
//             movq        $0, %rdi
//             movq        $1, %rsi
//             movq        $2, %rdx
//
//             movdqa      0*16(%rdi), %xmm8
//             movdqa      1*16(%rdi), %xmm9
//
//             movdqu      0*16(%rsi), %xmm0
//             movdqa      %xmm0, %xmm2
//             paddd       0*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             pshufd      $$0x0e, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//
//             movdqu      1*16(%rsi), %xmm0
//             movdqa      %xmm0, %xmm3
//             paddd       1*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//             sha256msg1  %xmm3, %xmm2
//
//             movdqu      2*16(%rsi), %xmm0
//             movdqa      %xmm0, %xmm3
//             paddd       2*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//             sha256msg1  %xmm4, %xmm3
//
//             movdqu      3*16(%rsi), %xmm0
//             movdqa      %xmm0, %xmm5
//             paddd       3*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             movdqa      %xmm5, %xmm6
//             palignr     $$4, %xmm6, %xmm4
//             paddd       %xmm6, %xmm2
//             sha256msg2  %xmm5, %xmm2
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//             sha256msg1  %xmm5, %xmm4
//
//             movdqa      %xmm2, %xmm0
//             paddd       4*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             movdqa      %xmm2, %xmm6
//             palignr     $$4, %xmm6, %xmm5
//             paddd       %xmm6, %xmm3
//             sha256msg2  %xmm2, %xmm3
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//             sha256msg1  %xmm2, %xmm5
//
//             movdqa      %xmm3, %xmm0
//             paddd       5*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             movdqa      %xmm2, %xmm6
//             palignr     $$4, %xmm6, %xmm5
//             paddd       %xmm6, %xmm3
//             sha256msg2  %xmm2, %xmm3
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//             sha256msg1  %xmm2, %xmm5
//
//             movdqa      %xmm4, %xmm0
//             paddd       6*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             movdqa      %xmm2, %xmm6
//             palignr     $$4, %xmm6, %xmm5
//             paddd       %xmm6, %xmm3
//             sha256msg2  %xmm2, %xmm3
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//             sha256msg1  %xmm2, %xmm5
//
//             movdqa      %xmm5, %xmm0
//             paddd       7*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             movdqa      %xmm2, %xmm6
//             palignr     $$4, %xmm6, %xmm5
//             paddd       %xmm6, %xmm3
//             sha256msg2  %xmm2, %xmm3
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//             sha256msg1  %xmm2, %xmm5
//
//             movdqa      %xmm2, %xmm0
//             paddd       8*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             movdqa      %xmm2, %xmm6
//             palignr     $$4, %xmm6, %xmm5
//             paddd       %xmm6, %xmm3
//             sha256msg2  %xmm2, %xmm3
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//             sha256msg1  %xmm2, %xmm5
//
//             movdqa      %xmm3, %xmm0
//             paddd       9*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             movdqa      %xmm2, %xmm6
//             palignr     $$4, %xmm6, %xmm5
//             paddd       %xmm6, %xmm3
//             sha256msg2  %xmm2, %xmm3
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//             sha256msg1  %xmm2, %xmm5
//
//             movdqa      %xmm4, %xmm0
//             paddd       10*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             movdqa      %xmm2, %xmm6
//             palignr     $$4, %xmm6, %xmm5
//             paddd       %xmm6, %xmm3
//             sha256msg2  %xmm2, %xmm3
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//             sha256msg1  %xmm2, %xmm5
//
//             movdqa      %xmm5, %xmm0
//             paddd       11*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             movdqa      %xmm2, %xmm6
//             palignr     $$4, %xmm6, %xmm5
//             paddd       %xmm6, %xmm3
//             sha256msg2  %xmm2, %xmm3
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//             sha256msg1  %xmm2, %xmm5
//
//             movdqa      %xmm2, %xmm0
//             paddd       12*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             movdqa      %xmm2, %xmm6
//             palignr     $$4, %xmm6, %xmm5
//             paddd       %xmm6, %xmm3
//             sha256msg2  %xmm2, %xmm3
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//             sha256msg1  %xmm2, %xmm5
//
//             movdqa      %xmm3, %xmm0
//             paddd       13*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             movdqa      %xmm3, %xmm6
//             palignr     $$4, %xmm2, %xmm6
//             paddd       %xmm6, %xmm4
//             sha256msg2  %xmm3, %xmm4
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//
//             movdqa      %xmm4, %xmm0
//             paddd       14*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             movdqa      %xmm4, %xmm6
//             palignr     $$4, %xmm3, %xmm6
//             paddd       %xmm6, %xmm5
//             sha256msg2  %xmm4, %xmm5
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//
//             movdqa      %xmm5, %xmm0
//             paddd       15*16(%rdx), %xmm0
//             sha256rnds2 %xmm8, %xmm9
//             pshufd      $$0x0E, %xmm0, %xmm0
//             sha256rnds2 %xmm9, %xmm8
//
//             "
//             //paddd       %xmm8, 0*16(%rdi)
//             //paddd       %xmm9, 1*16(%rdi)
//             :
//             : "m" (state), // rdi
//               "m" (block), // rsi
//               "m" (K)      // rdx
//             : "xmm0", "xmm2", "xmm3", "xmm4",
//               "xmm5", "xmm6", "xmm8", "xmm9",
//               "rdi", "rsi", "rdx", "memory"
//             );
//    }
//}
//
//mod x86ffi {
//    #[inline]
//    pub fn digest_block(state: &mut [u32; STATE_LEN], block: &[u32; BLOCK_LEN]) {
//        use intrinsics::sha256_digest_block;
//        sha256_digest_block(state, block);
//    }
//    //#[inline]
//    //pub fn digest(state: &mut [u32/*; 8*/], msg: &[u8/*; 64*/]) {
//    //    use intrinsics::sha256_digest;
//    //    sha256_digest(state, msg);
//    //}
//    //#[inline]
//    //pub fn hex_digest(hex_state: &mut [u8/*; 64*/], msg: &[u8/*; 64*/]) {
//    //    use intrinsics::sha256_hex_digest;
//    //    sha256_hex_digest(hex_state, msg);
//    //}
//}
//
//mod nettle {
//    #[link(name = "nettle")]
//    extern {
//        fn _nettle_sha256_compress(state: *mut u32, input: *const u8, k: *const u32);
//    }
//
//    #[inline]
//    pub fn digest_block_u8(state: &mut [u32; STATE_LEN], block: &[u8/*; (BLOCK_LEN*4)*/]) {
//        use super::K;
//
//        unsafe {
//            _nettle_sha256_compress(state.as_mut_ptr(), block.as_ptr(), K.as_ptr());
//        }
//    }
//}
//
//mod openssl {
//    //use libc::types::os::arch::c95::c_uchar;
//    //use libc::types::os::arch::c95::c_uint;
//    //use libc::types::os::arch::c95::c_ulong;
//    //use libc::types::os::arch::c95::size_t;
//    use super::{BLOCK_LEN, STATE_LEN};
//
//    #[repr(C)]
//    pub struct Sha256Ctx {
//        pub h: [u32; STATE_LEN],
//        pub nl: u32,
//        pub nh: u32,
//        pub data: [u32; BLOCK_LEN],
//        pub num: u32,
//        pub len: u32,
//    }
//    impl Sha256Ctx {
//        pub fn new() -> Self {
//            Sha256Ctx {h: [0; STATE_LEN], nl: 0, nh: 0,
//                       data: [0; BLOCK_LEN], num: 0, len: 0}
//        }
//    }
//
//    #[link(name = "crypto")]
//    extern {
//        fn SHA256_Init(ctx: *mut Sha256Ctx);
//        //fn SHA256_Update(ctx: *mut Sha256Ctx, data: *const u8, len: size_t);
//        fn SHA256_Transform(ctx: *mut Sha256Ctx, data: *const u8);
//        //fn SHA256_Final(md: *mut c_uchar, ctx: *mut Sha256Ctx);
//    }
//
//    #[inline]
//    pub fn digest_block_u8(state: &mut [u32; STATE_LEN], block: &[u8/*; 64*/]) {
//        let mut ctx = Sha256Ctx::new();
//
//        unsafe {
//            SHA256_Init(&mut ctx);
//            for i in 0..STATE_LEN {
//                ctx.h[i] = state[i];
//            }
//            SHA256_Transform(&mut ctx, block.as_ptr());
//            for i in 0..STATE_LEN {
//                state[i] = ctx.h[i];
//            }
//        }
//    }
//}



// -- SHA-256-specific

////#[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
//pub fn digest_block(state: &mut [u32; STATE_LEN], block: &[u8/*; 64*/]) {
//    //use self::slow::digest_block;
//    //use self::fast::digest_block;
//    use self::x86emu::digest_block;
//    //use self::x86ffi::digest_block;
//    //use self::x86asm::digest_block;
//    //use self::nettle::digest_block;
//    //use self::openssl::digest_block;
//
//    assert_eq!(block.len(), BLOCK_LEN*4);
//    let mut block2 = [0u32; 16];
//    (&mut block2[..]).write_be(block).unwrap();
//    digest_block(state, &block2);
//}

//pub fn digest(state: &mut [u32; STATE_LEN], msg: &[u8]) {
//    let mut h: Sha256 = Default::default();
//    h.0.get_mut().0 = *state;
//    h.write(msg);
//    *state = h.0.get_mut().0;
//}

/// The `Sha256State` private backend to a SHA-1 message digest algorithm.
pub struct Sha256State([u32; STATE_LEN]);

/// The `Sha256` public frontend to a SHA-1 message digest algorithm.
pub struct Sha256(BufWriter<PadWriter<Sha256State>>);

mod impls {
    use endian::{ReadEndian, WriteEndian};
    use std::default::Default;
    use std::hash::Hasher;
    use std::io::{Read, Write, BufWriter};
    use std::io;
    use utils::{PadWriter, Reset, State, Block, ToArray, FromArray};
    use super::{
        H, STATE_LEN, BLOCK_LEN, 
        Sha256, Sha256State,
    };

    //
    // Sha256State implementations
    //

    impl Default for Sha256State {
        
        /// Construct a default `Sha256State` object.
        fn default() -> Sha256State {
            Sha256State(H)
        }
    }

    impl Reset for Sha256State {
        
        /// TODO
        fn reset(&mut self) {
            //println!("Sha256State::reset()");
            self.0 = H;
        }
    }

    impl Hasher for Sha256State {
        
        /// TODO
        fn finish(&self) -> u64 {
            //println!("Sha256State::Hasher::finish()");
            ((self.0[0] as u64) << 32u64) | (self.0[1] as u64)
        }

        /// TODO
        fn write(&mut self, buf: &[u8]) {
             self.write_block(buf);
        }
    }

    impl State for Sha256State {
        type State = [u32; STATE_LEN];
    }
    
    impl Block for Sha256State {
        type Prefix = u8;
        type Suffix = u64;

        /// Return the block size in bytes.
        fn block_len() -> usize { BLOCK_LEN*4 }

        /// Reset this to the original state.
        fn write_block(&mut self, buf: &[u8]) {
            //use super::slow::digest_block;
            //use super::fast::digest_block;
            use super::x86emu::digest_block;
            ////use super::x86ffi::digest_block;
            ////use super::x86asm::digest_block;
            ////use super::nettle::digest_block;
            ////use super::openssl::digest_block;

            println!("Sha256State::Block::write_block(buf[0..{}])", buf.len());
            assert_eq!(buf.len(), <Self as Block>::block_len());
            let mut block = [0u32; BLOCK_LEN];
            (&mut block[..]).write_be(buf).unwrap();
            digest_block(&mut self.0, &block);
        }
    }

    impl FromArray<[u32; STATE_LEN]> for Sha256State {

        /// TODO
        fn from_array(&mut self, from: &[u32; STATE_LEN]) {

            // Set state
            self.0 = *from;
        }
    }
    
    impl ToArray<[u32; STATE_LEN]> for Sha256State {

        /// TODO
        fn to_array(&self) -> &[u32; STATE_LEN] {

            // Get state as array
            &self.0
        }
    }
    
    impl AsSlice<u32> for Sha256State {
        
        /// TODO
        fn as_slice<'a>(&'a self) -> &'a [u32] {
            
            // Get state as slice
            &self.0[..]
        }
    }

    //
    // Sha256 implementations
    //

    impl Default for Sha256 {
        
        /// Construct a default `Sha256` object.
        fn default() -> Sha256 {
            //println!("Sha256::default()");
            let hasher: Sha256State = Default::default();
            let mut padder = PadWriter::new(hasher);//0x80u8, 0u64, BLOCK_LEN*4);
            padder.prefix = 0x80u8;
            let buffer = BufWriter::new(padder);
            Sha256(buffer)
        }
    }

    impl State for Sha256 {
        type State = [u32; STATE_LEN];
    }
    
    impl Reset for Sha256 {
        
        /// TODO
        fn reset(&mut self) {
            
            // Reset padder
            self.0.get_mut().reset();

            // Reset hasher
            self.0.get_mut().get_mut().reset();
        }
    }

    impl Hasher for Sha256 {
        
        /// TODO
        fn finish(&self) -> u64 {

            // Finish hasher
            self.0.get_ref().get_ref().finish()
        }

        /// TODO
        fn write(&mut self, buf: &[u8]) {

            // Write to buffer
            self.0.write(buf).unwrap();
        }
    }

    impl FromArray<[u32; STATE_LEN]> for Sha256 {
        
        /// TODO
        fn from_array(&mut self, from: &[u32; STATE_LEN]) {
            self.0.get_mut().get_mut().0 = *from;
        }
    }
    
    impl ToArray<[u32; STATE_LEN]> for Sha256 {

        /// TODO
        fn to_array(&self) -> &[u32; STATE_LEN] {

            // Get hasher as array
            &self.0.get_ref().get_ref().0
        }
    }
    
    impl AsSlice<u32> for Sha256 {
        
        /// TODO
        fn as_slice<'a>(&'a self) -> &'a [u32] {

            // Get hasher as slice
            self.to_array().as_slice()
        }
    }

    impl Read for Sha256 {
        
        /// TODO
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {

            // Read state as big-endian
            //println!("Sha256::read(buf[0..{}])", buf.len());
            self.as_slice().read_be(buf)
        }
    }

    impl Write for Sha256 {

        /// TODO
        fn flush(&mut self) -> io::Result<()> {
            //println!("Sha256::flush()");
            self.0.flush()
        }

        /// TODO
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            //println!("Sha256::write(buf[0..{}])", buf.len());
            self.0.write(buf)
        }
    }
}

//mod io_impls {
//
//    impl Write for Sha256State {
//
//        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
//        }
//
//        fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
//            for block in buf.chunks(BLOCK_LEN*4) {
//                self.2.write(block);
//                if block.len() % (BLOCK_LEN*4) == 0 {
//                    self.flush();
//                }
//            }
//            Ok(())
//        }
//
//        fn flush(&mut self) -> io::Result<()> {
//            if self.2.len() % (BLOCK_LEN*4) != 0 {
//                return Err(io::Error::new(
//                        io::ErrorKind::InvalidInput,
//                        "input buffer length must be a multiple of block length",
//                        None));
//            }
//
//            utils::flush(digest_block, BLOCK_LEN*4,
//                         &mut self.0, &self.2);
//
//            self.2.clear();
//            Ok(())
//        }
//    }
//
//}
//
//pub struct Sha256 {
//    state: [u32; STATE_LEN],
//    work:  [u32; BLOCK_LEN*4],
//    buffer: [u8; BLOCK_LEN*4*2],
//
//    message: Vec<u8>,
//    finished: bool,
//    length: usize,
//}
//
//impl Sha256 {
//    fn new() -> Sha256 { Sha256 {
//            state: H,
//            work: [u32; BLOCK_LEN*4],
//            buffer: [u8; BLOCK_LEN*4*2],
//            message: Vec::with_capacity(BLOCK_LEN*4*2),
//            finished: false,
//            length: 0
//        }
//    }
//}

//pub fn digest(state: &mut [u8; 32], msg: &[u8/*; 64*/]) {
//    let digest: [u8; 32] = digest_to_hash::<[u8], Sha256>(msg);
//    *state = digest;
//}

//#[unstable(feature = "default", reason = "std::old_io and std::io are both unstable")]
pub fn digest_block<B: Default + State + Block + ToArray<<B as State>::State> + FromArray<<B as State>::State>>(state: &mut <B as State>::State, block: &[u8]) {
    //use self::slow::digest_block;
    //use self::fast::digest_block;
    //use self::x86emu::digest_block;
    ////use self::x86ffi::digest_block;
    ////use self::x86asm::digest_block;
    ////use self::nettle::digest_block;
    ////use self::openssl::digest_block;
    //assert_eq!(block.len(), BLOCK_LEN*4);
    //let mut block2 = [0u32; 16];
    //(&mut block2[..]).write_be(block).unwrap();
    //digest_block(state, &block2);
    
    let mut b: B = Default::default();
    b.from_array(&*state);
    b.write_block(block);
    *state = *b.to_array();
}

pub fn digest<H: Default + State + Hasher + Read + Write + ToArray<<H as State>::State>>(state: &mut <H as State>::State, msg: &[u8]) {
    let mut h: H = Default::default();
    Write::write(&mut h, msg).unwrap();
    Write::flush(&mut h).unwrap();

    // Copy
    //*state = h.0.get_ref().get_ref().0;
    *state = *h.to_array();
}

pub fn digest_to_bytes<H: Default + Reset + Hasher + Read + Write>(msg: &[u8]) -> Vec<u8> {
    let mut h: H = Default::default();
    Write::write(&mut h, msg).unwrap();
    Write::flush(&mut h).unwrap();

    // Serialize
    let mut bytes = vec![0u8; STATE_LEN*4];
    h.read(&mut bytes[..]).unwrap();
    bytes
}

// Common entry point for tests
pub fn digest_to_hex<H: Default + Reset + Hasher + Read + Write>(msg: &str) -> String {
    digest_to_bytes::<H>(&msg.as_bytes()).as_slice().to_hex()
}

#[cfg(test)]
pub mod tests {
    use test::Bencher;
    use super::{
        BLOCK_LEN,
        STATE_LEN,
        Sha256,
        Sha256State,
        digest,
        digest_block,
        digest_to_bytes,
        digest_to_hex,
    };

    //
    // Tests for `digest_to_hex`
    //

    #[test]
    fn sha256_hello() {

        assert_eq!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9", 
                   digest_to_hex::<Sha256>("hello world").as_slice());

        assert_eq!("7509e5bda0c762d2bac7f90d758b5b2263fa01ccbc542ab5e3df163be08e6ca9", 
                   digest_to_hex::<Sha256>("hello world!").as_slice());

        assert_eq!("db4067cec62c58bf8b2f8982071e77c082da9e00924bf3631f3b024fa54e7d7e", 
                   digest_to_hex::<Sha256>("hello World").as_slice());

        assert_eq!("e4ad0102dc2523443333d808b91a989b71c2439d7362aca6538d49f76baaa5ca", 
                   digest_to_hex::<Sha256>("hello World!").as_slice());

        assert_eq!("64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c", 
                   digest_to_hex::<Sha256>("Hello world").as_slice());

        assert_eq!("c0535e4be2b79ffd93291305436bf889314e4a3faec05ecffcbb7df31ad9e51a", 
                   digest_to_hex::<Sha256>("Hello world!").as_slice());

        assert_eq!("a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e", 
                   digest_to_hex::<Sha256>("Hello World").as_slice());

        assert_eq!("7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069", 
                   digest_to_hex::<Sha256>("Hello World!").as_slice());

        assert_eq!("09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b", 
                   digest_to_hex::<Sha256>("hello, world").as_slice());

        assert_eq!("68e656b251e67e8358bef8483ab0d51c6619f3e7a1a9f0e75838d41ff368f728", 
                   digest_to_hex::<Sha256>("hello, world!").as_slice());

        assert_eq!("211f927b277d1e8feeae2d929912b87ecdfbb3b6155833ccb438710d1694682d", 
                   digest_to_hex::<Sha256>("hello, World").as_slice());

        assert_eq!("04aa5d2533987c34839e8dbc8d8fcac86f0137e31c1c6ea4349ade4fcaf87ed8", 
                   digest_to_hex::<Sha256>("hello, World!").as_slice());

        assert_eq!("4ae7c3b6ac0beff671efa8cf57386151c06e58ca53a78d83f36107316cec125f", 
                   digest_to_hex::<Sha256>("Hello, world").as_slice());

        assert_eq!("315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3", 
                   digest_to_hex::<Sha256>("Hello, world!").as_slice());

        assert_eq!("03675ac53ff9cd1535ccc7dfcdfa2c458c5218371f418dc136f2d19ac1fbe8a5", 
                   digest_to_hex::<Sha256>("Hello, World").as_slice());

        assert_eq!("dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f", 
                   digest_to_hex::<Sha256>("Hello, World!").as_slice());
    }

    #[test]
    fn sha256_empty() {

        assert_eq!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", 
                   digest_to_hex::<Sha256>("").as_slice());
    }
    
    //
    // Tests for `digest_to_bytes`
    //
    
    #[test]
    fn sha256_hello_digest_to_bytes() {
        let bytes = digest_to_bytes::<Sha256>("hello world".as_bytes());
        
        assert_eq!(b"\xb9\x4d\x27\xb9\x93\x4d\x3e\x08\xa5\x2e\x52\xd7\xda\x7d\xab\xfa\xc4\x84\xef\xe3\x7a\x53\x80\xee\x90\x88\xf7\xac\xe2\xef\xcd\xe9", 
                   bytes.as_slice());
    }
    
    //
    // Tests for `digest`
    //

    #[test]
    fn sha256_hello_digest() {
        let mut words = [0u32; STATE_LEN];
        digest::<Sha256>(&mut words, "hello world".as_bytes());

        assert_eq!(words[0], 0xb94d27b9u32);
        assert_eq!(words[1], 0x934d3e08u32);
        assert_eq!(words[2], 0xa52e52d7u32);
        assert_eq!(words[3], 0xda7dabfau32);
        assert_eq!(words[4], 0xc484efe3u32);
        assert_eq!(words[5], 0x7a5380eeu32);
        assert_eq!(words[6], 0x9088f7acu32);
        assert_eq!(words[7], 0xe2efcde9u32);
    }
    
    //
    // Tests for `digest_block`
    //

    fn make_empty_block() -> Vec<u8> {
        let mut block = vec![0u8; BLOCK_LEN*4];
        assert_eq!(block.len(), BLOCK_LEN*4);
        block[0] = 0x80u8;
        block
    }

    fn make_hello_block() -> Vec<u8> {

        // this could use a concat_bytes!
        static HELLO_BLOCK: &'static [u8] = b"hello world\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x58";
        
        let mut block: Vec<u8> = Vec::with_capacity(BLOCK_LEN*4);
        unsafe { block.set_len(BLOCK_LEN*4) };
        (&mut block[..]).clone_from_slice(HELLO_BLOCK);
        assert_eq!(block.len(), BLOCK_LEN*4);
        block
    }

    #[test]
    fn sha256_empty_block() {
        use serialize::hex::ToHex;
        use endian::ReadEndian;
        let mut state = [0u32; STATE_LEN];
        (&mut state[..]).clone_from_slice(&super::H[..]);
        let block_vec = make_empty_block();
        let block = &block_vec[..];
        digest_block::<Sha256State>(&mut state, &block[..]);
        let mut bytes = vec![0u8; STATE_LEN*4];
        (&state[..]).read_be(&mut bytes[..]).unwrap();

        assert_eq!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", 
                   bytes.as_slice().to_hex());
    }

    #[test]
    fn sha256_hello_block() {
        use serialize::hex::ToHex;
        use endian::ReadEndian;
        let mut state = [0u32; STATE_LEN];
        (&mut state[..]).clone_from_slice(&super::H[..]);
        let block_vec = make_hello_block();
        let block = &block_vec[..];
        digest_block::<Sha256State>(&mut state, &block[..]);
        let mut bytes = vec![0u8; STATE_LEN*4];
        (&state[..]).read_be(&mut bytes[..]).unwrap();
        
        assert_eq!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
                   bytes.as_slice().to_hex());
    }

    //
    // Benchmarks for `digest_block`
    //
    
    #[bench]
    fn sha256_hello_blocks(b: & mut Bencher) {
        let mut state = [0u32; STATE_LEN];
        (&mut state[..]).clone_from_slice(&super::H[..]);
        let block_vec = make_hello_block();
        let block = &block_vec[..];
        b.iter( || { digest_block::<Sha256State>(&mut state, block) });
        b.bytes = 64u64;
    }
 
    #[bench]
    fn sha256_empty_blocks(b: & mut Bencher) {
        let mut state = [0u32; STATE_LEN];
        (&mut state[..]).clone_from_slice(&super::H[..]);
        let block_vec = make_empty_block();
        let block = &block_vec[..];
        b.iter( || { digest_block::<Sha256State>(&mut state, block) });
        b.bytes = 64u64;
    }

    //
    // Benchmarks for `digest`
    //
    
    #[bench]
    fn sha256_10(b: & mut Bencher) {
        let mut state = [0u32; STATE_LEN];
        let buf = [1u8; 10];
        let msg = &buf[..];
        b.iter( || { digest::<Sha256>(&mut state, msg); });
        b.bytes = msg.len() as u64;
    }
    #[bench]
    fn sha256_1k(b: & mut Bencher) {
        let mut state = [0u32; STATE_LEN];
        let buf = [1u8; 1024];
        let msg = &buf[..];
        b.iter( || { digest::<Sha256>(&mut state, msg); });
        b.bytes = msg.len() as u64;
    }
    #[bench]
    fn sha256_64k(b: & mut Bencher) {
        let mut state = [0u32; STATE_LEN];
        let buf = [1u8; 65536];
        let msg = &buf[..];
        b.iter( || { digest::<Sha256>(&mut state, msg); });
        b.bytes = msg.len() as u64;
    }

    //
    // Benchmarks for `digest_to_bytes`
    //
    
    #[bench]
    fn sha256_to_bytes_10(b: & mut Bencher) {
        let buf = [1u8; 10];
        let msg = &buf[..];
        b.iter( || { digest_to_bytes::<Sha256>(msg); });
        b.bytes = msg.len() as u64;
    }
    //#[bench]
    //fn sha256_to_bytes_1k(b: & mut Bencher) {
    //    let buf = [1u8; 1024];
    //    let msg = &buf[..];
    //    b.iter( || { digest_to_bytes::<Sha256>(msg); });
    //    b.bytes = msg.len() as u64;
    //}
    //#[bench]
    //fn sha256_to_bytes_64k(b: & mut Bencher) {
    //    let buf = [1u8; 65536];
    //    let msg = &buf[..];
    //    b.iter( || { digest_to_bytes::<Sha256>(msg); });
    //    b.bytes = msg.len() as u64;
    //}

    //
    // Benchmarks for `digest_to_hex`
    //
    
    #[bench]
    fn sha256_to_hex_10(b: & mut Bencher) {
        let buf = [0x20u8; 10];
        let msg = ::std::str::from_utf8(&buf[..]).unwrap();
        b.iter( || { digest_to_hex::<Sha256>(msg); });
        b.bytes = msg.len() as u64;
    }
    //#[bench]
    //fn sha256_to_hex_1k(b: & mut Bencher) {
    //    let buf = [0x20u8; 1024];
    //    let msg = ::std::str::from_utf8(&buf[..]).unwrap();
    //    b.iter( || { digest_to_hex::<Sha256>(msg); });
    //    b.bytes = msg.len() as u64;
    //}
    //#[bench]
    //fn sha256_to_hex_64k(b: & mut Bencher) {
    //    let buf = [0x20u8; 65536];
    //    let msg = ::std::str::from_utf8(&buf[..]).unwrap();
    //    b.iter( || { digest_to_hex::<Sha256>(msg); });
    //    b.bytes = msg.len() as u64;
    //}
}
