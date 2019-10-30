use biscuit::token::builder;
use biscuit::token::{Biscuit, Block};
use biscuit::token::default_symbol_table;
use biscuit::crypto::KeyPair;
use biscuit::datalog::{self, SymbolTable};
use wasm_bindgen::prelude::*;
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use std::default::Default;

use super::BiscuitBinder;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AtomBind {
  integer: Option<i64>,
  string: Option<String>,
  symbol: Option<String>,
  date: Option<u64>,
  variable: Option<u32>,
}

impl AtomBind {
  pub fn into_atom(self) -> builder::Atom {
    let AtomBind { integer, string, symbol, date, variable } = self;

    if let Some(i) = integer {
      builder::int(i)
    } else if let Some(s) = string {
      builder::string(&s)
    } else if let Some(s) = symbol {
      builder::symbol(&s)
    } else if let Some(i) = date {
      builder::Atom::Date(i)
    } else if let Some(i) = variable {
      builder::variable(i)
    } else {
      panic!("invalid atom: {:?}", AtomBind { integer, string, symbol, date, variable });
    }
  }
}

#[wasm_bindgen]
pub fn integer(i: i64) -> JsValue {
  JsValue::from_serde(&AtomBind { integer: Some(i), ..Default::default() }).unwrap()
}

#[wasm_bindgen]
pub fn string(s: &str) -> JsValue {
  JsValue::from_serde(&AtomBind { string: Some(s.to_string()), ..Default::default() }).unwrap()
}

#[wasm_bindgen]
pub fn symbol(s: &str) -> JsValue {
  JsValue::from_serde(&AtomBind { symbol: Some(s.to_string()), ..Default::default() }).unwrap()
}

#[wasm_bindgen]
pub fn date(i: u64) -> JsValue {
  JsValue::from_serde(&AtomBind { date: Some(i), ..Default::default() }).unwrap()
}

#[wasm_bindgen]
pub fn variable(i: u32) -> JsValue {
  JsValue::from_serde(&AtomBind { variable: Some(i), ..Default::default() }).unwrap()
}

//#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredicateBind {
  pub name: String,
  pub ids: Vec<AtomBind>,
}

impl PredicateBind {
  pub fn into_predicate(mut self) -> builder::Predicate {
    builder::Predicate {
      name: self.name,
      ids: self.ids.drain(..).map(|a| a.into_atom()).collect(),
    }
  }
}

#[wasm_bindgen]
pub struct FactBind(pub(crate) PredicateBind);

impl FactBind {
    pub fn convert(&self, symbols: &mut SymbolTable) -> datalog::Fact {
        datalog::Fact {
            predicate: self.0.clone().into_predicate().convert(symbols),
        }
    }

    pub fn into_fact(self) -> builder::Fact {
      builder::Fact(self.0.into_predicate())
    }
}

#[wasm_bindgen(js_name = fact)]
pub fn fact_bind(name: &str, ids: JsValue) -> FactBind {
    let ids: Vec<AtomBind> = ids.into_serde().expect("incorrect atom vec");
    FactBind(PredicateBind { name: name.to_string(), ids})
}

#[wasm_bindgen()]
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RuleBind{
    //rule: builder::Rule
    head_name: String,
    head_ids: Vec<AtomBind>,
    predicates: Vec<PredicateBind>,
    // FIXME: constraints
}

impl RuleBind {
    pub fn get_inner_rule(mut self) -> builder::Rule {
        //self.rule
        let head_ids = self.head_ids.drain(..).map(|a| a.into_atom()).collect::<Vec<_>>();
        let predicates = self.predicates.drain(..).map(|p| p.into_predicate()).collect::<Vec<_>>();
        builder::rule(&self.head_name, &head_ids, &predicates)
    }
}

