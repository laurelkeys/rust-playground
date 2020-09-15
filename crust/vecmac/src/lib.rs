// https://www.youtube.com/watch?v=q6paRBbLgNw

#[macro_export]
macro_rules! arr {
    // See https://doc.rust-lang.org/reference/macros-by-example.html#metavariables

    ($x:expr; $count:expr) => {
        {
            // @Optimization: calling `resize()` would allow us not to worry about
            // copying the metavariables, so we could have simply done the following:
            //  |
            //  |   let mut xs = Vec::new();
            //  |   xs.resize($count, $x);
            //

            let count = $count;

            let mut xs = Vec::with_capacity(count);

            let x = $x;
            for _ in 0..count { xs.push(x.clone()); }
            // @Note: this could be made slightly more efficient with:
            //  |
            //  |   xs.extend(::std::iter::repeat(x).take(count));
            //

            xs
        }
    };

    ($( $x:expr ),*) => {
        {
            const COUNT: usize = $crate::count![$( $x:expr ),*]; // check that `count!` is const

            #[allow(unused_mut)]
            let mut xs = Vec::with_capacity(COUNT);

            $( xs.push($x); )*

            xs
        }
    };

    // @Volatile: this has to be defined *after* the match it's
    // calling (i.e. the one right above), otherwise we would get to
    // an infinite recursion, as it would continuosly (re)match itself.
    ($( $x:expr, )*) => { $crate::arr![$( $x ),*] };

    // @Note: we could have grouped the two prior matches as:
    //  |
    //  |   ($( $x: expr ),* $( , )?) => {
    //  |       ...
    //  |   }
    //
    // However, that would allow for this undesired syntax: `arr![,]`.
}

#[doc(hidden)]
#[macro_export]
macro_rules! count {
    // Reference: https://danielkeep.github.io/tlborm/book/blk-counting.html#slice-length

    ($( $elem:expr ),*) => {
        <[()]>::len(&[
            $( $crate::count![$elem => ()] ),*
        ])
    };

    ($_elem:expr => $subst:expr) => { $subst };
}

//
// Test functions.
//

#[cfg(test)]
mod test {
    use super::*;

    // @Note: run `cargo expand --lib --tests` to see the macro expansions.

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
            "lakdjwaidjiwalfjhawligfjawilfjawlifwjalwijwfalijawfiljfaew", // <-- valid syntax
        ];

        let _: Vec<&'static str> = arr![
            "lakdjwaidjiwalfjhawligfjawilfjawlifwjalwijwfalijawfiljfaew",
            "lakdjwaidjiwalfjhawligfjawilfjawlifwjalwijwfalijawfiljfaew",
            "lakdjwaidjiwalfjhawligfjawilfjawlifwjalwijwfalijawfiljfaew",
            "lakdjwaidjiwalfjhawligfjawilfjawlifwjalwijwfalijawfiljfaew",
            "lakdjwaidjiwalfjhawligfjawilfjawlifwjalwijwfalijawfiljfaew" // <-- also valid
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
    fn clone_nonliteral_2() {
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
