extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

extern crate life;
use life::game::Board;
use life::gui::BoardRenderer;

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_ctx = sdl_context.video().unwrap();

  let window = sdl2::video::WindowBuilder::new(&video_ctx, "My window", 640, 640).build().unwrap();
  let mut renderer = window.renderer().present_vsync().build().unwrap();

  let mut life = Board::random();
  let board_renderer = BoardRenderer::new(Rect::new(0, 0, 640, 640));

  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut exit = false;
  while !exit {
    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();

    board_renderer.draw(&life, &mut renderer).unwrap();

    renderer.present();

    use sdl2::event::Event;
    match event_pump.wait_event_timeout(50) {
      Some(Event::KeyDown {..}) => {
        println!("Resetting after {} generations", life.generation);
        life = Board::random();
      }
      Some(Event::Quit {..}) => { exit = true; }
      Some(Event::Window {win_event_id: we, ..}) => { println!("{:?}", we); }
      Some(e) => { println!("{:?}", e); }
      _ => ()
    }

    life = life.next();
  }
}
