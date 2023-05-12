use crate::params::{self, LIMBS, Fp};
use crate::uint::{self};
use crate::rng::{self};
use crate::constants::{self, P};

pub fn fp_add3(x: &mut params::Fp, y: &params::Fp, z: &params::Fp) {
    // x: convert &mut params::Fp -> &params::UInt
    // y: convert &params::Fp -> &params::UInt
    // z: convert &params::Fp -> &params::UInt

    let mut x_uint = params::UInt { c: x.c };
    let mut y_uint = params::UInt { c: y.c };
    let mut z_uint = params::UInt { c: z.c };

    for i in 0..params::LIMBS {
        x_uint.c[i] = x.c[i];
        y_uint.c[i] = y.c[i];
        z_uint.c[i] = z.c[i];
    }

    uint::uint_add3(&mut x_uint, &y_uint, &z_uint);
}

pub fn fp_add2(x: &mut params::Fp, y: &params::Fp) {
    let mut tmp = params::Fp { c: [0; params::LIMBS] };
    for i in 0..params::LIMBS {
        tmp.c[i] = x.c[i];
    }
    fp_add3(x, &tmp, y);
}

pub fn fp_sub3(x: &mut params::Fp, y: &params::Fp, z: &params::Fp) {
    let mut x_uint = params::UInt { c: x.c };
    let mut y_uint = params::UInt { c: y.c };
    let mut z_uint = params::UInt { c: z.c };

    for i in 0..params::LIMBS {
        x_uint.c[i] = x.c[i];
        y_uint.c[i] = y.c[i];
        z_uint.c[i] = z.c[i];
    }

    uint::uint_sub3(&mut x_uint, &y_uint, &z_uint);
}

pub fn fp_sub2(x: &mut params::Fp, y: &params::Fp) {
    let mut tmp = params::Fp { c: [0; params::LIMBS] };
    for i in 0..params::LIMBS {
        tmp.c[i] = x.c[i];
    }
    fp_sub3(x, &tmp, y);
}


pub fn fp_mul3(x: &mut params::Fp, y: &params::Fp, z: &params::Fp) {
    let mut t: [u64; params::LIMBS + 1] = [0; params::LIMBS + 1];

    for k in 0..params::LIMBS {
        let m: u128 = constants::INV_MIN_P_MOD_R as u128 * (y.c[k] as u128 * z.c[0] as u128 + t[(k) % (params::LIMBS + 1)] as u128);

        let mut c: bool = false;
        let mut o: bool = false;

        for i in 0..params::LIMBS {
            let u: u128 = m * constants::P.c[i] as u128;
            o |= t[(k + i) % (params::LIMBS + 1)].checked_add(u as u64).map_or(true, |v| {
                t[(k + i) % (params::LIMBS + 1)] = v;
                false
            });
            c |= t[(k + i + 1) % (params::LIMBS + 1)].checked_add((u >> 64) as u64).map_or(true, |v| {
                t[(k + i + 1) % (params::LIMBS + 1)] = v;
                false
            });
        }
        t[params::LIMBS] = t[params::LIMBS].wrapping_add(o as u64);
    }
    for i in 0..params::LIMBS {
        x.c[i] = t[(params::LIMBS + i) % (params::LIMBS + 1)];
    }
}

pub fn fp_mul2(x: &mut params::Fp, y: &params::Fp) {
    let mut tmp = params::Fp { c: [0; params::LIMBS] };
    for i in 0..params::LIMBS {
        tmp.c[i] = x.c[i];
    }
    fp_mul3(x, &tmp, y);
}

pub fn fp_sq2(x: &mut params::Fp, y: &params::Fp) {
    fp_mul3(x, y, y);
}

pub fn fp_sq1(x: &mut params::Fp) {
    let mut tmp = params::Fp { c: [0; params::LIMBS] };
    for i in 0..params::LIMBS {
        tmp.c[i] = x.c[i];
    }
    fp_sq2(x, &tmp);
}

