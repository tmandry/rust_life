// Original example code taken from rust-ndarray project (MIT licensed).

use ndarray::prelude::*;

pub const INPUT: &'static [u8] = include_bytes!("life.txt");
//const INPUT: &'static [u8] = include_bytes!("lifelite.txt");

pub type BoardArray = Array2<u8>;

#[derive(Clone, Debug)]
pub struct Board {
    pub arr: BoardArray,
}

impl Board {
    pub fn blank(rows: usize, cols: usize) -> Board {
        Board{arr: Array::zeros((rows, cols))}
    }

    pub fn parse(x: &[u8], rows: usize, cols: usize) -> Board {
        // make a border of 0 cells
        let mut map = BoardArray::from_elem(((rows + 2), (cols + 2)), 0);
        let a = Array::from_iter(x.iter().filter_map(|&b| match b {
            b'#' => Some(1),
            b'.' => Some(0),
            _ => None,
        }));

        let a = a.into_shape((rows, cols)).unwrap();
        map.slice_mut(s![1..-1, 1..-1]).assign(&a);
        Board{arr: map}
    }

    pub fn scratch(rows: usize, cols: usize) -> BoardArray {
        BoardArray::zeros((rows, cols))
    }

    // Rules
    //
    // 2 or 3 neighbors: stay alive
    // 3 neighbors: birth
    // otherwise: death

    pub fn iterate(self: &mut Board, scratch: &mut BoardArray) {
        // compute number of neighbors
        let mut neigh = scratch.view_mut();
        let z = &mut self.arr;
        neigh.fill(0);
        neigh += &z.slice(s![0..-2, 0..-2]);
        neigh += &z.slice(s![0..-2, 1..-1]);
        neigh += &z.slice(s![0..-2, 2..  ]);

        neigh += &z.slice(s![1..-1, 0..-2]);
        neigh += &z.slice(s![1..-1, 2..  ]);

        neigh += &z.slice(s![2..  , 0..-2]);
        neigh += &z.slice(s![2..  , 1..-1]);
        neigh += &z.slice(s![2..  , 2..  ]);

        // birth where n = 3 and z[i] = 0,
        // survive where n = 2 || n = 3 and z[i] = 1
        let mut zv = z.slice_mut(s![1..-1, 1..-1]);

        // this is autovectorized amazingly well!
        zv.zip_mut_with(&neigh, |y, &n| {
            *y = ((n == 3) || (n == 2 && *y > 0)) as u8
        });
    }

    pub fn turn_on_corners(self: &mut Board) {
        let z = &mut self.arr;
        let n = z.rows();
        let m = z.cols();
        z[[1    , 1    ]] = 1;
        z[[1    , m - 2]] = 1;
        z[[n - 2, 1    ]] = 1;
        z[[n - 2, m - 2]] = 1;
    }
}
