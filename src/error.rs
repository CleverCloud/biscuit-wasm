use biscuit::error;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum Error {
    InternalError,
    /// error deserializing or verifying the token
    Format(Format),
    /// the authority block must have the index 0
    InvalidAuthorityIndex(u32),
    /// the block index does not match its position
    InvalidBlockIndex(InvalidBlockIndex),
    /// multiple blocks declare the same symbols
    SymbolTableOverlap,
    /// the symbol table is missing either "authority" or "ambient"
    MissingSymbols,
    /// tried to append a block to a sealed token
    Sealed,
    /// caveat validation failed
    FailedLogic(Logic),
    /// Datalog parsing error
    ParseError,
}

impl From<error::Token> for Error {
    fn from(e: error::Token) -> Self {
      match e {
        error::Token::InternalError => Error::InternalError,
        error::Token::Format(f) => Error::Format(f.into()),
        error::Token::InvalidAuthorityIndex(i) => Error::InvalidAuthorityIndex(i),
        error::Token::InvalidBlockIndex(i) => Error::InvalidBlockIndex(i.into()),
        error::Token::SymbolTableOverlap => Error::SymbolTableOverlap,
        error::Token::MissingSymbols => Error::MissingSymbols,
        error::Token::Sealed => Error::Sealed,
        error::Token::FailedLogic(l) => Error::FailedLogic(l.into()),
        error::Token::ParseError => Error::ParseError,
      }
    }
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvalidBlockIndex {
    pub expected: u32,
    pub found: u32,
}
impl From<error::InvalidBlockIndex> for InvalidBlockIndex {
    fn from(e: error::InvalidBlockIndex) -> Self {
      InvalidBlockIndex { expected: e.expected, found: e.found }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum Format {
    /// failed verifying the signature
    Signature(Signature),
    /// failed verifying the signature of a sealed token
    SealedSignature,
    /// the token does not provide intermediate public keys
    EmptyKeys,
    /// the root public key was not recognized
    UnknownPublicKey,
    /// could not deserialize the wrapper object
    DeserializationError(String),
    /// could not serialize the wrapper object
    SerializationError(String),
    /// could not deserialize the block
    BlockDeserializationError(String),
    /// could not serialize the block
    BlockSerializationError(String),
}

impl From<error::Format> for Format {
    fn from(e: error::Format) -> Self {
      match e {
        error::Format::Signature(s) => Format::Signature(s.into()),
        error::Format::SealedSignature => Format::SealedSignature,
        error::Format::EmptyKeys => Format::EmptyKeys,
        error::Format::UnknownPublicKey => Format::UnknownPublicKey,
        error::Format::DeserializationError(s) => Format::DeserializationError(s),
        error::Format::SerializationError(s) => Format::SerializationError(s),
        error::Format::BlockDeserializationError(s) => Format::BlockDeserializationError(s),
        error::Format::BlockSerializationError(s) => Format::BlockSerializationError(s),
      }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Signature {
    /// could not parse the signature elements
    InvalidFormat,
    /// the signature did not match
    InvalidSignature,
}

impl From<error::Signature> for Signature {
    fn from(e: error::Signature) -> Self {
      if e == error::Signature::InvalidFormat {
        Signature::InvalidFormat
      } else {
        Signature::InvalidSignature
      }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum Logic {
    /// a fact of the authority block did not have the authority tag
    InvalidAuthorityFact(String),
    /// a fact provided or generated by the verifier did not have the ambient tag
    InvalidAmbientFact(String),
    /// a fact provided or generated by a block had the authority or ambient tag
    InvalidBlockFact(InvalidBlockFact),
    /// a rule provided by a block generates facts the authority or ambient tag
    InvalidBlockRule(InvalidBlockRule),
    /// list of caveats that failed validation
    FailedCaveats(Vec<FailedCaveat>),
}

impl From<error::Logic> for Logic {
    fn from(e: error::Logic) -> Self {
      match e {
        error::Logic::InvalidAuthorityFact(s) => Logic::InvalidAuthorityFact(s),
        error::Logic::InvalidAmbientFact(s) => Logic::InvalidAmbientFact(s),
        error::Logic::InvalidBlockFact(i, s) => Logic::InvalidBlockFact(InvalidBlockFact { block_id: i, fact: s }),
        error::Logic::InvalidBlockRule(i, s) => Logic::InvalidBlockRule(InvalidBlockRule { block_id: i, rule: s }),
        error::Logic::FailedCaveats(mut v) => Logic::FailedCaveats(v.drain(..).map(|e| e.into()).collect()),
      }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvalidBlockFact {
  block_id: u32,
  fact: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvalidBlockRule {
  block_id: u32,
  rule: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum FailedCaveat {
    /// a caveat failed in a block
    Block(FailedBlockCaveat),
    /// a caveat provided by the verifier failed
    Verifier(FailedVerifierCaveat),
}

impl From<error::FailedCaveat> for FailedCaveat {
    fn from(e: error::FailedCaveat) -> Self {
      match e {
        error::FailedCaveat::Block(error::FailedBlockCaveat { block_id, caveat_id, rule }) => FailedCaveat::Block(FailedBlockCaveat { block_id, caveat_id, rule }),
        error::FailedCaveat::Verifier(error::FailedVerifierCaveat { caveat_id, rule }) => FailedCaveat::Verifier(FailedVerifierCaveat { caveat_id, rule }),
      }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailedBlockCaveat {
    pub block_id: u32,
    pub caveat_id: u32,
    /// pretty print of the rule that failed
    pub rule: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailedVerifierCaveat {
    pub caveat_id: u32,
    /// pretty print of the rule that failed
    pub rule: String,
}


