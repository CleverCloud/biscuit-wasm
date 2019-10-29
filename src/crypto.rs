use biscuit::crypto::KeyPair;

use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;


#[wasm_bindgen(js_name = newKeypair)]
pub fn keypair_new() -> KeyPair {
    let mut rng = OsRng::new().unwrap();
    KeyPair::new(&mut rng)
}