use std::fmt::Display;

use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub enum TsType {
    Basic(String),
    Array(Box<TsType>),
    Map(Box<TsType>, Box<TsType>),
    Union(Vec<TsType>),
}

impl Display for TsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", pretty_json)
    }
}