#[wasm_bindgen(js_name = rule)]
pub fn rule_bind(
    head_name: &str,
    head_ids: JsValue,
    predicates: JsValue,
) -> RuleBind {
    let head_ids: Vec<AtomBind> = head_ids.into_serde().expect("incorrect atom vec");
    //let head_ids: Vec<builder::Atom> = head_ids.drain(..).map(|a| a.into_atom()).collect();
    let predicates: Vec<PredicateBind> = predicates.into_serde().unwrap();
    RuleBind {
        head_name: head_name.to_string(),
        head_ids,
        predicates,
        //rule: builder::rule(head_name, head_ids.as_slice(), &predicates),
    }
}

#[wasm_bindgen()]
pub struct BiscuitBuilderBind {
    symbols_start: usize,
    symbols: SymbolTable,
    facts: Vec<datalog::Fact>,
    rules: Vec<datalog::Rule>,
    caveats: Vec<datalog::Rule>,
}

#[wasm_bindgen()]
impl BiscuitBuilderBind {
    #[wasm_bindgen(constructor)]
    pub fn new(base_symbols: JsValue) -> Self {
        let symbol_strings: Vec<String> = base_symbols.into_serde().expect("Can't format symbols table");
        let symbols = SymbolTable { symbols: symbol_strings };
        Self {
            symbols_start: symbols.symbols.len(),
            symbols,
            facts: vec![],
            rules: vec![],
            caveats: vec![],
        }
    }

    #[wasm_bindgen(js_name = newWithDefaultSymbols)]
    pub fn new_with_default_symbols() -> Self {
        let symbols = default_symbol_table();
        Self {
            symbols_start: symbols.symbols.len(),
            symbols,
            facts: vec![],
            rules: vec![],
            caveats: vec![],
        }
    }

    #[wasm_bindgen(js_name = addAuthorityFact)]
    pub fn add_authority_fact(&mut self, mut fact: FactBind) {
        /*FIXME: use the BiscuitBuilder API
        let authority_symbol = builder::Atom::Symbol("authority".to_string());
        if fact.0.ids.is_empty() || fact.0.ids[0] != authority_symbol {
            fact.0.ids.insert(0, authority_symbol);
        }*/

        let f = fact.convert(&mut self.symbols);
        self.facts.push(f);
    }

    #[wasm_bindgen(js_name = addAuthorityRule)]
    pub fn add_authority_rule(&mut self, mut rule_bind: RuleBind) {
        /*FIXME: use the BiscuitBuilder API
        let authority_symbol = builder::Atom::Symbol("authority".to_string());
        if rule_bind.rule.0.ids.is_empty() || rule_bind.rule.0.ids[0] != authority_symbol {
            rule_bind.rule.0.ids.insert(0, authority_symbol);
        }*/

        let r = rule_bind.get_inner_rule().convert(&mut self.symbols);
        self.rules.push(r);
    }

    #[wasm_bindgen(js_name = addAuthorityCaveat)]
    pub fn add_authority_caveat(&mut self, rule_bind: RuleBind) {
        let r = rule_bind.get_inner_rule().convert(&mut self.symbols);
        self.caveats.push(r);
    }

    #[wasm_bindgen(js_name = addRight)]
    pub fn add_right(&mut self, resource: &str, right: &str) {
        self.add_authority_fact(FactBind(PredicateBind{
            name: "right".to_string(),
            ids: vec![
              AtomBind { string: Some("authority".to_string()), ..Default::default()},
              AtomBind { string: Some(resource.to_string()), ..Default::default()},
              AtomBind { symbol: Some(right.to_string()), ..Default::default() }],
        }));
    }

    #[wasm_bindgen]
    pub fn build(mut self, root: crate::crypto::KeyPairBind) -> Result<BiscuitBinder, JsValue> {
        let mut rng = OsRng::new().expect("os range");
        let new_syms = self.symbols.symbols.split_off(self.symbols_start);

        self.symbols.symbols = new_syms;

        let authority_block = Block {
            index: 0,
            symbols: self.symbols,
            facts: self.facts,
            rules: self.rules,
            caveats: self.caveats,
        };

        Biscuit::new(&mut rng, &root.0, authority_block)
            .map_err(|e| { let e: crate::error::Error = e.into(); e})
            .map_err(|e| JsValue::from_serde(&e).unwrap())
            .map(|biscuit| BiscuitBinder(biscuit))
    }
}

