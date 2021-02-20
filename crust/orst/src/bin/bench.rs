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

struct BenchResult {
    comparisons: usize,
    time: f64,
}

fn bench<T: Ord + Clone, S: Sorter>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> BenchResult {
    let mut values = values.to_vec();
    counter.set(0);

    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();

    // @Note: asserting `values` is sorted also increments
    // `counter`, so we first need to store its value.
    let count = counter.get();

    // assert!(values.is_sorted()); // nightly ;(
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }

    BenchResult {
        comparisons: count,
        time: took.as_secs_f64(),
    }
}

macro_rules! print_bench {
    ($n:expr, $algorithm:expr, $sorter:expr, $values:expr, $counter:expr) => {
        let BenchResult { comparisons, time } = bench($sorter, $values, $counter);
        println!("{}\t{}\t{}\t{}", $algorithm, $n, comparisons, time);
    };
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));

    println!("algorithm\tn\tcomparisons\ttime");

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
            print_bench!(n, "Bubble", Bubble, &values, &counter);
            print_bench!(n, "Insertion (smart)", Insertion { naive: false }, &values, &counter);
            print_bench!(n, "Insertion (naive)", Insertion { naive: true }, &values, &counter);
            print_bench!(n, "Selection", Selection, &values, &counter);
            print_bench!(n, "Quick", Quick, &values, &counter);
            print_bench!(n, "Std", StdSorter, &values, &counter);
        }
    }
}
