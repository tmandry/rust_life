extern crate rand;

use std::ops::Index;

const N: usize = 200;

pub struct Life {
  a: Box<[[bool; N]; N]>,
  pub generation: u64,
}

impl Life {
  pub fn new() -> Life {
    let mut a = Box::new([[false; N]; N]);

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

  pub fn size(&self) -> (usize, usize) {
    (N, N)
  }

  pub fn len(&self) -> usize {
    N
  }

  pub fn neighbors(&self, r_start: usize, c_start: usize) -> u8 {
    let mut count: u8 = 0;
    for i in [-1, 0, 1].iter() {
      for j in [-1, 0, 1].iter() {
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

  pub fn update(&mut self) {
    let mut next = Box::new([[false; N]; N]);
    for r in 0..self.a.len() {
      for c in 0..self.a[r].len() {
        let n = self.neighbors(r, c);
        if self.a[r][c] && (n == 2 || n == 3) {
          next[r][c] = true;
        } else if !self.a[r][c] && n == 3 {
          next[r][c] = true;
        }
      }
    }
    self.a = next;
    self.generation += 1;
  }
}

impl Index<usize> for Life {
  type Output = [bool; N];
  fn index(&self, row_idx: usize) -> &[bool; N] {
    &self.a[row_idx]
  }
}
