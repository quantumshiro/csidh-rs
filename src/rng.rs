use core::panic;
use std::fs::File;
use std::io::{Read, Result};
use std::sync::Once;


static mut FD: Option<File> = None;
static INIT: Once = Once::new();

pub fn randombytes(buf: &mut [u8]) -> Result<()> {
    let mut fd = unsafe { File::open("/dev/urandom").expect("failed to open /dev/urandom") };
    for i in 0..buf.len() {
        let n = unsafe { fd.read(&mut buf[i..]).expect("failed to read from /dev/urandom") };
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
        randombytes(&mut x).expect("Failed to read random bytes");
        println!("{:?}", x);
    }
}