mod rng;
mod uint;
mod params;
mod constants;
mod fp;
mod mont;
mod csidh;

pub fn uint_print(x: &params::UInt) {
    // for (size_t i = 8*LIMBS-1; i < 8*LIMBS; --i)
    for i in (0..8*params::LIMBS).rev() {
        // printf("%02hhx", i[(unsigned char *) x->c]);
        print!("{:02x}", x.c[i/8] >> (8*(i%8)) as u64 & 0xff);
    }
}

pub fn fp_print(x: &params::Fp) {
    let mut y: params::UInt = params::UInt { c: x.c };
    fp::fp_dec(&mut y, x);
    uint_print(&y);
}

fn main() {
    let mut x = [0u8; 16];
    rng::random_bytes(&mut x).unwrap();
    println!("{:?}", x);
}
