use std::simd::i32x4;
use std::simd::i64x2;
use std::simd::u32x4;
use std::simd::u64x2;

#[inline]
pub fn left_i32(x: i32, y: usize) -> i32 {
    (((x as u32) << y) | ((x as u32) >> (32 - y))) as i32
}

#[inline]
pub fn left_i64(x: i64, y: usize) -> i64 {
    (((x as u64) << y) | ((x as u64) >> (64 - y))) as i64
}

#[inline]
pub fn left_i32x4(x: i32x4, yus: usize) -> i32x4 {
    let i32x4(x0, x1, x2, x3) = x;
    let u32x4(z0, z1, z2, z3) = left_u32x4(u32x4(x0 as u32, x1 as u32, x2 as u32, x3 as u32), yus);
    i32x4(z0 as i32, z1 as i32, z2 as i32, z3 as i32)
}

#[inline]
pub fn left_i64x2(x: i64x2, yus: usize) -> i64x2 {
    let i64x2(x0, x1) = x;
    let u64x2(z0, z1) = left_u64x2(u64x2(x0 as u64, x1 as u64), yus);
    i64x2(z0 as i64, z1 as i64)
}

#[inline]
pub fn left_u32(x: u32, y: usize) -> u32 {
    (x << y) | (x >> (32 - y))
}

#[inline]
pub fn left_u64(x: u64, y: usize) -> u64 {
    (x << y) | (x >> (64 - y))
}

#[inline]
pub fn left_u32x4(x: u32x4, yus: usize) -> u32x4 {
    let y: u32 = yus as u32;
    let ny: u32 = (32 - y) as u32;
    (x << u32x4(y, y, y, y)) | (x >> u32x4(ny, ny, ny, ny))
}

#[inline]
pub fn left_u64x2(x: u64x2, yus: usize) -> u64x2 {
    let y: u64 = yus as u64;
    let ny: u64 = (64 - y) as u64;
    (x << u64x2(y, y)) | (x >> u64x2(ny, ny))
}
    
