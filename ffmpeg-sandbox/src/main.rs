extern crate ffmpeg;
use ffmpeg::format::register_all;
use ffmpeg::format::format::list;
use ffmpeg::format::output_as;
use ffmpeg::format::context::output::Output;
use ffmpeg::format::format::Format;
use ffmpeg::StreamMut;
use ffmpeg::codec::context::Context;
use ffmpeg::codec::encoder::encoder::Encoder;
//use ffmpeg::codec::encoder::audio::Encoder;
use ffmpeg::util::format::sample::Sample;
use ffmpeg::util::format::sample::Type;
use ffmpeg::codec::id::Id;
// use ffmpeg::Dictionary;
use ffmpeg::util::frame::audio::Audio;
use ffmpeg::util::channel_layout::STEREO;
use ffmpeg::codec::packet::packet::Packet;
// use ffmpeg::format::Output;
use ffmpeg::util::rational::Rational;

fn main() {
    register_all();
    // for codec in list() {
    //     println!("{}", codec.name());
    // }
    ffmpeg::codec::encoder::find(Id::MP3).unwrap();
    let pathbuf = std::path::Path::new("./a.mp3").to_owned();
    let mut output: Output = output_as(&pathbuf, "mp3").unwrap();
    {
        let mut stream: StreamMut = output.add_stream(Id::MP3).unwrap();
        let mut codec: Context = stream.codec();
        let mut encoder: Encoder = codec.encoder();
        encoder.set_time_base(Rational(1, 44100));
        let mut audio = encoder.audio().unwrap();
        audio.set_format(Sample::F32(Type::Planar));
        audio.set_rate(44100);
        audio.set_channel_layout(STEREO);
        let mut enc = audio.open().unwrap();
        let frame = Audio::empty();
        let mut packet = Packet::new(1024*1024);
        enc.encode(&frame, &mut packet).unwrap();
        let mut packet = Packet::new(1024*1024);
        enc.flush(&mut packet).unwrap();
    }
    output.write_trailer().unwrap();
    ffmpeg::format::network::deinit();
}
