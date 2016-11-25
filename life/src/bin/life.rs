extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

extern crate life;
use life::Life;

struct LifeRenderer {}
impl LifeRenderer {
  fn draw(board: &Life, renderer: &mut Renderer) -> Result<(), String> {
    let (w, h) = renderer.window().unwrap().size();
    let rows = board.size().0 as u32;
    let cols = board.size().1 as u32;

    let line_width = 1;
    let total_line_width_h = line_width * (cols-1);
    let total_line_width_v = line_width * (rows-1);
    let cell_width = (w - total_line_width_h) / cols;
    let cell_height = (h - total_line_width_v) / rows;

    let total_cell_width = cell_width + line_width;
    let total_cell_height = cell_height + line_width;

    //try!(renderer.window_mut().unwrap().set_size(
    //  cell_width * cols + line_width * (cols-1),
    //  cell_height * rows + line_width * (rows-1)
    //).or(Err("Could not resize window")));
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

    for r in 0..board.len() {
      for c in 0..board[r].len() {
        if board[r][c] {
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

  let mut life = Life::random();

  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut exit = false;
  while !exit {
    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();

    LifeRenderer::draw(&life, &mut renderer).unwrap();

    renderer.present();

    use sdl2::event::Event;
    match event_pump.wait_event_timeout(50) {
      Some(Event::KeyDown {..}) => {
        println!("Resetting after {} generations", life.generation);
        life = Life::random();
      }
      Some(Event::Quit {..}) => { exit = true; }
      Some(Event::Window {win_event_id: we, ..}) => { println!("{:?}", we); }
      Some(e) => { println!("{:?}", e); }
      _ => ()
    }

    life = life.next();
  }
}
