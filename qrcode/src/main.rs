
static MJPG_FOURCC: &[u8; 4] = &[0x4d, 0x4a, 0x50, 0x47];
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mjpg_forcc = v4l::format::fourcc::FourCC::new(MJPG_FOURCC);
    let mut device = v4l::device::Device::with_path("/dev/video0")?;
    use v4l::video::Capture;
    let formats = device.enum_formats()?;
    let mjpeg_supported = formats.iter().any(|format| format.fourcc == mjpg_forcc);
    assert!(mjpeg_supported);
    let sizes = device.enum_framesizes(mjpg_forcc)?;
    let expect_format = v4l::format::Format::new(1280, 1280, mjpg_forcc);
    let actual_format = device.set_format(&expect_format)?;
    dbg!(actual_format);
    let mut stream = v4l::io::mmap::stream::Stream::with_buffers(
        &mut device,
        v4l::buffer::Type::VideoCapture,
        2
    )?;
    let decoder = bardecoder::default_decoder();
    loop {
        use v4l::io::traits::CaptureStream;
        let (buf, _) = stream.next()?;
        let reader = image::io::Reader::with_format(std::io::Cursor::new(buf), image::ImageFormat::JPEG);
        let img = reader.decode()?;
        let results = decoder.decode(&img);
        dbg!(results);
    }
}
