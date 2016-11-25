extern crate rand;

use std::ops::{Index, IndexMut};

const N: usize = 200;

pub struct Life {
  a: [[bool; N]; N],
  pub generation: u64,
}

impl Life {
  pub fn random() -> Life {
    let mut a = [[false; N]; N];

    for _ in 0..6400 {
      let r = rand::random::<usize>() % a.len();
      let c = rand::random::<usize>() % a[r].len();
      a[r][c] = true;
    }

    Life {
      a: a,
      generation: 0
    }
  }

  pub fn empty() -> Life {
    Life {
      a: [[false; N]; N],
      generation: 0
    }
  }

  pub fn size(&self) -> (usize, usize) {
    (N, N)
  }

  pub fn len(&self) -> usize {
    N
  }

  pub fn neighbors(&self, r_start: usize, c_start: usize) -> u8 {
    let mut count: u8 = 0;
    for i in [-1, 0, 1].into_iter() {
      for j in [-1, 0, 1].into_iter() {
        if i == &0 && j == &0 {
          continue;
        }

        let (r, c) = (r_start as i32 + i, c_start as i32 + j);
        if r > 0 && (r as usize) < N && c > 0 && (c as usize) < N {
          if self.a[r as usize][c as usize] {
            count += 1;
          }
        }
      }
    }
    count
  }

  pub fn next(&self) -> Life {
    let mut board = [[false; N]; N];
    for r in 0..self.a.len() {
      for c in 0..self.a[r].len() {
        let n = self.neighbors(r, c);
        if self.a[r][c] && (n == 2 || n == 3) {
          board[r][c] = true;
        } else if !self.a[r][c] && n == 3 {
          board[r][c] = true;
        }
      }
    }

    Life{ a: board, generation: self.generation + 1 }
  }

  pub fn difference(&self, other: &Life) -> usize {
    let mut diff = 0;
    for r in 0..self.len() {
      for c in 0..self[r].len() {
        if self[r][c] != other[r][c] {
          diff += 1;
        }
      }
    }
    diff
  }
}

impl Index<usize> for Life {
  type Output = [bool; N];
  fn index(&self, row_idx: usize) -> &[bool; N] {
    &self.a[row_idx]
  }
}
impl IndexMut<usize> for Life {
  fn index_mut(&mut self, row_idx: usize) -> &mut [bool; N] {
    &mut self.a[row_idx]
  }
}

extern crate optimizer;
use self::optimizer::{Cost, Neighbor};

const CANDIDATE_SIZE: usize = 10;
const NEIGHBOR_FLIPS: usize = 2;

struct LifeOptimizer {
  grid: [[bool; CANDIDATE_SIZE]; CANDIDATE_SIZE]
}

impl LifeOptimizer {
  // Returns a Life board that contains this candidate in the middle, but is otherwise empty.
  fn starting_board(&self) -> Life {
    let mut board = Life::empty();
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

impl Cost for LifeOptimizer {
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

impl Neighbor for LifeOptimizer {
  fn neighbor(&self) -> LifeOptimizer {
    let mut new_grid = self.grid;
    for _ in 0..NEIGHBOR_FLIPS {
      let r = rand::random::<usize>() % new_grid.len();
      let c = rand::random::<usize>() % new_grid[r].len();

      // Take the inverse of self.grid's cell. In the case where the same cell gets chosen twice,
      // this guarantees that there will still be a diference between self.grid and new_grid.
      new_grid[r][c] = !self.grid[r][c];
    }
    LifeOptimizer{ grid: new_grid }
  }
}
