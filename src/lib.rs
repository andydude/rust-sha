#![feature(asm, simd, simd_ffi, link_llvm_intrinsics, slicing_syntax, test, collections, io, core)]

extern crate "rustc-serialize" as serialize;
extern crate test;

pub mod logic;
pub mod stdish;

#[stable(feature = "cryptoil", since = "1.0.0")]
pub mod sha1;
