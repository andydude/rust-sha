
// commutative 3-ary boolean functions (the odd ones are commutative, the even ones are commutative and associative)

#[inline] pub fn sym_1(a: u64, b: u64, c: u64) -> u64 { ary_1(a, b, c) }      // 3, nor
#[inline] pub fn sym_2(a: u64, b: u64, c: u64) -> u64 { ary_22(a, b, c) }     // 3, uni, one, xand
#[inline] pub fn sym_3(a: u64, b: u64, c: u64) -> u64 { ary_23(a, b, c) }     // 3, nmaj, minority, --half
#[inline] pub fn sym_4(a: u64, b: u64, c: u64) -> u64 { ary_104(a, b, c) }    // 3, duo, two
#[inline] pub fn sym_5(a: u64, b: u64, c: u64) -> u64 { ary_105(a, b, c) }    // 3, nxor, nparity, --half
#[inline] pub fn sym_6(a: u64, b: u64, c: u64) -> u64 { ary_126(a, b, c) }    // 3, neqv
#[inline] pub fn sym_7(a: u64, b: u64, c: u64) -> u64 { ary_127(a, b, c) }    // 3, nand
#[inline] pub fn sym_8(a: u64, b: u64, c: u64) -> u64 { ary_128(a, b, c) }    // 3, and
#[inline] pub fn sym_9(a: u64, b: u64, c: u64) -> u64 { ary_129(a, b, c) }    // 3, eqv
#[inline] pub fn sym_10(a: u64, b: u64, c: u64) -> u64 { ary_150(a, b, c) }   // 3, xor, parity, --half =Eqv[Eqv[a, b], c]
#[inline] pub fn sym_11(a: u64, b: u64, c: u64) -> u64 { ary_151(a, b, c) }   // 3, nduo
#[inline] pub fn sym_12(a: u64, b: u64, c: u64) -> u64 { ary_232(a, b, c) }   // 3, maj, majority, --half
#[inline] pub fn sym_13(a: u64, b: u64, c: u64) -> u64 { ary_233(a, b, c) }   // 3, nuni, nxand
#[inline] pub fn sym_14(a: u64, b: u64, c: u64) -> u64 { ary_254(a, b, c) }   // 3, or

// 3-ary boolean functions

// the boolean function number is the same as Mathematica.
// unary and binary functions have been ommitted from this list.
// for more information on a particular boolean function, see also:
// http://www.wolframalpha.com/input/?i=BooleanFunction[232,3]
// where 232 refers to ary_232(a, b, c) in this file.

// there are only 70 3-ary boolean functions which have the property (--half)
// that half of their inputs yield false and the other half yield true.
// these functions numbers are given by [A014312] https://oeis.org/A014312
// 15, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58, 60, 71, 75, 77, 78, 83, 85, 
// 86, 89, 90, 92, 99, 101, 102, 105, 106, 108, 113, 114, 116, 120, 135, 139, 141, 142, 
// 147, 149, 150, 153, 154, 156, 163, 165, 166, 169, 170, 172, 177, 178, 180, 184, 195, 
// 197, 198, 201, 202, 204, 209, 210, 212, 216, 225, 226, 228, 232, 240

