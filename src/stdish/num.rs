use std::simd::u32x4;

/// An integer (or vector of integers) that can be set to zero or one.
pub trait PartialInt: Sized {
    // required
    fn zero() -> Self;
    fn one() -> Self;
    fn min_value() -> Self;
    fn max_value() -> Self;
}

impl PartialInt for u32 {
    #[inline]
    fn zero() -> Self {
        0u32
    }
    #[inline]
    fn one() -> Self {
        1u32
    }
    #[inline]
    fn min_value() -> Self {
        ::std::u32::MIN
    }
    #[inline]
    fn max_value() -> Self {
        ::std::u32::MAX
    }
}

impl PartialInt for u32x4 {
    fn zero() -> Self {
        u32x4(0u32, 0u32, 0u32, 0u32)
    }
    fn one() -> Self {
        u32x4(1u32, 1u32, 1u32, 1u32)
    }
    fn min_value() -> Self {
        u32x4(::std::u32::MIN,
              ::std::u32::MIN,
              ::std::u32::MIN,
              ::std::u32::MIN)
    }
    fn max_value() -> Self {
        u32x4(::std::u32::MAX,
              ::std::u32::MAX,
              ::std::u32::MAX,
              ::std::u32::MAX)
    }
}

/// An integer (or vector of integers) that can be rotated.
pub trait RotateInt: Sized {
    #[inline]
    fn rotate_left(self, n: usize) -> Self;
    #[inline]
    fn rotate_right(self, n: usize) -> Self;
}

impl RotateInt for u32 {
    fn rotate_left(self, n: usize) -> Self {
         (self << n) | (self >> (32 - n))
    }
    fn rotate_right(self, n: usize) -> Self {
         (self >> n) | (self << (32 - n))
    }
}

impl RotateInt for u32x4 {
    fn rotate_left(self, n: usize) -> Self {
        let y: u32 = n as u32;
        let ny: u32 = (32 - y) as u32;
        (self << u32x4(y, y, y, y)) | (self >> u32x4(ny, ny, ny, ny))
    }
            
    fn rotate_right(self, n: usize) -> Self {
        let y: u32 = n as u32;
        let ny: u32 = (32 - y) as u32;
        (self >> u32x4(y, y, y, y)) | (self << u32x4(ny, ny, ny, ny))
    }
}

/// An integer (or vector of integers) whose bytes can be swapped.
pub trait SwapBytesInt: Sized {
    fn swap_bytes(self) -> Self;

    // optional
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

impl SwapBytesInt for u32 {
    fn swap_bytes(self) -> Self {
        unsafe {
            ::std::intrinsics::bswap32(self)
        }
    }
}

impl SwapBytesInt for u32x4 {
    fn swap_bytes(self) -> Self {
        let u32x4(a, b, c, d) = self;
        u32x4(a.swap_bytes(),
              b.swap_bytes(),
              c.swap_bytes(),
              d.swap_bytes())
    }
}
