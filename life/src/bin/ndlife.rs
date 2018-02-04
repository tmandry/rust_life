// Original example code taken from rust-ndarray project (MIT licensed).

extern crate life;

use life::ndgame::*;

fn render(a: &Board) {
    for row in a.genrows() {
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
    let mut a = parse(INPUT);
    let mut scratch = Board::zeros((N, N));
    let steps = 100;
    turn_on_corners(&mut a);
    for _ in 0..steps {
        iterate(&mut a, &mut scratch);
        turn_on_corners(&mut a);
        //render(&a);
    }
    render(&a);
    let alive = a.iter().filter(|&&x| x > 0).count();
    println!("After {} steps there are {} cells alive", steps, alive);
}