#[inline] pub fn ary_1(a: u64, b: u64, c: u64) -> u64 { !(a | b | c) }                 // 3, nor, --c
#[inline] pub fn ary_2(a: u64, b: u64, c: u64) -> u64 { !(a | b | !c) }                // 3, --mostly-false
#[inline] pub fn ary_4(a: u64, b: u64, c: u64) -> u64 { !(a | !b | c) }                // 3, --mostly-false
#[inline] pub fn ary_6(a: u64, b: u64, c: u64) -> u64 { !a & (b ^ c) }                 // 3, --mostly-false
#[inline] pub fn ary_7(a: u64, b: u64, c: u64) -> u64 { !(a | (b & c)) }               // 3, --mostly-false
#[inline] pub fn ary_8(a: u64, b: u64, c: u64) -> u64 { !a & b & c }                   // 3, --mostly-false
#[inline] pub fn ary_9(a: u64, b: u64, c: u64) -> u64 { !(a | (b ^ c)) }               // 3, --mostly-false
#[inline] pub fn ary_11(a: u64, b: u64, c: u64) -> u64 { !a & (!b | c) }               // 3, --mostly-false
#[inline] pub fn ary_13(a: u64, b: u64, c: u64) -> u64 { !a & (b | !c) }               // 3, --mostly-false
#[inline] pub fn ary_14(a: u64, b: u64, c: u64) -> u64 { !a & (b | c) }                // 3, ranor, --mostly-false
#[inline] pub fn ary_16(a: u64, b: u64, c: u64) -> u64 { a & !(b | c) }                // 3, --mostly-false
#[inline] pub fn ary_18(a: u64, b: u64, c: u64) -> u64 { !b & (a ^ c) }                // 3, --mostly-false
#[inline] pub fn ary_19(a: u64, b: u64, c: u64) -> u64 { !(b | (a & c)) }              // 3, --mostly-false
#[inline] pub fn ary_20(a: u64, b: u64, c: u64) -> u64 { !c & (a ^ b) }                // 3, --mostly-false
#[inline] pub fn ary_21(a: u64, b: u64, c: u64) -> u64 { !(c | (a & b)) }              // 3, --mostly-false
#[inline] pub fn ary_22(a: u64, b: u64, c: u64) -> u64 { a^b^c^(a&b&c) }               // 3, uni, one, xand, --mostly-false
#[inline] pub fn ary_23(a: u64, b: u64, c: u64) -> u64 { !ary_232(a, b, c) }      // 3, minority, nmajority, --half
#[inline] pub fn ary_24(a: u64, b: u64, c: u64) -> u64 { ary_129(!a, b, c) }      // 3, --mostly-false, (a ^ b) & (a ^ c)
#[inline] pub fn ary_25(a: u64, b: u64, c: u64) -> u64 { !((a & b) | (b ^ c)) }        // 3, --mostly-false
#[inline] pub fn ary_26(a: u64, b: u64, c: u64) -> u64 { a^c^(a & b)^(a&b&c) }         // 3, --mostly-false
#[inline] pub fn ary_27(a: u64, b: u64, c: u64) -> u64 { (!a & c) | !(b | c) }         // 3, --half
#[inline] pub fn ary_28(a: u64, b: u64, c: u64) -> u64 { a^b^(a & c)^(a&b&c) }         // 3, --mostly-false
#[inline] pub fn ary_29(a: u64, b: u64, c: u64) -> u64 { (!a & b) ^ !(b | c) }         // 3, --half
#[inline] pub fn ary_30(a: u64, b: u64, c: u64) -> u64 { a ^ (b | c) }                 // 3, --half
#[inline] pub fn ary_31(a: u64, b: u64, c: u64) -> u64 { !(a & (b | c)) }              // 3, --mostly-true
#[inline] pub fn ary_32(a: u64, b: u64, c: u64) -> u64 { a | !b | c }                  // 3, --mostly-false
#[inline] pub fn ary_33(a: u64, b: u64, c: u64) -> u64 { !(b | (a ^ c)) }              // 3, --mostly-false
#[inline] pub fn ary_35(a: u64, b: u64, c: u64) -> u64 { !b & (c | !a) }               // 3, --mostly-false
#[inline] pub fn ary_36(a: u64, b: u64, c: u64) -> u64 { ary_129(a, !b, c) }      // 3, --mostly-false, (a ^ b) & (b ^ c)
#[inline] pub fn ary_37(a: u64, b: u64, c: u64) -> u64 { !((a & b) | (a ^ c)) }        // 3, --mostly-false
#[inline] pub fn ary_38(a: u64, b: u64, c: u64) -> u64 { (a&b)^(a&b&c)^b^c }           // 3, --mostly-false
#[inline] pub fn ary_39(a: u64, b: u64, c: u64) -> u64 { !((a&c) ^ (b&c) ^ a) }        // 3, --half
#[inline] pub fn ary_40(a: u64, b: u64, c: u64) -> u64 { c & (a ^ b) }                 // 3, --mostly-false
#[inline] pub fn ary_41(a: u64, b: u64, c: u64) -> u64 { !((a & b) | (a ^ b ^ c)) }    // 3, --mostly-false
#[inline] pub fn ary_42(a: u64, b: u64, c: u64) -> u64 { c & !(a & b) }                // 3, --mostly-false
#[inline] pub fn ary_43(a: u64, b: u64, c: u64) -> u64 { ary_232(!a, !b, c) }     // 3, --half
#[inline] pub fn ary_44(a: u64, b: u64, c: u64) -> u64 { (b | c) & (a ^ b) }           // 3, --mostly-false
#[inline] pub fn ary_45(a: u64, b: u64, c: u64) -> u64 { a ^ (b | !c) }                // 3, --half
#[inline] pub fn ary_46(a: u64, b: u64, c: u64) -> u64 { (!a & b) | (!b & c) }         // 3, --half
#[inline] pub fn ary_47(a: u64, b: u64, c: u64) -> u64 { !a | (!b & c) }               // 3, --mostly-true
#[inline] pub fn ary_49(a: u64, b: u64, c: u64) -> u64 { !b & (a | !c) }               // 3, --mostly-false
#[inline] pub fn ary_50(a: u64, b: u64, c: u64) -> u64 { !b & (a | c) }                // 3, --mostly-false
#[inline] pub fn ary_52(a: u64, b: u64, c: u64) -> u64 { (b&c)^(a&b&c)^a^b }           // 3, --mostly-false
#[inline] pub fn ary_53(a: u64, b: u64, c: u64) -> u64 { !((a & b) ^ (a & c) ^ c) }    // 3, --half
#[inline] pub fn ary_54(a: u64, b: u64, c: u64) -> u64 { b ^ (a | c) }                 // 3, --half
#[inline] pub fn ary_55(a: u64, b: u64, c: u64) -> u64 { !(b & (a | c)) }              // 3, --mostly-true
#[inline] pub fn ary_56(a: u64, b: u64, c: u64) -> u64 { (a | c) & (a ^ b) }           // 3, --mostly-false
#[inline] pub fn ary_57(a: u64, b: u64, c: u64) -> u64 { b ^ (a | !c) }                // 3, MD5_I, --half
//pub fn ary_58(a: u64, b: u64, c: u64) -> u64 {0}     // 3, --half
//pub fn ary_59(a: u64, b: u64, c: u64) -> u64 {0}     // 3, --mostly-
//pub fn ary_60(a: u64, b: u64, c: u64) -> u64 {0}     // 3, --half
//pub fn ary_61(a: u64, b: u64, c: u64) -> u64 {0}     // 3, --mostly-
//pub fn ary_62(a: u64, b: u64, c: u64) -> u64 {0}     // 3, --mostly-
//pub fn ary_63(a: u64, b: u64, c: u64) -> u64 {0}     // 3, --mostly-
//pub fn ary_64(a: u64, b: u64, c: u64) -> u64 {0}     // 3, --mostly-
//pub fn ary_65(a: u64, b: u64, c: u64) -> u64 {0}     // 3, --mostly-
//pub fn ary_66(a: u64, b: u64, c: u64) -> u64 { ary_129(a, b, !c) }      // 3, --mostly-false
//pub fn ary_67(a: u64, b: u64, c: u64) -> u64 {0}                 // 3, --mostly-
//pub fn ary_68(a: u64, b: u64, c: u64) -> u64 {0}                 // 3, --mostly-
//pub fn ary_69(a: u64, b: u64, c: u64) -> u64 {0}                 // 3, --mostly-
//pub fn ary_70(a: u64, b: u64, c: u64) -> u64 {0}                 // 3, --mostly-
//pub fn ary_71(a: u64, b: u64, c: u64) -> u64 {0}                 // 3, --half
//pub fn ary_72(a: u64, b: u64, c: u64) -> u64 {0}                 // 3, --mostly-
//pub fn ary_73(a: u64, b: u64, c: u64) -> u64 {0}                 // 3, --mostly-
//pub fn ary_74(a: u64, b: u64, c: u64) -> u64 {0}                 // 3, --mostly-
//pub fn ary_75(a: u64, b: u64, c: u64) -> u64 {0}                 // 3, --half
//pub fn ary_76(a: u64, b: u64, c: u64) -> u64 {0}                 // 3, --mostly-
//pub fn ary_77(a: u64, b: u64, c: u64) -> u64 { ary_232(!a, b, !c) }         // 3, --half
//pub fn ary_78(a: u64, b: u64, c: u64) -> u64 {0}         // --half
//pub fn ary_79(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_80(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_81(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_82(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_83(a: u64, b: u64, c: u64) -> u64 {0}         // --half
//pub fn ary_84(a: u64, b: u64, c: u64) -> u64 { ((a | b) & !c }   // 3, lanor
//pub fn ary_86(a: u64, b: u64, c: u64) -> u64 {0}         // --half
//pub fn ary_87(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_88(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_89(a: u64, b: u64, c: u64) -> u64 {0}         // --half
//pub fn ary_90(a: u64, b: u64, c: u64) -> u64 {0}         // --half
//pub fn ary_91(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_92(a: u64, b: u64, c: u64) -> u64 {0}         // --half
//pub fn ary_93(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_94(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_95(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_96(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_97(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_98(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_99(a: u64, b: u64, c: u64) -> u64 {0}         // --half
//pub fn ary_100(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_101(a: u64, b: u64, c: u64) -> u64 {0}            // --half
//pub fn ary_102(a: u64, b: u64, c: u64) -> u64 {0}            // --half
//pub fn ary_103(a: u64, b: u64, c: u64) -> u64 {0}
pub fn ary_104(a: u64, b: u64, c: u64) -> u64 { (b | c) & (a ^ (b & c)) }  // 3, duo, two
pub fn ary_105(a: u64, b: u64, c: u64) -> u64 { !(a ^ b ^ c) }             // 3, nxor, nparity, --half
//pub fn ary_106(a: u64, b: u64, c: u64) -> u64 {0}            // --half
//pub fn ary_107(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_108(a: u64, b: u64, c: u64) -> u64 {0}            // --half
//pub fn ary_109(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_110(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_111(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_112(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_113(a: u64, b: u64, c: u64) -> u64 { ary_232(a, !b, !c) }            // 3, --half
//pub fn ary_114(a: u64, b: u64, c: u64) -> u64 {0}            // --half
//pub fn ary_115(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_116(a: u64, b: u64, c: u64) -> u64 {0}            // --half
//pub fn ary_117(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_118(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_119(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_120(a: u64, b: u64, c: u64) -> u64 {0}            // --half
//pub fn ary_121(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_122(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_123(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_124(a: u64, b: u64, c: u64) -> u64 {0}
//pub fn ary_125(a: u64, b: u64, c: u64) -> u64 {0}
pub fn ary_126(a: u64, b: u64, c: u64) -> u64 { !ary_129(a, b, c) }       // 3, neqv
pub fn ary_127(a: u64, b: u64, c: u64) -> u64 { !(a & b & c) }                 // 3, nand