pub fn fp_pow(x: &mut params::Fp, e: &params::UInt) {
    let mut y = *x;
    *x = constants::FP_1;
    for i in 0..params::LIMBS {
        let mut t = e.c[i];
        for i in 0..64 {
            if t & 1 == 1 {
                fp_mul2(x, &y);
            }
            fp_sq1(&mut y);
            t >>= 1;
        }
    }
}

pub fn fp_inv(x: &mut params::Fp) {
    let mut tmp = params::Fp { c: [0; params::LIMBS] };
    for i in 0..params::LIMBS {
        tmp.c[i] = x.c[i];
    }

    fp_pow(&mut tmp, &constants::P_MINUS_2);
}

pub fn fp_issquare(x: &params::Fp) -> bool {
    let mut y = *x;
    fp_pow(&mut y, &constants::P_MINUS_1_HALVES);
    // return !memcmp(x, &fp_1, sizeof(fp));
    for i in 0..params::LIMBS {
        if x.c[i] != constants::FP_1.c[i] {
            return false;
        }
    }
    true
}


pub fn fp_random(x: &mut params::Fp) {
    // convert params::Fp -> &mut [u8];
    let mut x_bytes: [u8; params::LIMBS * 8] = [0; params::LIMBS * 8];
    for i in 0..params::LIMBS {
        for j in 0..8 {
            x_bytes[i * 8 + j] = x.c[i] as u8;
        }
    }
    loop {
        rng::randombytes(&mut x_bytes).unwrap();
        let m: u64 = (1u64 << (constants::PBITS % 64)) - 1;
        x.c[params::LIMBS - 1] &= m;

        for i in (0..params::LIMBS).rev() {
            if x.c[i] < constants::P.c[i] {
                break;
            }
            if x.c[i] > constants::P.c[i] {
                continue;
            }
        }
    } 
}

// Montgomery arithmetic
pub fn fp_enc(x: &mut params::Fp, y: &params::UInt) {
    let mut y_fp = params::Fp { c: [0; params::LIMBS] };
    for i in 0..params::LIMBS {
        y_fp.c[i] = y.c[i];
    }
    fp_mul3(x, &y_fp, &constants::R_SQUARED_MOD_P);
}

pub fn fp_dec(x: &mut params::UInt, y: &params::Fp) {
   let mut x_fp = params::Fp { c: [0; params::LIMBS] };
    for i in 0..params::LIMBS {
        x_fp.c[i] = x.c[i];
    }
    // convert uint::IINT_1 to fp_uint
    let mut fp_uint_1 = params::Fp { c: [0; params::LIMBS] };
    for i in 0..params::LIMBS {
        fp_uint_1.c[i] = uint::UINT_1.c[i];
    }
    fp_mul3(&mut x_fp, y, &fp_uint_1);
}

pub fn fp_set(x: &mut params::Fp, y: u64) {
    // x convert Fp to UInt
    let mut x_uint = params::UInt { c: [0; params::LIMBS] };
    for i in 0..params::LIMBS {
        x_uint.c[i] = x.c[i];
    }
    uint::uint_set(&mut x_uint, y);

    fp_enc(x, &x_uint);
}
pub fn reduce_once(x: &mut params::UInt) {
    let mut t = params::UInt { c: [0; params::LIMBS] };

    if !uint::uint_sub3(&mut t, x, &constants::P) {
        *x = t;
    }
}

#[cfg(test)]
mod fp_test {
    use super::*;

    #[test]
    fn fp_mul_test() {
        let mut x = params::Fp { c: [0; params::LIMBS] };
        let mut y = params::Fp { c: [0; params::LIMBS] };
        let mut z = params::Fp { c: [0; params::LIMBS] };
        fp_set(&mut x, 2);
        fp_set(&mut y, 3);
        fp_mul3(&mut z, &x, &y);
        assert_eq!(z.c[0], 6);          
    }
}