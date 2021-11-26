use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::rc::Rc;
use js_sys::Math::random;

#[wasm_bindgen(start)]
pub fn _start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let cnv = document.create_element("canvas").unwrap();
    let cnv = cnv.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    cnv.set_width(300);
    cnv.set_height(300);
    let ctx = cnv.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    body.append_with_node_1(&cnv).unwrap();

    let recur = Rc::new(RefCell::new(None));
    *recur.borrow_mut() = Some(Closure::wrap(Box::new({
        let recur = recur.clone();
        let x = RefCell::new(100.0);
        let y = RefCell::new(100.0);
        move ||{
            //web_sys::console::log_1(&JsValue::from("Hello, world!"));
            ctx.set_stroke_style(&JsValue::from("black".to_string()));
            ctx.begin_path();
            ctx.move_to(*x.borrow(), *y.borrow());
            *x.borrow_mut() += if random() > 0.5 { 1.0 } else { -1.0 };
            *y.borrow_mut() += if random() > 0.5 { 1.0 } else { -1.0 };
            ctx.line_to(*x.borrow(), *y.borrow());
            ctx.close_path();
            ctx.stroke();
            let window = web_sys::window().unwrap();
            window
                .request_animation_frame((recur.borrow().as_ref().unwrap() as &Closure<dyn FnMut()>).as_ref().unchecked_ref())
                .unwrap();
        }
    }) as Box<dyn FnMut()>));
    let window = web_sys::window().unwrap();
    window
        .request_animation_frame((recur.borrow().as_ref().unwrap() as &Closure<dyn FnMut()>).as_ref().unchecked_ref())
        .unwrap();
}

