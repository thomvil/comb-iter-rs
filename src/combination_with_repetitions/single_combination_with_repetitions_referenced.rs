use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct SingleCombinationWithRepetitionsReferenced<'a, T: 'a> {
    scwr: SingleCombinationWithRepetitions,
    elems: &'a [T],
}

impl<'a, T> SingleCombinationWithRepetitionsReferenced<'a, T> {
    pub fn new(elems: &'a [T], idx: usize) -> Self {
        Self {
            scwr: SingleCombinationWithRepetitions::new(elems.len(), idx),
            elems,
        }
    }
}

impl<'a, T: 'a> Iterator for SingleCombinationWithRepetitionsReferenced<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.scwr.next().map(|e| &self.elems[e])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIST: [char; 3] = ['a', 'b', 'c'];

    fn test_idx_ref_slice(idx: usize, slice: &[char]) {
        let scwrr = SCWRR::new(&LIST, idx);
        assert_eq!(slice, scwrr.copied().collect::<Vec<_>>().as_slice());
    }

    #[test]
    fn iter_referenced() {
        test_idx_ref_slice(0, &['a']);
        test_idx_ref_slice(1, &['b']);
        test_idx_ref_slice(2, &['c']);
        test_idx_ref_slice(3, &['a', 'a']);
        test_idx_ref_slice(4, &['a', 'b']);
        test_idx_ref_slice(5, &['a', 'c']);
        test_idx_ref_slice(6, &['b', 'b']);
        test_idx_ref_slice(7, &['b', 'c']);
        test_idx_ref_slice(8, &['c', 'c']);
        test_idx_ref_slice(15, &['b', 'b', 'b']);
    }
}
