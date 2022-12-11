use std::path::Path;

use image::{GenericImageView, Pixel, Rgb};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Display {
  pub width: u32,
  pub height: u32,
  pub fps: u8,
  pub scanline_h: u32,
  pub scanline_v: u32,
}

impl Display {
  pub fn new(width: u32, height: u32, fps: u8) -> Self {
    Display {
      width,
      height,
      fps,
      scanline_h: 0,
      scanline_v: 718,
    }
  }

  pub fn draw_screen(&mut self, canvas: &mut Canvas<Window>, path: &str) {
    let pixel = self.get_pixel(path);

    canvas.set_draw_color(Color::RGB(pixel[0], pixel[1], pixel[2]));
    let rect = Rect::new(self.scanline_h as i32, self.scanline_v as i32, 1, 1);
    canvas.fill_rect(rect).unwrap();
    canvas.present();
  }

  fn get_pixel(&self, path: &str) -> Rgb<u8> {
    let img = image::open(&Path::new(path)).unwrap();

    img.get_pixel(self.scanline_h, self.scanline_v).to_rgb()
  }
}