// 50%

#[inline] pub fn ary_128(a: u64, b: u64, c: u64) -> u64 { a & b & c }                  // 3, and                       =And[a, b, c]
#[inline] pub fn ary_129(a: u64, b: u64, c: u64) -> u64 { !((a ^ b) | (a ^ c)) }       // 3, eqv, equivalent,          =Equivalent[a, b, c]
#[inline] pub fn ary_130(a: u64, b: u64, c: u64) -> u64 { !(a ^ b) & c }               // 3, econd, laeqvand           =And[Equivalent[a, b], c]
#[inline] pub fn ary_131(a: u64, b: u64, c: u64) -> u64 { ary_129(a, b, b & c) }  // 3
#[inline] pub fn ary_132(a: u64, b: u64, c: u64) -> u64 { ary_130(a, c, b) }      // 3
#[inline] pub fn ary_133(a: u64, b: u64, c: u64) -> u64 { ary_129(a, c, b & c) }  // 3
#[inline] pub fn ary_134(a: u64, b: u64, c: u64) -> u64 { (b | c) & (a ^ b ^ c) }      // 3, ?
#[inline] pub fn ary_135(a: u64, b: u64, c: u64) -> u64 { !(a ^ (b & c))}              // 3, eboth, raeqvand, --half   =Equivalent[a, And[b, c]]
#[inline] pub fn ary_137(a: u64, b: u64, c: u64) -> u64 { ary_129(a | c, b, c) }  // 3
#[inline] pub fn ary_138(a: u64, b: u64, c: u64) -> u64 { (!a | b) & c }               // 3, icond, laimpand           =And[Implies[a, b], c]
#[inline] pub fn ary_139(a: u64, b: u64, c: u64) -> u64 { !(a | b) ^ (b & c) }         // 3, ?, --half
#[inline] pub fn ary_140(a: u64, b: u64, c: u64) -> u64 { ary_138(a, c, b) }      // 3
#[inline] pub fn ary_141(a: u64, b: u64, c: u64) -> u64 { !((a&c) ^ (b&c) ^ a ^ c) }   // 3, ?, --half
#[inline] pub fn ary_142(a: u64, b: u64, c: u64) -> u64 { ary_232(!a, b, c) }     // 3, --half
#[inline] pub fn ary_143(a: u64, b: u64, c: u64) -> u64 { !a | (b & c) }               // 3, iboth, ranand, raimpand,  =Implies[a, And[b, c]]
#[inline] pub fn ary_144(a: u64, b: u64, c: u64) -> u64 { ary_130(b, c, a) }      // 3, raandeqv, --mostly-false
#[inline] pub fn ary_145(a: u64, b: u64, c: u64) -> u64 { ary_129(a & c, b, c) }  // 3
#[inline] pub fn ary_146(a: u64, b: u64, c: u64) -> u64 { (a | c) & (a ^ b ^ c) }      // 3, ?,        =And[Xor[a, b, c], Implies[b, a]]
#[inline] pub fn ary_147(a: u64, b: u64, c: u64) -> u64 { ary_135(b, c, a) }      // --half
#[inline] pub fn ary_148(a: u64, b: u64, c: u64) -> u64 { (a | b) & (a ^ b ^ c) }      // 3, ?,        =And[Xor[a, b, c], Implies[c, a]]
#[inline] pub fn ary_149(a: u64, b: u64, c: u64) -> u64 { ary_135(c, a, b) }      // 3, laandeqv, --half
#[inline] pub fn ary_150(a: u64, b: u64, c: u64) -> u64 { a ^ b ^ c }                  // 3, xor, parity, MD5_H, SHA1_F1, SHA1_F3, --half, --sym
#[inline] pub fn ary_151(a: u64, b: u64, c: u64) -> u64 { !(a | b) | (a ^ b ^ c) }     // 3, nduo, ntwo
#[inline] pub fn ary_152(a: u64, b: u64, c: u64) -> u64 { (a | c) & !(b ^ c) }         // 3
#[inline] pub fn ary_154(a: u64, b: u64, c: u64) -> u64 { c ^ (a & !b) }               // 3, laimpeqv, --half, (a IMP b) EQV c 
#[inline] pub fn ary_155(a: u64, b: u64, c: u64) -> u64 { (!a&c) | (b&c) | !(b|c) }    // 3
#[inline] pub fn ary_156(a: u64, b: u64, c: u64) -> u64 { b ^ (a & !c) }               // 3, --half
#[inline] pub fn ary_157(a: u64, b: u64, c: u64) -> u64 { (!a&b) | (b&c) | !(b|c) }    // 3
#[inline] pub fn ary_158(a: u64, b: u64, c: u64) -> u64 { (b & c) | (a ^ b ^ c) }      // 3
#[inline] pub fn ary_159(a: u64, b: u64, c: u64) -> u64 { !(a & (b ^ c)) }             // 3, raimpeqv, a IMP (b EQV c)
#[inline] pub fn ary_161(a: u64, b: u64, c: u64) -> u64 { ary_129(a, b | c, c) }  // 3
#[inline] pub fn ary_162(a: u64, b: u64, c: u64) -> u64 { ary_138(b, a, c) }      // 3, laifand((a IF b) AND c), (a | !b) & c
#[inline] pub fn ary_163(a: u64, b: u64, c: u64) -> u64 { (a & c) | !(a | b) }         // 3, --half
#[inline] pub fn ary_164(a: u64, b: u64, c: u64) -> u64 { (b | c) & !(a ^ c) }         // 3, =Equivalent[a, Implies[b, c], c]
#[inline] pub fn ary_166(a: u64, b: u64, c: u64) -> u64 { c ^ (b | !a) }               // 3, laifeqv, --half, =Equivalent[Implies[b, a], c]
#[inline] pub fn ary_167(a: u64, b: u64, c: u64) -> u64 { !((a&c)^(b&c)^(a&b&c)^a) }   // 3
#[inline] pub fn ary_168(a: u64, b: u64, c: u64) -> u64 { (a | b) & c }                // 3, laorand
#[inline] pub fn ary_169(a: u64, b: u64, c: u64) -> u64 { !(c ^ (a | b)) }             // 3, laoreqv, --half, =Equivalent[Or[a, b], c]
#[inline] pub fn ary_171(a: u64, b: u64, c: u64) -> u64 { !(a | b) | c }               // 3, laorimp
#[inline] pub fn ary_172(a: u64, b: u64, c: u64) -> u64 { (a&b) ^ (a&c) ^ b }          // 3, --half
#[inline] pub fn ary_173(a: u64, b: u64, c: u64) -> u64 { !((b&c) ^ (a&b&c) ^ a ^ c) } // 3
#[inline] pub fn ary_174(a: u64, b: u64, c: u64) -> u64 { c | (b & !a) }               // 3, laifimplies, --mostly-true
#[inline] pub fn ary_176(a: u64, b: u64, c: u64) -> u64 { a & (!b | c) }               // 3, raandimp
#[inline] pub fn ary_177(a: u64, b: u64, c: u64) -> u64 { !((a&c) ^ (b&c) ^ b ^ c) }   // 3, --half
#[inline] pub fn ary_178(a: u64, b: u64, c: u64) -> u64 { ary_232(a, !b, c) }     // 3, --half
#[inline] pub fn ary_179(a: u64, b: u64, c: u64) -> u64 { (a & c) | !b }               // 3, =Implies[b, And[a, c]]
#[inline] pub fn ary_180(a: u64, b: u64, c: u64) -> u64 { a ^ (b | !c) }               // 3, raeqvimp, --half, =Equivalent[a, Implies[b, c]]
#[inline] pub fn ary_181(a: u64, b: u64, c: u64) -> u64 { !((a&b)^(a&c)^(a&b&c)^c) }   // 3
#[inline] pub fn ary_182(a: u64, b: u64, c: u64) -> u64 { (a ^ b ^ c) | (a & c)}       // 3, =Or[Xor[a, b, c], And[a, c]]
#[inline] pub fn ary_183(a: u64, b: u64, c: u64) -> u64 { !(b & (a ^ c)) }             // 3, =Implies[b, Equivalent[a, c]]
#[inline] pub fn ary_184(a: u64, b: u64, c: u64) -> u64 { (a&b)^(b&c)^a }              // 3, --half
#[inline] pub fn ary_185(a: u64, b: u64, c: u64) -> u64 { (a|b|!c)&(!b|c) }            // 3
#[inline] pub fn ary_186(a: u64, b: u64, c: u64) -> u64 { !(!a | b) | c }              // 3, laimplies, --mostly-true
#[inline] pub fn ary_188(a: u64, b: u64, c: u64) -> u64 { (a & b & c) ^ a ^ b }        // 3
#[inline] pub fn ary_189(a: u64, b: u64, c: u64) -> u64 { !ary_129(a, b, !c) }    // 3
#[inline] pub fn ary_190(a: u64, b: u64, c: u64) -> u64 { c | (a ^ b) }                // 3
#[inline] pub fn ary_191(a: u64, b: u64, c: u64) -> u64 { !a | !b | c }                // 3, raimplies(a IMP (b IMP c)), laandimp((a AND b) IMP c)
#[inline] pub fn ary_193(a: u64, b: u64, c: u64) -> u64 { (b | !c) & !(a ^ b) }        // 3
#[inline] pub fn ary_194(a: u64, b: u64, c: u64) -> u64 { (b | c) & !(a ^ b) }         // 3
#[inline] pub fn ary_196(a: u64, b: u64, c: u64) -> u64 { b & (a | !c) }               // 3
#[inline] pub fn ary_197(a: u64, b: u64, c: u64) -> u64 { !((a&b)^(a&c)^a^c) }         // 3, --half
#[inline] pub fn ary_198(a: u64, b: u64, c: u64) -> u64 { b ^ (c & !a) }               // 3, --half
#[inline] pub fn ary_199(a: u64, b: u64, c: u64) -> u64 { !((a&b)^(b&c)^(a&b&c)^a) }   // 3
#[inline] pub fn ary_200(a: u64, b: u64, c: u64) -> u64 { (a | c) & b }                // 3
#[inline] pub fn ary_201(a: u64, b: u64, c: u64) -> u64 { !((a & c)^a^b^c) }           // 3, --half
#[inline] pub fn ary_202(a: u64, b: u64, c: u64) -> u64 { c ^ (a & (b ^ c)) }          // 3, MD5_F, SHA1_F0, --half
#[inline] pub fn ary_203(a: u64, b: u64, c: u64) -> u64 { !((b&c)^(a&b&c)^a^b) }       // 3
#[inline] pub fn ary_205(a: u64, b: u64, c: u64) -> u64 { !(a | c) | b }               // 3
#[inline] pub fn ary_206(a: u64, b: u64, c: u64) -> u64 { b | (c & !a) }               // 3
#[inline] pub fn ary_208(a: u64, b: u64, c: u64) -> u64 { a & (b | !c) }               // 3, raandif
#[inline] pub fn ary_209(a: u64, b: u64, c: u64) -> u64 { !((a & b)^(b & c)^b^c) }     // 3, --half
#[inline] pub fn ary_210(a: u64, b: u64, c: u64) -> u64 { a ^ c ^ (b & c) }            // 3, --half
#[inline] pub fn ary_211(a: u64, b: u64, c: u64) -> u64 { (!a | b | c) & (a | !b) }    // 3, --mostly-true
#[inline] pub fn ary_212(a: u64, b: u64, c: u64) -> u64 { ary_232(a, b, !c) }     // 3, --half
#[inline] pub fn ary_213(a: u64, b: u64, c: u64) -> u64 { (a & b) | !c }               // 3, lanand, laandif, --mostly-true
#[inline] pub fn ary_214(a: u64, b: u64, c: u64) -> u64 { (a&b) | (a^b^c) }            // 3
#[inline] pub fn ary_215(a: u64, b: u64, c: u64) -> u64 { !(c & (a ^ b)) }             // 3
#[inline] pub fn ary_216(a: u64, b: u64, c: u64) -> u64 { (a & !c) | (b & c) }         // 3, --half
#[inline] pub fn ary_217(a: u64, b: u64, c: u64) -> u64 { !((a&b)^(a&b&c)^b^c) }       // 3
#[inline] pub fn ary_218(a: u64, b: u64, c: u64) -> u64 { (a&b&c) ^ a ^ c }            // 3
#[inline] pub fn ary_219(a: u64, b: u64, c: u64) -> u64 { !ary_129(a, !b, c) }    // 3
#[inline] pub fn ary_220(a: u64, b: u64, c: u64) -> u64 { b | (a & !c) }               // 3
#[inline] pub fn ary_222(a: u64, b: u64, c: u64) -> u64 { b | (a ^ c) }                // 3, =imp(eqv(a,c),b), --mostly-true
#[inline] pub fn ary_223(a: u64, b: u64, c: u64) -> u64 { b | !(a & c) }               // 3, raimpliesif, --mostly-true
#[inline] pub fn ary_224(a: u64, b: u64, c: u64) -> u64 { a & (b | c) }                // 3, --mostly-false
#[inline] pub fn ary_225(a: u64, b: u64, c: u64) -> u64 { !(a ^ (b | c)) }             // 3, --half
#[inline] pub fn ary_226(a: u64, b: u64, c: u64) -> u64 { (a & b) ^ (b & c) ^ c }      // 3, --half
#[inline] pub fn ary_227(a: u64, b: u64, c: u64) -> u64 { !(a^b^(a & c)^(a&b&c)) }     // 3, --mostly-true
#[inline] pub fn ary_228(a: u64, b: u64, c: u64) -> u64 { b ^ (c & (a ^ b)) }          // 3, MD5_G, --half
#[inline] pub fn ary_229(a: u64, b: u64, c: u64) -> u64 { !(a^c^(a & b)^(a&b&c)) }     // 3, --mostly-true
#[inline] pub fn ary_230(a: u64, b: u64, c: u64) -> u64 { (a&b&c) ^ b ^ c }            // 3, --mostly-true
#[inline] pub fn ary_231(a: u64, b: u64, c: u64) -> u64 { !ary_129(!a, b, c) }    // 3, --mostly-true, !((a ^ b) & (a ^ c))
#[inline] pub fn ary_232(a: u64, b: u64, c: u64) -> u64 { (a & b) ^ (a & c) ^ (b & c) }// 3, majority, SHA1_F2, --half
#[inline] pub fn ary_233(a: u64, b: u64, c: u64) -> u64 { !((a&b&c)^a^b^c) }           // 3, nuni, none, nxand, Not[OneBit[a, b, c]], --mostly-true
#[inline] pub fn ary_234(a: u64, b: u64, c: u64) -> u64 { c | (a & b) }                // 3, --mostly-true
#[inline] pub fn ary_235(a: u64, b: u64, c: u64) -> u64 { c | !(a ^ b) }               // 3, --mostly-true
#[inline] pub fn ary_236(a: u64, b: u64, c: u64) -> u64 { (a & c) | b }                // 3, --mostly-true
#[inline] pub fn ary_237(a: u64, b: u64, c: u64) -> u64 { b | !(a ^ c) }               // 3, --mostly-true
#[inline] pub fn ary_239(a: u64, b: u64, c: u64) -> u64 { !a | b | c }                 // 3, --mostly-true
#[inline] pub fn ary_241(a: u64, b: u64, c: u64) -> u64 { a | !(b | c) }               // 3, --mostly-true
#[inline] pub fn ary_242(a: u64, b: u64, c: u64) -> u64 { a | (!b & c) }               // 3, raif, --mostly-true
#[inline] pub fn ary_244(a: u64, b: u64, c: u64) -> u64 { a | (b & !c) }               // 3, --mostly-true
#[inline] pub fn ary_246(a: u64, b: u64, c: u64) -> u64 { a | (b ^ c) }                // 3, --mostly-true
#[inline] pub fn ary_247(a: u64, b: u64, c: u64) -> u64 { a | !b | !c }                // 3, laif((a IF b) IF c), raifand(a IF (b AND c)), --mostly-true
#[inline] pub fn ary_248(a: u64, b: u64, c: u64) -> u64 { a | (b & c) }                // 3, raorand, --mostly-true
#[inline] pub fn ary_249(a: u64, b: u64, c: u64) -> u64 { a | !(b ^ c) }               // 3, raifxor, raoreqv, --mostly-true
#[inline] pub fn ary_251(a: u64, b: u64, c: u64) -> u64 { a | !b | c }                 // 3, --mostly-true
#[inline] pub fn ary_253(a: u64, b: u64, c: u64) -> u64 { a | b | !c }                 // 3, --mostly-true
#[inline] pub fn ary_254(a: u64, b: u64, c: u64) -> u64 { a | b | c }                  // 3, or, --mostly-true

