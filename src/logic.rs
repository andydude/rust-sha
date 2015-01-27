#![macro_use]

/* Ternary boolean functions
 *
 * the boolean function number is the same as Mathematica.
 * unary and binary functions have been ommitted from this list.
 * for more information on a particular boolean function, see also:
 * http://www.wolframalpha.com/input/?i=BooleanFunction[232,3]
 * where 232 refers to bool3ary_232(a, b, c) in this file.
 *
 * there are 256 possible 3-ary boolean functions, but some of them are boring.
 * there are only 70 3-ary boolean functions which have the property (--half)
 * that half of their inputs yield false and the other half yield true.
 * these functions numbers are given by [A014312] https://oeis.org/A014312
 * 15, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58, 60, 71, 75, 77, 78, 83, 85,
 * 86, 89, 90, 92, 99, 101, 102, 105, 106, 108, 113, 114, 116, 120, 135, 139, 141, 142,
 * 147, 149, 150, 153, 154, 156, 163, 165, 166, 169, 170, 172, 177, 178, 180, 184, 195,
 * 197, 198, 201, 202, 204, 209, 210, 212, 216, 225, 226, 228, 232, 240
 */

macro_rules! bool3ary_1 { ($a:expr, $b:expr, $c:expr) => (!($a | $b | $c)) }                     	// 3, nor
macro_rules! bool3ary_2 { ($a:expr, $b:expr, $c:expr) => (!($a | $b | !$c)) }                    	// 3, --mostly-false
macro_rules! bool3ary_4 { ($a:expr, $b:expr, $c:expr) => (!($a | !$b | $c)) }                    	// 3, --mostly-false
macro_rules! bool3ary_6 { ($a:expr, $b:expr, $c:expr) => (!$a & ($b ^ $c)) }                     	// 3, --mostly-false
macro_rules! bool3ary_7 { ($a:expr, $b:expr, $c:expr) => (!($a | ($b & $c))) }                   	// 3, --mostly-false
macro_rules! bool3ary_8 { ($a:expr, $b:expr, $c:expr) => (!$a & $b & $c) }                       	// 3, --mostly-false
macro_rules! bool3ary_9 { ($a:expr, $b:expr, $c:expr) => (!($a | ($b ^ $c))) }                   	// 3, --mostly-false
macro_rules! bool3ary_11 { ($a:expr, $b:expr, $c:expr) => (!$a & (!$b | $c)) }                   	// 3, --mostly-false
macro_rules! bool3ary_13 { ($a:expr, $b:expr, $c:expr) => (!$a & ($b | !$c)) }                   	// 3, --mostly-false
macro_rules! bool3ary_14 { ($a:expr, $b:expr, $c:expr) => (!$a & ($b | $c)) }                    	// 3, ranor, --mostly-false
macro_rules! bool3ary_16 { ($a:expr, $b:expr, $c:expr) => ($a & !($b | $c)) }                    	// 3, --mostly-false
macro_rules! bool3ary_18 { ($a:expr, $b:expr, $c:expr) => (!$b & ($a ^ $c)) }                    	// 3, --mostly-false
macro_rules! bool3ary_19 { ($a:expr, $b:expr, $c:expr) => (!($b | ($a & $c))) }                  	// 3, --mostly-false
macro_rules! bool3ary_20 { ($a:expr, $b:expr, $c:expr) => (!$c & ($a ^ $b)) }                    	// 3, --mostly-false
macro_rules! bool3ary_21 { ($a:expr, $b:expr, $c:expr) => (!($c | ($a & $b))) }                  	// 3, --mostly-false
macro_rules! bool3ary_22 { ($a:expr, $b:expr, $c:expr) => ($a^$b^$c^($a&$b&$c)) }                	// 3, uni, one, xand, --mostly-false
macro_rules! bool3ary_23 { ($a:expr, $b:expr, $c:expr) => (!bool3ary_232($a, $b, $c)) }          	// 3, minority, nmajority, --half
macro_rules! bool3ary_24 { ($a:expr, $b:expr, $c:expr) => (bool3ary_129(!$a, $b, $c)) }          	// 3, --mostly-false, ($a ^ $b) & ($a ^ $c)
macro_rules! bool3ary_25 { ($a:expr, $b:expr, $c:expr) => (!(($a & $b) | ($b ^ $c))) }           	// 3, --mostly-false
macro_rules! bool3ary_26 { ($a:expr, $b:expr, $c:expr) => ($a^$c^($a & $b)^($a&$b&$c)) }         	// 3, --mostly-false
macro_rules! bool3ary_27 { ($a:expr, $b:expr, $c:expr) => ((!$a & $c) | !($b | $c)) }            	// 3, --half
macro_rules! bool3ary_28 { ($a:expr, $b:expr, $c:expr) => ($a^$b^($a & $c)^($a&$b&$c)) }         	// 3, --mostly-false
macro_rules! bool3ary_29 { ($a:expr, $b:expr, $c:expr) => ((!$a & $b) ^ !($b | $c)) }            	// 3, --half
macro_rules! bool3ary_30 { ($a:expr, $b:expr, $c:expr) => ($a ^ ($b | $c)) }                     	// 3, --half
macro_rules! bool3ary_31 { ($a:expr, $b:expr, $c:expr) => (!($a & ($b | $c))) }                  	// 3, --mostly-true
macro_rules! bool3ary_32 { ($a:expr, $b:expr, $c:expr) => ($a | !$b | $c) }                      	// 3, --mostly-false
macro_rules! bool3ary_33 { ($a:expr, $b:expr, $c:expr) => (!($b | ($a ^ $c))) }                  	// 3, --mostly-false
macro_rules! bool3ary_35 { ($a:expr, $b:expr, $c:expr) => (!$b & ($c | !$a)) }                   	// 3, --mostly-false
macro_rules! bool3ary_36 { ($a:expr, $b:expr, $c:expr) => (bool3ary_129($a, !$b, $c)) }          	// 3, --mostly-false, ($a ^ $b) & ($b ^ $c)
macro_rules! bool3ary_37 { ($a:expr, $b:expr, $c:expr) => (!(($a & $b) | ($a ^ $c))) }           	// 3, --mostly-false
macro_rules! bool3ary_38 { ($a:expr, $b:expr, $c:expr) => (($a&$b)^($a&$b&$c)^$b^$c) }           	// 3, --mostly-false
macro_rules! bool3ary_39 { ($a:expr, $b:expr, $c:expr) => (!(($a&$c) ^ ($b&$c) ^ $a)) }          	// 3, --half
macro_rules! bool3ary_40 { ($a:expr, $b:expr, $c:expr) => ($c & ($a ^ $b)) }                     	// 3, --mostly-false
macro_rules! bool3ary_41 { ($a:expr, $b:expr, $c:expr) => (!(($a & $b) | ($a ^ $b ^ $c))) }      	// 3, --mostly-false
macro_rules! bool3ary_42 { ($a:expr, $b:expr, $c:expr) => ($c & !($a & $b)) }                    	// 3, --mostly-false
macro_rules! bool3ary_43 { ($a:expr, $b:expr, $c:expr) => (bool3ary_232(!$a, !$b, $c)) }         	// 3, --half
macro_rules! bool3ary_44 { ($a:expr, $b:expr, $c:expr) => (($b | $c) & ($a ^ $b)) }              	// 3, --mostly-false
macro_rules! bool3ary_45 { ($a:expr, $b:expr, $c:expr) => ($a ^ ($b | !$c)) }                    	// 3, --half
macro_rules! bool3ary_46 { ($a:expr, $b:expr, $c:expr) => ((!$a & $b) | (!$b & $c)) }            	// 3, --half
macro_rules! bool3ary_47 { ($a:expr, $b:expr, $c:expr) => (!$a | (!$b & $c)) }                   	// 3, --mostly-true
macro_rules! bool3ary_49 { ($a:expr, $b:expr, $c:expr) => (!$b & ($a | !$c)) }                   	// 3, --mostly-false
macro_rules! bool3ary_50 { ($a:expr, $b:expr, $c:expr) => (!$b & ($a | $c)) }                    	// 3, --mostly-false
macro_rules! bool3ary_52 { ($a:expr, $b:expr, $c:expr) => (($b&$c)^($a&$b&$c)^$a^$b) }           	// 3, --mostly-false
macro_rules! bool3ary_53 { ($a:expr, $b:expr, $c:expr) => (!(($a & $b) ^ ($a & $c) ^ $c)) }      	// 3, --half
macro_rules! bool3ary_54 { ($a:expr, $b:expr, $c:expr) => ($b ^ ($a | $c)) }                     	// 3, --half
macro_rules! bool3ary_55 { ($a:expr, $b:expr, $c:expr) => (!($b & ($a | $c))) }                  	// 3, --mostly-true
macro_rules! bool3ary_56 { ($a:expr, $b:expr, $c:expr) => (($a | $c) & ($a ^ $b)) }              	// 3, --mostly-false
macro_rules! bool3ary_57 { ($a:expr, $b:expr, $c:expr) => ($b ^ ($a | !$c)) }                    	// 3, MD5I, --half
macro_rules! bool3ary_58 { ($a:expr, $b:expr, $c:expr) => (($a & !$b) | (!$a & $b)) }            	// 3, --half
macro_rules! bool3ary_59 { ($a:expr, $b:expr, $c:expr) => ((!$a & $c) | !$b) }                   	// 3, --mostly-true
macro_rules! bool3ary_61 { ($a:expr, $b:expr, $c:expr) => (($a&!$b)|(!$a&$b)|(!$b&!$c)) }        	// 3, --mostly-true
macro_rules! bool3ary_62 { ($a:expr, $b:expr, $c:expr) => ((!$a|!$b)&($a|$b|$c)) }               	// 3, --mostly-true
macro_rules! bool3ary_64 { ($a:expr, $b:expr, $c:expr) => ($c^($a&$b)^($a&$c)^($b&$c)) }         	// 3, --mostly-false
macro_rules! bool3ary_65 { ($a:expr, $b:expr, $c:expr) => (!($a^$b^$c^($a&$b)^($b&$c))) }        	// 3, --mostly-false
macro_rules! bool3ary_66 { ($a:expr, $b:expr, $c:expr) => (bool3ary_129($a, $b, !$c)) }          	// 3, --mostly-false
macro_rules! bool3ary_67 { ($a:expr, $b:expr, $c:expr) => (!($a^$b^($a&$b&$c))) }                	// 3, --mostly-false
macro_rules! bool3ary_69 { ($a:expr, $b:expr, $c:expr) => ((!$a | $b) & !$c) }                   	// 3, --mostly-false
macro_rules! bool3ary_70 { ($a:expr, $b:expr, $c:expr) => (($a&$c)^($a&$b&$c)^$b^$c) }           	// 3, --mostly-false
macro_rules! bool3ary_71 { ($a:expr, $b:expr, $c:expr) => ((!$a&!$b)|($b&!$c)) }                 	// 3, --half
macro_rules! bool3ary_72 { ($a:expr, $b:expr, $c:expr) => (($a & $b) ^($b & $c)) }               	// 3, --mostly-false
macro_rules! bool3ary_73 { ($a:expr, $b:expr, $c:expr) => (!(($a&$c)^($a&$b&$c)^$a^$b^$c)) }     	// 3, --mostly-false
macro_rules! bool3ary_74 { ($a:expr, $b:expr, $c:expr) => (($a&$b&!$c)|(!$a&$c)) }               	// 3, --mostly-false
macro_rules! bool3ary_75 { ($a:expr, $b:expr, $c:expr) => (!(($b&$c)^$a^$b)) }                   	// 3, --half
macro_rules! bool3ary_76 { ($a:expr, $b:expr, $c:expr) => ((!$a | !$c) & $b) }                   	// 3, --mostly-false
macro_rules! bool3ary_77 { ($a:expr, $b:expr, $c:expr) => (bool3ary_232(!$a, $b, !$c)) }            // 3, --half
macro_rules! bool3ary_78 { ($a:expr, $b:expr, $c:expr) => (($a&$c)^($b&$c)^$b^$c) }             	// 3, --half
macro_rules! bool3ary_79 { ($a:expr, $b:expr, $c:expr) => (!$a | ($b & !$c)) }						// 3
macro_rules! bool3ary_81 { ($a:expr, $b:expr, $c:expr) => (($a | !$b) & !$c) }						// 3, --mostly-false
macro_rules! bool3ary_82 { ($a:expr, $b:expr, $c:expr) => (($a&!$c)|(!$a&!$b&$c)) }					// 3, --mostly-false
macro_rules! bool3ary_83 { ($a:expr, $b:expr, $c:expr) => (($a&!$c)|(!$a&!$b)) }                	// 3, --half
macro_rules! bool3ary_84 { ($a:expr, $b:expr, $c:expr) => (($a | $b) & !$c) }        				// 3, lanor
macro_rules! bool3ary_86 { ($a:expr, $b:expr, $c:expr) => (($a&$b) ^ $a ^ $b ^ $c) }             	// 3, --half
macro_rules! bool3ary_87 { ($a:expr, $b:expr, $c:expr) => ((!$a & !$b) | !$c) }						// 3, --mostly-true
macro_rules! bool3ary_88 { ($a:expr, $b:expr, $c:expr) => (($a&!$c)|(!$a&$b&$c)) }					// 3, --mostly-false
macro_rules! bool3ary_89 { ($a:expr, $b:expr, $c:expr) => (!(($a & $b) ^ $b ^ $c)) }             	// 3, --half
macro_rules! bool3ary_91 { ($a:expr, $b:expr, $c:expr) => ((!$a|!$c)&($a|!$b|$c)) }					// 3
macro_rules! bool3ary_92 { ($a:expr, $b:expr, $c:expr) => (($a & !$c) | (!$a & $b)) }            	// 3, --half
macro_rules! bool3ary_93 { ($a:expr, $b:expr, $c:expr) => ((!$a & $b) | !$c) }						// 3
macro_rules! bool3ary_94 { ($a:expr, $b:expr, $c:expr) => ((!$a|!$c)&($a|$b|$c)) }					// 3, --mostly-true
macro_rules! bool3ary_96 { ($a:expr, $b:expr, $c:expr) => (($a & $b) ^ ($a & $c)) }					// 3, --mostly-false
macro_rules! bool3ary_97 { ($a:expr, $b:expr, $c:expr) => (!($a^$b^$c^($b&$c)^($a&$b&$c))) }		// 3
macro_rules! bool3ary_98 { ($a:expr, $b:expr, $c:expr) => (($a&$b&!$c)|(!$b&$c)) }					// 3
macro_rules! bool3ary_99 { ($a:expr, $b:expr, $c:expr) => (!(($a & $c) ^ $a ^ $b)) }             	// 3, --half
macro_rules! bool3ary_100 { ($a:expr, $b:expr, $c:expr) => (($a&!$b&$c)|($b&!$c)) }					// 3
macro_rules! bool3ary_101 { ($a:expr, $b:expr, $c:expr) => (!(($a & $b) ^ $a ^ $c)) }            	// 3, --half
macro_rules! bool3ary_103 { ($a:expr, $b:expr, $c:expr) => ((!$a|$b|$c)&(!$b|!$c)) }				// 3
macro_rules! bool3ary_104 { ($a:expr, $b:expr, $c:expr) => (($b | $c) & ($a ^ ($b & $c))) }  		// 3, duo, two
macro_rules! bool3ary_105 { ($a:expr, $b:expr, $c:expr) => (!($a ^ $b ^ $c)) }             			// 3, nxor, nparity, --half
macro_rules! bool3ary_106 { ($a:expr, $b:expr, $c:expr) => (($a & $b) ^ $c) }                   	// 3, --half
macro_rules! bool3ary_107 { ($a:expr, $b:expr, $c:expr) => (!($a^$b^($a&$c)^($b&$c)^($a&$b&$c)))}	// 3
macro_rules! bool3ary_108 { ($a:expr, $b:expr, $c:expr) => (($a & $c) ^ $b) }                    	// 3, --half
macro_rules! bool3ary_109 { ($a:expr, $b:expr, $c:expr) => (!(($a&$b)^($b&$c)^($a&$b&$c)^$a^$c))}	// 3
macro_rules! bool3ary_110 { ($a:expr, $b:expr, $c:expr) => ((!$a|!$b|!$c)&($b|$c)) }				// 3
macro_rules! bool3ary_111 { ($a:expr, $b:expr, $c:expr) => (!(($a&$b)^($a&$c)^$a)) }				// 3
macro_rules! bool3ary_112 { ($a:expr, $b:expr, $c:expr) => ($a & (!$b | !$c)) }						// 3
macro_rules! bool3ary_113 { ($a:expr, $b:expr, $c:expr) => (bool3ary_232($a, !$b, !$c)) }         	// 3, --half
macro_rules! bool3ary_114 { ($a:expr, $b:expr, $c:expr) => (($a&!$c)|(!$b&$c)) }                 	// 3, --half
macro_rules! bool3ary_115 { ($a:expr, $b:expr, $c:expr) => (($a & !$c) | !$b) }						// 3
macro_rules! bool3ary_116 { ($a:expr, $b:expr, $c:expr) => (($a&$b)^($b&$c)^$a^$b) }             	// 3, --half
macro_rules! bool3ary_117 { ($a:expr, $b:expr, $c:expr) => (($a & !$b) | !$c) }						// 3
macro_rules! bool3ary_118 { ($a:expr, $b:expr, $c:expr) => (($a|$b|$c)&(!$b|!$c)) }					// 3
macro_rules! bool3ary_120 { ($a:expr, $b:expr, $c:expr) => (($b & $c) ^ $a) }                    	// 3, --half
macro_rules! bool3ary_121 { ($a:expr, $b:expr, $c:expr) => (!(($a&$b)^($a&$c)^($a&$b&$c)^$b^$c))}	// 3
macro_rules! bool3ary_122 { ($a:expr, $b:expr, $c:expr) => (($a&$c)^($a&$b&$c)^$a^$c) }				// 3
macro_rules! bool3ary_123 { ($a:expr, $b:expr, $c:expr) => (!(($a&$b)^($b&$c)^$b)) }				// 3
macro_rules! bool3ary_124 { ($a:expr, $b:expr, $c:expr) => (($a&$b)^($a&$b&$c)^$a^$b) }				// 3
macro_rules! bool3ary_125 { ($a:expr, $b:expr, $c:expr) => (!(($a&$c)^($b&$c)^$c)) }				// 3
macro_rules! bool3ary_126 { ($a:expr, $b:expr, $c:expr) => (!bool3ary_129($a, $b, $c)) }            // 3, neqv
macro_rules! bool3ary_127 { ($a:expr, $b:expr, $c:expr) => (!($a & $b & $c)) }                   	// 3, nand

