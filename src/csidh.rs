use crate::uint;
use crate::fp;
use crate::mont;
use crate::rng;
use crate::constants;
use crate::params;

use std::ptr;

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

