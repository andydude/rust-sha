#![feature(asm, simd, simd_ffi, link_llvm_intrinsics, slicing_syntax, test, collections, io, core, staged_api)]
//#![staged_api]

extern crate "rustc-serialize" as serialize;
extern crate test;

#[stable(feature = "cryptoi_api", since = "1.0.0")]
pub mod logic;

#[stable(feature = "cryptoi_api", since = "1.0.0")]
pub mod stdish;

#[stable(feature = "cryptoi_api", since = "1.0.0")]
pub mod sha1;