// 50%

macro_rules! bool3ary_128 { ($a:expr, $b:expr, $c:expr) => ($a & $b & $c) }                      	// 3, and                       =and[$a, $b, $c]
macro_rules! bool3ary_129 { ($a:expr, $b:expr, $c:expr) => (!(($a ^ $b) | ($a ^ $c))) }          	// 3, eqv, equiv$alent,          =Equiv$alent[$a, $b, $c]
macro_rules! bool3ary_130 { ($a:expr, $b:expr, $c:expr) => (!($a ^ $b) & $c) }                   	// 3, e$cond, laeqvand           =and[Equiv$alent[$a, $b], $c]
macro_rules! bool3ary_131 { ($a:expr, $b:expr, $c:expr) => (bool3ary_129($a, $b, $b & $c)) }     	// 3
macro_rules! bool3ary_132 { ($a:expr, $b:expr, $c:expr) => (bool3ary_130($a, $c, $b)) }          	// 3
macro_rules! bool3ary_133 { ($a:expr, $b:expr, $c:expr) => (bool3ary_129($a, $c, $b & $c)) }     	// 3
macro_rules! bool3ary_134 { ($a:expr, $b:expr, $c:expr) => (($b | $c) & ($a ^ $b ^ $c)) }        	// 3, ?
macro_rules! bool3ary_135 { ($a:expr, $b:expr, $c:expr) => (!($a ^ ($b & $c))) }                 	// 3, e$both, raeqvand, --half   =Equiv$alent[$a, and[$b, $c]]
macro_rules! bool3ary_137 { ($a:expr, $b:expr, $c:expr) => (bool3ary_129($a | $c, $b, $c)) }     	// 3
macro_rules! bool3ary_138 { ($a:expr, $b:expr, $c:expr) => ((!$a | $b) & $c) }                   	// 3, i$cond, laimpand           =and[Implies[$a, $b], $c]
macro_rules! bool3ary_139 { ($a:expr, $b:expr, $c:expr) => (!($a | $b) ^ ($b & $c)) }            	// 3, ?, --half
macro_rules! bool3ary_140 { ($a:expr, $b:expr, $c:expr) => (bool3ary_138($a, $c, $b)) }          	// 3
macro_rules! bool3ary_141 { ($a:expr, $b:expr, $c:expr) => (!(($a&$c) ^ ($b&$c) ^ $a ^ $c)) }    	// 3, ?, --half
macro_rules! bool3ary_142 { ($a:expr, $b:expr, $c:expr) => (bool3ary_232(!$a, $b, $c)) }         	// 3, --half
macro_rules! bool3ary_143 { ($a:expr, $b:expr, $c:expr) => (!$a | ($b & $c)) }                   	// 3, i$both, ranand, raimpand,  =Implies[$a, and[$b, $c]]
macro_rules! bool3ary_144 { ($a:expr, $b:expr, $c:expr) => (bool3ary_130($b, $c, $a)) }          	// 3, raandeqv, --mostly-false
macro_rules! bool3ary_145 { ($a:expr, $b:expr, $c:expr) => (bool3ary_129($a & $c, $b, $c)) }     	// 3
macro_rules! bool3ary_146 { ($a:expr, $b:expr, $c:expr) => (($a | $c) & ($a ^ $b ^ $c)) }        	// 3, ?,        =and[Xor[$a, $b, $c], Implies[$b, $a]]
macro_rules! bool3ary_147 { ($a:expr, $b:expr, $c:expr) => (bool3ary_135($b, $c, $a)) }          	// 3, --half
macro_rules! bool3ary_148 { ($a:expr, $b:expr, $c:expr) => (($a | $b) & ($a ^ $b ^ $c)) }        	// 3, ?,        =and[Xor[$a, $b, $c], Implies[$c, $a]]
macro_rules! bool3ary_149 { ($a:expr, $b:expr, $c:expr) => (bool3ary_135($c, $a, $b)) }          	// 3, laandeqv, --half
macro_rules! bool3ary_150 { ($a:expr, $b:expr, $c:expr) => ($a ^ $b ^ $c) }                      	// 3, xor, parity, MD5H, SHA1F1, SHA1F3, --half, --sym, SHA1P
macro_rules! bool3ary_151 { ($a:expr, $b:expr, $c:expr) => (!($a | $b) | ($a ^ $b ^ $c)) }       	// 3, nduo, ntwo
macro_rules! bool3ary_152 { ($a:expr, $b:expr, $c:expr) => (($a | $c) & !($b ^ $c)) }            	// 3
macro_rules! bool3ary_154 { ($a:expr, $b:expr, $c:expr) => ($c ^ ($a & !$b)) }                   	// 3, laimpeqv, --half, ($a IMP $b) EQV $c
macro_rules! bool3ary_155 { ($a:expr, $b:expr, $c:expr) => ((!$a&$c) | ($b&$c) | !($b|$c)) }     	// 3
macro_rules! bool3ary_156 { ($a:expr, $b:expr, $c:expr) => ($b ^ ($a & !$c)) }                   	// 3, --half
macro_rules! bool3ary_157 { ($a:expr, $b:expr, $c:expr) => ((!$a&$b) | ($b&$c) | !($b|$c)) }     	// 3
macro_rules! bool3ary_158 { ($a:expr, $b:expr, $c:expr) => (($b & $c) | ($a ^ $b ^ $c)) }        	// 3
macro_rules! bool3ary_159 { ($a:expr, $b:expr, $c:expr) => (!($a & ($b ^ $c))) }                 	// 3, raimpeqv, $a IMP ($b EQV $c)
macro_rules! bool3ary_161 { ($a:expr, $b:expr, $c:expr) => (bool3ary_129($a, $b | $c, $c)) }     	// 3
macro_rules! bool3ary_162 { ($a:expr, $b:expr, $c:expr) => (bool3ary_138($b, $a, $c)) }          	// 3, laifand(($a IF $b) AND $c), ($a | !$b) & $c
macro_rules! bool3ary_163 { ($a:expr, $b:expr, $c:expr) => (($a & $c) | !($a | $b)) }            	// 3, --half
macro_rules! bool3ary_164 { ($a:expr, $b:expr, $c:expr) => (($b | $c) & !($a ^ $c)) }            	// 3, =Equiv$alent[$a, Implies[$b, $c], $c]
macro_rules! bool3ary_166 { ($a:expr, $b:expr, $c:expr) => ($c ^ ($b | !$a)) }                   	// 3, laifeqv, --half, =Equiv$alent[Implies[$b, $a], $c]
macro_rules! bool3ary_167 { ($a:expr, $b:expr, $c:expr) => (!(($a&$c)^($b&$c)^($a&$b&$c)^$a)) }  	// 3
macro_rules! bool3ary_168 { ($a:expr, $b:expr, $c:expr) => (($a | $b) & $c) }                    	// 3, laorand
macro_rules! bool3ary_169 { ($a:expr, $b:expr, $c:expr) => (!($c ^ ($a | $b))) }                 	// 3, laoreqv, --half, =Equiv$alent[Or[$a, $b], $c]
macro_rules! bool3ary_171 { ($a:expr, $b:expr, $c:expr) => (!($a | $b) | $c) }                   	// 3, laorimp
macro_rules! bool3ary_172 { ($a:expr, $b:expr, $c:expr) => (($a&$b) ^ ($a&$c) ^ $b) }            	// 3, --half
macro_rules! bool3ary_173 { ($a:expr, $b:expr, $c:expr) => (!(($b&$c) ^ ($a&$b&$c) ^ $a ^ $c)) } 	// 3
macro_rules! bool3ary_174 { ($a:expr, $b:expr, $c:expr) => ($c | ($b & !$a)) }                   	// 3, laifimplies, --mostly-true
macro_rules! bool3ary_176 { ($a:expr, $b:expr, $c:expr) => ($a & (!$b | $c)) }                   	// 3, raandimp
macro_rules! bool3ary_177 { ($a:expr, $b:expr, $c:expr) => (!(($a&$c) ^ ($b&$c) ^ $b ^ $c)) }    	// 3, --half
macro_rules! bool3ary_178 { ($a:expr, $b:expr, $c:expr) => (bool3ary_232($a, !$b, $c)) }         	// 3, --half
macro_rules! bool3ary_179 { ($a:expr, $b:expr, $c:expr) => (($a & $c) | !$b) }                   	// 3, =Implies[$b, and[$a, $c]]
macro_rules! bool3ary_180 { ($a:expr, $b:expr, $c:expr) => ($a ^ ($b | !$c)) }                   	// 3, raeqvimp, --half, =Equiv$alent[$a, Implies[$b, $c]]
macro_rules! bool3ary_181 { ($a:expr, $b:expr, $c:expr) => (!(($a&$b)^($a&$c)^($a&$b&$c)^$c)) }  	// 3
macro_rules! bool3ary_182 { ($a:expr, $b:expr, $c:expr) => (($a ^ $b ^ $c) | ($a & $c)) }        	// 3, =Or[Xor[$a, $b, $c], and[$a, $c]]
macro_rules! bool3ary_183 { ($a:expr, $b:expr, $c:expr) => (!($b & ($a ^ $c))) }                 	// 3, =Implies[$b, Equiv$alent[$a, $c]]
macro_rules! bool3ary_184 { ($a:expr, $b:expr, $c:expr) => (($a&$b)^($b&$c)^$a) }                	// 3, --half
macro_rules! bool3ary_185 { ($a:expr, $b:expr, $c:expr) => (($a|$b|!$c)&(!$b|$c)) }              	// 3
macro_rules! bool3ary_186 { ($a:expr, $b:expr, $c:expr) => (!(!$a | $b) | $c) }                  	// 3, laimplies, --mostly-true
macro_rules! bool3ary_188 { ($a:expr, $b:expr, $c:expr) => (($a & $b & $c) ^ $a ^ $b) }          	// 3
macro_rules! bool3ary_189 { ($a:expr, $b:expr, $c:expr) => (!bool3ary_129($a, $b, !$c)) }           // 3
macro_rules! bool3ary_190 { ($a:expr, $b:expr, $c:expr) => ($c | ($a ^ $b)) }                    	// 3
macro_rules! bool3ary_191 { ($a:expr, $b:expr, $c:expr) => (!$a | !$b | $c) }                    	// 3, raimplies($a IMP ($b IMP $c)), laandimp(($a AND $b) IMP $c)
macro_rules! bool3ary_193 { ($a:expr, $b:expr, $c:expr) => (($b | !$c) & !($a ^ $b)) }           	// 3
macro_rules! bool3ary_194 { ($a:expr, $b:expr, $c:expr) => (($b | $c) & !($a ^ $b)) }            	// 3
macro_rules! bool3ary_196 { ($a:expr, $b:expr, $c:expr) => ($b & ($a | !$c)) }                   	// 3
macro_rules! bool3ary_197 { ($a:expr, $b:expr, $c:expr) => (!(($a&$b)^($a&$c)^$a^$c)) }          	// 3, --half
macro_rules! bool3ary_198 { ($a:expr, $b:expr, $c:expr) => ($b ^ ($c & !$a)) }                   	// 3, --half
macro_rules! bool3ary_199 { ($a:expr, $b:expr, $c:expr) => (!(($a&$b)^($b&$c)^($a&$b&$c)^$a)) }  	// 3
macro_rules! bool3ary_200 { ($a:expr, $b:expr, $c:expr) => (($a | $c) & $b) }                    	// 3
macro_rules! bool3ary_201 { ($a:expr, $b:expr, $c:expr) => (!(($a & $c)^$a^$b^$c)) }             	// 3, --half
macro_rules! bool3ary_202 { ($a:expr, $b:expr, $c:expr) => ($c ^ ($a & ($b ^ $c))) }             	// 3, MD5F, SHA1F0, --half, SHA1C
macro_rules! bool3ary_203 { ($a:expr, $b:expr, $c:expr) => (!(($b&$c)^($a&$b&$c)^$a^$b)) }       	// 3
macro_rules! bool3ary_205 { ($a:expr, $b:expr, $c:expr) => (!($a | $c) | $b) }                   	// 3
macro_rules! bool3ary_206 { ($a:expr, $b:expr, $c:expr) => ($b | ($c & !$a)) }                   	// 3
macro_rules! bool3ary_208 { ($a:expr, $b:expr, $c:expr) => ($a & ($b | !$c)) }                   	// 3, raandif
macro_rules! bool3ary_209 { ($a:expr, $b:expr, $c:expr) => (!(($a & $b)^($b & $c)^$b^$c)) }      	// 3, --half
macro_rules! bool3ary_210 { ($a:expr, $b:expr, $c:expr) => ($a ^ $c ^ ($b & $c)) }               	// 3, --half
macro_rules! bool3ary_211 { ($a:expr, $b:expr, $c:expr) => ((!$a | $b | $c) & ($a | !$b)) }      	// 3, --mostly-true
macro_rules! bool3ary_212 { ($a:expr, $b:expr, $c:expr) => (bool3ary_232($a, $b, !$c)) }         	// 3, --half
macro_rules! bool3ary_213 { ($a:expr, $b:expr, $c:expr) => (($a & $b) | !$c) }                   	// 3, lanand, laandif, --mostly-true
macro_rules! bool3ary_214 { ($a:expr, $b:expr, $c:expr) => (($a&$b) | ($a^$b^$c)) }              	// 3
macro_rules! bool3ary_215 { ($a:expr, $b:expr, $c:expr) => (!($c & ($a ^ $b))) }                 	// 3
macro_rules! bool3ary_216 { ($a:expr, $b:expr, $c:expr) => (($a & !$c) | ($b & $c)) }            	// 3, --half
macro_rules! bool3ary_217 { ($a:expr, $b:expr, $c:expr) => (!(($a&$b)^($a&$b&$c)^$b^$c)) }       	// 3
macro_rules! bool3ary_218 { ($a:expr, $b:expr, $c:expr) => (($a&$b&$c) ^ $a ^ $c) }              	// 3
macro_rules! bool3ary_219 { ($a:expr, $b:expr, $c:expr) => (!bool3ary_129($a, !$b, $c)) }        	// 3
macro_rules! bool3ary_220 { ($a:expr, $b:expr, $c:expr) => ($b | ($a & !$c)) }                   	// 3
macro_rules! bool3ary_222 { ($a:expr, $b:expr, $c:expr) => ($b | ($a ^ $c)) }                    	// 3, =imp(eqv($a,$c),$b), --mostly-true
macro_rules! bool3ary_223 { ($a:expr, $b:expr, $c:expr) => ($b | !($a & $c)) }                   	// 3, raimpliesif, --mostly-true
macro_rules! bool3ary_224 { ($a:expr, $b:expr, $c:expr) => ($a & ($b | $c)) }                    	// 3, --mostly-false
macro_rules! bool3ary_225 { ($a:expr, $b:expr, $c:expr) => (!($a ^ ($b | $c))) }                 	// 3, --half
macro_rules! bool3ary_226 { ($a:expr, $b:expr, $c:expr) => (($a & $b) ^ ($b & $c) ^ $c) }        	// 3, --half
macro_rules! bool3ary_227 { ($a:expr, $b:expr, $c:expr) => (!($a^$b^($a & $c)^($a&$b&$c))) }     	// 3, --mostly-true
macro_rules! bool3ary_228 { ($a:expr, $b:expr, $c:expr) => ($b ^ ($c & ($a ^ $b))) }             	// 3, MD5G, --half
macro_rules! bool3ary_229 { ($a:expr, $b:expr, $c:expr) => (!($a^$c^($a & $b)^($a&$b&$c))) }     	// 3, --mostly-true
macro_rules! bool3ary_230 { ($a:expr, $b:expr, $c:expr) => (($a&$b&$c) ^ $b ^ $c) }              	// 3, --mostly-true
macro_rules! bool3ary_231 { ($a:expr, $b:expr, $c:expr) => (!bool3ary_129(!$a, $b, $c)) }        	// 3, --mostly-true, !(($a ^ $b) & ($a ^ $c))
macro_rules! bool3ary_232 { ($a:expr, $b:expr, $c:expr) => (($a & $b) ^ ($a & $c) ^ ($b & $c)) } 	// 3, majority, SHA1F2, --half, SHA1M
macro_rules! bool3ary_233 { ($a:expr, $b:expr, $c:expr) => (!(($a&$b&$c)^$a^$b^$c)) }            	// 3, nuni, none, nxand, --mostly-true
macro_rules! bool3ary_234 { ($a:expr, $b:expr, $c:expr) => ($c | ($a & $b)) }                    	// 3, --mostly-true
macro_rules! bool3ary_235 { ($a:expr, $b:expr, $c:expr) => ($c | !($a ^ $b)) }                   	// 3, --mostly-true
macro_rules! bool3ary_236 { ($a:expr, $b:expr, $c:expr) => (($a & $c) | $b) }                    	// 3, --mostly-true
macro_rules! bool3ary_237 { ($a:expr, $b:expr, $c:expr) => ($b | !($a ^ $c)) }                   	// 3, --mostly-true
macro_rules! bool3ary_239 { ($a:expr, $b:expr, $c:expr) => (!$a | $b | $c )}                     	// 3, --mostly-true
macro_rules! bool3ary_241 { ($a:expr, $b:expr, $c:expr) => ($a | !($b | $c)) }                   	// 3, --mostly-true
macro_rules! bool3ary_242 { ($a:expr, $b:expr, $c:expr) => ($a | (!$b & $c)) }                   	// 3, raif, --mostly-true
macro_rules! bool3ary_244 { ($a:expr, $b:expr, $c:expr) => ($a | ($b & !$c)) }                   	// 3, --mostly-true
macro_rules! bool3ary_246 { ($a:expr, $b:expr, $c:expr) => ($a | ($b ^ $c)) }                    	// 3, --mostly-true
macro_rules! bool3ary_247 { ($a:expr, $b:expr, $c:expr) => ($a | !$b | !$c) }                    	// 3, --mostly-true
macro_rules! bool3ary_248 { ($a:expr, $b:expr, $c:expr) => ($a | ($b & $c)) }                    	// 3, raorand, --mostly-true
macro_rules! bool3ary_249 { ($a:expr, $b:expr, $c:expr) => ($a | !($b ^ $c)) }                   	// 3, raifxor, raoreqv, --mostly-true
macro_rules! bool3ary_251 { ($a:expr, $b:expr, $c:expr) => ($a | !$b | $c) }                     	// 3, --mostly-true
macro_rules! bool3ary_253 { ($a:expr, $b:expr, $c:expr) => ($a | $b | !$c) }                     	// 3, --mostly-true
macro_rules! bool3ary_254 { ($a:expr, $b:expr, $c:expr) => ($a | $b | $c) }                      	// 3, or, --mostly-true

