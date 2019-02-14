extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, World!");
}

use futures::prelude::*;
use wasm_bindgen_futures::{JsFuture, future_to_promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::*;
use web_sys::*;
use serde_derive::*;

#[wasm_bindgen]
pub fn main(){
    let window = window().unwrap();
    let document = window.document().unwrap();
    let media_devices = window.navigator().media_devices().unwrap();
    #[derive(Serialize)]
    struct Constraints {
        audio: bool,
        video: bool,
    }
    let constraints = JsValue::from_serde(&Constraints{audio: true, video: false}).unwrap();
    let prm = media_devices.get_user_media_with_constraints(<MediaStreamConstraints as JsCast>::unchecked_from_js_ref(&constraints)).unwrap();
    {
        let cb = Closure::wrap(Box::new(move |media_stream: JsValue|{
            console::log(&js_sys::Array::from(&JsValue::from_str("Hello, 💩!")));
            let src = Url::create_object_url_with_source(&media_stream.into()).unwrap();
            let audio = HtmlAudioElement::new_with_src(&src).unwrap();
            (audio.as_ref() as &HtmlMediaElement).set_autoplay(true);
            (audio.as_ref() as &HtmlMediaElement).set_controls(true);
            let cb = Closure::wrap(Box::new(move |ev|{
                console::log(&js_sys::Array::from(&JsValue::from_str("Hello, 💩!")));
                let actx = AudioContext::new().unwrap();
                let processor = (actx.as_ref() as &BaseAudioContext).create_script_processor().unwrap();
                let cb = Closure::wrap(Box::new(move |ev: Event|{
                    // let abuf = (ev.as_ref() as &AudioProcessingEvent).input_buffer().unwrap();
                    // やってられん
                    console::log(&js_sys::Array::from(&JsValue::from_str("Hello, 💩!")));
                }) as Box<dyn FnMut(Event)>);
                processor.set_onaudioprocess(Some(cb.as_ref().unchecked_ref()));
                (processor.as_ref() as &AudioNode).connect_with_audio_node((actx.as_ref() as &BaseAudioContext).destination().as_ref());
                cb.forget();
            }) as Box<dyn FnMut(Event)>);
            (audio.as_ref() as &EventTarget).add_event_listener_with_callback("loadedmetadata", cb.as_ref().unchecked_ref()).unwrap();
            (document.body().unwrap().as_ref() as &Node).append_child(audio.as_ref()).unwrap();
            cb.forget();
        }) as Box<dyn FnMut(JsValue)>);
        let prm = prm.then(&cb);
        cb.forget();
    }
    
}