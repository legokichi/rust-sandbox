extern crate rayon;
//extern crate cv;
extern crate opencv;
extern crate zmq;
extern crate libc;

use opencv::core::{Size, Mat};
use opencv::highgui::{VideoCapture, VideoWriter, CV_CAP_PROP_FRAME_WIDTH, CV_CAP_PROP_FRAME_HEIGHT, CV_CAP_PROP_FPS};
use libc::{ c_void, c_char, size_t };
use std::ffi::{ CStr, CString };

#[link(name = "hello", kind = "static")]
extern "C" {
  fn hello();
}

use rayon::prelude::*;

fn main() {
  std::thread::spawn(|| {
    unsafe{
      hello();
    }
  });
  let mut reader = VideoCapture::new("/home/legokichi/Github/a.mp4").unwrap();
  let width = VideoCapture::get(&mut reader, CV_CAP_PROP_FRAME_WIDTH).unwrap() as i32;
  let height = VideoCapture::get(&mut reader, CV_CAP_PROP_FRAME_HEIGHT).unwrap() as i32;
  let fps = VideoCapture::get(&mut reader, CV_CAP_PROP_FPS).unwrap() as f64;
  println!("rust: {},{},{}", width, height, fps);
  let mut writer = VideoWriter::new("appsrc ! videoconvert ! x264enc ! mpegtsmux ! filesink location=rust.mp4  ", 0, fps, Size{ width: width, height: height }, true).unwrap();
  if ! VideoCapture::is_opened(&reader).unwrap() {
    println!("rust: not opened");
    std::process::exit(1);
  }
  if ! VideoWriter::is_opened(&writer).unwrap() {
    println!("rust: not opened");
    std::process::exit(1);
  }
  let mut i = 0_u32;
  loop {
    let mat = Mat::new().unwrap();
    if ! reader.read(&mat).unwrap() { std::process::exit(0); }
    let size = Mat::size(&mat).unwrap();
    println!("rust: {},{},{},{}", i, size.width, size.height, Mat::channels(&mat).unwrap());
    writer.write(&mat).unwrap();
    i += 1;
  }
}
