use serialize::hex::ToHex;
use arith_u32;
use logic_u32;

pub static SHA1_INITIAL_HASH: [u32; 5] = [
	0x67452301u32, // digits are (34*n + 1) where n = 3, 2, 1, 0
	0xefcdab89u32, // digits are (34*n + 1) where n = 7, 6, 5, 4
	0x98badcfeu32, // digits are (34*n + 16) where n = 4, 5, 6, 7
	0x10325476u32, // digits are (34*n + 16) where n = 0, 1, 2, 3
	0xc3d2e1f0u32  // digits are (15*n) where n = 13, 14, 15, 16
];

pub static SHA1_CONSTANT_POOL: [u32; 4] = [
	0x5a827999u32, // digits of floor(sqrt(2)*2^30)
	0x6ed9eba1u32, // digits of floor(sqrt(3)*2^30)
	0x8f1bbcdcu32, // digits of floor(sqrt(5)*2^30)
	0xca62c1d6u32  // digits of floor(sqrt(10)*2^30)
];

/*
 * sha1::init_hash(hash: &[u32; 5]);
 * sha1::init_work(work: &[u32; 80]);
 * sha1::rounds4_0(hash: &mut [u32; 5], subwork: &[u32; 4]);
 * sha1::rounds4_1(hash: &mut [u32; 5], subwork: &[u32; 4]);
 * sha1::rounds4_2(hash: &mut [u32; 5], subwork: &[u32; 4]);
 * sha1::rounds4_3(hash: &mut [u32; 5], subwork: &[u32; 4]);
 * sha1::rounds4(hash: &mut [u32; 5], subwork: &[u32; 4], qround: u8);
 * sha1::rounds20(hash: &mut [u32; 5], subwork: &[u32; 4], qround: u8);
 * sha1::rounds80(hash: &mut [u32; 5], subwork: &[u32; 4]);

 * sha1::block(hash: &[u32; 5], msg_block: &[u32; 16]);
 * sha1::pad(msg_blocks: &[u32], msg: &[u8], size: u64);
 * sha1::digest(hash: &[u32; 5], msg: &[u8]);
 * sha1::hexdigest(msg: &[u8]) -> String;
 */

#[inline]
pub fn init_hash(hash: &mut [u32]) {
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
        work[t] = arith_u32::rotl(work[t-3]^work[t-8]^work[t-14]^work[t-16], 1);
    }
}

//macro_rules! blockinit_asm {
//}

macro_rules! get_f {
    ($kon:expr, $a:expr, $b:expr, $c:expr) => {
        {
            if $kon == 0 {
                logic_u32::ary_202($a, $b, $c)
            } else if $kon == 2 {
                logic_u32::ary_232($a, $b, $c)
            } else {
                logic_u32::ary_150($a, $b, $c)
            }
        }
    }
}

