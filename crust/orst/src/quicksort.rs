use super::Sorter;

pub struct Quick;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is not empty");

    // Split `rest` into a left side (with values less than or equal to
    // the pivot) and a right side (with values greater than the pivot).
    //
    // `slice`: [`pivot` | ...`rest`]
    //
    // `rest`: [..."<=" | ... | ...">"]
    //                  ↑     ↑
    //               `left` `right`

    let mut left = 0; // rest[..left] is "<= pivot"
    let mut right = rest.len(); // rest[right..] is "> pivot"

    while left != right {
        if &rest[left] <= pivot {
            // Already on the correct side.
            left += 1;
        } else if &rest[right - 1] > pivot {
            // Already on the correct side.
            right -= 1;
        } else {
            // Move elements to the correct side (left holds a right
            // and right holds a left, so we just swap them).
            rest.swap(left, right - 1);
            left += 1;
            right -= 1;
        }
    }

    // @Note: `left` and `right` are indices into `rest`, which
    // is one element less than `slice` (i.e. the `pivot`). We could
    // have re-aligned them to point into `slice` by adding +1 to each.

    // Place the pivot at its final location.
    // [pivot, ..."<=", ...">"] -> [..."<=", pivot, ...">"]
    slice.swap(0, left);

    // [..."<=", pivot, ...">"] -> ([..."<="], [pivot, ...">"])
    let (left, right) = slice.split_at_mut(left);

    quicksort(left);
    quicksort(&mut right[1..]);
}

impl Sorter for Quick {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        quicksort(slice);
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    Quick.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
