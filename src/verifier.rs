use crate::builder::{Fact, Rule, Predicate, Atom, Constraint, ConstraintKind, ConstraintData};
use crate::Biscuit;

use biscuit::token::builder;

use std::time::{Duration, SystemTime};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Verifier {
    facts: Vec<builder::Fact>,
    rules: Vec<builder::Rule>,
    caveats: Vec<builder::Rule>,
}

#[wasm_bindgen]
impl Verifier {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Verifier{
            facts: vec![],
            rules: vec![],
            caveats: vec![],
        }
    }

    #[wasm_bindgen(js_name = addFact)]
    pub fn add_fact(&mut self, fact: Fact) {
        self.facts.push(builder::Fact(fact.0));
    }

    #[wasm_bindgen(js_name = addRule)]
    pub fn add_rule(&mut self, rule_bind: Rule) {
        self.rules.push(rule_bind.into_rule());
    }

    #[wasm_bindgen(js_name = addCaveat)]
    pub fn add_caveat(&mut self, caveat: Rule) {
        self.caveats.push(caveat.into_rule());
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
    pub fn set_time(&mut self, i: u64) {
        self.facts.retain(|f| f.0.name != "time");

        let t = SystemTime::UNIX_EPOCH + Duration::new(i, 0);
        self.facts
            .push(builder::fact("time", &[builder::s("ambient"), builder::date(&t)]));
    }

    #[wasm_bindgen(js_name = revocationCheck)]
    pub fn revocation_check(&mut self, ids: &[i64]) {
        let head_name =  "revocation_check".to_string();
        let mut head_ids = vec![Atom { variable: Some("id".to_string()), ..Default::default() }];
        let mut predicates = vec![Predicate {
            name: "revocation_id".to_string(),
            ids: vec![Atom { variable: Some("id".to_string()), ..Default::default() }] }];
        let mut constraints = vec![Constraint {
            id: "id".to_string(),
            kind: ConstraintKind::Integer,
            operation: "in".to_string(),
            data: ConstraintData::IntegerSet(ids.iter().cloned().collect()),
          }];

        let head_ids = head_ids.drain(..).map(|a| a.into_atom()).collect::<Vec<_>>();
        let predicates = predicates.drain(..).map(|p| p.into_predicate()).collect::<Vec<_>>();
        let constraints = constraints.drain(..).map(|p| p.into_constraint()).collect::<Vec<_>>();

        let caveat = Rule { rule: builder::constrained_rule(&head_name, &head_ids, &predicates, &constraints) };

        self.add_caveat(caveat);
    }

    #[wasm_bindgen]
    pub fn verify(&self, root_key: &crate::crypto::PublicKey, biscuit: Biscuit) -> Result<String, JsValue> {
        let mut verifier = biscuit.0.verify(root_key.0)
            .map_err(|e| { let e: crate::error::Error = e.into(); e})
            .map_err(|e| JsValue::from_serde(&e).expect("error serde"))?;

        for fact in self.facts.iter() {
            verifier.add_fact(fact.clone());
        }

        for rule in self.rules.iter() {
            verifier.add_rule(rule.clone());
        }

        for caveat in self.caveats.iter() {
            verifier.add_caveat(caveat.clone());
        }

        verifier.verify()
          .map_err(|e| {let e: crate::error::Error = e.into(); e})
          .map_err(|e| JsValue::from_serde(&e).expect("error serde"))?;

        Ok(verifier.print_world())
    }
}
