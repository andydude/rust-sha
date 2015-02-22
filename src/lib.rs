#![feature(asm, collections, core, hash, io, libc, link_llvm_intrinsics, simd, simd_ffi, staged_api, test)]
//#![staged_api]

extern crate "rustc-serialize" as serialize;
extern crate endian;
extern crate libc;
extern crate test;

#[stable(feature = "default", since = "1.0.0")]
pub mod logic;

//#[stable(feature = "default", since = "1.0.0")]
//pub mod stdish;

//#[unstable(feature = "cryptoil_internals", reason = "1.0.0")]
//pub mod intrinsics;

#[unstable(feature = "cryptoil_internals", reason = "1.0.0")]
pub mod utils;

//#[stable(feature = "default", since = "1.0.0")]
//pub mod sha1;

#[stable(feature = "default", since = "1.0.0")]
pub mod sha2;
