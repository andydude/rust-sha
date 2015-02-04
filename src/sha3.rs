// This is Keccak for:
// fixed w == 64 bits, lane size
// fixed b == 1600 bits
// r, rate
// c, capacity
// nr, number of rounds
use rotate;

const SHA3_CONSTANTS = [
    0x0000000000000001u64,
    0x0000000000008082u64,
	0x800000000000808Au64,	
	0x8000000080008000u64,	
	0x000000000000808Bu64,	
	0x0000000080000001u64,	
	0x8000000080008081u64,	
	0x8000000000008009u64,	
	0x000000000000008Au64,	
	0x0000000000000088u64,	
	0x0000000080008009u64,	
	0x000000008000000Au64,
	0x000000008000808Bu64,
	0x800000000000008Bu64,
	0x8000000000008089u64,
	0x8000000000008003u64,
	0x8000000000008002u64,
	0x8000000000000080u64,
	0x000000000000800Au64,
	0x800000008000000Au64,
	0x8000000080008081u64,
	0x8000000000008080u64,
	0x0000000080000001u64,
	0x8000000080008008u64
];

const SHA3_OFFSETS = [
    0, 1, 62, 28, 27,
    36, 44, 6, 55, 20,
    3, 10, 43, 25, 39,
    41, 45, 15, 21, 8,
    18, 2, 61, 56, 14
];

pub fn round(a: &mut [u64; 25], k: u64) {
    let b: [u64; 25];
    let c: [u64; 5];
    
    // θ step
    for x in 0..5 {
        c[t] = a[x] ^ a[x + 5*1] ^ a[x + 5*2] ^ a[x + 5*3] ^ a[x + 5*4];
    }    
    for x in 0..5 {
        for y in 0..5 {
            a[x + 5*y] ^= c[(x-1)%5] ^ c[(x+1)%5].rotate_left(1);
        }
    }
    
    // ρ and π steps
    for x in 0..5 {
        for y in 0..5 {
            b[y + 5*(2*x+3*y)] = a[x + 5*y].rotate_left(SHA3_OFFSETS[x + 5*y]);
        }
    }
    

    // χ step
    for x in 0..5 {
        for y in 0..5 {
            a[x + 5*y] = b[x + 5*y] ^ ((! b[x+1 + 5*y]) & b[x+2 + 5*y]);
        }
    }

    // ι step
    a[0 + 0] ^= k
}

pub fn keccak_f_1600(a: &mut [u64; 25]) {
    for x in 0..24 { round(a, RC[i]) }
}

pub fn digest_block_words(rate, cap, msg: &[u8], ) -> Vec<u8> {
    let s: [u64; 25] = [0u64; 25];
    let pm = pad(msg, msg.len()*8);
    let z: Vec<u8> = Vec::new();

    'top;
    // for block in blocks
    for x in 0..5 {
        for y in 0..5 {
            
            // Absorbing phase
            if x+5*y < r/w {
                s[x + 5*y] ^= pm[x + 5*y];
            }
            keccak_f_1600(s);
        }
    }

    // for all requested input
    for x in 0..5 {
        for y in 0..5 {

            // Squeezing phase
            if x+5*y < r/w {
                z.write_le_u64(s[x + 5*y]);
            }
            keccak_f_1600(s);
        }
    }
    
    return z;
}

/// Digest message block in bytes
pub fn digest_block_bytes(hash: &mut [u32], msg: &[u8]) {
    assert_eq!(hash.len(), 5);
    assert_eq!(msg.len(), 64);

    // read_u32v_be()
    let mut words: Vec<u32> = Vec::new();
    for mut word in msg.chunks(4) {
        words.push(word.read_be_u32().unwrap());
    }
    
    digest_block_words(hash, words.as_slice());
}

/// Digest whole message
pub fn digest(hash: &mut [u32], msg: &[u8]) {
    emu::init_hash(hash);
    let bytes = pad(msg, msg.len());
    for msg_block in bytes.chunks(64) {
        digest_block_bytes(hash, msg_block);
    }
}

/// Digest whole message, return hex string
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
/// Pad message
pub fn pad(msg: &[u8], msg_length: usize) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.push_all(msg);
    bytes.push(0x01u8);
    //for _ in 0us..((191 - length) % 200) {
    for _ in 0us..((198 - length) % 200) {
        bytes.push(0u8);
    }
    //bytes.write_be_u64(8*length as u64).unwrap();
    bytes.push(0x80u8);
    bytes
}


pub trait Digest {
    fn input(&mut self, input: &[u8]);
    fn result(&mut self, out: &mut [u8]);
    fn reset(&mut self);
    fn output_bits(&self) -> usize;
}


struct Keccak {
    rate: int;
    cap: int;
    
}

pub struct Sha3256 {
    engine: Keccak
}

impl Digest for Sha3_256 {
    fn input(&mut self, input: &[u8]);
    fn result(&mut self, out: &mut [u8]);
    fn reset(&mut self);
    fn output_bits(&self) -> usize;

    fn input_str(&mut self, input: &str) { ... }
    fn result_bytes(&mut self) -> Vec<u8> { ... }
    fn result_str(&mut self) -> String { ... }
}
