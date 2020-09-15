// https://www.youtube.com/watch?v=rAl-9HwD858

#[derive(Debug)]
pub struct StrSplit<'a, T> {
    remainder: Option<&'a str>,
    delimiter: T,
}

impl<'a, T> StrSplit<'a, T> {
    pub fn new(haystack: &'a str, delimiter: T) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}
/// Describes the delimiter's position in the haystack.
pub struct DelimiterPosition {
    start: usize,
    end: usize,
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<DelimiterPosition>;
}

impl<'a, T> Iterator for StrSplit<'a, T>
where
    T: Delimiter,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(remainder) = self.remainder.as_mut() {
            // @Note: the type of `remainder` inside this block is `&mut &str`,
            // since `as_mut()` converts `&mut Option<T>` into `Option<&mut T>`.
            if let Some(DelimiterPosition { start, end }) = self.delimiter.find_next(remainder) {
                let until_delimiter = &remainder[..start];
                *remainder = &remainder[end..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }

        // @Note: we could also use pattern matching with the `?` operator to write:
        //  |
        //  |   let remainder = self.remainder.as_mut()?;
        //  |   if let Some(DelimiterPosition { start, end }) = self.delimiter.find_next(remainder) {
        //  |       let until_delim = &remainder[..start];
        //  |       *remainder = &remainder[end..];
        //  |       Some(until_delim)
        //  |   } else {
        //  |       self.remainder.take()
        //  |   }
        //
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<DelimiterPosition> {
        s.find(self).map(|start| DelimiterPosition {
            start,
            end: start + self.len(),
        })
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<DelimiterPosition> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| DelimiterPosition {
                start,
                end: start + self.len_utf8(),
            })
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
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
