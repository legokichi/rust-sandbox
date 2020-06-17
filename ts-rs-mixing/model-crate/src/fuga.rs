use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
#[serde(rename_all="camelCase")]
pub struct Fuga {
    #[serde(with = "serde_bytes")]
    pub bytes: Vec<u8>,
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = r###"
export interface Fuga {
    bytes: Uint8Array;
}
export function fuga(o: Fuga): Fuga;
"###;

    #[wasm_bindgen(skip_typescript)]
    pub fn fuga(o: JsValue) -> Result<JsValue, JsValue> {
        let fuga: Fuga = serde_wasm_bindgen::from_value(o)?;
        let o = serde_wasm_bindgen::to_value(&fuga)?;
        Ok(o)
    }
}
