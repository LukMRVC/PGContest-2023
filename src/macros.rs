macro_rules! query {
    ($querydata:ident, $srchdata:ident, $gramtype:ty, $query_ngrams:ident, $srchgrams:ident, $true_filter_chunks:ident, true, true) => {
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

                    let sum: usize = $srchdata
                        .iter()
                        .enumerate()
                        .filter(|(wid, _)| &$srchgrams[*wid].str_len.abs_diff(qwlen) <= t)
                        .filter(|(wid, _)| <$gramtype>::dist(&$srchgrams[*wid], &query_qgram) <= t2)
                        .filter(|(wid, _)| {
                            $true_filter_chunks[*wid].matches(
                                query_ngram_list.unwrap(),
                                *t,
                                &mut match_set,
                            )
                        })
                        .map(|(id, word)| ukkonen_map(id + 1, word, qwlen, qwbytes, t + 1))
                        .sum();
                    acc + sum
                },
            )
            .sum()
    };

    ($querydata:ident, $srchdata:ident, $gramtype:ty, $query_ngrams:ident, $srchgrams:ident, $true_filter_chunks:ident, false, true) => {
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

                    let sum: usize = $srchdata
                        .iter()
                        .enumerate()
                        .filter(|(wid, _)| &$srchgrams[*wid].str_len.abs_diff(qwlen) <= t)
                        .filter(|(wid, _)| <$gramtype>::dist(&$srchgrams[*wid], &query_qgram) <= t2)
                        .map(|(id, word)| ukkonen_map(id + 1, word, qwlen, qwbytes, t + 1))
                        .sum();
                    acc + sum
                },
            )
            .sum()
    };

    ($querydata:ident, $srchdata:ident, $gramtype:ty, $query_ngrams:ident, $srchgrams:ident, $true_filter_chunks:ident, true, false) => {
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

                    let sum: usize = $srchdata
                        .iter()
                        .enumerate()
                        .filter(|(wid, _)| <$gramtype>::dist(&$srchgrams[*wid], &query_qgram) <= t2)
                        .filter(|(wid, _)| {
                            $true_filter_chunks[*wid].matches(
                                query_ngram_list.unwrap(),
                                *t,
                                &mut match_set,
                            )
                        })
                        .map(|(id, word)| ukkonen_map(id + 1, word, qwlen, qwbytes, t + 1))
                        .sum();
                    acc + sum
                },
            )
            .sum()
    };

    ($querydata:ident, $srchdata:ident, $gramtype:ty, $query_ngrams:ident, $srchgrams:ident, $true_filter_chunks:ident, false, false) => {
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

                    let sum: usize = $srchdata
                        .par_iter()
                        .enumerate()
                        .filter(|(wid, _)| <$gramtype>::dist(&$srchgrams[*wid], &query_qgram) <= t2)
                        .map(|(id, word)| ukkonen_map(id + 1, word, qwlen, qwbytes, t + 1))
                        .sum();
                    acc + sum
                },
            )
            .sum()
    };
}

// The syntax for filtering will be (querydata, srchdata, is_dna, use_true_match_filter, use_length_filter)
macro_rules! filtering {
    ($querydata:ident, $srchdata:ident, $gramtype:ty, $use_true_match:expr, $use_length_filter:expr) => {{
        let srchgrams: Vec<$gramtype> = $srchdata
            .par_iter()
            .map(|data| <$gramtype>::new(data))
            .collect();

        let mut true_filter_chunks: Vec<TrueMatchFilter> = vec![];
        let mut query_ngrams: Vec<Vec<(i32, usize)>> = vec![];
        if $use_true_match {
            true_filter_chunks = $srchdata
                .clone()
                .par_iter_mut()
                .map(|record| record_to_chunk_filter(record))
                .collect();

            query_ngrams = $querydata
                .clone()
                .iter_mut()
                .map(|(query_record, _)| record_to_ngrams(query_record))
                .collect();
        }

        match ($use_true_match, $use_length_filter) {
            (true, true) => crate::macros::query!(
                $querydata,
                $srchdata,
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
