pub mod dnaqgram_filter;
pub mod qgram_filter;
pub mod true_match_filter;

pub trait NgramFilter {
    fn dist(q1: &Self, q2: &Self) -> usize;
}
