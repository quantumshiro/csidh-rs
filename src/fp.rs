use crate::params::{self, LIMBS, Fp};
use crate::uint::{self};
use crate::rng::{self};
use crate::constants::{self, P};


// Montgomery arithmetic
pub fn fp_mul3(x: &mut params::Fp, y: &params::Fp, z: &params::Fp) {
    let mut t: [u64; params::LIMBS + 1] = [0; params::LIMBS + 1];

    for k in 0..params::LIMBS {
        let m = constants::INV_MIN_P_MOD_R * (y.c[k] * z.c[0] + t[(k) % (params::LIMBS + 1)]);

        let mut c: bool = false;
        let mut o: bool = false;

        for i in 0..params::LIMBS {
            let u: u128 = (m as u128) * (constants::P.c[i] as u128);
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

pub fn fp_enc(x: &mut params::Fp, y: &params::UInt) {
    let mut y_fp = params::Fp { c: [0; params::LIMBS] };
    for i in 0..params::LIMBS {
        y_fp.c[i] = y.c[i];
    }
    fp_mul3(x, &y_fp, &constants::R_SQUARED_MOD_P);
}



pub fn reduce_once(x: &mut params::UInt) {
    let mut t = params::UInt { c: [0; params::LIMBS] };

    if !uint::uint_sub3(&mut t, x, &constants::P) {
        *x = t;
    }
}