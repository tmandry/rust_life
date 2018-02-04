#![feature(test)]

extern crate test;

extern crate life;

#[macro_use]
extern crate ndarray;

#[cfg(test)]
mod tests {

    const INPUT: &'static [u8] = include_bytes!("../src/life.txt");
    const N: usize = 100;

    use life::game;
    use life::ndgame;

    use test::Bencher;

    #[bench]
    fn bench_game(b: &mut Bencher)
    {
        let board = game::Board::parse(INPUT);
        b.iter(|| {
            let mut b = board.clone();
            for _ in 0..100 {
                b = b.next();
            }
        })
    }

    #[bench]
    fn bench_ndgame(b: &mut Bencher)
    {
        let board = ndgame::Board::parse(INPUT, N, N);
        let mut scratch = ndgame::Board::scratch(N, N);
        b.iter(|| {
            let mut b = board.clone();
            for _ in 0..100 {
                b.iterate(&mut scratch);
            }
        })
    }

}
