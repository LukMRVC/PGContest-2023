use super::NgramFilter;
use rayon::prelude::*;

pub struct DNAQgram {
    ranking_profiles: Vec<u8>,
}

impl DNAQgram {
    // SIZE of Q-gram
    const Q: usize = 1;
    // size of the alphabet
    const SIGMA: usize = 4;
    // my ASCII alphabet translations
    const TRANSLATE_MAP: [usize; 256] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 64
        0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, // 80
        0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 96
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 112
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 128
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 144
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 160
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 176
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 192
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 208
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 224
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 240
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 256
    ];

    const PROFILE_LEN: usize = Self::SIGMA;

    pub fn new(s: &Vec<Vec<u8>>) -> Self {
        let mut ranking_profiles = Vec::with_capacity(s.len() * 4);
        for chunk in ranking_profiles.par_iter(4) {}

        DNAQgram { ranking_profiles }
    }
}

impl NgramFilter for DNAQgram {
    #[inline(always)]
    fn dist(q1: &DNAQgram, q2: &DNAQgram) -> usize {
        q1.ranking_profile
            .iter()
            .zip(q2.ranking_profile.iter())
            .fold(0, |accum, (r1, r2)| accum + r1.abs_diff(*r2)) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dna_gram_translate_map() {
        let s = "AAGCT".to_owned();
        let mut dnaqgram = DNAQgram::new(s.as_bytes());
        assert_eq!(dnaqgram.ranking_profile, [2, 1, 1, 1]);
    }

    #[test]
    fn recognizes_dna_alphabet() {
        let s = "AGCT".to_owned();
        let s = s.into_bytes();

        let mut is_dna = true;
        for char_byte in s.iter() {
            if char_byte != &65 && char_byte != &67 && char_byte != &71 && char_byte != &84 {
                is_dna = false;
                break;
            }
        }

        assert_eq!(is_dna, true);

        let s = "AGCV".to_owned();
        let s = s.into_bytes();

        let mut is_dna = true;
        for char_byte in s.iter() {
            if char_byte != &65 && char_byte != &67 && char_byte != &71 && char_byte != &84 {
                is_dna = false;
                break;
            }
        }
        assert_eq!(is_dna, false);
    }
}
