extern crate cpal;
extern crate gust;


use gust::backend::line_chart::LineChart;
use gust::frontend::write::render_graph;

fn main() {
    let mut lchart = LineChart::new();
    // Setup the default input device and stream with the default input format.
    let device = cpal::default_input_device().expect("Failed to get default input device");
    println!("Default input device: {}", device.name());
    let format = device.default_input_format().expect("Failed to get default input format");
    println!("Default input format: {:?}", format);
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_input_stream(&device, &format).expect("Failed to build input stream");
    println!("{:?}", stream_id);
    event_loop.play_stream(stream_id);
    let mut i = 0_u64;
    let mut ended = false;
    event_loop.run(|stream_id, stream_data| {
        if i > 100000  && !ended {
            event_loop.destroy_stream(stream_id);
            ended = true;
            println!("end");
            render_graph(&lchart, gust::backend::general::FileType::HTML).unwrap();
            return;
        }
        match stream_data {
                cpal::StreamData::Input{ buffer } => {
                    match buffer {
                        cpal::UnknownTypeInputBuffer::U16(buf) => { println!("U16: {}", buf.len()); },
                        cpal::UnknownTypeInputBuffer::I16(buf) => { println!("I16: {}", buf.len()); },
                        cpal::UnknownTypeInputBuffer::F32(buf) => {
                            for j in 0..buf.len() {
                                lchart.add_data((i + j as u64) as i64, (buf[j]*10000_f32) as i64, 0);
                                i += 1;
                            }
                        },
                    }
                },
                _ => unreachable!(),
        }
    });
}