// commutative 3-ary boolean functions (the odd ones are commutative, the even ones are commutative and associative)

macro_rules! comm3ary_1 { ($a:expr, $b:expr, $c:expr) =>  (bool3ary_1!($a, $b, $c)) }  	// 3, nor
macro_rules! comm3ary_2 { ($a:expr, $b:expr, $c:expr) =>  (bool3ary_22!($a, $b, $c)) } 	// 3, one, uni, xand
macro_rules! comm3ary_3 { ($a:expr, $b:expr, $c:expr) =>  (bool3ary_23!($a, $b, $c)) } 	// 3, nmaj, minority, --half
macro_rules! comm3ary_4 { ($a:expr, $b:expr, $c:expr) =>  (bool3ary_104!($a, $b, $c)) }	// 3, two, duo
macro_rules! comm3ary_5 { ($a:expr, $b:expr, $c:expr) =>  (bool3ary_105!($a, $b, $c)) }	// 3, nxor, nparity, --half
macro_rules! comm3ary_6 { ($a:expr, $b:expr, $c:expr) =>  (bool3ary_126!($a, $b, $c)) }	// 3, neqv
macro_rules! comm3ary_7 { ($a:expr, $b:expr, $c:expr) =>  (bool3ary_127!($a, $b, $c)) }	// 3, nand
macro_rules! comm3ary_8 { ($a:expr, $b:expr, $c:expr) =>  (bool3ary_128!($a, $b, $c)) }	// 3, and
macro_rules! comm3ary_9 { ($a:expr, $b:expr, $c:expr) =>  (bool3ary_129!($a, $b, $c)) }	// 3, eqv
macro_rules! comm3ary_10 { ($a:expr, $b:expr, $c:expr) => (bool3ary_150!($a, $b, $c)) }	// 3, xor, parity, --half
macro_rules! comm3ary_11 { ($a:expr, $b:expr, $c:expr) => (bool3ary_151!($a, $b, $c)) }	// 3, ntwo
macro_rules! comm3ary_12 { ($a:expr, $b:expr, $c:expr) => (bool3ary_232!($a, $b, $c)) }	// 3, maj, majority, --half
macro_rules! comm3ary_13 { ($a:expr, $b:expr, $c:expr) => (bool3ary_233!($a, $b, $c)) }	// 3, none
macro_rules! comm3ary_14 { ($a:expr, $b:expr, $c:expr) => (bool3ary_254!($a, $b, $c)) }	// 3, or

