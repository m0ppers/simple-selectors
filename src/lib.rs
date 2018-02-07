//! Parser for k8s style selectors
//!
//! Simple label matching syntax as in `label = value`

#[macro_use]
#[cfg(test)]
extern crate assert_matches;
extern crate itertools;

mod parser;
mod parseerror;

pub use parser::LabelMap;
pub use parser::parse;
pub use parseerror::ParseError;
