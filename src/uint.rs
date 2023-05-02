use crate::params;

pub const UINT_1: params::UInt = params::UInt {
    c: [1; params::LIMBS],
};

pub fn uint_set(x: &mut params::UInt, y: u64) {
    x.c[0] = y;
    for i in 1..params::LIMBS {
        x.c[i] = 0;
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
}