// 3-ary reducible $boole$an fun$ctions that can be expressed as binary or unary:

//bool3ary_3     // 2
//bool3ary_5     // 2
//bool3ary_10    // 2
//bool3ary_12    // 2
//bool3ary_15    // 1, --half
//bool3ary_17    // 2
//bool3ary_34    // 2
//bool3ary_48    // 2
//bool3ary_51    // 1, --half
//bool3ary_60    // 2, --half
//bool3ary_63    //
//bool3ary_68    //
//bool3ary_80    //
//bool3ary_85    // 1, --half
//bool3ary_90    // --half
//bool3ary_95    //
//bool3ary_102   // --half
//bool3ary_119   //
//bool3ary_136   // 2
//bool3ary_153   // 2, --half
//bool3ary_160   // 2
//bool3ary_165   // 2, --half
//bool3ary_170   // 1, --half
//bool3ary_175   // 2
//bool3ary_187   // 2
//bool3ary_192   // 2
//bool3ary_195   // 2, --half
//bool3ary_204   // 1, --half
//bool3ary_207   // 2
//bool3ary_221   // 2
//bool3ary_238   // 2
//bool3ary_240   // 1, --half
//bool3ary_243   // 3
//bool3ary_245   // 2
//bool3ary_250   // 2
//bool3ary_252   // 2
