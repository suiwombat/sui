// extern crate untrusted;
// use untrusted;
extern crate ring;

use ring::{rand, signature};

pub fn main() {
    // Generate a key pair in PKCS#8 (v2) format.
    let rng = rand::SystemRandom::new();
    let _pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)
        .unwrap()
        .as_ref()
        .to_vec();

    println!("{:#?}", _pkcs8_bytes);
}
