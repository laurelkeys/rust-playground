use super::Sorter;

pub struct Insertion {
    pub naive: bool,
}

impl Sorter for Insertion {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        // @Note: `unsorted` partitions / splits `slice` into
        // elements that are sorted and those that aren't:
        //
        // `slice`: [ "sorted" | "not sorted" ]
        //                     ↑
        //                `unsorted`

        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted, so we take slice[unsorted]
            // and place it in the correct location at slice[..=unsorted].
            if self.naive {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                // We can also use binary search to find the correct index i for
                // slice[unsorted], and then rotate slice[i..=unsorted] by 1 element
                // right, which makes slice[unsorted] wrap around and go to slice[i].
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,  // index of a matching element
                    Err(i) => i, // index where a matching element could be inserted
                };

                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn it_works_naive() {
    let mut things = vec![4, 2, 5, 3, 1];
    Insertion { naive: true }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

#[test]
fn it_works_smart() {
    let mut things = vec![4, 2, 5, 3, 1];
    Insertion { naive: false }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
