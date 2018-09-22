use stdweb::*;
use stdweb::unstable::*;
use stdweb::web::*;
use stdweb::web::html_element::*;

fn main() {
    stdweb::initialize();
    set_timeout(init, 0);
    stdweb::event_loop();
}

fn init(){
    let mut signals: Vec<f64> = vec![0.0; 1024];
    for i in 0..signals.len() {
        signals[i] = (i as f64/10.0).sin();
    }

    let width: f64 = signals.len() as f64;
    let height: f64 = 800.0;

    let cnv: Element = document().create_element("canvas").unwrap();
    let cnv: CanvasElement = TryFrom::try_from(cnv).unwrap();
    cnv.set_width(width as u32);
    cnv.set_height(height as u32);
    js!( @{&cnv}.style.boxSizing = "border-box"; );
    js!( @{&cnv}.style.border = "1px solid black"; );
    let ctx: CanvasRenderingContext2d = cnv.get_context().unwrap();
    
    ctx.begin_path();
    ctx.move_to(0_f64, height as f64/2_f64);
    for (i, val) in signals.iter().enumerate() {
        ctx.line_to(i as f64, (1.0 + val) * height as f64/2_f64);
    }
    ctx.stroke();
    let body: HtmlElement = document().body().unwrap();
    body.append_child(&cnv);
    
}