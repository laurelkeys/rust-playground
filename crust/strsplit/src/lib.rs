// https://www.youtube.com/watch?v=rAl-9HwD858

#[derive(Debug)]
pub struct StrSplit<'remainder, 'delimiter> {
    remainder: Option<&'remainder str>,
    delimiter: &'delimiter str,
}

impl<'remainder, 'delimiter> StrSplit<'remainder, 'delimiter> {
    pub fn new(haystack: &'remainder str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'remainder> Iterator for StrSplit<'remainder, '_> {
    type Item = &'remainder str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(remainder) = self.remainder.as_mut() {
            // @Note: the type of `remainder` inside this block is `&mut &str`,
            // since `as_mut()` converts `&mut Option<T>` into `Option<&mut T>`.
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delim = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delim)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }

        // @Note: we could also use pattern matching with the `?` operator to write:
        //  |
        //  |   let remainder = self.remainder.as_mut()?;
        //  |
        //  |   if let Some(next_delim) = remainder.find(self.delimiter) {
        //  |       let until_delim = &remainder[..next_delim];
        //  |       *remainder = &remainder[(next_delim + self.delimiter.len())..];
        //  |       Some(until_delim)
        //  |   } else {
        //  |       self.remainder.take()
        //  |   }
        //
    }
}

#[cfg(test)]
pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, &format!("{}", c))
        .next()
        .expect("StrSplit always gives at least one result")
}

//
// Test functions.
//

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let haystack = "a b c d e";

        // @Note: the following could also be tested as:
        //  |
        //  |   let letters = StrSplit::new(haystack, " ");
        //  |   assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
        //

        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn delimiter_on_tail_returns_empty() {
        let haystack = "a b c d ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
    }

    #[test]
    fn until_char_works() {
        assert_eq!(until_char("hello world", 'o'), "hell");
        assert_eq!(until_char("hello world", 'z'), "hello world");
    }
}
