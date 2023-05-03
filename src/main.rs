mod rng;
mod uint;
mod params;
mod constants;
mod fp;


fn main() {
    let mut x = [0u8; 16];
    rng::random_bytes(&mut x).unwrap();
    println!("{:?}", x);
}
