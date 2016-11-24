extern crate sdl2;
extern crate rand;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Renderer;

struct Life {
  a: Box<[[bool; 50]; 50]>
}

impl Life {
  fn new() -> Life {
    let mut a = Box::new([[false; 50]; 50]);

    for i in 0..300 {
      let r = rand::random::<usize>() % 50;
      let c = rand::random::<usize>() % 50;
      a[r][c] = true;
    }

    Life {
      a: a
    }
  }

  fn size(&self) -> (u32, u32) {
    (50, 50)
  }

  fn update(&self) {
  }
}

struct LifeRenderer {}
impl LifeRenderer {
  fn draw(board: &Life, renderer: &mut Renderer) -> Result<(), String> {
    let (w, h) = renderer.window().unwrap().size();
    let (rows, cols) = board.size();

    let line_width = 1;
    let total_line_width_h = line_width * (cols-1);
    let total_line_width_v = line_width * (rows-1);
    let cell_width = (w - total_line_width_h) / cols;
    let cell_height = (h - total_line_width_v) / rows;

    let total_cell_width = cell_width + line_width;
    let total_cell_height = cell_height + line_width;

    try!(renderer.window_mut().unwrap().set_size(
      cell_width * cols + line_width * (cols-1),
      cell_height * rows + line_width * (rows-1)
    ).or(Err("Could not resize window")));
    let (w, h) = renderer.window().unwrap().size();

    renderer.set_draw_color(Color::RGB(220,220,220));

    for i in 1..cols {
      let offset = (i*total_cell_width - line_width) as i32;
      try!(renderer.fill_rect(Rect::new(offset, 0, line_width, h)));
    }

    for i in 1..rows {
      let offset = (i*total_cell_height - line_width) as i32;
      try!(renderer.fill_rect(Rect::new(0, offset, w, line_width)));
    }

    renderer.set_draw_color(Color::RGB(50,50,220));

    for r in 0..board.a.len() {
      for c in 0..board.a[r].len() {
        if board.a[r][c] {
          let x = total_cell_width as i32 * c as i32;
          let y = total_cell_height as i32 * r as i32;
          try!(renderer.fill_rect(Rect::new(x, y, cell_width, cell_height)));
        }
      }
    }

    Ok(())
  }
}

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_ctx = sdl_context.video().unwrap();

  let window = sdl2::video::WindowBuilder::new(&video_ctx, "My window", 640, 640).build().unwrap();
  let mut renderer = window.renderer().present_vsync().build().unwrap();

  let mut life = Life::new();

  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut dirty = true;
  let mut exit = false;
  while !exit {
    if dirty {
      renderer.set_draw_color(Color::RGB(255, 255, 255));
      renderer.clear();

      LifeRenderer::draw(&life, &mut renderer).unwrap();

      renderer.present();
      dirty = false;
    }

    use sdl2::event::Event;
    match event_pump.wait_event() {
      Event::KeyDown {..} => { life = Life::new(); dirty = true; }
      Event::Quit {..} => { exit = true; }
      _ => ()
    }
  }
}
