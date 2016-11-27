extern crate optimizer;
extern crate rand;
use self::optimizer::{Cost, Neighbor};

use game::Board;

const CANDIDATE_SIZE: usize = 10;
const NEIGHBOR_FLIPS: usize = 1;
const INIT_CELL_PROB: f32   = 0.3;

pub struct Pattern {
  grid: [[bool; CANDIDATE_SIZE]; CANDIDATE_SIZE]
}

impl Pattern {
  pub fn random() -> Pattern {
    let mut p = Pattern{grid: [[false; CANDIDATE_SIZE]; CANDIDATE_SIZE]};
    for r in 0..CANDIDATE_SIZE {
      for c in 0..CANDIDATE_SIZE {
        if rand::random::<f32>() < INIT_CELL_PROB {
          p.grid[r][c] = true;
        }
      }
    }
    p
  }

  // Returns a Board board that contains this candidate in the middle, but is otherwise empty.
  fn starting_board(&self) -> Board {
    let mut board = Board::empty();
    let r_start = board.len() / 2 - CANDIDATE_SIZE / 2;
    let c_start = board[0].len() / 2 - CANDIDATE_SIZE / 2;
    for r in 0..CANDIDATE_SIZE {
      for c in 0..CANDIDATE_SIZE {
        board[r_start+r][c_start+c] = self.grid[r][c];
      }
    }
    board
  }
}

impl Cost for Pattern {
  fn cost(&self) -> f64 {
    // Take the difference between the 18th and 20th iteration.
    // We want to maximize this, so count the proportion of cells that did not change as the cost.
    let mut board = self.starting_board();
    for _ in 0..18 {
      board = board.next();
    }
    let last = board.next().next();
    let total_cells = board.len() * board[0].len();
    (total_cells - last.difference(&board)) as f64 / total_cells as f64
  }
}

impl Neighbor for Pattern {
  fn neighbor(&self) -> Pattern {
    let mut new_grid = self.grid;
    for _ in 0..NEIGHBOR_FLIPS {
      let r = rand::random::<usize>() % new_grid.len();
      let c = rand::random::<usize>() % new_grid[r].len();

      // Take the inverse of self.grid's cell. In the case where the same cell gets chosen twice,
      // this guarantees that there will still be a diference between self.grid and new_grid.
      new_grid[r][c] = !self.grid[r][c];
    }
    Pattern{ grid: new_grid }
  }
}
