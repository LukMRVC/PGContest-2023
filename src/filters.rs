use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Qgram<'a> {
    profile: HashMap<&'a [u8], usize>,
}

impl<'a> Qgram<'a> {
    pub fn new(s: &'a [u8], q: usize) -> Self {
        let mut prof = HashMap::<&'a [u8], usize>::new();
        let end = s.len() - q + 1;
        for i in 0..end {
            let slice = &s[i..(i + q)];
            if let Some(val) = prof.get_mut(slice) {
                *val += 1;
            } else {
                prof.insert(slice, 1);
            }
        }

        Qgram { profile: prof }
    }

    pub fn dist(&self, other: &Qgram) -> usize {
        let mut union = HashSet::<&'a [u8]>::new();
        union.extend(self.profile.keys());
        union.extend(other.profile.keys());

        let mut agg = 0;
        for k in union.into_iter() {
            let (mut v0, mut v1) = (0, 0);
            if let Some(val) = self.profile.get(k) {
                v0 += val;
            }
            if let Some(val) = other.profile.get(k) {
                v1 += val;
            }
            agg += v0.abs_diff(v1);
        }

        agg
    }

    pub fn Dist(q1: &Qgram, q2: &Qgram) -> usize {
        q1.dist(q2)
    }
}
