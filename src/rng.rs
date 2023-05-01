use core::panic;
use std::fs::File;
use std::io::{Read, Result};
use std::sync::Once;


static mut FD: Option<File> = None;
static INIT: Once = Once::new();

pub fn random_bytes(x: &mut [u8]) -> Result<()> {

    INIT.call_once(|| {
        let file = File::open("/dev/urandom").expect("failed to open /dev/urandom");
        unsafe {
            FD = Some(file);
        }
    });

    for i in 0..x.len() {
        let n = unsafe {
            FD.as_ref().unwrap().read(&mut x[i..]).unwrap()
        };
        if n == 0 {
            panic!("EOF reached");
        }
    }
    Ok(())
}

#[cfg(test)]
mod rng_test {
    use super::*;

    #[test]
    fn test_random_bytes() {
        let mut x = [0u8; 16];
        random_bytes(&mut x).unwrap();
        println!("{:?}", x);
    }
}