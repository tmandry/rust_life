extern crate rand;

use std::ops::{Index, IndexMut};

const N: usize = 100;

#[derive(Clone)]
pub struct Board {
  a: [[bool; N]; N],
  pub generation: u64,
}

impl Board {
  pub fn random() -> Board {
    let mut a = [[false; N]; N];

    for _ in 0..6400 {
      let r = rand::random::<usize>() % a.len();
      let c = rand::random::<usize>() % a[r].len();
      a[r][c] = true;
    }

    Board {
      a: a,
      generation: 0
    }
  }

  pub fn empty() -> Board {
    Board {
      a: [[false; N]; N],
      generation: 0
    }
  }

  pub fn parse(x: &[u8]) -> Board {
    let mut a = [[false; N]; N];

    for r in 0..N {
      for c in 0..N {
        a[r][c] = match x[r*N + c] {
          b'#' => true,
          b'.' => false,
          _    => false,
        }
      }
    }

    Board {
      a: a,
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

  pub fn next(&self) -> Board {
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

    Board{ a: board, generation: self.generation + 1 }
  }

  pub fn difference(&self, other: &Board) -> usize {
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

impl Index<usize> for Board {
  type Output = [bool; N];
  fn index(&self, row_idx: usize) -> &[bool; N] {
    &self.a[row_idx]
  }
}
impl IndexMut<usize> for Board {
  fn index_mut(&mut self, row_idx: usize) -> &mut [bool; N] {
    &mut self.a[row_idx]
  }
}
