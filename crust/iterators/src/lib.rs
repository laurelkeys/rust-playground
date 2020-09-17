// https://www.youtube.com/watch?v=yozQ9C69pNs

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    // @Note: `IntoIter` is the iterator type for `O::Item`
    // (you can see it as `Iterator<Item = O::Item>`).
    inner: Option<<O::Item as IntoIterator>::IntoIter>, // front iterator, used by `next()`
    inner_back: Option<<O::Item as IntoIterator>::IntoIter>, // back iterator, used by `next_back()`
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
            inner_back: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner_iter) = self.inner.as_mut() {
                // @Note: we could rewrite the if-statement above in two other ways:
                //  |
                //  |   if let Some(inner_iter) = &mut self.inner {
                //
                // or:
                //  |
                //  |   if let Some(ref mut inner_iter) = self.inner {
                //
                // In all three cases, the type of `inner_iter` is going to be
                // `&mut IntoIterator::IntoIter<Iterator::Item<O>>`.

                if let Some(item) = inner_iter.next() {
                    return Some(item);
                }

                self.inner = None
            }

            if let Some(next_inner_iter) = self.outer.next() {
                self.inner = Some(next_inner_iter.into_iter());
            } else {
                return self.inner_back.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner_back_iter) = self.inner_back.as_mut() {
                if let Some(item) = inner_back_iter.next_back() {
                    return Some(item);
                }

                self.inner_back = None
            }

            if let Some(next_inner_back_iter) = self.outer.next_back() {
                self.inner_back = Some(next_inner_back_iter.into_iter());
            } else {
                return self.inner.as_mut()?.next_back();
            }
        }
    }
}

// Extension trait.
pub trait IntoIteratorExt: IntoIterator + Sized {
    fn our_flatten(self) -> Flatten<Self::IntoIter>
    where
        Self::Item: IntoIterator;
}

// Blanket implementation.
impl<T> IntoIteratorExt for T
where
    T: IntoIterator,
{
    fn our_flatten(self) -> Flatten<Self::IntoIter>
    where
        Self::Item: IntoIterator,
    {
        flatten(self.into_iter())
    }
}

//
// Test functions.
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
        // @Note: trying to create an empty iterator over the unit type wouldn't
        // comply to `flatten()`'s bounds because `()` doesn't implement `IntoIterator`:
        //  |
        //  |   assert_eq!(flatten(std::iter::empty::<()>()).count(), 0);
        //
    }

    #[test]
    fn empty_wide() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn inf() {
        let mut iter = flatten((0..).map(|i| 0..i));
        // 0 => 0..0 => empty
        // 1 => 0..1 => [0]
        // 2 => 0..2 => [0, 1]
        // 3 => 0..3 => [0, 1, 2]
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
    }

    #[test]
    fn deep() {
        assert_eq!(flatten(flatten(vec![vec![vec![0, 1]]])).count(), 2); // yields `0`, `1`
        assert_eq!(flatten(vec![vec![vec![0, 1]]]).count(), 1); // yields `vec![0, 1]`

        let mut two_deep = flatten(flatten(vec![vec![vec![0, 1]]]));
        assert_eq!(two_deep.next(), Some(0));
        assert_eq!(two_deep.next(), Some(1));
        assert_eq!(two_deep.next(), None);

        let mut one_deep = flatten(vec![vec![vec![0, 1]]]);
        assert_eq!(one_deep.next(), Some(vec![0, 1]));
        assert_eq!(one_deep.next(), None);
    }

    #[test]
    fn ext() {
        assert_eq!(vec![vec![0, 1]].into_iter().our_flatten().count(), 2);
        // @Note: if the extension trait was made for `Iterator` (as in the video),
        // instead of for `IntoIterator`, omitting `into_iter()` wouldn't be possible:
        assert_eq!(vec![vec![0, 1], vec![0, 1]].our_flatten().count(), 4);
    }
}
