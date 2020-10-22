#![allow(unused_variables)]
#![feature(test)]

// @Note: to run the benchmark, see https://rustwasm.github.io/docs/book/game-of-life/time-profiling.html#making-time-run-faster

fn main() {
    extern crate test;
    extern crate game_of_life;

    #[bench]
    fn universe_ticks(b: &mut test::Bencher) {
        let mut universe = game_of_life::Universe::new();

        b.iter(|| {
            universe.tick();
        });
    }
}
