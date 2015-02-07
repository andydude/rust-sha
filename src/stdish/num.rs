use std::simd::u32x4;
use std::num::Int;


// traits

/// An integer (or vector of integers) that can be set to zero or one.
#[unstable(feature = "default", reason = "std::hash::Hasher is unstable")]
pub trait PartialInt: Sized {
    // required
    fn zero() -> Self;
    fn one() -> Self;
    fn min_value() -> Self;
    fn max_value() -> Self;
}

/// An integer (or vector of integers) that can be rotated.
#[unstable(feature = "default", reason = "std::hash::Hasher is unstable")]
pub trait RotateInt: Sized {
    // required
    fn rotate_left(self, n: usize) -> Self;
    fn rotate_right(self, n: usize) -> Self;
}

/// An integer (or vector of integers) whose bytes can be swapped.
#[unstable(feature = "default", reason = "std::hash::Hasher is unstable")]
pub trait SwapBytesInt: Sized {
    // required
    fn swap_bytes(self) -> Self;

    // provided
    #[inline]
    fn from_be(x: Self) -> Self {
        if cfg!(target_endian = "big") { x } else { x.swap_bytes() }
    }
    #[inline]
    fn from_le(x: Self) -> Self {
        if cfg!(target_endian = "little") { x } else { x.swap_bytes() }
    }
    #[inline]
    fn to_be(self) -> Self {
        if cfg!(target_endian = "big") { self } else { self.swap_bytes() }
    }
    #[inline]
    fn to_le(self) -> Self {
        if cfg!(target_endian = "little") { self } else { self.swap_bytes() }
    }
}

// implementations

impl PartialInt for u32x4 {
    
    #[unstable(feature = "default", reason = "std::hash::Hasher is unstable")]
    #[inline]
    fn zero() -> Self {
        u32x4(0u32, 0u32, 0u32, 0u32)
    }
    
    #[unstable(feature = "default", reason = "std::hash::Hasher is unstable")]
    #[inline]
    fn one() -> Self {
        u32x4(1u32, 1u32, 1u32, 1u32)
    }
    
    #[unstable(feature = "default", reason = "std::hash::Hasher is unstable")]
    #[inline]
    fn min_value() -> Self {
        u32x4(::std::u32::MIN,
              ::std::u32::MIN,
              ::std::u32::MIN,
              ::std::u32::MIN)
    }
    
    #[unstable(feature = "default", reason = "std::hash::Hasher is unstable")]
    fn max_value() -> Self {
        u32x4(::std::u32::MAX,
              ::std::u32::MAX,
              ::std::u32::MAX,
              ::std::u32::MAX)
    }
}

impl RotateInt for u32x4 {
    
    #[unstable(feature = "default", reason = "std::hash::Hasher is unstable")]
    #[inline]
    fn rotate_left(self, n: usize) -> Self {
        let y: u32 = n as u32;
        let ny: u32 = (32 - y) as u32;
        (self << u32x4(y, y, y, y)) | (self >> u32x4(ny, ny, ny, ny))
    }
            
    #[unstable(feature = "default", reason = "std::hash::Hasher is unstable")]
    #[inline]
    fn rotate_right(self, n: usize) -> Self {
        let y: u32 = n as u32;
        let ny: u32 = (32 - y) as u32;
        (self >> u32x4(y, y, y, y)) | (self << u32x4(ny, ny, ny, ny))
    }
}

impl SwapBytesInt for u32x4 {
    
    #[unstable(feature = "default", reason = "std::hash::Hasher is unstable")]
    #[inline]
    fn swap_bytes(self) -> Self {
        let u32x4(a, b, c, d) = self;
        u32x4(a.swap_bytes(),
              b.swap_bytes(),
              c.swap_bytes(),
              d.swap_bytes())
    }
}