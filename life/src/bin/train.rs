extern crate life;
use life::pattern_finder::Pattern;
use life::gui::BoardRenderer;

extern crate optimizer;
use optimizer::Cost;
use optimizer::annealing::{Annealer, Schedule};

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::event::Event;

/*
fn mean(arr: &[f64]) -> f64 {
  arr.iter().sum::<f64>() / arr.len() as f64
}

fn stddev(arr: &[f64], mu: f64) -> f64 {
  (arr.iter().map(|x| (x - mu) * (x - mu)).sum::<f64>() / arr.len() as f64).sqrt()
}
*/

struct BasicSchedule;
impl Schedule for BasicSchedule {
  fn temp(step: u32, _step_max: u32) -> f64 {
    0.00125 / (1. + (1. + step as f64).ln())
  }
}

fn visit_cb(p: &Pattern, board_renderer: &BoardRenderer, mut renderer: &mut Renderer, event_pump: &mut EventPump) {
  loop {
    match event_pump.poll_event() {
      None => { break; }
      _ => ()
    }
  }
  renderer.set_draw_color(Color::RGB(255, 255, 255));
  renderer.clear();
  board_renderer.draw(&p.starting_board(), &mut renderer).unwrap();
  renderer.present();
}

fn train(start: Pattern, mut renderer: &mut Renderer, mut event_pump: &mut EventPump) -> Pattern {
  let board_renderer = BoardRenderer::new(Rect::new(0, 0, 640, 640)).with_board_rect(life::gui::BoardRect::new(/*95,95*/45,45,10,10));

  let cb = |p: &Pattern| visit_cb(&p, &board_renderer, &mut renderer, &mut event_pump);

  let start_cost = start.cost();
  println!("Start cost: {}", start_cost);

  let mut annealer = Annealer::<Pattern>::new(start);
  annealer.set_visit_cb(Box::new(cb));

  let final_state = annealer.optimize::<BasicSchedule>(7000);
  let end_cost = final_state.cost();
  println!("Start cost:  {}", start_cost);
  println!("End cost:    {}", end_cost);
  println!("Improvement: {}", -(end_cost-start_cost));

  final_state.clone()
}

fn present(pattern: &Pattern, steps: u32, mut renderer: &mut Renderer, event_pump: &mut EventPump) {
  let board_renderer = BoardRenderer::new(Rect::new(0, 0, 640, 640));

  let mut board = pattern.starting_board();
  let mut exit = false;
  let mut step = 0;
  while !exit && step < steps {
    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    board_renderer.draw(&board, &mut renderer).unwrap();
    renderer.present();

    match event_pump.wait_event_timeout(100) {
      Some(Event::Quit {..}) => { exit = true; }
      _ => ()
    }

    board = board.next();
    step += 1;
  }
}

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_ctx = sdl_context.video().unwrap();

  let window = sdl2::video::WindowBuilder::new(&video_ctx, "My window", 640, 640).build().unwrap();
  let mut renderer = window.renderer().present_vsync().build().unwrap();
  let mut event_pump = sdl_context.event_pump().unwrap();

  let start_state = Pattern::random();
  present(&start_state, 200, &mut renderer, &mut event_pump);
  let final_state = train(start_state, &mut renderer, &mut event_pump);
  present(&final_state, 100000, &mut renderer, &mut event_pump);
}
