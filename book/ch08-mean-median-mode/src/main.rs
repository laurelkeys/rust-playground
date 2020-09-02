use std::collections::HashMap;
use std::io;

fn main() {
    println!("Type a list of whitespace-separated numbers:");

    let mut numbers = String::new();

    io::stdin()
        .read_line(&mut numbers)
        .expect("Failed to read line");

    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|number| {
            number
                .parse::<i32>()
                .expect("Failed to convert input to integer")
        })
        .collect();

    println!();
    println!("Mean: {:?}", mean(&numbers));
    println!("Median: {:?}", median(&numbers));
    println!("Mode: {:?}", mode(&numbers));
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

/// Returns the mode of `numbers`, i.e. the value(s) that occurs most often.
fn mode(numbers: &[i32]) -> Vec<i32> {
    let mut each_count = HashMap::new();

    for &number in numbers {
        let count: &mut i32 = each_count.entry(number).or_insert(0);
        *count += 1;
        // @Note: this could also be written as:
        //  |
        //  |   *each_count.entry(number).or_insert(0) += 1;
        //
    }

    if let Some((_, max_count)) = each_count
        .iter()
        .max_by_key(|(_, &count)| count)
    {
        // @Note: I first tried the following, but it does not preserve
        // the order in which elements are present in `numbers`:
        //  |
        //  |   each_count
        //  |       .into_iter()
        //  |       .filter(|&(_, count)| count == max_count)
        //  |       .map(|(number, _)| number)
        //  |       .collect::<Vec<i32>>()
        //

        let mut mode = numbers
            .iter()
            .filter(|number| each_count.get(number) == Some(max_count))
            .copied() // @Note: without this, we'd only be able to `collect` into a Vec<&i32>
            .collect::<Vec<i32>>();

        // Remove possible duplicates.
        mode.dedup();
        mode
    } else {
        Vec::new() // `numbers` is empty
    }
}

//
// Test functions.
//

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mean() {
        assert_eq!(
            mean(&[9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24, 25]),
            16.75
        );
        assert_eq!(mean(&[1, 2, 2, 3, 4, 7, 9]), 4.0);
        assert_eq!(mean(&[42]), 42.0);
        assert_eq!(mean(&[]), 0.0);
    }

    #[test]
    fn test_median() {
        assert_eq!(
            median(&[9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24, 25]),
            Some(15.5)
        );
        assert_eq!(median(&[1, 2, 2, 3, 4, 7, 9]), Some(3.0));
        assert_eq!(median(&[1, 3, 3, 6, 7, 8, 9]), Some(6.0));
        assert_eq!(median(&[1, 3, 3, 4, 5, 7, 8, 9]), Some(4.5));
        assert_eq!(median(&[1, 2, 3, 4]), Some(2.5));
        assert_eq!(median(&[42]), Some(42.0));
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn test_mode() {
        assert_eq!(
            mode(&[9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24, 25]),
            vec![13]
        );
        assert_eq!(mode(&[1, 2, 2, 3, 4, 7, 9]), vec![2]);
        assert_eq!(mode(&[1, 2, 3, 4]), vec![1, 2, 3, 4]);
        assert_eq!(mode(&[1]), vec![1]);
        assert_eq!(mode(&[]), vec![]);
    }
}
