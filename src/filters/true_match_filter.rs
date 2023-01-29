const TRANSLATE_MAP: [i32; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 16
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, // 32
    0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, // 48
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

const ALPHABET_SIZE: i32 = 64;

fn nchunks(doc: &[u8], n: usize) -> Vec<(i32, usize)> {
    let total_chunks = (doc.len() + n - 1) / n;
    let mut chunks: Vec<(i32, usize)> = Vec::with_capacity(total_chunks);

    for (i, nchunk) in doc.chunks(n).enumerate() {
        let nchunk_num = rank(&nchunk[0..n], n);
        chunks.push((nchunk_num, i * n));
    }

    chunks.sort();
    chunks
}

pub fn ngrams(doc: &[u8], n: usize) -> Vec<(i32, usize)> {
    let total_ngrams = doc.len() - n + 1;
    let mut ngrams_vec: Vec<(i32, usize)> = Vec::with_capacity(total_ngrams);
    let mut last_ranking = rank(&doc[0..n], n);
    ngrams_vec.push((last_ranking, 0));
    for (i, ngram) in doc.windows(n).skip(1).enumerate() {
        last_ranking = (last_ranking
            - TRANSLATE_MAP[doc[i] as usize] * ALPHABET_SIZE.pow((n - 1) as u32))
            * ALPHABET_SIZE
            + TRANSLATE_MAP[ngram[n - 1] as usize];
        ngrams_vec.push((last_ranking, i + 1));
    }

    ngrams_vec.sort();
    ngrams_vec
}

fn rank(slice: &[u8], n: usize) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += TRANSLATE_MAP[slice[i - 1] as usize] * (ALPHABET_SIZE.pow((n - i) as u32));
    }
    sum
}

pub struct TrueMatchFilter {
    lbstr: usize,
    chunks: Vec<(i32, usize)>,
    n: usize,
}

impl TrueMatchFilter {
    pub fn new(word: &[u8], n: usize) -> Self {
        Self {
            chunks: nchunks(word, n),
            lbstr: (word.len() + n - 1) / n,
            n,
        }
    }

    pub fn matches(
        &self,
        ngram_list: &[(i32, usize)],
        threshold: usize,
        match_set: &mut Vec<(i32, usize, usize)>,
    ) -> bool {
        let lb = self.lbstr.saturating_sub(threshold);
        let mut mismatches = 0;

        // let mut hashmap_cache = FxHashMap::default();

        for (chunk, chunk_pos) in self.chunks.iter() {
            let mut match_idx = 0;
            let srch_res = ngram_list.binary_search_by_key(chunk, |&(a, _)| a);
            if let Ok(mut srch_idx) = srch_res {
                while srch_idx > 0 && ngram_list[srch_idx - 1].0 == *chunk {
                    srch_idx -= 1;
                }
                match_idx = srch_idx;
            } else {
                // I can insert usize::MAX since I know that strings are of limited length
                mismatches += 1;
                if mismatches > self.chunks.len() - lb {
                    match_set.clear();
                    return false;
                }
            }

            let (mut match_ngram, mut ngram_pos) =
                (ngram_list[match_idx].0, ngram_list[match_idx].1);
            while match_ngram == *chunk {
                if chunk_pos.abs_diff(ngram_pos) <= threshold {
                    match_set.push((*chunk, ngram_pos, *chunk_pos));
                }
                match_idx += 1;
                if match_idx >= ngram_list.len() {
                    break;
                }
                (match_ngram, ngram_pos) = (ngram_list[match_idx].0, ngram_list[match_idx].1);
            }
        }

        if match_set.len() < lb {
            match_set.clear();
            return false;
        }
        self.true_match(match_set, lb)
    }

