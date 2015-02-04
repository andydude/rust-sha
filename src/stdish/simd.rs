//use std::simd::u32x4;

//use std::num::{Int, ToPrimitive, NumCast};
//use std::ops::{Add, Sub, Mul, Div, Rem, Not};
//#[experimental]
//#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
//#[repr(C)]
//#[simd]
//pub struct u32x4(pub u32, pub u32, pub u32, pub u32);

//impl ::std::num::ToPrimitive for u32x4 {
//    fn to_i64(&self) -> Option<i64> {
//        let &u32x4(a, _, _, _) = self;
//        a.to_i64()
//    }
//    fn to_u64(&self) -> Option<u64> {
//        let &u32x4(a, _, _, _) = self;
//        a.to_u64()
//    }
//}
//
//impl ::std::num::NumCast for u32x4 {
//    fn from<T: ::std::num::ToPrimitive>(n: T) -> Option<u32x4> {
//        let u = n.to_u32().unwrap();
//        Some(u32x4(u, u, u, u))
//    }
//}
//
//impl ::std::ops::Add for u32x4 {
//    type Output = u32x4;
//    fn add(self, rhs: u32x4) -> u32x4 {
//        self + rhs
//    }
//}
//
//impl ::std::ops::Sub for u32x4 {
//    type Output = u32x4;
//    fn sub(self, rhs: u32x4) -> u32x4 {
//        self - rhs
//    }
//}
//
//impl ::std::ops::Mul for u32x4 {
//    type Output = u32x4;
//    fn mul(self, rhs: u32x4) -> u32x4 {
//        self * rhs
//    }
//}
//
//impl ::std::ops::Div for u32x4 {
//    type Output = u32x4;
//    fn div(self, rhs: u32x4) -> u32x4 {
//        self / rhs
//    }
//}
//
//impl ::std::ops::Rem for u32x4 {
//    type Output = u32x4;
//    fn rem(self, rhs: u32x4) -> u32x4 {
//        self % rhs
//    }
//}
//
//impl ::std::ops::Not for u32x4 {
//    type Output = u32x4;
//    fn not(self) -> u32x4 {
//        !self
//    }
//}
//
//impl ::std::ops::BitAnd for u32x4 {
//    type Output = u32x4;
//    fn bitand(self, rhs: u32x4) -> u32x4 {
//        self & rhs
//    }
//}
//
//impl ::std::ops::BitOr for u32x4 {
//    type Output = u32x4;
//    fn bitor(self, rhs: u32x4) -> u32x4 {
//        self | rhs
//    }
//}
//
//impl ::std::ops::BitXor for u32x4 {
//    type Output = u32x4;
//    fn bitxor(self, rhs: u32x4) -> u32x4 {
//        self ^ rhs
//    }
//}
//
//impl ::std::ops::Shl<usize> for u32x4 {
//    type Output = u32x4;
//    fn shl(self, rhs: usize) -> u32x4 {
//        let u32x4(a, b, c, d) = self;
//        u32x4(a.shl(rhs),
//              b.shl(rhs),
//              c.shl(rhs),
//              d.shl(rhs))
//    }
//}
//
//impl ::std::ops::Shr<usize> for u32x4 {
//    type Output = u32x4;
//    fn shr(self, rhs: usize) -> u32x4 {
//        let u32x4(a, b, c, d) = self;
//        u32x4(a.shr(rhs),
//              b.shr(rhs),
//              c.shr(rhs),
//              d.shr(rhs))
//    }
//}
//
//impl ::std::cmp::Eq for u32x4 {}
//impl ::std::cmp::Ord for u32x4 {
//    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
//        self.partial_cmp(other).unwrap_or(::std::cmp::Ordering::Less)
//    }
//}
