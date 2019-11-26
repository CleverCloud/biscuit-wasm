use crate::builder::{Fact, Rule, Predicate, Atom, Constraint, ConstraintKind, ConstraintData};
use crate::Biscuit;

use biscuit::token::builder;

use std::time::SystemTime;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Verifier {
    facts: Vec<builder::Fact>,
    rules: Vec<builder::Rule>,
    block_caveats: Vec<builder::Rule>,
    authority_caveats: Vec<builder::Rule>,
}

#[wasm_bindgen]
impl Verifier {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Verifier{
            facts: vec![],
            rules: vec![],
            block_caveats: vec![],
            authority_caveats: vec![],
        }
    }

    #[wasm_bindgen(js_name = addFact)]
    pub fn add_fact(&mut self, fact: Fact) {
        self.facts.push(builder::Fact(fact.0.into_predicate()));
    }

    #[wasm_bindgen(js_name = addRule)]
    pub fn add_rule(&mut self, rule_bind: Rule) {
        self.rules.push(rule_bind.into_rule());
    }

    #[wasm_bindgen(js_name = addBlockCaveat)]
    pub fn add_block_caveat(&mut self, caveat: Rule) {
        self.block_caveats.push(caveat.into_rule());
    }

    #[wasm_bindgen(js_name = addAuthorityCaveat)]
    pub fn add_authority_caveat(&mut self, caveat: Rule) {
        self.authority_caveats.push(caveat.into_rule());
    }

    #[wasm_bindgen(js_name = addResource)]
    pub fn add_resource(&mut self, resource: &str) {
        self.facts
            .push(builder::fact("resource", &[builder::s("ambient"), builder::string(resource)]));
    }


    #[wasm_bindgen(js_name = addOperation)]
    pub fn add_operation(&mut self, operation: &str) {
        self.facts
            .push(builder::fact("operation", &[builder::s("ambient"), builder::s(operation)]));
    }

    #[wasm_bindgen(js_name = setTime)]
    pub fn set_time(&mut self) {
        self.facts.retain(|f| f.0.name != "time");

        self.facts
            .push(builder::fact("time", &[builder::s("ambient"), builder::date(&SystemTime::now())]));
    }

    #[wasm_bindgen(js_name = revocationCheck)]
    pub fn revocation_check(&mut self, ids: &[i64]) {
        let caveat = Rule {
          head_name: "revocation_check".to_string(),
          head_ids: vec![Atom { variable: Some(0), ..Default::default() }],
          predicates: vec![Predicate { name: "revocation_id".to_string(), ids: vec![Atom { variable: Some(0), ..Default::default() }] }],
          constraints: vec![Constraint {
            id: 0,
            kind: ConstraintKind::Integer,
            operation: "in".to_string(),
            data: ConstraintData::IntegerSet(ids.iter().cloned().collect()),
          }],
        };
        self.add_block_caveat(caveat);
    }

    #[wasm_bindgen]
    pub fn verify(&self, root_key: &crate::crypto::PublicKey, biscuit: Biscuit) -> Result<(), JsValue> {
        let mut verifier = biscuit.0.verify(root_key.0)
            .map_err(|e| { let e: crate::error::Error = e.into(); e})
            .map_err(|e| JsValue::from_serde(&e).expect("error serde"))?;

        for fact in self.facts.iter() {
            verifier.add_fact(fact.clone());
        }

        for rule in self.rules.iter() {
            verifier.add_rule(rule.clone());
        }

        for caveat in self.authority_caveats.iter() {
            verifier.add_authority_caveat(caveat.clone());
        }

        for caveat in self.block_caveats.iter() {
            verifier.add_block_caveat(caveat.clone());
        }

        verifier.verify()
          .map_err(|e| {let e: crate::error::Error = e.into(); e})
          .map_err(|e| JsValue::from_serde(&e).expect("error serde"))
          .map(|_| {
            //FIXME: queries not supported yet
          })
    }
}
