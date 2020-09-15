// https://www.youtube.com/watch?v=q6paRBbLgNw

#[macro_export]
macro_rules! arr {
    // See https://doc.rust-lang.org/reference/macros-by-example.html#metavariables

    () => { Vec::new() };

    ($( $x: expr ),+ $( , )?) => {
        {
            let mut xs = Vec::new();
            $( xs.push($x); )*
            xs
        }
    };

    ($x: expr; $count: expr) => {
        {
            let mut xs = Vec::new();
            let x = $x;
            for _ in 0..$count {
                xs.push(x.clone());
            }
            xs
        }
    };
}

//
// Test functions.
//

#[cfg(test)]
mod test {
    use super::*;

    // @Note: use `cargo expand --lib --tests` to see the result of macro expansions.

    #[test]
    fn empty_vec() {
        let x: Vec<u32> = arr![];
        assert!(x.is_empty());
    }

    #[test]
    fn single_elem() {
        let x: Vec<u32> = arr![42];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 1);
        assert_eq!(x[0], 42);
    }

    #[test]
    fn two_elems() {
        let x: Vec<u32> = arr![42, 43];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 43);
    }

    #[test]
    fn trailing_comma() {
        let _: Vec<&'static str> = arr![
            "lakdjwaidjiwalfjhawligfjawilfjawlifwjalwijwfalijawfiljfaew",
            "lakdjwaidjiwalfjhawligfjawilfjawlifwjalwijwfalijawfiljfaew",
            "lakdjwaidjiwalfjhawligfjawilfjawlifwjalwijwfalijawfiljfaew",
            "lakdjwaidjiwalfjhawligfjawilfjawlifwjalwijwfalijawfiljfaew",
            "lakdjwaidjiwalfjhawligfjawilfjawlifwjalwijwfalijawfiljfaew",
        ];
    }

    #[test]
    fn clone_2() {
        let x: Vec<u32> = arr![42; 2];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 42);
    }

    #[test]
    fn clone_2_nonliterals() {
        let mut y = Some(42);
        let x: Vec<u32> = arr![y.take().unwrap(); 2];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 42);
    }
}

/// ```compile_fail
/// let x: Vec<u32> = arr::arr![42; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;