use crate::uint;
use crate::fp;
use crate::mont;
use crate::rng;
use crate::constants;
use crate::params;

use std::ptr;
use std::cmp::Ordering;

pub struct PrivateKey {
    pub e: [i8; (params::NUM_PRIMES + 1)/2],
}

pub struct PublicKey {
    pub a: params::Fp,
}

// const public_key base = {0}; /* A = 0 */
pub const BASE: PublicKey = PublicKey {
     a: params::Fp { 
        c: [0, 0, 0, 0, 0, 0, 0, 0] 
    } 
};

pub fn csidh_private(private: &mut PrivateKey) {
    unsafe { ptr::write_bytes(&mut private.e, 0, (params::NUM_PRIMES + 1)/2) };
    
    for mut i in 0..(private.e).len() {
        let mut buf: [u8; 64] = [0; 64];
        rng::random_bytes(&mut buf).unwrap();
        for &byte in &buf {
            if byte <= params::MAX_EXPONENT as u8 && byte >= (-(params::MAX_EXPONENT as i8))as u8 {
                private.e[i/2] = ((byte & 0xf) << (i % 2 * 4)) as i8;
                if { i += 1; i } >= params::NUM_PRIMES {
                    break;
                }
            }
        }

    }

}

pub fn validate_rec(p: &mut params::Proj, a: &params::Proj, lower: usize, upper: usize, order: &mut params::UInt, is_supersingular: &mut bool) -> bool {
    assert!(lower < upper);

    if upper - lower == 1 {
        if p.z != constants::FP_0 {
            let mut tmp: params::UInt = params::UInt { c: [0; params::LIMBS] };
            uint::uint_set(&mut tmp, params::PRIMES[lower].into());
            let mut p_tmp = *p;
            mont::xMUL(&mut p_tmp, a, p, &mut tmp);

            if p.z != constants::FP_0 {
                *is_supersingular = false;
                return true;
            }
            let order_tmp = *order;
            uint::uint_mul3_64(order, &order_tmp, params::PRIMES[lower] as u64);
            if uint::uint_sub3(&mut tmp, &constants::FOUR_SQRT_P, order) {
                *is_supersingular = true;
                return true;
            }
        }
        return false;
    }
    let mid = lower + (upper - lower + 1) / 2;
    let mut cl = uint::UINT_1;
    let mut cu = uint::UINT_1;

    let cu_tmp = uint::UINT_1;

    for i in lower..mid {
        uint::uint_mul3_64(&mut cu, &cu_tmp, params::PRIMES[i].into());
    }

    let cl_tmp = uint::UINT_1;

    for i in mid..upper {
        uint::uint_mul3_64(&mut cl, &cl_tmp, params::PRIMES[i].into());
    }

    let mut q = params::Proj {
        x: params::Fp { c: [0; params::LIMBS] },
        z: params::Fp { c: [0; params::LIMBS] },
    };

    mont::xMUL(&mut q, a, p, &mut cu);
    let mut p_copy = *p;
    mont::xMUL(&mut p_copy, a, p, &cl);

    let left_result = validate_rec(p, a, lower, mid, order, is_supersingular);
    let right_result = validate_rec(&mut q, a, mid, upper, order, is_supersingular);

    left_result || right_result
}