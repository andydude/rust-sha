
/// TODO
pub mod consts {

    /// TODO
    pub const K: [u64; 24] = [
        0x0000000000000001, 0x0000000000008082,
        0x800000000000808a, 0x8000000080008000,
        0x000000000000808b, 0x0000000080000001,
        0x8000000080008081, 0x8000000000008009,
        0x000000000000008a, 0x0000000000000088,
        0x0000000080008009, 0x000000008000000a,
        0x000000008000808b, 0x800000000000008b,
        0x8000000000008089, 0x8000000000008003,
        0x8000000000008002, 0x8000000000000080,
        0x000000000000800a, 0x800000008000000a,
        0x8000000080008081, 0x8000000000008080,
        0x0000000080000001, 0x8000000080008008,
    ];
}

pub mod ops {
    use bswap::leu64;

    macro_rules! dump_state {
        ($state:expr) => {
            {
                let mut state3 = [0u8; 200];
                leu64::encode_slice(&mut state3[..], $state);
                for z in 0..12 {
                    for byte in 0..16 {
                        print!(" {:02x}", state3[16*z + byte]);
                    }
                    println!("");
                }
                for byte in 0..8 {
                    print!(" {:02x}", state3[16*12 + byte]);
                }
                println!("");
            }
        }
    }
    macro_rules! rotate_left {
        ($a:expr, $b:expr) => (($a << $b) ^ ($a >> (64 - $b)))
    }
    macro_rules! rotate_right {
        ($a:expr, $b:expr) => (($a >> $b) ^ ($a << (64 - $b)))
    }
    macro_rules! digest_round {
        ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident,
         $f:ident, $g:ident, $h:ident, $i:ident, $j:ident,
         $k:ident, $m:ident, $n:ident, $o:ident, $p:ident,
         $q:ident, $r:ident, $s:ident, $t:ident, $u:ident,
         $v:ident, $w:ident, $x:ident, $y:ident, $z:ident, $ir:ident, $ty:ident) => {
            {
                use super::consts::K;
                
                // temporary variables
                let tmp: $ty;
                let tca: $ty;
                let tcb: $ty;
                let tcc: $ty;
                let tcd: $ty;
                let tce: $ty;
                let mut tda: $ty;
                let mut tdb: $ty;
                let mut tdc: $ty;
                let mut tdd: $ty;
                let mut tde: $ty;
                
                // Theta step
                tca = $a ^ $f ^ $k ^ $q ^ $v;
                tcb = $b ^ $g ^ $m ^ $r ^ $w;
                tcc = $c ^ $h ^ $n ^ $s ^ $x;
                tcd = $d ^ $i ^ $o ^ $t ^ $y;
                tce = $e ^ $j ^ $p ^ $u ^ $z;
                tda = tce ^ rotate_left!(tcb, 1);
                tdb = tca ^ rotate_left!(tcc, 1);
                tdc = tcb ^ rotate_left!(tcd, 1);
                tdd = tcc ^ rotate_left!(tce, 1);
                tde = tcd ^ rotate_left!(tca, 1);
                $a ^= tda;
                $b ^= tdb;
                $c ^= tdc;
                $d ^= tdd;
                $e ^= tde;
                $f ^= tda;
                $g ^= tdb;
                $h ^= tdc;
                $i ^= tdd;
                $j ^= tde;
                $k ^= tda;
                $m ^= tdb;
                $n ^= tdc;
                $o ^= tdd;
                $p ^= tde;
                $q ^= tda;
                $r ^= tdb;
                $s ^= tdc;
                $t ^= tdd;
                $u ^= tde;
                $v ^= tda;
                $w ^= tdb;
                $x ^= tdc;
                $y ^= tdd;
                $z ^= tde;

                //{
                //    let state = [$a, $b, $c, $d, $e,
                //                 $f, $g, $h, $i, $j,
                //                 $k, $m, $n, $o, $p,
                //                 $q, $r, $s, $t, $u,
                //                 $v, $w, $x, $y, $z];
                //    println!("");
                //    println!("Round #{}", $ir);
                //    println!("After Theta");
                //    dump_state!(&state[..]);
                //}

                // Rho and Pi steps
                tmp = rotate_left!($b, 1);
                $b = rotate_left!($g, 44);
                $g = rotate_left!($j, 20);
                $j = rotate_left!($x, 61);
                $x = rotate_left!($p, 39);
                $p = rotate_left!($v, 18);
                $v = rotate_left!($c, 62);
                $c = rotate_left!($n, 43);
                $n = rotate_left!($o, 25);
                $o = rotate_left!($u,  8);
                $u = rotate_left!($y, 56);
                $y = rotate_left!($q, 41);
                $q = rotate_left!($e, 27);
                $e = rotate_left!($z, 14);
                $z = rotate_left!($w,  2);
                $w = rotate_left!($i, 55);
                $i = rotate_left!($r, 45);
                $r = rotate_left!($f, 36);
                $f = rotate_left!($d, 28);
                $d = rotate_left!($t, 21);
                $t = rotate_left!($s, 15);
                $s = rotate_left!($m, 10);
                $m = rotate_left!($h,  6);
                $h = rotate_left!($k,  3);
                $k = tmp;

                //{
                //    let state = [$a, $b, $c, $d, $e,
                //                 $f, $g, $h, $i, $j,
                //                 $k, $m, $n, $o, $p,
                //                 $q, $r, $s, $t, $u,
                //                 $v, $w, $x, $y, $z];
                //    println!("After Rho and Pi");
                //    dump_state!(&state[..]);
                //}

                // Chi and Iota steps
                tda = !$b & $c;
                tdb = !$c & $d;
                tdc = !$d & $e;
                tdd = !$e & $a;
                tde = !$a & $b;
                $a ^= tda ^ K[$ir]; 
                $b ^= tdb;          
                $c ^= tdc;          
                $d ^= tdd;          
                $e ^= tde;          
                tda = !$g & $h;
                tdb = !$h & $i;
                tdc = !$i & $j;
                tdd = !$j & $f;
                tde = !$f & $g;
                $f ^= tda; 
                $g ^= tdb;          
                $h ^= tdc;          
                $i ^= tdd;          
                $j ^= tde;          
	            tda = !$m & $n;
	            tdb = !$n & $o;
	            tdc = !$o & $p;
	            tdd = !$p & $k;
	            tde = !$k & $m;
                $k ^= tda;
                $m ^= tdb;
                $n ^= tdc;
                $o ^= tdd;
                $p ^= tde;
	            tda = !$r & $s;
	            tdb = !$s & $t;
	            tdc = !$t & $u;
	            tdd = !$u & $q;
	            tde = !$q & $r;
                $q ^= tda;
                $r ^= tdb;
                $s ^= tdc;
                $t ^= tdd;
                $u ^= tde;                
	            tda = !$w & $x;
	            tdb = !$x & $y;
	            tdc = !$y & $z;
	            tdd = !$z & $v;
	            tde = !$v & $w;
                $v ^= tda;
                $w ^= tdb;
                $x ^= tdc;
                $y ^= tdd;
                $z ^= tde;

                //{
                //    let state = [$a, $b, $c, $d, $e,
                //                 $f, $g, $h, $i, $j,
                //                 $k, $m, $n, $o, $p,
                //                 $q, $r, $s, $t, $u,
                //                 $v, $w, $x, $y, $z];
                //    println!("After Chi and Iota");
                //    dump_state!(&state[..]);
                //}
            }
        }
    }
    
    pub fn digest_block(state: &mut [u64; 25], block: &[u8]) {
        assert!(block.len() <= 200);
        let mut block2 = [0u64; 25];
        leu64::decode_slice(&mut block2[..block.len()/8], block);
        for il in 0..block.len()/8 {
            state[il] ^= block2[il];
        }

        //{
        //    println!("");
        //
        //    println!("Xor'd state (in bytes)");
        //    dump_state!(state);
        //
        //    println!("Xor'd state (as lanes of integers)");
        //    for y in 0..5 {
        //        for x in 0..5 {
        //            println!(" [{}, {}] = {:016x}", x, y, state[x + 5*y]);
        //        }
        //    }
        //}

        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];
        let mut e = state[4];
        let mut f = state[5];
        let mut g = state[6];
        let mut h = state[7];
        let mut i = state[8];
        let mut j = state[9];
        let mut k = state[10];
        let mut m = state[11];
        let mut n = state[12];
        let mut o = state[13];
        let mut p = state[14];
        let mut q = state[15];
        let mut r = state[16];
        let mut s = state[17];
        let mut t = state[18];
        let mut u = state[19];
        let mut v = state[20];
        let mut w = state[21];
        let mut x = state[22];
        let mut y = state[23];
        let mut z = state[24];
        
        for ir in 0..24 {
            digest_round!(a, b, c, d, e,
                          f, g, h, i, j,
                          k, m, n, o, p,
                          q, r, s, t, u,
                          v, w, x, y, z, ir, u64);
        }

        *state = [a, b, c, d, e,
                  f, g, h, i, j,
                  k, m, n, o, p,
                  q, r, s, t, u,
                  v, w, x, y, z];

        //{
        //    println!("");
        //    println!("After Permutation");
        //    dump_state!(state);
        //
        //    println!("State (as lanes of integers)");
        //    for y in 0..5 {
        //        for x in 0..5 {
        //            println!(" [{}, {}] = {:016x}", x, y, state[x + 5*y]);
        //        }
        //    }
        //    println!("Hash val is");
        //    {
        //        let mut state4 = [0u8; 64];
        //        leu64::encode_slice(&mut state4[..], &state[0..8]);
        //        for z in 0..2 {
        //            for byte in 0..16 {
        //                print!(" {:02x}", state4[16*z + byte]);
        //            }
        //            println!("");
        //        }
        //    }            
        //}

    }
}

