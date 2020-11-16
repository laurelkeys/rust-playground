// @Todo: implement merge and heap sort, and plot the bench results.

pub trait Sorter {
    fn sort<T: Ord>(&self, slice: &mut [T]);
}

mod bubblesort;
mod insertionsort;
mod selectionsort;
mod quicksort;

pub use bubblesort::Bubble;
pub use insertionsort::Insertion;
pub use selectionsort::Selection;
pub use quicksort::Quick;

pub struct StdSorter;
impl Sorter for StdSorter {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        slice.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 5, 3, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }
}
