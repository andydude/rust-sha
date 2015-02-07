#![cfg(test)]
use super::Sha1;
use stdish::io::Hasher;
use test::Bencher;

/// Digest whole message, return hex string
#[unstable(feature = "cryptoil_internal", reason = "will be trait method")]
pub fn hex_digest(msg: &str) -> String {
    Sha1::new().hex_digest(msg)
}


#[test]
fn test_empty_string() {
    assert_eq!("da39a3ee5e6b4b0d3255bfef95601890afd80709", hex_digest("").as_slice());
}

#[test]
fn test_hello_world() {
    assert_eq!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed", hex_digest("hello world").as_slice());
    assert_eq!("430ce34d020724ed75a196dfc2ad67c77772d169", hex_digest("hello world!").as_slice());
    assert_eq!("22c219648f00c61e5b3b1bd81ffa8e7767e2e3c5", hex_digest("hello World").as_slice());
    assert_eq!("788245b4dad73c1e5a630c126c484c7a2464f280", hex_digest("hello World!").as_slice());
    assert_eq!("7b502c3a1f48c8609ae212cdfb639dee39673f5e", hex_digest("Hello world").as_slice());
    assert_eq!("d3486ae9136e7856bc42212385ea797094475802", hex_digest("Hello world!").as_slice());
    assert_eq!("0a4d55a8d778e5022fab701977c5d840bbc486d0", hex_digest("Hello World").as_slice());
    assert_eq!("2ef7bde608ce5404e97d5f042f95f89f1c232871", hex_digest("Hello World!").as_slice());
    assert_eq!("b7e23ec29af22b0b4e41da31e868d57226121c84", hex_digest("hello, world").as_slice());
    assert_eq!("1f09d30c707d53f3d16c530dd73d70a6ce7596a9", hex_digest("hello, world!").as_slice());
    assert_eq!("ca3c58516ddef44b25693df5a915206e1bd094da", hex_digest("hello, World").as_slice());
    assert_eq!("dd0588c172986c32636ffdd8cc690de7b41bf253", hex_digest("hello, World!").as_slice());
    assert_eq!("e02aa1b106d5c7c6a98def2b13005d5b84fd8dc8", hex_digest("Hello, world").as_slice());
    assert_eq!("943a702d06f34599aee1f8da8ef9f7296031d699", hex_digest("Hello, world!").as_slice());
    assert_eq!("907d14fb3af2b0d4f18c2d46abe8aedce17367bd", hex_digest("Hello, World").as_slice());
    assert_eq!("0a0a9f2a6772942557ab5355d76af442f8f65e01", hex_digest("Hello, World!").as_slice());
}

#[test]
fn test_multi_block() {
    let s = "GNU LESSER GENERAL PUBLIC LICENSE Version 3, 29 June 2007 Copyright (C) 2007 Free Software Foundation, Inc. <http://fsf.org/>";
    assert_eq!("a31e8cb8a139d146a0070fa13795d6766acaccd4", Sha1::new().hex_digest(s).as_slice());
}


#[bench]
fn bench_hello_world(b: & mut Bencher) {
    let s = "hello world";

    b.iter(|| Sha1::new().hex_digest(s));
}

#[bench]
fn bench_multi_block(b: & mut Bencher) {
    let s = "GNU LESSER GENERAL PUBLIC LICENSE Version 3, 29 June 2007 Copyright (C) 2007 Free Software Foundation, Inc. <http://fsf.org/>";

    b.iter(|| Sha1::new().hex_digest(s));
}


#[bench]
pub fn padding(bh: & mut Bencher) {
    use super::padding;
    let mut bytes = vec![1u8; 25];
    bh.iter( || {
            padding(&mut bytes, 25u64);
        });
    bh.bytes = 64 as u64;
}

#[bench]
pub fn block_64(bh: & mut Bencher) {
    use super::digest_block_bytes;
    let mut result = [0u8; 20];
    let bytes = [1u8; 64];
    bh.iter( || {
            digest_block_bytes(&mut result, &bytes);
        });
    bh.bytes = bytes.len() as u64;
}

#[bench]
pub fn sha1_10(bh: & mut Bencher) {
    let mut sh = Sha1::new();
    let mut result = [0u8; 20];
    let bytes = [1u8; 10];
    bh.iter( || {
            sh.digest(&bytes, &mut result);
        });
    bh.bytes = bytes.len() as u64;
}
#[bench]
pub fn sha1_1k(bh: & mut Bencher) {
    let mut sh = Sha1::new();
    let mut result = [0u8; 20];
    let bytes = [1u8; 1024];
    bh.iter( || {
            sh.digest(&bytes, &mut result);
        });
    bh.bytes = bytes.len() as u64;
}
#[bench]
pub fn sha1_64k(bh: & mut Bencher) {
    let mut sh = Sha1::new();
    let bytes = [1u8; 65536];
    let mut result = [0u8; 20];
    bh.iter( || {
            sh.digest(&bytes, &mut result);
        });
    bh.bytes = bytes.len() as u64;
}
