extern crate biscuit_auth as biscuit;

use biscuit::datalog;
use biscuit::token;
use wasm_bindgen::prelude::*;
use rand::rngs::OsRng;

pub mod builder;
pub mod crypto;
pub mod verifier;
pub mod error;

use crate::builder::*;

#[wasm_bindgen]
pub struct SymbolTable(datalog::SymbolTable);

#[wasm_bindgen()]
pub struct BlockBuilder {
  facts: Vec<Fact>,
  rules: Vec<Rule>,
  caveats: Vec<Rule>,
}

#[wasm_bindgen()]
impl BlockBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        BlockBuilder {
          facts: vec![],
          rules: vec![],
          caveats: vec![],
        }
    }

    #[wasm_bindgen(js_name = addFact)]
    pub fn add_fact(&mut self, fact: Fact) {
        self.facts.push(fact);
    }

    #[wasm_bindgen(js_name = addRule)]
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    #[wasm_bindgen(js_name = addCaveat)]
    pub fn add_caveat(&mut self, caveat: Rule) {
        self.caveats.push(caveat);
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Biscuit(token::Biscuit);

#[wasm_bindgen]
impl Biscuit {
    /*#[wasm_bindgen(constructor)]
    pub fn new(base_symbols: JsValue) -> BiscuitBuilder {
        let symbol_strings: Option<Vec<String>> = base_symbols.into_serde().expect("Can't format symbols table");
        let symbols = symbol_strings.map(|symbols| SymbolTable { symbols }).unwrap_or_else(default_symbol_table);

        BiscuitBuilder {
            symbols,
            facts: vec![],
            rules: vec![],
            caveats: vec![],
        }
    }*/

    #[wasm_bindgen(constructor)]
    pub fn new() -> BiscuitBuilder {

        BiscuitBuilder {
            symbols: token::default_symbol_table(),
            facts: vec![],
            rules: vec![],
            caveats: vec![],
        }
    }

    #[wasm_bindgen]
    pub fn from(slice: &[u8]) -> Result<Biscuit, JsValue> {
        token::Biscuit::from(slice)
            .map_err(|e| { let e: error::Error = e.into(); e})
            .map_err(|e| JsValue::from_serde(&e).expect("biscuit from error"))
            .map(Biscuit)
    }

    #[wasm_bindgen(js_name = fromSealed)]
    pub fn from_sealed(slice: &[u8], secret: &[u8]) -> Result<Biscuit, JsValue> {
        token::Biscuit::from_sealed(slice, secret)
            .map_err(|e| { let e: error::Error = e.into(); e})
            .map_err(|e| JsValue::from_serde(&e).expect("biscuit from error"))
            .map(Biscuit)
    }

    #[wasm_bindgen(js_name = toVec)]
    pub fn to_vec(&self) -> Result<Vec<u8>, JsValue> {
        match self.0.clone().container().as_ref() {
            None => Err(JsValue::from_serde(&error::Error::InternalError).unwrap()),
            Some(c) => c.to_vec()
              .map_err(|e| { let e: error::Format = e.into(); error::Error::Format(e)})
              .map_err(|e| JsValue::from_serde(&e).unwrap()),
        }
    }

    #[wasm_bindgen(js_name = createBlock)]
    pub fn create_block(&self) -> BlockBuilder {
        BlockBuilder::new()
    }

    #[wasm_bindgen]
    pub fn append(
        &self,
        keypair: crypto::KeyPair,
        block_builder: BlockBuilder,
    ) -> Result<Biscuit, JsValue> {
        let mut builder = self.0.create_block();

        for fact in block_builder.facts {
          builder.add_fact(fact.into_fact());
        }

        for rule in block_builder.rules {
          builder.add_rule(rule.into_rule());
        }

        for caveat in block_builder.caveats {
          builder.add_caveat(caveat.into_rule());
        }

        let block = builder.build();

        let mut rng = OsRng;
        self.0.append(&mut rng, &keypair.0, block)
            .map_err(|e| { let e: error::Error = e.into(); e})
            .map_err(|e| JsValue::from_serde(&e).expect("error append"))
            .map(Biscuit)
    }

    #[wasm_bindgen]
    pub fn print(&self) -> String {
      self.0.print()
    }
}
