#include "hello.h"
#include <cstdlib>
#include <iostream>
#include <string>
#include <opencv2/opencv.hpp>
#include <opencv2/highgui/highgui.hpp>

void hello(){
  std::cout << "hello cpp" << std::endl;
  cv::namedWindow("hi", 1);
  auto reader = cv::VideoCapture{"/home/legokichi/Github/a.mp4"};
  int width = reader.get(CV_CAP_PROP_FRAME_WIDTH);
  int height = reader.get(CV_CAP_PROP_FRAME_HEIGHT);
  double fps = reader.get(CV_CAP_PROP_FPS);
  printf("C++: %d,%d,%f\n", width, height, fps);
  /*
  gst-launch-1.0 -e filesrc location=a.mp4 ! qtdemux ! avdec_h264 ! videoconvert ! x264enc ! mp4mux faststart=true ! filesink location=mp4mux_faststart.mp4 &
  gst-launch-1.0 -e filesrc location=a.mp4 ! qtdemux ! avdec_h264 ! videoconvert ! x264enc ! mp4mux ! filesink location=mp4mux.mp4 &
  gst-launch-1.0 -e filesrc location=a.mp4 ! qtdemux ! avdec_h264 ! videoconvert ! x264enc ! qtmux ! filesink location=qtmux.mp4 &
  gst-launch-1.0 -e filesrc location=a.mp4 ! qtdemux ! avdec_h264 ! videoconvert ! x264enc ! mpegtsmux ! filesink location=mpegtsmux.mp4 &
  */
  auto writer = cv::VideoWriter("appsrc ! videoconvert ! x264enc ! mp4mux faststart=true ! filesink location=cpp.mp4  ", 0, fps, cv::Size{ width, height }, true);
  if(!reader.isOpened()){
    printf("C++: not opened\n");
    exit(1);
  }
  if(!writer.isOpened()){
    printf("C++: not opened\n");
    exit(1);
  }
  auto i = 0;
  while(true){
    auto mat = cv::Mat{};
    if(!reader.read(mat)){ exit(0); }
    auto size = mat.size();
    printf("C++: %d,%d,%d,%d\n", i, size.width, size.height, mat.channels());
    cv::imshow("hi", mat);
    if(cv::waitKey(30) >= 0){ break; }
    writer.write(mat);
    i += 1;
  }
}
