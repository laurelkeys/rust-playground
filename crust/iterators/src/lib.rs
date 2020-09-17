// @Todo: continue from https://youtu.be/yozQ9C69pNs?t=2580

pub fn flatten<I>(iter: I) -> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter)
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    // @Note: `IntoIter` is the iterator type of `O::Item`
    // (you can see it as `Iterator<Item = O::Item>`).
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
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
            /* inner_iter: &mut IntoIterator::IntoIter<Iterator::Item<O>> */
            // if let Some(inner_iter) = &mut self.inner {
            // if let Some(ref mut inner_iter) = self.inner {
            if let Some(inner_iter) = self.inner.as_mut() {
                if let Some(item) = inner_iter.next() {
                    return Some(item);
                }
                self.inner = None
            }

            // @Note: the `?` below guarantees we will eventually break out of the loop.
            let next_inner_iter = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_iter);
        }
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
        assert_eq!(
            flatten(vec![Vec::<()>::new(), vec![], vec![]].into_iter()).count(),
            0
        );
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
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]].into_iter()).count(), 2);
    }
}
