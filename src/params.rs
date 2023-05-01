const LIMBS: usize = 8;
const NUM_PRIMES: usize = 74;
const MAX_EXPONENT: usize = 5;

#[derive(Clone, Copy)]
struct UInt {
    c: [u64; LIMBS],
}

#[derive(Clone, Copy)]
struct Fp {
    c: [u64; LIMBS],
}

#[derive(Clone, Copy)]
struct Proj {
    x: Fq,
    z: Fq,
}

static PRIMES: [u32; NUM_PRIMES] = [
    3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
    97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
    191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
    283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 587,
];