    fn true_match(&self, match_set: &mut Vec<(i32, usize, usize)>, lb: usize) -> bool {
        match_set.sort_by_key(|&(_, b, c)| (b, c));
        match_set.insert(0, (i32::MAX, usize::MAX, usize::MAX));
        let mut opt = [0; 256];

        #[inline(always)]
        fn compatible(m1: &(i32, usize, usize), m2: &(i32, usize, usize), n: usize) -> bool {
            if m2.0 == i32::MAX {
                return true;
            }
            m1.2 != m2.2 && m1.1 >= (m2.1 + n) // return value of compatible
        }

        for k in 1..match_set.len() {
            let mut mx = i32::MIN;
            let mn = std::cmp::min(k, match_set.len() - lb + 1);
            for i in 1..=mn {
                if compatible(&match_set[k], &match_set[k - i], self.n) && opt[k - i] > mx {
                    mx = opt[k - i] + 1;
                }
            }
            opt[k] = mx;
        }
        match_set.clear();
        opt.iter().skip(lb).max().unwrap() >= &(lb as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_is_right() {
        let dna = "ABAACAGTAB".to_owned();
        assert_eq!(TRANSLATE_MAP[b'A' as usize], 1);
        let chunks = nchunks(dna.as_bytes(), 2);

        assert_eq!(chunks, vec![(65, 2), (66, 0), (66, 8), (193, 4), (468, 6)]);
    }

    #[test]
    fn ranking_is_correct() {
        let slice = "ABC";
        let n = 3;
        let result = rank(slice.as_bytes(), n);
        assert_eq!(result, ALPHABET_SIZE.pow(2) + 2 * ALPHABET_SIZE + 3);

        let slice = "$";
        let n = 1;
        let result = rank(slice.as_bytes(), n);
        assert_eq!(result, ALPHABET_SIZE - 1);
    }

    #[test]
    fn ngrams_are_correct() {
        let slice = "ABAA";
        let n = 2;
        let nvec = ngrams(slice.as_bytes(), n);
        assert_eq!(nvec, vec![(65, 2), (66, 0), (129, 1)]);
    }

    #[test]
    fn searching_is_correct() {
        let vc = vec![
            (64, 2),
            (65, 0),
            (65, 8),
            (65, 8),
            (65, 8),
            (65, 8),
            (65, 8),
            (190, 4),
            (461, 6),
        ];

        let srch = 65;

        let srch_res = vc.binary_search_by_key(&srch, |&(a, _b)| a);
        assert_eq!(srch_res.is_ok(), true);
        if let Ok(mut srch_idx) = srch_res {
            while srch_idx > 0 && vc[srch_idx - 1].0 == srch {
                srch_idx -= 1;
            }
            assert_eq!(srch_idx, 1);
        }
    }

    #[test]
    fn true_match_is_correct() {
        let s = "abcdcdab";
        let q = "bccdabcd";
        let n = 2;
        let q_ngrams = ngrams(q.as_bytes(), n);
        let fil = TrueMatchFilter::new(s.as_bytes(), n);
        let mut match_set: Vec<(i32, usize, usize)> = Vec::new();

        let matches = fil.matches(&q_ngrams, 2, &mut match_set);

        assert_eq!(matches, true);

        let s = "TAGTATTCTCTTACCTTCTGGATATTAGGAACAATATCATAAGAAGGTTGTACACCCTTTGCGATATTGGGAGTAATATCGTCCTGTATTCCCCTGGATAT$";
        let q = "TAGTATTCTCTTACCTTCTGGATATTAGGAATATCATAAGAAGGTTGTACACCCTTTGCGATATTGGGAGTAATATCGTCCTGTATTCCCCTGGATAT$";
        let n = 2;
        let q_ngrams = ngrams(q.as_bytes(), n);
        let fil = TrueMatchFilter::new(s.as_bytes(), n);
        let mut match_set: Vec<(i32, usize, usize)> = Vec::new();

        let matches = fil.matches(&q_ngrams, 2, &mut match_set);

        assert_eq!(matches, false);
    }

    #[test]
    fn dna_string_matches() {
        let s = "TAGTATTCTCTTACCTTCTGGATATTAGGAACAATATCATAAGAAGGTTGTACACCCTTTGCGATATTGGGAGTAATATCGTCCTGTATTCCCCTGGATAT$";
        let q = "TAGTATTCTCTTACCTTCTGGATATTAGGAATATCATAAGAAGGTTGTACACCCTTTGCGATATTGGGAGTAATATCGTCCTGTATTCCCCTGGATAT$";

        let q_ngrams = ngrams(q.as_bytes(), 2);
        assert_eq!(q.as_bytes().len(), 99);
        let mut match_set: Vec<(i32, usize, usize)> = Vec::new();

        let fil = TrueMatchFilter::new(s.as_bytes(), 2);
        let matches = fil.matches(&q_ngrams, 12, &mut match_set);

        assert_eq!(matches, true);
    }

    #[test]
    fn normal_string_matches() {
        let s = "kafe";
        let q = "dale$";

        let q_ngrams = ngrams(q.as_bytes(), 2);
        let fil = TrueMatchFilter::new(s.as_bytes(), 2);
        let mut match_set: Vec<(i32, usize, usize)> = Vec::new();

        let matches = fil.matches(&q_ngrams, 2, &mut match_set);

        assert_eq!(matches, true);
    }

    #[test]
    fn normal_string_matches_2() {
        let s = "karel$";
        let q = "kryl$";

        let q_ngrams = ngrams(q.as_bytes(), 2);
        let fil = TrueMatchFilter::new(s.as_bytes(), 2);
        let mut match_set: Vec<(i32, usize, usize)> = Vec::new();

        let matches = fil.matches(&q_ngrams, 2, &mut match_set);

        assert_eq!(matches, true);
    }

    #[test]
    fn normal_string_matches_3() {
        let s = "AAAAAAA$";
        let q = "AAAABAA$";

        let q_ngrams = ngrams(q.as_bytes(), 2);
        let fil = TrueMatchFilter::new(s.as_bytes(), 2);
        let mut match_set: Vec<(i32, usize, usize)> = Vec::new();

        let matches = fil.matches(&q_ngrams, 1, &mut match_set);

        assert_eq!(matches, true);
    }

    #[test]
    fn normal_string_matches_4() {
        let s = "GCTCTGTCGCCCAGGCTGGAGTGCAGTGGCATGATCTCGGCTCACTGCAACCTCCACCTCCCAGGTTCAAGTGATTCTCCTGCCTCAGCCTCCCGAGTAGC$";
        let q = "CTCTGTTGCCCAGGCTGGAGTGCACTGGCGTGAGTCTCGGCTCACTGCAACCTCTGCTTCCCAGGTTTAAGCGATTCTCCTGCTTCAGCCTCCCAAGTAGC$";

        let q_ngrams = ngrams(q.as_bytes(), 2);
        let fil = TrueMatchFilter::new(s.as_bytes(), 2);
        let mut match_set: Vec<(i32, usize, usize)> = Vec::new();

        let matches = fil.matches(&q_ngrams, 12, &mut match_set);

        assert_eq!(matches, true);
    }
}
