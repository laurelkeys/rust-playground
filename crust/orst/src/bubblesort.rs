use super::Sorter;

pub struct Bubble;

impl Sorter for Bubble {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    Bubble::sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
