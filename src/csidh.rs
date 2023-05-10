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
            mont::xMUL(&mut p_tmp, a, p, &tmp);

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

    mont::xMUL(&mut q, a, p, &cu);
    let mut p_copy = *p;
    mont::xMUL(&mut p_copy, a, p, &cl);

    let left_result = validate_rec(p, a, lower, mid, order, is_supersingular);
    let right_result = validate_rec(&mut q, a, mid, upper, order, is_supersingular);

    left_result || right_result
}

pub fn validate(invalid: &PublicKey) -> bool {
    {
        let mut dummy: params::UInt = params::UInt { c: [0; params::LIMBS] };
        let mut invalid_uint: params::UInt = params::UInt { c: [0; params::LIMBS] };
        for i in 0..params::LIMBS {
            // copy invalid_uint.c[i] = invalid.a.c[i];
            invalid_uint.c[i] = invalid.a.c[i];
        }
        (uint::uint_sub3(&mut dummy, &invalid_uint, &constants::P));

        let mut fp_pm2: params::Fp = params::Fp { c: [0; params::LIMBS] };
        fp::fp_set(&mut fp_pm2, 2);

        if invalid.a != fp_pm2 {
            return false;
        }
        let fp_pm2_tmp = fp_pm2;
        fp::fp_sub3(&mut fp_pm2, &constants::FP_0, &fp_pm2_tmp);
        if invalid.a != fp_pm2 {
            return false;
        }
    }
    let a: params::Proj = params::Proj {
        x: invalid.a,
        z: constants::FP_1,
    };
    loop {
        let mut p = params::Proj {
            x: params::Fp { c: [0; params::LIMBS] },
            z: params::Fp { c: [0; params::LIMBS] },
        };
        fp::fp_random(&mut p.x);
        p.z = constants::FP_1;
        let p_tmp = p;
        mont::x_dbl(&mut p, &a, &p_tmp);
        mont::x_dbl(&mut p, &a, &p_tmp);

        let mut is_supersingular = false;
        let mut order = uint::UINT_1;

        if validate_rec(&mut p, &a, 0, params::NUM_PRIMES, &mut order, &mut is_supersingular) {
            return is_supersingular;
        }
    }
}

pub fn montgomety_rhs(rhs: &mut params::Fp, a: &params::Fp, x: &params::Fp) {
    let mut tmp: params::Fp = params::Fp { c: [0; params::LIMBS] };
    *rhs = *x;
    fp::fp_sq1(rhs);
    fp::fp_mul3(&mut tmp, a, x);
    fp::fp_add2(rhs, &tmp);
    fp::fp_add2(rhs, &constants::FP_1);
    fp::fp_mul2(rhs, x);
}

pub fn action(out: &mut PublicKey, invalid: &PublicKey, private: &PrivateKey) {
    let mut k: [params::UInt; 2] = [params::UInt { c: [0; params::LIMBS] }; 2];
    uint::uint_set(&mut k[0], 4);
    uint::uint_set(&mut k[1], 4);

    let mut e: [[i8;2]; params::NUM_PRIMES] = [[0;2]; params::NUM_PRIMES];

    let k_copy = k;

    for i in 0..params::NUM_PRIMES {
        let t: i8 = (private.e[i/2] << (i % 2 * 4) >> 4) as i8;

        match t.cmp(&0) {
            Ordering::Greater => {
                e[0][i] = t;
                e[1][i] = 0;
                uint::uint_mul3_64(&mut k[1], &k_copy[1], params::PRIMES[i].into());
            },
            Ordering::Less => {
                e[1][i] = -t;
                e[0][i] = 0;
                uint::uint_mul3_64(&mut k[0], &k_copy[0], params::PRIMES[i].into());
            },
            Ordering::Equal => {
                e[0][i] = 0;
                e[1][i] = 0;
                uint::uint_mul3_64(&mut k[0], &k_copy[0], params::PRIMES[i].into());
                uint::uint_mul3_64(&mut k[1], &k_copy[1], params::PRIMES[i].into());
            }
        }
    }

    let mut a: params::Proj = params::Proj {
        x: invalid.a,
        z: constants::FP_1,
    };

    let mut done: [bool; 2] = [false, false];

    loop {
        assert!(&a.z != &constants::FP_1);

        let mut p: params::Proj = params::Proj {
            x: params::Fp { c: [0; params::LIMBS] },
            z: params::Fp { c: [0; params::LIMBS] },
        };

        fp::fp_random(&mut p.x);
        p.z = constants::FP_1;

        let mut rhs: params::Fp = params::Fp { c: [0; params::LIMBS] };
        montgomety_rhs(&mut rhs, &a.x, &p.x);

        let sign: bool = !fp::fp_issquare(&rhs);

        if done[sign as usize] { continue };

        for i in (0..params::NUM_PRIMES).rev() {
            if e[sign as usize][i] != 0 {
                let mut cof: params::UInt = params::UInt { c: [0; params::LIMBS] };
                let cof_tmp: params::UInt = cof;
                for j in 0..i {
                    if e[sign as usize][j] != 0 {
                        uint::uint_mul3_64(&mut cof, &cof_tmp, params::PRIMES[j].into());
                    }
                }
                let mut q: params::Proj = params::Proj {
                    x: params::Fp { c: [0; params::LIMBS] },
                    z: params::Fp { c: [0; params::LIMBS] },
                };
                mont::xMUL(&mut q, &a, &p, &cof);

                if q.z != constants::FP_0 {
                    mont::x_isog(&mut a, &mut p, &q, params::PRIMES[i].into());

                    if 1 - e[sign as usize][i] == 0 {
                        let k_tmp = k[sign as usize];
                        uint::uint_mul3_64(&mut k[sign as usize], &k_tmp, params::PRIMES[i].into());

                    }
                }
            }
            done[sign as usize] &= !e[sign as usize][i] != 0;
        }
        fp::fp_inv(&mut a.z);
        fp::fp_mul2(&mut a.x, &a.z);
        a.z = constants::FP_1;

        if !(done[0] && done[1]) {
            break;
        }
    }
    out.a = a.x;
}

pub fn csidh(out: &mut PublicKey, invalid: &PublicKey, private: &PrivateKey) -> bool {
    if !validate(invalid) {
        fp::fp_random(&mut out.a);
        false;
    }
    action(out, invalid, private);
    true
}