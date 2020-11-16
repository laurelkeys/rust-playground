use orst::*;

use rand::prelude::*;
use std::cell::Cell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>, // stores the number of comparisons made
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}

impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.t.cmp(&other.t)
    }
}

fn bench<T: Ord + Clone, S: Sorter>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> usize {
    let mut values = values.to_vec();
    counter.set(0);
    sorter.sort(&mut values);

    // @Note: asserting `values` is sorted also increments
    // `counter`, so we first need to store its value.
    let count = counter.get();

    // assert!(values.is_sorted()); // nightly ;(
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }

    count
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));

    for &n in &[0, 1, 10, 100, 1000, 10_000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Rc::clone(&counter),
            });
        }

        for _ in 0..10 {
            values.shuffle(&mut rand);

            let took = bench(Bubble, &values, &counter);
            println!("Bubble {} {}", n, took);

            let took = bench(Insertion { naive: false }, &values, &counter);
            println!("Insertion (smart) {} {}", n, took);

            let took = bench(Insertion { naive: true }, &values, &counter);
            println!("Insertion (naive) {} {}", n, took);

            let took = bench(Selection, &values, &counter);
            println!("Selection {} {}", n, took);

            let took = bench(Quick, &values, &counter);
            println!("Quick {} {}", n, took);
        }
    }
}
