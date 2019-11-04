use biscuit::token::builder;
use biscuit::token;
use biscuit::token::default_symbol_table;
use biscuit::datalog::{self, SymbolTable};
use wasm_bindgen::prelude::*;
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use std::default::Default;
use std::collections::HashSet;
use std::time::{SystemTime, Duration};

use super::Biscuit;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Atom {
  pub(crate) integer: Option<i64>,
  pub(crate) string: Option<String>,
  pub(crate) symbol: Option<String>,
  pub(crate) date: Option<u64>,
  pub(crate) variable: Option<u32>,
}

impl Atom {
  pub fn into_atom(self) -> builder::Atom {
    let Atom { integer, string, symbol, date, variable } = self;

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
      panic!("invalid atom: {:?}", Atom { integer, string, symbol, date, variable });
    }
  }
}

#[wasm_bindgen]
pub fn integer(i: i64) -> JsValue {
  JsValue::from_serde(&Atom { integer: Some(i), ..Default::default() }).unwrap()
}

#[wasm_bindgen]
pub fn string(s: &str) -> JsValue {
  JsValue::from_serde(&Atom { string: Some(s.to_string()), ..Default::default() }).unwrap()
}

#[wasm_bindgen]
pub fn symbol(s: &str) -> JsValue {
  JsValue::from_serde(&Atom { symbol: Some(s.to_string()), ..Default::default() }).unwrap()
}

#[wasm_bindgen]
pub fn date(i: u64) -> JsValue {
  JsValue::from_serde(&Atom { date: Some(i), ..Default::default() }).unwrap()
}

