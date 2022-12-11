use image::{GenericImageView, Pixel, Rgb, DynamicImage};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Display {
  pub width: u32,
  pub height: u32,
  pub scanline_h: u32,
  pub scanline_v: u32,
}

impl Display {
  pub fn new(width: u32, height: u32) -> Self {
    Display {
      width,
      height,
      scanline_h: 0,
      scanline_v: 0,
    }
  }

  pub fn draw_screen(&mut self, canvas: &mut Canvas<Window>, img: &DynamicImage) {
    let pixel = self.get_pixel(img);

    canvas.set_draw_color(Color::RGB(pixel[0], pixel[1], pixel[2]));

    let rect = Rect::new(self.scanline_h as i32, self.scanline_v as i32, 0, 0);
    canvas.fill_rect(rect).unwrap();
  }

  fn get_pixel(&self, img: &DynamicImage) -> Rgb<u8> {
    img.get_pixel(self.scanline_h, self.scanline_v).to_rgb()
  }
}