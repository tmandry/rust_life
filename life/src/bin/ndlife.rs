// Original example code taken from rust-ndarray project (MIT licensed).

extern crate life;

use life::ndgame::*;

const N: usize = 100;

fn render(a: &Board) {
    for row in a.arr.genrows() {
        for &x in row {
            if x > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    let mut b = Board::parse(INPUT, N, N);
    let mut scratch = Board::scratch(N, N);
    let steps = 100;
    b.turn_on_corners();
    for _ in 0..steps {
        b.iterate(&mut scratch);
        b.turn_on_corners();
        //render(&a);
    }
    render(&b);
    let alive = b.arr.iter().filter(|&&x| x > 0).count();
    println!("After {} steps there are {} cells alive", steps, alive);
}
