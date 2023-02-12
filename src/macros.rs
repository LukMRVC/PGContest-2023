macro_rules! query {
    // rarely used
    ($querydata:ident, $srchdata:ident, $len_map:ident, $gramtype:ty, $query_ngrams:ident,
        $srchgrams:ident, $true_filter_chunks:ident, $indexes:ident, true, true) => {
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
    // NOT DNA data
    ($querydata:ident, $srchdata:ident, $len_map:ident, $gramtype:ty, $query_ngrams:ident,
        $srchgrams:ident, $true_filter_chunks:ident, $indexes:ident, false, true) => {
        $querydata
            .par_iter()
            .fold(
                || 0usize,
                |acc, (query_word, t)| {
                    let qwlen = query_word.len();
                    let qwbytes = query_word.as_bytes();
                    let query_qgram = <$gramtype>::new(qwbytes);
                    let t2 = *t * 2;

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
                        .map(|(_, word)| ukkonen_map(word.1, &word.0, qwlen, qwbytes, t + 1))
                        .sum();
                    acc + sum
                },
            )
            .sum()
    };
    // commonly used for DNA
    ($querydata:ident, $srchdata:ident, $len_map:ident, $gramtype:ty, $query_ngrams:ident,
        $srchgrams:ident, $true_filter_chunks:ident, $indexes:ident, true, false) => {
        $querydata
            .par_iter()
            .enumerate()
            .fold(
                || 0usize,
                |acc, (qid, (query_word, t))| {
                    let qwlen = query_word.len();
                    let qwbytes = query_word.as_bytes();
                    let query_qgram = <$gramtype>::new(qwbytes);
                    let query_ngram_list = &$query_ngrams.get(qid);
                    let querygrams = query_ngram_list.unwrap();
                    let t2 = *t * 2;
                    let mut match_set: Vec<(i32, usize, usize)> = Vec::with_capacity(128);

                    let mut candidates: FxHashSet<usize> = FxHashSet::default();
                    for ct in 0..=*t {
                        for (sig, sig_pos) in querygrams.iter() {
                            let maybe_listings = $indexes[ct].get(sig);
                            if let Some(listings) = maybe_listings {
                                for (cid, cpos) in listings {
                                    if cpos.abs_diff(*sig_pos) <= *t {
                                        candidates.insert(*cid);
                                    }
                                }
                            }
                        }
                    }

                    let mut candidates: Vec<usize> = candidates.drain().collect();
                    // candidates.sort();
                    let sum: usize = candidates.iter()
                        .filter(|c| <$gramtype>::dist(&$srchgrams[**c], &query_qgram) <= t2)
                        .filter(|c| {
                            $true_filter_chunks[**c].matches(querygrams, *t, &mut match_set)
                        })
                        .map(|c| {
                            ukkonen_map($srchdata[*c].1, &$srchdata[*c].0, qwlen, qwbytes, t + 1)
                        })
                        .sum();

                    // let sum: usize = $srchdata
                    //     .iter()
                    //     .enumerate()
                    //     .filter(|(wid, _)| <$gramtype>::dist(&$srchgrams[*wid], &query_qgram) <= t2)
                    //     .filter(|(wid, _)| {
                    //         $true_filter_chunks[*wid].matches(querygrams, *t, &mut match_set)
                    //     })
                    //     .map(|(_, word)| ukkonen_map(word.1, &word.0, qwlen, qwbytes, t + 1))
                    //     .sum();
                    acc + sum
                },
            )
            .sum()
    };
    // small DNA
    ($querydata:ident, $srchdata:ident, $len_map:ident, $gramtype:ty, $query_ngrams:ident,
        $srchgrams:ident, $true_filter_chunks:ident, $indexes:ident, false, false) => {
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
    ($querydata:ident, $srchdata:ident, $len_map:ident, $gramtype:ty, $max_threshold:ident, $use_true_match:expr, $use_length_filter:expr, $n:expr) => {{
        let srchgrams: Vec<$gramtype> = $srchdata
            .par_iter()
            .map(|data| <$gramtype>::new(&data.0))
            .collect();

        let mut true_filter_chunks: Vec<TrueMatchFilter> = vec![];
        let mut query_ngrams: Vec<Vec<(i32, usize)>> = vec![];
        // let max_threshold = $tset.last().unwrap();
        let mut indexes: Vec<FxHashMap<i32, Vec<(usize, usize)>>> =
            vec![FxHashMap::default(); $max_threshold + 1];
        if $use_true_match {
            true_filter_chunks = $srchdata
                .par_iter()
                .map(|record| record_to_chunk_filter(&record.0, $n))
                .collect();

            let start = std::time::Instant::now();
            let mut occurrences: BTreeMap<i32, usize> = BTreeMap::default();

            let percent_count = ($srchdata.len() as f32 * 0.1).floor() as usize;
            let percent_iteration = ($srchdata.len() / percent_count);
            // get occurences map to get global ordering
            for i in (0..true_filter_chunks.len()).step_by(percent_iteration) {
                for (chunk, _) in &true_filter_chunks[i].chunks {
                    occurrences
                        .entry(*chunk)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }

            // sort by occurence
            true_filter_chunks
                .par_iter_mut()
                .for_each(|fchunk| fchunk.index_chunks(&occurrences));

            for (id, record) in true_filter_chunks.iter().enumerate() {
                let mut t = 0;
                for (chunk, chunk_pos) in record.chunks.iter().take($max_threshold + 1) {
                    indexes[t]
                        .entry(*chunk)
                        .and_modify(|listings| listings.push((id, *chunk_pos)))
                        .or_insert(vec![(id, *chunk_pos)]);
                    t += 1;
                }
            }
            println!("Building indexes took {}ms", start.elapsed().as_millis());

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
                indexes,
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
                indexes,
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
                indexes,
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
                indexes,
                false,
                true
            ),
        }
    }};
}

pub(crate) use filtering;
pub(crate) use query;
