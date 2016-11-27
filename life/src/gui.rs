use game::Board;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

use std::ops::Range;
use std::option::Option;

#[derive(Clone)]
pub struct BoardRect {
  pub r: usize,
  pub c: usize,
  pub rows: usize,
  pub cols: usize
}
impl BoardRect {
  fn new(r: usize, c: usize, rows: usize, cols: usize) -> BoardRect {
    BoardRect { r: r, c: c, rows: rows, cols: cols }
  }
  fn row_range(&self) -> Range<usize> {
    self.r..(self.r + self.rows)
  }
  fn col_range(&self) -> Range<usize> {
    self.c..(self.c + self.cols)
  }
}

pub struct LifeRenderer {
  draw_rect: Rect,
  board_rect: Option<BoardRect>,
}
impl LifeRenderer {
  pub fn new(draw_rect: Rect) -> LifeRenderer {
    LifeRenderer {
      draw_rect: draw_rect,
      board_rect: None
    }
  }

  pub fn draw(&self, board: &Board, renderer: &mut Renderer) -> Result<(), String> {
    let (w, h) = self.draw_rect.size();
    let board_rect = self.board_rect.as_ref().cloned().unwrap_or(
      BoardRect::new(0, 0, board.size().0, board.size().1)
    );

    let line_width = 1;
    let total_line_width_h = line_width * (board_rect.cols-1) as u32;
    let total_line_width_v = line_width * (board_rect.rows-1) as u32;
    let cell_width = (w - total_line_width_h) / board_rect.cols as u32;
    let cell_height = (h - total_line_width_v) / board_rect.rows as u32;

    let total_cell_width = cell_width + line_width;
    let total_cell_height = cell_height + line_width;

    let (w, h) = renderer.window().unwrap().size();

    // Draw lines

    renderer.set_draw_color(Color::RGB(220,220,220));

    for i in 1..board_rect.cols as u32 {
      let offset = (i*total_cell_width - line_width) as i32;
      try!(renderer.fill_rect(Rect::new(offset, 0, line_width, h)));
    }

    for i in 1..board_rect.rows as u32 {
      let offset = (i*total_cell_height - line_width) as i32;
      try!(renderer.fill_rect(Rect::new(0, offset, w, line_width)));
    }

    // Draw blocks

    renderer.set_draw_color(Color::RGB(50,50,220));

    for r in board_rect.row_range() {
      for c in board_rect.col_range() {
        if board[r][c] {
          let x = total_cell_width as i32 * (c - board_rect.c) as i32;
          let y = total_cell_height as i32 * (r - board_rect.r) as i32;
          try!(renderer.fill_rect(Rect::new(x, y, cell_width, cell_height)));
        }
      }
    }

    Ok(())
  }
}
