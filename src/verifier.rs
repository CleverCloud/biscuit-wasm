use crate::builder::*;
use crate::BiscuitBinder;

use biscuit::token::builder::*;
use biscuit::datalog::{Constraint, ConstraintKind, IntConstraint};

use std::time::SystemTime;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Verifier {
    facts: Vec<Fact>,
    rules: Vec<Rule>,
    block_caveats: Vec<Rule>,
    authority_caveats: Vec<Rule>,
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
    pub fn add_fact(&mut self, fact: FactBind) {
        self.facts.push(fact.into());
    }

    #[wasm_bindgen(js_name = addRule)]
    pub fn add_rule(&mut self, rule_bind: RuleBind) {
        self.rules.push(rule_bind.get_inner_rule());
    }

    #[wasm_bindgen(js_name = addBlockCaveat)]
    pub fn add_block_caveat(&mut self, caveat: RuleBind) {
        self.block_caveats.push(caveat.get_inner_rule());
    }

    #[wasm_bindgen(js_name = addAuthorityCaveat)]
    pub fn add_authority_caveat(&mut self, caveat: RuleBind) {
        self.block_caveats.push(caveat.get_inner_rule());
    }

    #[wasm_bindgen(js_name = addResource)]
    pub fn add_resource(&mut self, resource: &str) {
        self.facts
            .push(fact("resource", &[s("ambient"), string(resource)]));
    }


    #[wasm_bindgen(js_name = addOperation)]
    pub fn add_operation(&mut self, operation: &str) {
        self.facts
            .push(fact("operation", &[s("ambient"), s(operation)]));
    }

    #[wasm_bindgen(js_name = setTime)]
    pub fn set_time(&mut self) {
        self.facts.retain(|f| f.0.name != "time");

        self.facts
            .push(fact("time", &[s("ambient"), date(&SystemTime::now())]));
    }

    #[wasm_bindgen(js_name = revocationCheck)]
    pub fn revocation_check(&mut self, ids: &[i64]) {
        let caveat = constrained_rule(
            "revocation_check",
            &[Atom::Variable(0)],
            &[pred("revocation_id", &[Atom::Variable(0)])],
            &[Constraint {
                id: 0,
                kind: ConstraintKind::Int(IntConstraint::NotIn(ids.iter().cloned().collect())),
            }],
        );
        self.add_block_caveat(RuleBind::from(caveat));
    }

    #[wasm_bindgen]
    pub fn verify(&self, biscuit: BiscuitBinder) -> Result<(), JsValue> {
        let mut symbols = biscuit.0.symbols().clone();

        let mut ambient_facts = vec![];
        let mut ambient_rules = vec![];
        let mut authority_caveats = vec![];
        let mut block_caveats = vec![];

        for fact in self.facts.iter() {
            ambient_facts.push(fact.convert(&mut symbols));
        }

        for rule in self.rules.iter() {
            ambient_rules.push(rule.convert(&mut symbols));
        }

        for caveat in self.authority_caveats.iter() {
            authority_caveats.push(caveat.convert(&mut symbols));
        }

        for caveat in self.block_caveats.iter() {
            block_caveats.push(caveat.convert(&mut symbols));
        }

        biscuit.0.check(&symbols, ambient_facts, ambient_rules, authority_caveats, block_caveats)
            .map_err(|e| JsValue::from_serde(&e).expect("error serde"))
    }
}