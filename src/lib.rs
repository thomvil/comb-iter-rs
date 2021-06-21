#![allow(unused_imports, dead_code)]

use std::ops::Range;

mod combination_with_repetitions;

pub use combination_with_repetitions::*;

// Shorthands
#[allow(clippy::upper_case_acronyms)]
pub type CWR = CombinationWithRepetitions;
pub type CWRG = CombinationWithRepetitionsGenerator;
pub(crate) type SCWR = SingleCombinationWithRepetitions;
pub(crate) type SCWRR<'a, T> = SingleCombinationWithRepetitionsReferenced<'a, T>;
