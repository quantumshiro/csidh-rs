use crate::params::{self, LIMBS};

pub const UINT_1: params::UInt = params::UInt {
    c: [1; params::LIMBS],
};

pub fn uint_set(x: &mut params::UInt, y: u64) {
    x.c[0] = y;
    for i in 1..params::LIMBS {
        x.c[i] = 0;
    }
}

pub fn uint_bit(x: &params::UInt, k: u64) -> bool {
    (x.c[(k / 64) as usize] >> (k % 64)) & 1 == 1
}

pub fn uint_add3(x: &mut params::UInt, y: &params::UInt, z: &params::UInt) -> bool {
    let mut carry: bool = false;
    for i in 0..params::LIMBS {
        let (sum1, overflow1) = y.c[i].overflowing_add(carry as u64);
        let (sum2, overflow2) = sum1.overflowing_add(z.c[i]);
        x.c[i] = sum2;
        carry = overflow1 || overflow2;
    }
    carry
}

pub fn uint_sub3(x: &mut params::UInt, y: &params::UInt, z: &params::UInt) -> bool {
    let mut borrow = false;
    for i in 0..LIMBS {
        let (diff1, overflow1) = y.c[i].overflowing_sub(borrow as u64);
        let (diff2, overflow2) = diff1.overflowing_sub(z.c[i]);
        x.c[i] = diff2;
        borrow = overflow1 || overflow2;
    }
    borrow
}

pub fn uint_mul3_64(x: &mut params::UInt, y: &params::UInt, z: u64) {
    let mut carry: bool = false;
    for i in 0..params::LIMBS {
        let t = (y.c[i] as u128) * (z as u128) + (carry as u128);
        carry = (t >> 64) != 0;
        x.c[i] = t as u64;
    }
}


#[cfg(test)]
mod uint_test {
    use super::*;

    #[test]
    fn test_uint_set() {
        let mut x = params::UInt { c: [0; params::LIMBS] };
        uint_set(&mut x, 1);
        assert_eq!(x.c[0], 1);
        assert_eq!(x.c[1], 0);
    }

    #[test]
    fn test_uint_bit() {
        let mut x = params::UInt { c: [0; params::LIMBS] };
        uint_set(&mut x, 1);
        assert!(uint_bit(&x, 0));
    }

    #[test]
    fn test_uint_add3() {
        let mut x = params::UInt { c: [0; params::LIMBS] };
        let y = params::UInt { c: [1; params::LIMBS] };
        let z = params::UInt { c: [1; params::LIMBS] };
        assert!(uint_add3(&mut x, &y, &z));
        assert_eq!(x.c[0], 2);
        assert_eq!(x.c[1], 0);
    }

    #[test]
    fn test_uint_sub3() {
        let mut x = params::UInt { c: [0; params::LIMBS] };
        let y = params::UInt { c: [1; params::LIMBS] };
        let z = params::UInt { c: [1; params::LIMBS] };
        assert!(!uint_sub3(&mut x, &y, &z));
        assert_eq!(x.c[0], 0);
        assert_eq!(x.c[1], 0);
    }

    #[test]
    fn test_uint_mul3_64() {
        let mut x = params::UInt { c: [0; params::LIMBS] };
        let y = params::UInt { c: [1; params::LIMBS] };
        uint_mul3_64(&mut x, &y, 2);
        assert_eq!(x.c[0], 2);
        assert_eq!(x.c[1], 0);
    }

}