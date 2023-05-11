use rand::{rngs::OsRng, RngCore};

use crate::uint;
use crate::fp;
use crate::mont;
use crate::rng;
use crate::constants;
use crate::params;

use std::ptr;
use std::cmp::Ordering;
use std::io::Read;

#[derive(Debug)]
pub struct PrivateKey {
    pub e: [i8; (params::NUM_PRIMES + 1)/2],
}

#[derive(Debug)]

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
    private.e.fill(0);
    for i in 0..params::NUM_PRIMES {
        let mut buf = [0u8; 64];
        rng::randombytes(&mut buf).unwrap();
        for j in 0..buf.len() {
            let v = buf[j] as i8;
            if v <= (params::MAX_EXPONENT as i8) && v >= -(params::MAX_EXPONENT as i8) {
                private.e[i/2] |= (v & 0xf) << (4 * (i % 2));
                if  i >= params::NUM_PRIMES {
                    break;
                }
            }
        }
    }
}

pub fn validate_rec(
    p: &mut params::Proj, 
    a: &params::Proj, 
    lower: usize,
    upper: usize, 
    order: &mut params::UInt,
    is_supersingular: &mut bool) -> bool {
    assert!(lower < upper);

    if upper - lower == 1 {
        if p.z != constants::FP_0 {
            let mut tmp: params::UInt = params::UInt { c: [0; params::LIMBS] };
            uint::uint_set(&mut tmp, params::PRIMES[lower].into());
            let p_tmp = *p;
            mont::xMUL(p, a, &p_tmp, &tmp);

            if p.z != constants::FP_0 {
                *is_supersingular = false;
                return true;
            }
            let order_copy = *order;
            uint::uint_mul3_64(order, &order_copy, params::PRIMES[lower].into());

            if uint::uint_sub3(&mut tmp, &constants::FOUR_SQRT_P, order) {
                *is_supersingular = true;
                return true;
            }
        }
        return false;
    }
    let mid = lower + (upper - lower + 1) / 2;
    let mut cl: params::UInt = uint::UINT_1;
    let mut cu: params::UInt = uint::UINT_1;

    let cu_copy = cu;
    let cl_copy = cl;
    for i in lower..mid {
        uint::uint_mul3_64(&mut cu, &cu_copy, params::PRIMES[i].into());
    }
    for i in mid..upper {
        uint::uint_mul3_64(&mut cl, &cl_copy, params::PRIMES[i].into());
    }

    let mut q: params::Proj = params::Proj {
        x: constants::FP_1,
        z: constants::FP_1,
    };

    let p_copy = *p;
    mont::xMUL(&mut q, a, p, &cu);
    mont::xMUL(p, a, &p_copy, &cl);
    
    return validate_rec(&mut q, a, mid, upper, order, is_supersingular)
        || validate_rec(p, a, lower, mid, order, is_supersingular);
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
        return false;
    }
    action(out, invalid, private);
    true
}

#[cfg(test)]
mod csidh_test {
    use super::*;

    #[test]
    fn test_private() {
        let mut private: PrivateKey = PrivateKey {
            e: [0; params::NUM_PRIMES / 2],
        };
        csidh_private(&mut private);
        dbg!(private);
    }

    #[test]
    fn test_validate_rec() {
        let mut p: params::Proj = params::Proj {
            x: params::Fp { c: [0; params::LIMBS] },
            z: params::Fp { c: [0; params::LIMBS] },
        };

        let a: params::Proj = params::Proj {
            x: params::Fp { c: [0; params::LIMBS] },
            z: params::Fp { c: [0; params::LIMBS] },
        };

        let upper: usize = 1 << (params::NUM_PRIMES / 2);
        let lower: usize = 1 << (params::NUM_PRIMES / 3);
        let mut order: params::UInt = params::UInt { c: [0; params::LIMBS] };
        
        dbg!(validate_rec(&mut p, &a, lower, upper, &mut order, &mut true));
    }
}