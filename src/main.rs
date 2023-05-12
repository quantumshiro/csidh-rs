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

/*
pub fn fp_print(x: &params::Fp) {
    let mut y: params::UInt = params::UInt { c: x.c };
    fp::fp_dec(&mut y, x);
    uint_print(&y);
}
*/

fn main() {
    let mut private_alice: csidh::PrivateKey = csidh::PrivateKey {
        e: [0; (params::NUM_PRIMES + 1) / 2]
    };
    let mut private_bob: csidh::PrivateKey = csidh::PrivateKey {
        e: [0; (params::NUM_PRIMES + 1) / 2]
    };
    let mut public_alice: csidh::PublicKey = csidh::PublicKey {
        a: params::Fp { c: [0; params::LIMBS] }
    };
    let mut public_bob: csidh::PublicKey;
    let mut shared_alice: csidh::PublicKey;
    let mut shared_bob: csidh::PublicKey;

    println!("\n");

    csidh::csidh_private(&mut private_alice);
    println!("Alice's private key:");
    for i in 0..(params::NUM_PRIMES + 1) / 2 {
        print!("{:02x}", private_alice.e[i]);
        print!(" ");
    }

    csidh::csidh_private(&mut private_bob);
    println!("\nBob's private key:");
    for i in 0..(params::NUM_PRIMES + 1) / 2 {
        print!("{:02x}", private_bob.e[i]);
        print!(" ");
    }

    csidh::csidh(&mut public_alice, &csidh::BASE, &private_bob);
    
}
