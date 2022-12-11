mod display;

use std::path::Path;

use display::Display;

fn main() {
  let mut crt: Display = Display::new(1280, 720);

  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem
    .window("CRT", crt.width, crt.height)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

  let mut canvas = window.into_canvas().build().unwrap();
  canvas.clear();
  canvas.present();

  let mut frame_file = 1;
  let mut file_path = ("/home/rodney/.video/".to_string() + &pad_start(frame_file.to_string(), 6, "0") + ".png").to_owned();
  let mut img = image::open(&Path::new(&file_path)).unwrap();

  loop {
    crt.draw_screen(&mut canvas, &img);

    crt.scanline_h += 1;
    if crt.scanline_h == crt.width {
      crt.scanline_h = 0;
      crt.scanline_v += 1;
      canvas.present();
      if crt.scanline_v == crt.height {
        crt.scanline_v = 0;
        frame_file += 1;
        file_path = ("/home/rodney/.video/".to_string() + &pad_start(frame_file.to_string(), 6, "0") + ".png").to_owned();
        img = image::open(&Path::new(&file_path)).unwrap();
      }
    }
  }
}

fn pad_start(text: String, length: u8, fill: &str) -> String {
  let to_fill = length - text.chars().count() as u8;

  let mut new_str: String = "".to_string();

  for _i in 0..to_fill {
    new_str += &fill;
  }

  new_str + &text
}
