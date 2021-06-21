use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct SingleCombinationWithRepetitions {
    nb_elem: usize,
    comb_size: usize,
    offset_idx: usize,
    inner_el: usize,
    inner_offset: usize,
    res_idx: usize,
}

impl SingleCombinationWithRepetitions {
    pub fn new(nb_elem: usize, idx: usize) -> Self {
        let comb_size = CWR::size_for_idx(nb_elem, idx);
        Self {
            nb_elem,
            comb_size,
            offset_idx: idx - CWR::offset(nb_elem, comb_size),
            inner_el: 0,
            inner_offset: 0,
            res_idx: 0,
        }
    }
}

impl Iterator for SingleCombinationWithRepetitions {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.res_idx {
            r_i if r_i >= self.comb_size => None,
            r_i if r_i == self.comb_size - 1 => {
                self.res_idx += 1;
                Some(self.inner_el + self.offset_idx)
            }
            _ => loop {
                let inner_comb_size = (self.comb_size - self.res_idx).saturating_sub(2) + 1;
                let total_nb = CWR::total_nb(self.nb_elem - self.inner_el, inner_comb_size);
                dbg!(self.inner_offset, self.inner_offset + total_nb);
                if self.offset_idx >= self.inner_offset
                    && self.offset_idx < self.inner_offset + total_nb
                {
                    self.offset_idx -= self.inner_offset;
                    self.inner_offset = 0;
                    self.res_idx += 1;
                    return Some(self.inner_el);
                }
                self.inner_offset += total_nb;
                self.inner_el += 1;
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_idx_slice(nb_elem: usize, idx: usize, slice: &[usize]) {
        let scwr = SCWR::new(nb_elem, idx);
        assert_eq!(slice, scwr.collect::<Vec<_>>().as_slice());
    }

    #[test]
    fn iter() {
        test_idx_slice(3, 0, &[0]);
        test_idx_slice(3, 1, &[1]);
        test_idx_slice(3, 2, &[2]);
        test_idx_slice(3, 3, &[0, 0]);
        test_idx_slice(3, 4, &[0, 1]);
        test_idx_slice(3, 5, &[0, 2]);
        test_idx_slice(3, 6, &[1, 1]);
        test_idx_slice(3, 7, &[1, 2]);
        test_idx_slice(3, 8, &[2, 2]);
        test_idx_slice(3, 15, &[1, 1, 1]);

        test_idx_slice(4, 10, &[1, 3]);
        test_idx_slice(4, 15, &[0, 0, 1]);
    }
}