#[wasm_bindgen]
pub fn variable(i: u32) -> JsValue {
  JsValue::from_serde(&Atom { variable: Some(i), ..Default::default() }).unwrap()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Predicate {
  pub name: String,
  pub ids: Vec<Atom>,
}

impl Predicate {
  pub fn into_predicate(mut self) -> builder::Predicate {
    builder::Predicate {
      name: self.name,
      ids: self.ids.drain(..).map(|a| a.into_atom()).collect(),
    }
  }
}

#[wasm_bindgen]
pub struct Fact(pub(crate) Predicate);

impl Fact {
    pub fn convert(&self, symbols: &mut SymbolTable) -> datalog::Fact {
        datalog::Fact {
            predicate: self.0.clone().into_predicate().convert(symbols),
        }
    }

    pub fn into_fact(self) -> builder::Fact {
      builder::Fact(self.0.into_predicate())
    }
}

#[wasm_bindgen]
pub fn fact(name: &str, ids: JsValue) -> Fact {
    let ids: Vec<Atom> = ids.into_serde().expect("incorrect atom vec");
    Fact(Predicate { name: name.to_string(), ids})
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Constraint {
  pub id: u32,
  pub kind: ConstraintKind,
  pub operation: String,
  pub data: ConstraintData,
}

/*
#[wasm_bindgen]
pub fn constraint_test() -> JsValue {
  let c = Constraint {
    id: 42,
    kind: ConstraintKind::Integer,
    operation: "in".to_string(),
    data: ConstraintData::IntegerSet((vec![1, 2]).drain(..).collect()),
  };

  JsValue::from_serde(&c).unwrap()
}
*/

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConstraintKind {
  Integer,
  String,
  Date,
  Symbol,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConstraintData {
  Integer(i64),
  IntegerSet(HashSet<i64>),
  String(String),
  StringSet(HashSet<String>),
  SymbolSet(HashSet<String>),
  Date(u32),
}

impl Constraint {
    pub fn into_constraint(self) -> builder::Constraint {
      let kind = match (self.kind, self.operation.as_str(), self.data) {
        (ConstraintKind::Integer, "<", ConstraintData::Integer(i)) => builder::ConstraintKind::Integer(builder::IntConstraint::Lower(i)),
        (ConstraintKind::Integer, ">", ConstraintData::Integer(i)) => builder::ConstraintKind::Integer(builder::IntConstraint::Larger(i)),
        (ConstraintKind::Integer, "<=", ConstraintData::Integer(i)) => builder::ConstraintKind::Integer(builder::IntConstraint::LowerOrEqual(i)),
        (ConstraintKind::Integer, ">=", ConstraintData::Integer(i)) => builder::ConstraintKind::Integer(builder::IntConstraint::LargerOrEqual(i)),
        (ConstraintKind::Integer, "=", ConstraintData::Integer(i)) => builder::ConstraintKind::Integer(builder::IntConstraint::Equal(i)),
        (ConstraintKind::Integer, "in", ConstraintData::IntegerSet(s)) => builder::ConstraintKind::Integer(builder::IntConstraint::In(s)),
        (ConstraintKind::Integer, "not in", ConstraintData::IntegerSet(s)) => builder::ConstraintKind::Integer(builder::IntConstraint::NotIn(s)),

        (ConstraintKind::String, "prefix", ConstraintData::String(s)) => builder::ConstraintKind::String(builder::StrConstraint::Prefix(s)),
        (ConstraintKind::String, "suffix", ConstraintData::String(s)) => builder::ConstraintKind::String(builder::StrConstraint::Suffix(s)),
        (ConstraintKind::String, "=", ConstraintData::String(s)) => builder::ConstraintKind::String(builder::StrConstraint::Equal(s)),
        (ConstraintKind::String, "in", ConstraintData::StringSet(s)) => builder::ConstraintKind::String(builder::StrConstraint::In(s)),
        (ConstraintKind::String, "not in", ConstraintData::StringSet(s)) => builder::ConstraintKind::String(builder::StrConstraint::NotIn(s)),

        (ConstraintKind::Date, "<", ConstraintData::Date(i)) => {
          let t = SystemTime::UNIX_EPOCH + Duration::from_secs(i as u64);
          builder::ConstraintKind::Date(builder::DateConstraint::Before(t))
        }
        (ConstraintKind::Date, ">", ConstraintData::Date(i)) => {
          let t = SystemTime::UNIX_EPOCH + Duration::from_secs(i as u64);
          builder::ConstraintKind::Date(builder::DateConstraint::After(t))
        }

        (ConstraintKind::Symbol, "in", ConstraintData::StringSet(s)) => builder::ConstraintKind::Symbol(builder::SymbolConstraint::In(s)),
        (ConstraintKind::Symbol, "not in", ConstraintData::StringSet(s)) => builder::ConstraintKind::Symbol(builder::SymbolConstraint::NotIn(s)),
        (k, _, d) => panic!("invalid constraint: {:?}", Constraint { id: self.id, kind: k, operation: self.operation, data: d }),
      };

      builder::Constraint {
        id: self.id, kind
      }
    }
}


#[wasm_bindgen()]
#[derive(Debug, Clone, PartialEq)]
pub struct Rule{
    pub(crate) head_name: String,
    pub(crate) head_ids: Vec<Atom>,
    pub(crate) predicates: Vec<Predicate>,
    pub(crate) constraints: Vec<Constraint>,
}

impl Rule {
    pub fn into_rule(mut self) -> builder::Rule {
        let head_ids = self.head_ids.drain(..).map(|a| a.into_atom()).collect::<Vec<_>>();
        let predicates = self.predicates.drain(..).map(|p| p.into_predicate()).collect::<Vec<_>>();
        let constraints = self.constraints.drain(..).map(|p| p.into_constraint()).collect::<Vec<_>>();
        builder::constrained_rule(&self.head_name, &head_ids, &predicates, &constraints)
    }
}

#[wasm_bindgen]
pub fn rule(
    head_name: &str,
    head_ids: JsValue,
    predicates: JsValue,
) -> Rule {
    let head_ids: Vec<Atom> = head_ids.into_serde().expect("incorrect atom vec");
    let predicates: Vec<Predicate> = predicates.into_serde().unwrap();
    Rule {
        head_name: head_name.to_string(),
        head_ids,
        predicates,
        constraints: vec![],
    }
}

#[wasm_bindgen]
pub fn constrained_rule(
    head_name: &str,
    head_ids: JsValue,
    predicates: JsValue,
    constraints: JsValue,
) -> Rule {
    let head_ids: Vec<Atom> = head_ids.into_serde().expect("incorrect atom vec");
    let predicates: Vec<Predicate> = predicates.into_serde().unwrap();
    let constraints: Vec<Constraint> = constraints.into_serde().unwrap();

    Rule {
        head_name: head_name.to_string(),
        head_ids,
        predicates,
        constraints,
    }
}

#[wasm_bindgen()]
pub struct BiscuitBuilder {
    pub(crate) symbols: SymbolTable,
    pub(crate) facts: Vec<Fact>,
    pub(crate) rules: Vec<Rule>,
    pub(crate) caveats: Vec<Rule>,
}

#[wasm_bindgen()]
impl BiscuitBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(base_symbols: JsValue) -> Self {
        let symbol_strings: Vec<String> = base_symbols.into_serde().expect("Can't format symbols table");
        let symbols = SymbolTable { symbols: symbol_strings };
        Self {
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
            symbols,
            facts: vec![],
            rules: vec![],
            caveats: vec![],
        }
    }

    #[wasm_bindgen(js_name = addAuthorityFact)]
    pub fn add_authority_fact(&mut self, fact: Fact) {
        self.facts.push(fact);
    }

    #[wasm_bindgen(js_name = addAuthorityRule)]
    pub fn add_authority_rule(&mut self, rule_bind: Rule) {
        self.rules.push(rule_bind);
    }

    #[wasm_bindgen(js_name = addAuthorityCaveat)]
    pub fn add_authority_caveat(&mut self, rule_bind: Rule) {
        self.caveats.push(rule_bind);
    }

    #[wasm_bindgen(js_name = addRight)]
    pub fn add_right(&mut self, resource: &str, right: &str) {
        self.add_authority_fact(Fact(Predicate{
            name: "right".to_string(),
            ids: vec![
              Atom { string: Some("authority".to_string()), ..Default::default()},
              Atom { string: Some(resource.to_string()), ..Default::default()},
              Atom { symbol: Some(right.to_string()), ..Default::default() }],
        }));
    }

    #[wasm_bindgen]
    pub fn build(self, root: crate::crypto::KeyPair) -> Result<Biscuit, JsValue> {
        let mut rng = OsRng::new().expect("os range");
        let symbols = self.symbols;
        let mut builder = token::Biscuit::builder_with_symbols(&mut rng, &root.0, symbols);

        for fact in self.facts {
          builder.add_authority_fact(&fact.into_fact());
        }

        for rule in self.rules {
          builder.add_authority_rule(&rule.into_rule());
        }

        for caveat in self.caveats {
          builder.add_authority_caveat(&caveat.into_rule());
        }

        builder.build()
            .map_err(|e| { let e: crate::error::Error = e.into(); e})
            .map_err(|e| JsValue::from_serde(&e).unwrap())
            .map(Biscuit)
    }
}