macro_rules! rounds4_0_asm {
    ($h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr,
     $w0:expr, $w1:expr, $w2:expr, $w3:expr) => {
        {
            let a: u32 = $h0;
            let b: u32 = $h1;
            let c: u32 = $h2;
            let d: u32 = $h3;
            let e: u32 = $h4;

            asm!("
                 sha1nexte %xmm0, %xmm1
                 sha1rnds4 %xmm0, %xmm1, 0
                 ");

        }
    }
}

macro_rules! rounds4_naive {
    ($h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr,
     $w0:expr, $w1:expr, $w2:expr, $w3:expr, $q:expr) => {
        {
            let a: u32 = $h0;
            let b: u32 = $h1;
            let c: u32 = $h2;
            let d: u32 = $h3;
            let e: u32 = $h4;

            let j = arith_u32::rotl_30(b);
            let k = SHA1_CONSTANT_POOL[$q] + arith_u32::rotl_5(a);
            let d3 = get_f!($q, b, c, d) + k + e + $w0;
            let c3 = get_f!($q, a, arith_u32::rotl_5(b), c) + k + d + $w1;
            let e4 = arith_u32::rotl_30(a);
            let d4 = arith_u32::rotl_30(d3);

            $h4 = e4;
            $h3 = d4;
            $h2 = arith_u32::rotl_30(c3);
            $h1 = get_f!($q, d3, e4, j) + k + c + $w2;
            $h0 = get_f!($q, c3, d4, e4) + k + j + $w3;
        }
    }
}


// x86::SHA1RNDS4 x, y, 0
// x86::SHA1NEXTE
#[inline]
pub fn rounds4_0(hash: &mut [u32], work: &[u32]) {
    rounds4_naive!(hash[0], hash[1], hash[2], hash[3], hash[4], 
                   work[0], work[1], work[2], work[3], 0);
}

// x86::SHA1RNDS4 x, y, 1
// x86::SHA1NEXTE
#[inline]
pub fn rounds4_1(hash: &mut [u32], work: &[u32]) {
    rounds4_naive!(hash[0], hash[1], hash[2], hash[3], hash[4], 
                   work[0], work[1], work[2], work[3], 1);
}

// x86::SHA1RNDS4 x, y, 2
// x86::SHA1NEXTE
#[inline]
pub fn rounds4_2(hash: &mut [u32], work: &[u32]) {
    rounds4_naive!(hash[0], hash[1], hash[2], hash[3], hash[4], 
                   work[0], work[1], work[2], work[3], 2);
}

// x86::SHA1RNDS4 x, y, 3
// x86::SHA1NEXTE
#[inline]
pub fn rounds4_3(hash: &mut [u32], work: &[u32]) {
    rounds4_naive!(hash[0], hash[1], hash[2], hash[3], hash[4], 
                   work[0], work[1], work[2], work[3], 3);
}



// x86::SHA1RNDS4
// x86::SHA1NEXTE
#[inline]
pub fn rounds4(hash: &mut [u32], work: &[u32], qround: usize) {
    assert_eq!(hash.len(), 5);
    assert_eq!(work.len(), 4);
    if qround == 0 {
        rounds4_0(hash, work);
    } else if qround == 1 {
        rounds4_1(hash, work);
    } else if qround == 2 {
        rounds4_2(hash, work);
    } else if qround == 3 {
        rounds4_3(hash, work);
    } else {
        panic!("unexpected round number")
    }
}

pub fn rounds80(hash: &mut [u32], work: &[u32]) {
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
    rounds4_3(hash, &work[72..78]);
    rounds4_3(hash, &work[78..80]);
}

pub fn block(hash: &mut [u32], msg_block: &[u32]) {
    assert_eq!(hash.len(), 5);
    assert_eq!(msg_block.len(), 16);
    
    let mut work = [0u32; 80];

    for t in 0us..16us {
        work[t] = msg_block[t];
    }
    
    init_work(&mut work);
    
    rounds80(hash, &work);
}

pub fn block_of_u8(hash: &mut [u32], msg_block: &[u8]) {
}

pub fn pad(msg: &[u8], length: usize) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.push_all(msg);
    bytes.push(0x80u8);
    for _ in 0us..((55 - length) % 64) {
        bytes.push(0u8);
    }
    bytes.write_be_u64(8*length as u64);
    bytes
}

pub fn digest(hash: &mut [u32], msg: &[u8]) {
    init_hash(hash);

    let bytes = pad(msg, msg.len());
        
    // read_u32v_be()
    let mut words: Vec<u32> = Vec::new();
    for word in bytes.chunks(4) {
        words.push(word.read_be_u32().unwrap());
    }

    for msg_block in words.chunks(16) {
        block(hash, msg_block);
    }
}

pub fn hexdigest(msg: &[u8]) -> String {
    let mut hash = [0u32; 5];
    digest(&mut hash, msg);
    
    // write_u32v_be()
    let mut bytes: Vec<u8> = Vec::new();
    bytes.write_be_u32(hash[0]);
    bytes.write_be_u32(hash[1]);
    bytes.write_be_u32(hash[2]);
    bytes.write_be_u32(hash[3]);
    bytes.write_be_u32(hash[4]);
    
    bytes.to_hex()
}


#[test]
fn test_01_hello_world() {
    assert_eq!(hexdigest("hello world"), "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
}
