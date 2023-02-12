macro_rules! query {
    ($querydata:ident, $srchdata:ident, $len_map:ident, $gramtype:ty, $query_ngrams:ident, $srchgrams:ident, $true_filter_chunks:ident, true, true) => {
        $querydata
            .par_iter()
            .enumerate()
            .fold(
                || 0usize,
                |acc, (qid, (query_word, t))| {
                    // let mut sum = 0;
                    let qwlen = query_word.len();
                    let qwbytes = query_word.as_bytes();
                    let query_qgram = <$gramtype>::new(qwbytes);
                    let query_ngram_list = &$query_ngrams.get(qid);
                    let t2 = *t * 2;
                    let mut match_set: Vec<(i32, usize, usize)> = Vec::with_capacity(128);

                    let srchdata_len = $srchdata.len();
                    let start_idx = *$len_map.get(&qwlen.saturating_sub(*t)).unwrap_or(&0);
                    let end_idx = $len_map.get(&(qwlen + *t + 1)).unwrap_or(&srchdata_len);
                    let idx_diff = *end_idx - start_idx;

                    let sum: usize = $srchdata
                        .iter()
                        .enumerate()
                        .skip(start_idx)
                        .take(idx_diff)
                        .filter(|(wid, _)| <$gramtype>::dist(&$srchgrams[*wid], &query_qgram) <= t2)
                        .filter(|(wid, _)| {
                            $true_filter_chunks[*wid].matches(
                                query_ngram_list.unwrap(),
                                *t,
                                &mut match_set,
                            )
                        })
                        .map(|(_, word)| ukkonen_map(word.1, &word.0, qwlen, qwbytes, t + 1))
                        .sum();
                    acc + sum
                },
            )
            .sum()
    };

    ($querydata:ident, $srchdata:ident, $len_map:ident, $gramtype:ty, $query_ngrams:ident, $srchgrams:ident, $true_filter_chunks:ident, false, true) => {
        $querydata
            .par_iter()
            .fold(
                || 0usize,
                |acc, (query_word, t)| {
                    // let mut sum = 0;
                    let qwlen = query_word.len();
                    let qwbytes = query_word.as_bytes();
                    let query_qgram = <$gramtype>::new(qwbytes);
                    let t2 = *t * 2;

                    let srchdata_len = $srchdata.len();
                    let start_idx = *$len_map.get(&qwlen.saturating_sub(*t)).unwrap_or(&0);
                    let end_idx = $len_map.get(&(qwlen + *t + 1)).unwrap_or(&srchdata_len);
                    let idx_diff = *end_idx - start_idx;
                    // dbg!(query_word);
                    // dbg!(start_idx);

                    let sum: usize = $srchdata
                        .iter()
                        .enumerate()
                        .skip(start_idx)
                        .take(idx_diff)
                        .filter(|(wid, _)| <$gramtype>::dist(&$srchgrams[*wid], &query_qgram) <= t2)
                        .map(|(_, word)| ukkonen_map(word.1, &word.0, qwlen, qwbytes, t + 1))
                        .sum();
                    acc + sum
                },
            )
            .sum()
    };

    ($querydata:ident, $srchdata:ident, $len_map:ident, $gramtype:ty, $query_ngrams:ident, $srchgrams:ident, $true_filter_chunks:ident, true, false) => {
        $querydata
            .par_iter()
            .enumerate()
            .fold(
                || 0usize,
                |acc, (qid, (query_word, t))| {
                    // let mut sum = 0;
                    let qwlen = query_word.len();
                    let qwbytes = query_word.as_bytes();
                    let query_qgram = <$gramtype>::new(qwbytes);
                    let query_ngram_list = &$query_ngrams.get(qid);
                    let t2 = *t * 2;
                    let mut match_set: Vec<(i32, usize, usize)> = Vec::with_capacity(128);

                    let srchdata_len = $srchdata.len();
                    let start_idx = *$len_map.get(&qwlen.saturating_sub(*t)).unwrap_or(&0);
                    let end_idx = $len_map.get(&(qwlen + *t + 1)).unwrap_or(&srchdata_len);
                    let idx_diff = *end_idx - start_idx;

                    let sum: usize = $srchdata
                        .iter()
                        .enumerate()
                        .skip(start_idx)
                        .take(idx_diff)
                        .filter(|(wid, _)| <$gramtype>::dist(&$srchgrams[*wid], &query_qgram) <= t2)
                        .filter(|(wid, _)| {
                            $true_filter_chunks[*wid].matches(
                                query_ngram_list.unwrap(),
                                *t,
                                &mut match_set,
                            )
                        })
                        .map(|(_, word)| ukkonen_map(word.1, &word.0, qwlen, qwbytes, t + 1))
                        .sum();
                    acc + sum
                },
            )
            .sum()
    };

    ($querydata:ident, $srchdata:ident, $len_map:ident, $gramtype:ty, $query_ngrams:ident, $srchgrams:ident, $true_filter_chunks:ident, false, false) => {
        $querydata
            .par_iter()
            .fold(
                || 0usize,
                |acc, (query_word, t)| {
                    // let mut sum = 0;
                    let qwlen = query_word.len();
                    let qwbytes = query_word.as_bytes();
                    let query_qgram = <$gramtype>::new(qwbytes);
                    let t2 = *t * 2;
                    let srchdata_len = $srchdata.len();

                    let start_idx = *$len_map.get(&qwlen.saturating_sub(*t)).unwrap_or(&0);
                    let end_idx = $len_map.get(&(qwlen + *t + 1)).unwrap_or(&srchdata_len);
                    let idx_diff = *end_idx - start_idx;

                    let sum: usize = $srchdata
                        .par_iter()
                        .enumerate()
                        .skip(start_idx)
                        .take(idx_diff)
                        .filter(|(wid, _)| <$gramtype>::dist(&$srchgrams[*wid], &query_qgram) <= t2)
                        .map(|(_, word)| ukkonen_map(word.1, &word.0, qwlen, qwbytes, t + 1))
                        .sum();
                    acc + sum
                },
            )
            .sum()
    };
}

