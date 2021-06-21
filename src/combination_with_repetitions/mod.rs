use super::*;

mod combination_with_repetitions_generator;
mod combination_with_repetitions_referenced_generator;
mod single_combination_with_repetitions;
mod single_combination_with_repetitions_referenced;

pub use combination_with_repetitions_generator::*;
pub use combination_with_repetitions_referenced_generator::*;
pub(crate) use single_combination_with_repetitions::*;
pub(crate) use single_combination_with_repetitions_referenced::*;

pub struct CombinationWithRepetitions;

impl CombinationWithRepetitions {
    #[inline]
    pub fn total_nb(nb_elem: usize, comb_size: usize) -> usize {
        match (nb_elem, comb_size) {
            (0, _) | (_, 0) => 0,
            (1, _) => 1,
            (_, 1) => nb_elem,
            _ => {
                let n = nb_elem as u128;
                let c = comb_size as u128;
                ((n..(n + c)).product::<u128>() / (2..=c).product::<u128>()) as usize
            }
        }
    }

    pub fn size_for_idx(nb_elem: usize, idx: usize) -> usize {
        let mut res = 0;
        let mut offset = 0;
        let mut total_nb = 1;
        loop {
            total_nb = total_nb * (nb_elem + res) / (res + 1);
            if idx >= offset && idx < offset + total_nb {
                return res + 1;
            }
            res += 1;
            offset += total_nb;
        }
    }

    pub fn index_bounds_for_size(nb_elem: usize, comb_size: usize) -> (usize, usize) {
        (
            Self::offset(nb_elem, comb_size),
            Self::offset(nb_elem, comb_size + 1),
        )
    }

    pub fn indices_for_size(nb_elem: usize, comb_size: usize) -> Range<usize> {
        let (lower, upper) = Self::index_bounds_for_size(nb_elem, comb_size);
        lower..upper
    }

    pub fn index_bound_up_to_size(nb_elem: usize, comb_size: usize) -> usize {
        Self::offset(nb_elem, comb_size + 1)
    }

    pub fn indices_up_to_size(nb_elem: usize, comb_size: usize) -> Range<usize> {
        0..Self::index_bound_up_to_size(nb_elem, comb_size)
    }

    pub fn offset(nb_elem: usize, comb_size: usize) -> usize {
        (1..comb_size).map(|cs| Self::total_nb(nb_elem, cs)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_nb() {
        assert_eq!(0, CWR::total_nb(0, 5));
        assert_eq!(0, CWR::total_nb(5, 0));
        assert_eq!(1, CWR::total_nb(1, 5));
        assert_eq!(5, CWR::total_nb(5, 1));
        assert_eq!(70, CWR::total_nb(5, 4));
    }

    #[test]
    fn size_for_idx() {
        assert_eq!(1, CWR::size_for_idx(3, 0));
        assert_eq!(1, CWR::size_for_idx(3, 2));
        assert_eq!(2, CWR::size_for_idx(3, 3));
        assert_eq!(2, CWR::size_for_idx(3, 8));
        assert_eq!(3, CWR::size_for_idx(3, 9));
        assert_eq!(3, CWR::size_for_idx(3, 18));
        assert_eq!(4, CWR::size_for_idx(3, 19));

        assert_eq!(1, CWR::size_for_idx(4, 0));
        assert_eq!(1, CWR::size_for_idx(4, 3));
        assert_eq!(2, CWR::size_for_idx(4, 4));
        assert_eq!(2, CWR::size_for_idx(4, 13));
        assert_eq!(3, CWR::size_for_idx(4, 14));
    }

    #[test]
    fn index_bounds_for_size() {
        assert_eq!((0, 3), CWR::index_bounds_for_size(3, 1));
        assert_eq!((3, 9), CWR::index_bounds_for_size(3, 2));
        assert_eq!((9, 19), CWR::index_bounds_for_size(3, 3));
        assert_eq!((0, 4), CWR::index_bounds_for_size(4, 1));
        assert_eq!((4, 14), CWR::index_bounds_for_size(4, 2));
        assert_eq!((14, 34), CWR::index_bounds_for_size(4, 3));
    }

    #[test]
    fn index_bound_up_to_size() {
        assert_eq!(3, CWR::index_bound_up_to_size(3, 1));
        assert_eq!(9, CWR::index_bound_up_to_size(3, 2));
        assert_eq!(19, CWR::index_bound_up_to_size(3, 3));
        assert_eq!(4, CWR::index_bound_up_to_size(4, 1));
        assert_eq!(14, CWR::index_bound_up_to_size(4, 2));
        assert_eq!(34, CWR::index_bound_up_to_size(4, 3));
    }
}
