use std::collections::HashMap;

fn main() {
    let numbers = [42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];

    println!("mean: {:?}", mean(&numbers));
    println!("median: {:?}", median(&numbers));
    println!("mode: {:?}", mode(&numbers));
}

/// Returns the mean of `numbers`, i.e. the average value.
fn mean(numbers: &[i32]) -> f32 {
    match numbers.len() {
        0 => 0.0,
        _ => numbers.iter().sum::<i32>() as f32 / numbers.len() as f32,
    }
}

/// Returns the median of `numbers`, i.e. when sorted, the value in the middle position
/// if there is an odd amount of elements, otherwise, the average between the two middle values.
fn median(numbers: &[i32]) -> Option<f32> {
    let len = numbers.len();

    if len > 0 {
        // @Performance: median could be computed in O(n).
        let mut sorted: Vec<i32> = numbers.to_vec();
        sorted.sort();

        let middle = len / 2;

        Some(match len % 2 {
            0 => (sorted[middle - 1] + sorted[middle]) as f32 / 2.0,
            _ => sorted[middle] as f32,
        })
    } else {
        None
    }
}

/// Returns the mode of `numbers`, i.e. the value that occurs most often.
/// If `numbers` is empty, `None` is returned.
fn mode(numbers: &[i32]) -> Option<i32> {
    let mut each_count = HashMap::new();

    for &number in numbers {
        let count: &mut i32 = each_count.entry(number).or_insert(0);
        *count += 1;
        // @Note: this could also be written as:
        //  |
        //  |   *each_count.entry(number).or_insert(0) += 1;
        //
    }

    // @Robustness: when several elements are equally maximum in `max_by_key`, the last one
    // is returned. Check what is the most "consistently correct" thing to do in this case.
    each_count
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(number, _)| number)
}

//
// Test functions.
//

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mean() {
        assert_eq!(mean(&[9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24, 25]), 16.75);
        assert_eq!(mean(&[1, 2, 2, 3, 4, 7, 9]), 4.0);
        assert_eq!(mean(&[42]), 42.0);
        assert_eq!(mean(&[]), 0.0);
    }

    #[test]
    fn test_median() {
        assert_eq!(median(&[9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24, 25]), Some(15.5));
        assert_eq!(median(&[1, 2, 2, 3, 4, 7, 9]), Some(3.0));
        assert_eq!(median(&[1, 3, 3, 6, 7, 8, 9]), Some(6.0));
        assert_eq!(median(&[1, 3, 3, 4, 5, 7, 8, 9]), Some(4.5));
        assert_eq!(median(&[42]), Some(42.0));
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn test_mode() {
        assert_eq!(mode(&[9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24, 25]), Some(13));
        assert_eq!(mode(&[1, 2, 2, 3, 4, 7, 9]), Some(2));
        assert_eq!(mode(&[1]), Some(1));
        assert_eq!(mode(&[]), None);
    }
}
