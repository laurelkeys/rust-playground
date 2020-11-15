use super::Sorter;

pub struct Insertion;

impl Sorter for Insertion {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        // `unsorted` partitions / splits `slice` into elements
        // that are sorted and those that aren't:
        //
        // `slice`: [ "sorted" | "not sorted" ]
        //                     ↑
        //                `unsorted`

        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted, so we take slice[unsorted]
            // and place it in the correct location at slice[..=unsorted].
            let mut i = unsorted;
            while i > 0 && slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);
                i -= 1;
            }

            // @Note: we could also use binary search to find the correct index i
            // for slice[unsorted], and then rotate slice[i..=unsorted] by 1 element
            // right, which would make slice[unsorted] wrap around and go to slice[i].
            //  |
            //  |   let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
            //  |       Ok(i) => i,
            //  |       Err(i) => i,
            //  |   };
            //  |   slice[i..=unsorted].rotate_right(1);
            //
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    Insertion::sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