// N-ary boolean functions that could be defined as
// 3-ary boolean functions where an argument is ignored

//pub fn ary_3(a: u64, b: u64, c: u64) -> u64 { !(a | b) }     // 2
//pub fn ary_5(a: u64, b: u64, c: u64) -> u64 { !(a | c) }     // 2
//pub fn ary_10(a: u64, b: u64, c: u64) -> u64 { !a & c }      // 2
//pub fn ary_12(a: u64, b: u64, c: u64) -> u64 { !a & b }      // 2
//pub fn ary_15(a: u64, b: u64, c: u64) -> u64 { !a }          // 1, --half
//pub fn ary_17(a: u64, b: u64, c: u64) -> u64 { !b & !c }     // 2
//pub fn ary_34(a: u64, b: u64, c: u64) -> u64 { !b | c }      // 2
//pub fn ary_48(a: u64, b: u64, c: u64) -> u64 { a & !b }      // 2
//pub fn ary_51(a: u64, b: u64, c: u64) -> u64 { !b }          // 1, --half
//pub fn ary_85(a: u64, b: u64, c: u64) -> u64 { !c }          // 1, --half
//pub fn ary_136(a: u64, b: u64, c: u64) -> u64 { b & c }      // 2
//pub fn ary_153(a: u64, b: u64, c: u64) -> u64 { !(b ^ c) }   // 2, --half
//pub fn ary_160(a: u64, b: u64, c: u64) -> u64 { a & c }      // 2
//pub fn ary_165(a: u64, b: u64, c: u64) -> u64 { !(a ^ c) }   // 2, --half
//pub fn ary_170(a: u64, b: u64, c: u64) -> u64 { c }          // 1, --half
//pub fn ary_175(a: u64, b: u64, c: u64) -> u64 { !a | c }     // 2
//pub fn ary_187(a: u64, b: u64, c: u64) -> u64 { !b | c }     // 2
//pub fn ary_192(a: u64, b: u64, c: u64) -> u64 { a ^ b }      // 2
//pub fn ary_195(a: u64, b: u64, c: u64) -> u64 { !(a ^ b) }   // 2, --half
//pub fn ary_204(a: u64, b: u64, c: u64) -> u64 { b }          // 1, --half
//pub fn ary_207(a: u64, b: u64, c: u64) -> u64 { !a | b }     // 2
//pub fn ary_221(a: u64, b: u64, c: u64) -> u64 { b | !c }     // 2
//pub fn ary_238(a: u64, b: u64, c: u64) -> u64 { b | c }      // 2
//pub fn ary_240(a: u64, b: u64, c: u64) -> u64 { a }          // 1, --half
//pub fn ary_243(a: u64, b: u64, c: u64) -> u64 { a | !b }     // 3
//pub fn ary_245(a: u64, b: u64, c: u64) -> u64 { a | !c }     // 2
//pub fn ary_250(a: u64, b: u64, c: u64) -> u64 { a | c }      // 2
//pub fn ary_252(a: u64, b: u64, c: u64) -> u64 { a | b }      // 2
