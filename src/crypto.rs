use biscuit::crypto::{KeyPair, PublicKey};

use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KeyPairBind(pub(crate) KeyPair);

#[wasm_bindgen(js_name = newKeypair)]
pub fn keypair_new() -> KeyPairBind {
    let mut rng = OsRng::new().unwrap();
    KeyPairBind(KeyPair::new(&mut rng))
}

#[wasm_bindgen(js_name = publicKey)]
pub fn public_key(keypair: &KeyPairBind) -> PublicKeyBind {
    PublicKeyBind(keypair.0.public())
}

#[wasm_bindgen]
pub struct PublicKeyBind(pub(crate) PublicKey);

