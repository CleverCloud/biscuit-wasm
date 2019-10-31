use biscuit::crypto;

use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KeyPair(pub(crate) crypto::KeyPair);

#[wasm_bindgen]
impl KeyPair {
  #[wasm_bindgen(constructor)]
  pub fn new() -> KeyPair {
      let mut rng = OsRng::new().unwrap();
      KeyPair(crypto::KeyPair::new(&mut rng))
  }

  #[wasm_bindgen(js_name = publicKey)]
  pub fn public_key(&self) -> PublicKey {
      PublicKey(self.0.public())
  }
}


#[wasm_bindgen]
pub struct PublicKey(pub(crate) crypto::PublicKey);

