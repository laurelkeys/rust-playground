use super::Sorter;

pub struct Selection;

impl Sorter for Selection {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        // @Note: `unsorted` partitions / splits `slice` into
        // elements that are sorted and those that aren't:
        //
        // `slice`: [ "sorted" | "not sorted" ]
        //                     ↑
        //                `unsorted`

        for unsorted in 0..slice.len() {
            let mut smallest_in_rest = unsorted;
            for i in (unsorted + 1)..slice.len() {
                if slice[i] < slice[smallest_in_rest] {
                    smallest_in_rest = i;
                }
            }

            // @Note: we could also compute the index of the
            // smallest element in the rest of the slice as:
            //  |
            //  |   let smallest_in_rest = slice[unsorted..]
            //  |       .iter()
            //  |       .enumerate()
            //  |       .min_by_key(|&(_, value)| value)
            //  |       .map(|(i, _)| unsorted + i)
            //  |       .expect("slice is not empty");
            //

            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest);
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    Selection.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
