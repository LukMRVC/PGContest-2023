pub trait NgramFilter {
    fn dist(q1: &Self, q2: &Self) -> usize;
}

#[derive(Debug, Clone)]
pub struct Qgram {
    // profile: Vec<&'a [u8]>,
    // ranking_profile: Vec<usize>,
    ranking_profile: [u8; 63],
}

impl Qgram {
    // SIZE of Q-gram
    const Q: usize = 1;
    // size of the alphabet
    const SIGMA: usize = 63;
    // my ASCII alphabet translations
    const TRANSLATE_MAP: [usize; 256] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, // 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, // 48
        54, 55, 56, 57, 58, 59, 60, 61, 62, 0, 0, 0, 0, 0, 0, 0, // 64
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // 80
        16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 0, 0, 0, 0, 0, 0, // 96
        0, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, // 112
        41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 0, 0, 0, 0, 0, // 128
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

    pub fn new(s: &[u8]) -> Self {
        let mut ranking_profile = [0; Self::PROFILE_LEN];
        let sdist = s.len() - Self::Q + 1;
        // let mut init_rank = Self::rank2(&s[0..Self::Q]);
        ranking_profile[Self::TRANSLATE_MAP[s[0] as usize]] += 1;
        for s_i in s.iter().take(sdist).skip(1) {
            // let r = (init_rank - Self::TRANSLATE_MAP[s[(i - 1)] as usize] * Self::SIGMA)
            //     * Self::SIGMA
            //     + Self::TRANSLATE_MAP[s[i + Self::Q - 1] as usize];
            let r = Self::TRANSLATE_MAP[*s_i as usize];
            ranking_profile[r] += 1;
        }

        Qgram { ranking_profile }
    }

    #[inline(always)]
    fn rank2(slice: &[u8]) -> usize {
        // for Q = 2;
        Self::TRANSLATE_MAP[slice[0] as usize] * Self::SIGMA
            + Self::TRANSLATE_MAP[slice[1] as usize]
    }

    #[inline(always)]
    fn rank3(slice: &[u8]) -> usize {
        Self::TRANSLATE_MAP[slice[0] as usize] * (Self::SIGMA * Self::SIGMA)
            + Self::TRANSLATE_MAP[slice[1] as usize] * Self::SIGMA
            + Self::TRANSLATE_MAP[slice[2] as usize]
    }
}

impl NgramFilter for Qgram {
    fn dist(q1: &Qgram, q2: &Qgram) -> usize {
        let d = q1
            .ranking_profile
            .iter()
            .zip(q2.ranking_profile.iter())
            .fold(0, |accum, (d1, d2)| accum + d1.abs_diff(*d2));
        d as usize
    }
}

pub struct DNAQgram {
    ranking_profile: [u8; 4],
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

    pub fn new(s: &[u8]) -> Self {
        let mut ranking_profile = [0; Self::PROFILE_LEN];
        let sdist = s.len() - Self::Q + 1;
        // let mut init_rank = Self::rank2(&s[0..Self::Q]);
        ranking_profile[Self::TRANSLATE_MAP[s[0] as usize]] += 1;
        for s_i in s.iter().take(sdist).skip(1) {
            // let r = (init_rank - Self::TRANSLATE_MAP[s[(i - 1)] as usize] * Self::SIGMA)
            //     * Self::SIGMA
            //     + Self::TRANSLATE_MAP[s[i + Self::Q - 1] as usize];
            let r = Self::TRANSLATE_MAP[*s_i as usize];
            ranking_profile[r] += 1;
        }

        DNAQgram { ranking_profile }
    }
}

impl NgramFilter for DNAQgram {
    #[inline(always)]
    fn dist(q1: &DNAQgram, q2: &DNAQgram) -> usize {
        let d1 = q1.ranking_profile[0].abs_diff(q2.ranking_profile[0]);
        let d2 = q1.ranking_profile[1].abs_diff(q2.ranking_profile[1]);
        let d3 = q1.ranking_profile[2].abs_diff(q2.ranking_profile[2]);
        let d4 = q1.ranking_profile[3].abs_diff(q2.ranking_profile[3]);
        (d1 + d2 + d3 + d4) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dna_gram_translate_map() {
        let s = "AGCT".to_owned();
        let mut dnaqgram = DNAQgram::new(s.as_bytes());
        assert_eq!(dnaqgram.ranking_profile, [1; 4]);
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
