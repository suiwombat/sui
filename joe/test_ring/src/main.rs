extern crate alloc;

use ring::{agreement, error, rand, test, test_file};

pub fn main() {
    let rng = rand::SystemRandom::new();
    let private_key =
        agreement::EphemeralPrivateKey::generate(&agreement::ECDH_P256, &rng).unwrap();

    println!("{:?}", private_key);
}
