pub mod DNAQGramFilter;
pub mod QGramFilter;

pub trait NgramFilter {
    fn dist(q1: &Self, q2: &Self) -> usize;
}
