use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[serde(tag = "type", content = "value", rename_all="camelCase")]
pub enum Piyo {
    Unit,
    Hoge(super::hoge::Hoge),
    Fuga(super::fuga::Fuga),
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = r###"
export type Piyo =
    | { type: "hoge"; value: Hoge; }
    | { type: "fuga"; value: Fuga; }
    ;
export function piyo(o: Piyo): Piyo;
"###;

    #[wasm_bindgen(skip_typescript)]
    pub fn piyo(o: JsValue) -> Result<JsValue, JsValue> {
        let piyo: Piyo = serde_wasm_bindgen::from_value(o)?;
        let o = serde_wasm_bindgen::to_value(&piyo)?;
        Ok(o)
    }
}
