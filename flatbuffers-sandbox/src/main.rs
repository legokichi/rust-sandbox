use serde::{Serialize, Deserialize};

schemafy::schemafy!(
    root: Schema // Optional name for the root type (if one exists)
    "src/schema.json"
);


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nested: Schema = serde_json::from_str(r#"{ "append": "abc" }"#)?;
    println!("{:?}", nested);
    Ok(())
}


// #[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use serde::Serialize;
use typescript_definitions::TypescriptDefinition;
/*
#[derive(Serialize, TypescriptDefinition)]
#[serde(tag = "tag", content = "fields")]
/// Important info about Enum
enum Enum {
    V1 {
        #[serde(rename = "Foo")]
        foo: bool,
    },
    V2 {
        #[serde(rename = "Bar")]
        bar: i64,
        #[serde(rename = "Baz")]
        baz: u64,
    },
    V3 {
        #[serde(rename = "Quux")]
        quux: String,
    },
    #[serde(skip)]
    Internal {
        err: String
    },
}*/