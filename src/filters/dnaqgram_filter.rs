use super::NgramFilter;

pub struct DNAQgram {
    pub str_len: usize,
    ranking_profile: [u8; 16],
}

impl DNAQgram {
    // SIZE of Q-gram
    const Q: usize = 2;
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

    const PROFILE_LEN: usize = Self::SIGMA * Self::SIGMA;

    pub fn new(s: &[u8]) -> Self {
        let mut ranking_profile = [0; Self::PROFILE_LEN];
        let sdist = s.len() - Self::Q + 1;
        let mut init_rank = Self::rank2(&s[0..Self::Q]);
        ranking_profile[Self::TRANSLATE_MAP[s[0] as usize]] += 1;
        for i in 1..sdist {
            let r = (init_rank - Self::TRANSLATE_MAP[s[(i - 1)] as usize] * Self::SIGMA)
                * Self::SIGMA
                + Self::TRANSLATE_MAP[s[i + Self::Q - 1] as usize];

            // let r = (init_rank - Self::TRANSLATE_MAP[s[(i - 1)] as usize] * Self::SIGMA)
            //     * Self::SIGMA
            //     + Self::TRANSLATE_MAP[s[i + Self::Q - 1] as usize];
            // let r = Self::TRANSLATE_MAP[*s_i as usize];
            ranking_profile[r] += 1;
            init_rank = r;
        }

        DNAQgram {
            str_len: s.len(),
            ranking_profile,
        }
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

impl NgramFilter for DNAQgram {
    #[inline(always)]
    fn dist(q1: &DNAQgram, q2: &DNAQgram) -> usize {
        q1.ranking_profile
            .iter()
            .zip(q2.ranking_profile.iter())
            .fold(0, |accum, (r1, r2)| accum + r1.abs_diff(*r2)) as usize
            / Self::Q
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dna_gram_translate_map() {
        let s = "AAGCT".to_owned();
        let dnaqgram = DNAQgram::new(s.as_bytes());
        // assert_eq!(dnaqgram.ranking_profile, [2, 1, 1, 1]);
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
