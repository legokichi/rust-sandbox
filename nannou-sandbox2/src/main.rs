use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
let draw = app.draw();

    draw.background().color(BLUE);

    draw.to_frame(app, &frame).unwrap();
}

