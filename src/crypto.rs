use biscuit::crypto;

use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KeyPair(pub(crate) crypto::KeyPair);

#[wasm_bindgen]
impl KeyPair {
  #[wasm_bindgen(constructor)]
  pub fn new() -> KeyPair {
      let mut rng = OsRng;
      KeyPair(crypto::KeyPair::new(&mut rng))
  }

  #[wasm_bindgen(js_name = publicKey)]
  pub fn public_key(&self) -> PublicKey {
      PublicKey(self.0.public())
  }

  #[wasm_bindgen(js_name=fromBytes)]
  pub fn from_bytes(slice: &[u8]) -> Result<KeyPair, JsValue> {
    let mut data = [0u8; 32];
    if slice.len() != 32 {
      panic!("invalid key");
    }

   data.copy_from_slice(slice);
    if let Some(key) = crypto::PrivateKey::from_bytes(data) {
      Ok(KeyPair(crypto::KeyPair::from(key)))
    } else {
      panic!("invalid key");
    }
  }

  #[wasm_bindgen(js_name=toBytes)]
  pub fn to_bytes(&self, slice: &mut[u8]) {
    slice.copy_from_slice(&self.0.private().to_bytes())
  }
}


#[wasm_bindgen]
pub struct PublicKey(pub(crate) crypto::PublicKey);

#[wasm_bindgen]
impl PublicKey {
  #[wasm_bindgen(js_name=toBytes)]
  pub fn to_bytes(&self, slice: &mut[u8]) {
    slice.copy_from_slice(&self.0.to_bytes())
  }
}
