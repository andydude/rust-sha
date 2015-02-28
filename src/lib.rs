#![feature(core, hash, io, collections, link_llvm_intrinsics, simd, simd_ffi, staged_api, std_misc, test)]
//#![staged_api]
// collections hash asm core libc

extern crate "rustc-serialize" as serialize;
extern crate endian;
//extern crate libc;
extern crate test;
extern crate bswap;

#[stable(feature = "default", since = "1.0.0")]
pub mod logic;

//#[stable(feature = "default", since = "1.0.0")]
//pub mod stdish;

//#[unstable(feature = "cryptoil_internals", reason = "1.0.0")]
//pub mod intrinsics;

#[unstable(feature = "cryptoil_internals", reason = "1.0.0")]
pub mod utils;

#[stable(feature = "default", since = "1.0.0")]
pub mod sha;
