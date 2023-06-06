use std::fs::File;
use std::io::Read;


pub fn randombytes(buf: &mut [u8]) -> std::io::Result<()> {
    let mut file = match File::open("/dev/urandom") {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    match file.read_exact(buf) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
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