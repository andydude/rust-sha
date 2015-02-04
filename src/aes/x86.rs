use std::simd::u32x4;

extern {

    #[link_name = "llvm.x86.aesni.aesdec"]
    pub fn aesdec(a: u32x4, b: u32x4) -> u32x4;

    #[link_name = "llvm.x86.aesni.aesdeclast"]
    pub fn aesdeclast(a: u32x4, b: u32x4) -> u32x4;

    #[link_name = "llvm.x86.aesni.aesenc"]
    pub fn aesenc(a: u32x4, b: u32x4) -> u32x4;

    #[link_name = "llvm.x86.aesni.aesenclast"]
    pub fn aesenclast(a: u32x4, b: u32x4) -> u32x4;

    #[link_name = "llvm.x86.aesni.aesimc"]
    pub fn aesimc(a: u32x4, b: u32x4) -> u32x4;
    
    #[link_name = "llvm.x86.aesni.aeskeygenassist"]
    pub fn aeskeygenassist(a: u32x4, b: u32x4) -> u32x4;
    
    #[link_name = "llvm.x86.pclmulqdq"]
    pub fn pclmulqdq(a: u32x4, b: u32x4) -> u32x4;

}

/// Checks CPUID.1.ECX[25]
pub fn has_aes() -> bool {
    let mut c: u32;
    
    unsafe {
        asm!("mov $$1, %eax
              cpuid
              mov %ecx, $0"
             : "=r"(c)
             :: "eax", "ebx", "ecx", "edx")
    }

    ((c >> 25) & 1) != 0
}
