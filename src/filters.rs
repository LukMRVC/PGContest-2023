#[derive(Debug, Clone)]
pub struct Qgram {
    // profile: Vec<&'a [u8]>,
    // ranking_profile: Vec<usize>,
    ranking_profile: [usize; 26],
}

impl Qgram {
    // SIZE of Q-gram
    const Q: usize = 1;
    // size of the alphabet
    const SIGMA: usize = 26;
    // my ASCII alphabet translations
    const TRANSLATE_MAP: [usize; 256] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 64
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // 80
        16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 0, 0, 0, 0, 0, 0, // 96
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
        let mut init_rank = Self::TRANSLATE_MAP[s[0] as usize];
        ranking_profile[init_rank] += 1;
        for i in 1..sdist {
            // let r = (init_rank - Self::TRANSLATE_MAP[s[(i - 1)] as usize] * Self::SIGMA)
            //     * Self::SIGMA
            //     + Self::TRANSLATE_MAP[s[i + Self::Q - 1] as usize];
            let r = Self::TRANSLATE_MAP[s[i] as usize];
            ranking_profile[r] += 1;
            init_rank = r;
        }

        Qgram { ranking_profile }
    }

    pub fn dist(q1: &Qgram, q2: &Qgram, t: Option<usize>) -> usize {
        let t = t.unwrap_or(usize::MAX);
        let mut diffs = [0usize; 26];
        // q1.ranking_profile
        //     .iter()
        //     .zip(q2.ranking_profile.iter())
        //     .fold(0, |accum, (d1, d2)| accum + d1.abs_diff(*d2))
        diffs[0] = q1.ranking_profile[0].abs_diff(q2.ranking_profile[0]);
        diffs[1] = q1.ranking_profile[1].abs_diff(q2.ranking_profile[1]);
        diffs[2] = q1.ranking_profile[2].abs_diff(q2.ranking_profile[2]);
        diffs[3] = q1.ranking_profile[3].abs_diff(q2.ranking_profile[3]);
        diffs[4] = q1.ranking_profile[4].abs_diff(q2.ranking_profile[4]);
        diffs[5] = q1.ranking_profile[5].abs_diff(q2.ranking_profile[5]);
        diffs[6] = q1.ranking_profile[6].abs_diff(q2.ranking_profile[6]);
        diffs[7] = q1.ranking_profile[7].abs_diff(q2.ranking_profile[7]);
        diffs[8] = q1.ranking_profile[8].abs_diff(q2.ranking_profile[8]);
        diffs[9] = q1.ranking_profile[9].abs_diff(q2.ranking_profile[9]);
        diffs[10] = q1.ranking_profile[10].abs_diff(q2.ranking_profile[10]);
        diffs[11] = q1.ranking_profile[11].abs_diff(q2.ranking_profile[11]);
        diffs[12] = q1.ranking_profile[12].abs_diff(q2.ranking_profile[12]);
        diffs[13] = q1.ranking_profile[13].abs_diff(q2.ranking_profile[13]);
        diffs[14] = q1.ranking_profile[14].abs_diff(q2.ranking_profile[14]);
        diffs[15] = q1.ranking_profile[15].abs_diff(q2.ranking_profile[15]);
        diffs[16] = q1.ranking_profile[16].abs_diff(q2.ranking_profile[16]);
        diffs[17] = q1.ranking_profile[17].abs_diff(q2.ranking_profile[17]);
        diffs[18] = q1.ranking_profile[18].abs_diff(q2.ranking_profile[18]);
        diffs[19] = q1.ranking_profile[19].abs_diff(q2.ranking_profile[19]);
        diffs[20] = q1.ranking_profile[20].abs_diff(q2.ranking_profile[20]);
        diffs[21] = q1.ranking_profile[21].abs_diff(q2.ranking_profile[21]);
        diffs[22] = q1.ranking_profile[22].abs_diff(q2.ranking_profile[22]);
        diffs[23] = q1.ranking_profile[23].abs_diff(q2.ranking_profile[23]);
        diffs[24] = q1.ranking_profile[24].abs_diff(q2.ranking_profile[24]);
        diffs[25] = q1.ranking_profile[25].abs_diff(q2.ranking_profile[25]);

        diffs.iter().sum()
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
