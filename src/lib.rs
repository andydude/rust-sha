#![allow(unstable)]
#![feature(asm, simd, simd_ffi, link_llvm_intrinsics, slicing_syntax)]

extern crate "rustc-serialize" as serialize;
extern crate test;

pub mod logic;
pub mod rotate;
pub mod sha1;
