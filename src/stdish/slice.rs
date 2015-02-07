
//#[inline]
#[stable(feature="cryptoil_api", since="1.0.0")]
pub fn transmute_memory<D: Sized, S: Sized>(dst: &mut D, src: &S) {
    unsafe {
        //let sa = ::std::mem::align_of::<S>();
        //let da = ::std::mem::align_of::<D>();
        //println!("d:{} == s:{}", da, sa);
        let size = ::std::mem::size_of::<S>();
        assert_eq!(::std::mem::size_of::<D>(), size);
        let d = ::std::mem::transmute::<(&mut D, usize), &mut [u8]>((dst, size));
        let s = ::std::mem::transmute::<(&S, usize), &[u8]>((src, size));
        ::std::slice::bytes::copy_memory(d, s);
    }
}

#[cfg(all(test, target_endian = "little"))]
mod tests {
    use serialize::hex::ToHex;
    use std::simd::u32x4;
    use super::transmute_memory;

    #[test]
    fn u32_to_u8() {
        let x: [u32; 4] = [0x04030201, 0x08070605, 0x12111009, 0x16151413];
        let mut y: [u8; 16] = [0u8; 16];
        transmute_memory(&mut y, &x);

        println!("{}", y.as_slice().to_hex());
        
        assert_eq!(y[0], 1u8);
        assert_eq!(y[1], 2u8);
        assert_eq!(y[2], 3u8);
        assert_eq!(y[3], 4u8);
        assert_eq!(y[4], 5u8);
        assert_eq!(y[5], 6u8);
        assert_eq!(y[6], 7u8);
        assert_eq!(y[7], 8u8);
        assert_eq!(y[8], 9u8);
        assert_eq!(y[9], 0x10u8);
        assert_eq!(y[10], 0x11u8);
        assert_eq!(y[11], 0x12u8);
        assert_eq!(y[12], 0x13u8);
        assert_eq!(y[13], 0x14u8);
        assert_eq!(y[14], 0x15u8);
        assert_eq!(y[15], 0x16u8);
    }


    #[test]
    fn u8_to_u32() {
        let x: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16];
        let mut y: [u32; 4] = [0u32; 4];
        transmute_memory(&mut y, &x);

        println!("{}", x.as_slice().to_hex());

        assert_eq!(y[0], 0x04030201u32);
        assert_eq!(y[1], 0x08070605u32);
        assert_eq!(y[2], 0x12111009u32);
        assert_eq!(y[3], 0x16151413u32);
    }


    #[test]
    fn u32x4_to_u8() {
        let x: u32x4 = u32x4(0x04030201u32,
                             0x08070605u32,
                             0x12111009u32,
                             0x16151413u32);
        let mut y: [u8; 16] = [0u8; 16];
        transmute_memory(&mut y, &x);

        println!("{}", y.as_slice().to_hex());
        
        assert_eq!(y[0], 1u8);
        assert_eq!(y[1], 2u8);
        assert_eq!(y[2], 3u8);
        assert_eq!(y[3], 4u8);
        assert_eq!(y[4], 5u8);
        assert_eq!(y[5], 6u8);
        assert_eq!(y[6], 7u8);
        assert_eq!(y[7], 8u8);
        assert_eq!(y[8], 9u8);
        assert_eq!(y[9], 0x10u8);
        assert_eq!(y[10], 0x11u8);
        assert_eq!(y[11], 0x12u8);
        assert_eq!(y[12], 0x13u8);
        assert_eq!(y[13], 0x14u8);
        assert_eq!(y[14], 0x15u8);
        assert_eq!(y[15], 0x16u8);
    }


    #[test]
    fn u8_to_u32x4() {
        let x: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16];
        let mut y: u32x4 = u32x4(0, 0, 0, 0);
        transmute_memory(&mut y, &x);
        let u32x4(a, b, c, d) = y;

        println!("{}", x.as_slice().to_hex());

        assert_eq!(a, 0x04030201u32);
        assert_eq!(b, 0x08070605u32);
        assert_eq!(c, 0x12111009u32);
        assert_eq!(d, 0x16151413u32);
    }
    
}
