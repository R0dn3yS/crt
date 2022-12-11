mod display;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use display::Display;

fn main() {
  let mut crt: Display = Display::new(1280, 720, 25);

  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem
    .window("CRT", crt.width, crt.height)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

  let mut canvas = window.into_canvas().index(find_sdl_gl_driver().unwrap()).present_vsync().build().unwrap();
  canvas.clear();
  canvas.present();

  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut frame_file = 1;

  'crtloop: loop {
    for evt in event_pump.poll_iter() {
      match evt {
        Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => {
          break 'crtloop
        },
        _ => ()
      }
    }

    let file_path = ("/home/rodney/.video/".to_string() + &pad_start(frame_file.to_string(), 6, "0") + ".png").to_owned();

    crt.draw_screen(&mut canvas, &file_path);

    crt.scanline_h += 1;
    if crt.scanline_h == crt.width {
      crt.scanline_h = 0;
      crt.scanline_v += 1;
      if crt.scanline_v == crt.height {
        crt.scanline_v = 0;
        frame_file += 1;
      }
    }
  }
}

fn find_sdl_gl_driver() -> Option<u32> {
  for (index, item) in sdl2::render::drivers().enumerate() {
    if item.name == "opengl" {
      return Some(index as u32);
    }
  }
  None
}

fn pad_start(text: String, length: u8, fill: &str) -> String {
  let to_fill = length - text.chars().count() as u8;

  let mut new_str: String = "".to_string();

  for _i in 0..to_fill {
    new_str += &fill;
  }

  new_str + &text
}
