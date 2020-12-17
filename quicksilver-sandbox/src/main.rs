use quicksilver::{Window, Settings, Graphics, Input, QuicksilverError, Timer};
use quicksilver::graphics::Color;
use quicksilver::geom::{Rectangle, Vector, Transform, Circle};

fn main() {

    quicksilver::run(
        Settings{
            size: Vector::new(300.0, 300.0),
            ..Settings::default()
        },
        app
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<(), QuicksilverError> {
    let mut draw_timer = Timer::time_per_second(15.0);
    let mut i = 0_usize;
    loop {
        while let Some(ev) = input.next_event().await {
            dbg!(ev);
        }
        if draw_timer.exhaust().is_some() {
            gfx.clear(Color::WHITE);
            Randmark::from(Vector::new(10.0, 10.0)).draw(&mut gfx);
            gfx.set_transform(
                Transform::translate(Vector::new(150.0, 150.0))
                * Transform::rotate(-4.0 * i as f32)
            );
            let rect = Rectangle::new(Vector::new(-50.0, -50.0), Vector::new(100.0, 100.0));
            gfx.stroke_rect(&rect, Color::RED);
            gfx.present(&window)?;
            i += 1;
        }
    }
}
//#[derive(Default, Eq, Debug, PartialEq, Ord, PartialOrd,  Hash)]
struct Agent{
    particles: Vec<Vector>,

}
#[derive(Default, Debug, derive_more::From)]
struct Randmark(Vector);
impl Randmark {
    fn draw(&self, gfx: &mut Graphics){
        gfx.stroke_circle(&Circle::new(self.0, 10.0), Color::BLACK);
    }
}
