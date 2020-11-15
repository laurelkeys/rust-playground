use super::Sorter;

pub struct Bubble;

impl Sorter for Bubble {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        let mut swapped = true;
        while swapped {
            swapped = false;
            // @Note: using `0..(slice.len() - 1)` instead (and comparing
            // slice[i] with slice[i + 1]) would panic on an empty slice!
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    Bubble.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
