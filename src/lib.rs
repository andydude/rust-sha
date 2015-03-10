#![allow(unused_features)]
#![feature(core, hash, io, collections, link_llvm_intrinsics, simd, simd_ffi, staged_api, std_misc, test)]
//#![staged_api]
// collections hash asm core libc

extern crate "rustc-serialize" as serialize;
extern crate test;
extern crate bswap;

#[stable(feature = "default", since = "1.0.0")]
pub mod logic;

#[unstable(feature = "sha_internals", reason = "1.0.0")]
pub mod utils;

#[stable(feature = "default", since = "1.0.0")]
pub mod sha1;

#[stable(feature = "default", since = "1.0.0")]
pub mod sha224;

#[stable(feature = "default", since = "1.0.0")]
pub mod sha256;

#[stable(feature = "default", since = "1.0.0")]
pub mod sha384;

#[stable(feature = "default", since = "1.0.0")]
pub mod sha512;

#[stable(feature = "default", since = "1.0.0")]
pub mod sha512224;

#[stable(feature = "default", since = "1.0.0")]
pub mod sha512256;

#[stable(feature = "default", since = "1.0.0")]
pub mod sha3224;

#[stable(feature = "default", since = "1.0.0")]
pub mod sha3256;

#[stable(feature = "default", since = "1.0.0")]
pub mod sha3384;

#[stable(feature = "default", since = "1.0.0")]
pub mod sha3512;

#[stable(feature = "default", since = "1.0.0")]
pub mod shake128;

#[stable(feature = "default", since = "1.0.0")]
pub mod shake256;

#[stable(feature = "default", since = "1.0.0")]
pub mod keccak;