// The syntax for filtering will be (querydata, srchdata, is_dna, use_true_match_filter, use_length_filter)
macro_rules! filtering {
    ($querydata:ident, $srchdata:ident, $len_map:ident, $gramtype:ty, $use_true_match:expr, $use_length_filter:expr, $n:expr) => {{
        let srchgrams: Vec<$gramtype> = $srchdata
            .par_iter()
            .map(|data| <$gramtype>::new(&data.0))
            .collect();

        let mut true_filter_chunks: Vec<TrueMatchFilter> = vec![];
        let mut query_ngrams: Vec<Vec<(i32, usize)>> = vec![];
        if $use_true_match {
            true_filter_chunks = $srchdata
                .par_iter()
                .map(|record| record_to_chunk_filter(&record.0, $n))
                .collect();

            query_ngrams = $querydata
                .clone()
                .iter_mut()
                .map(|(query_record, _)| record_to_ngrams(query_record, $n))
                .collect();
        }

        match ($use_true_match, $use_length_filter) {
            (true, true) => crate::macros::query!(
                $querydata,
                $srchdata,
                $len_map,
                $gramtype,
                query_ngrams,
                srchgrams,
                true_filter_chunks,
                true,
                true
            ),
            (true, false) => crate::macros::query!(
                $querydata,
                $srchdata,
                $len_map,
                $gramtype,
                query_ngrams,
                srchgrams,
                true_filter_chunks,
                true,
                false
            ),
            (false, false) => crate::macros::query!(
                $querydata,
                $srchdata,
                $len_map,
                $gramtype,
                query_ngrams,
                srchgrams,
                true_filter_chunks,
                false,
                false
            ),
            _ => crate::macros::query!(
                $querydata,
                $srchdata,
                $len_map,
                $gramtype,
                query_ngrams,
                srchgrams,
                true_filter_chunks,
                false,
                true
            ),
        }
    }};
}

pub(crate) use filtering;
pub(crate) use query;
