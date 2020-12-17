use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use derive_more::{From, Into};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, From, Into)]
struct ISO8601String(
    #[serde(with = "chrono::serde::ts_milliseconds")]
    DateTime<Utc>
);

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &str = load_str!("primitive.d.ts");
