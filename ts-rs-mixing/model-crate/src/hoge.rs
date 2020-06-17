use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Hoge {
    pub foo: String,
    pub bar: String,
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;
    use js_sys::{Error};

    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = r###"
export interface Hoge {
    foo: string;
    bar: string;
}
export function hoge(o: Hoge): Hoge;
"###;

    #[wasm_bindgen(skip_typescript)]
    pub fn hoge(o: JsValue) -> Result<JsValue, JsValue> {
        let hoge: Hoge = o.into_serde::<Hoge>()
            .map_err(into)?;
        let o = JsValue::from_serde(&hoge)
            .map_err(into)?;
        Ok(o)
    }

    fn into(err: serde_json::Error) -> Error {
        Error::new(&format!("{}", err))
    }
}
