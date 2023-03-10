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
                    let querygrams = query_ngram_list.unwrap();

                    let mut candidates: FxHashSet<usize> = FxHashSet::default();
                    for ct in 0..=*t {
                        for (sig, sig_pos) in querygrams.iter() {
                            let maybe_listings = $indexes[ct].get(sig);
                            if let Some(listings) = maybe_listings {
                                for (cid, cpos) in listings.iter() {
                                    if cpos.abs_diff(*sig_pos) <= *t {
                                        candidates.insert(*cid);
                                    }
                                }
                            }
                        }
                    }

                    let candidates: Vec<usize> = candidates.drain().collect();
                    let sum: usize = candidates.iter()
                    .filter(|c| $srchgrams[**c].str_len.abs_diff(qwlen) <= *t && <$gramtype>::dist(&$srchgrams[**c], &query_qgram) <= t2)
                    .map(|c| {
                        ukkonen_map($srchdata[*c].1, &$srchdata[*c].0, qwlen, qwbytes, t + 1)
                    })
                    .sum();
                    // dbg!(candidates.len());

                    // let srchdata_len = $srchdata.len();
                    // let start_idx = *$len_map.get(&qwlen.saturating_sub(*t)).unwrap_or(&0);
                    // let end_idx = $len_map.get(&(qwlen + *t + 1)).unwrap_or(&srchdata_len);
                    // let idx_diff = *end_idx - start_idx;

                    // let sum: usize = $srchdata
                    //     .iter()
                    //     .enumerate()
                    //     .skip(start_idx)
                    //     .take(idx_diff)
                    //     .filter(|(wid, _)| <$gramtype>::dist(&$srchgrams[*wid], &query_qgram) <= t2)
                    //     .filter(|(wid, _)| {
                    //         $true_filter_chunks[*wid].matches(
                    //             query_ngram_list.unwrap(),
                    //             *t,
                    //             &mut match_set,
                    //         )
                    //     })
                    //     .map(|(_, word)| ukkonen_map(word.1, &word.0, qwlen, qwbytes, t + 1))
                    //     .sum();
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
                    let n = $true_filter_chunks[0].n;


                    let mut candidates: FxHashSet<usize> = FxHashSet::default();
                    for ct in 0..=*t {
                        let pref_sig_count = (((qwlen - ct) as f32 / n as f32).ceil()) as usize;
                        let pref_querygram_len = std::cmp::min(qwlen - (pref_sig_count - ct) + 1, querygrams.len() - 1);
                        let querygrams = &querygrams[..pref_querygram_len];

                        for (sig, sig_pos) in querygrams.iter() {
                            let maybe_listings = $indexes[ct].get(sig);
                            if let Some(listings) = maybe_listings {
                                let mut skip = 0;
                                let step = 4;
                                for i in (0..listings.len()).step_by(step) {
                                    if sig_pos.abs_diff(listings[i].1) <= *t {
                                        let mut j = 1;
                                        while sig_pos.abs_diff(listings[i.saturating_sub(j)].1) <= *t && j < step {
                                            j += 1;
                                        }
                                        skip = i.saturating_sub(j - 1);
                                        break;
                                    }
                                }

                                for (cid, cpos) in listings.iter().skip(skip) {
                                    if sig_pos.abs_diff(*cpos) <= *t {
                                        candidates.insert(*cid);
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    let candidates: Vec<usize> = candidates.drain().collect();
                    // println!("Got {} candidates", candidates.len());
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
    ($querydata:ident, $srchdata:ident, $len_map:ident, $gramtype:ty, $tset:ident, $use_true_match:expr, $use_length_filter:expr, $n:expr) => {{
        let srchgrams: Vec<$gramtype> = $srchdata
            .par_iter()
            .map(|data| <$gramtype>::new(&data.0))
            .collect();

        let mut true_filter_chunks: Vec<TrueMatchFilter> = vec![];
        let mut query_ngrams: Vec<Vec<(i32, usize)>> = vec![];
        let tset: Vec<usize> = Vec::from_iter($tset.into_iter());
        let max_threshold = tset.last().unwrap();
        let mut indexes: Vec<HashMap<i32, Vec<(usize, usize)>>> =
            vec![HashMap::default(); max_threshold + 1];
        if $use_true_match {
            true_filter_chunks = $srchdata
                .par_iter()
                .map(|record| record_to_chunk_filter(&record.0, $n))
                .collect();

            // let start = std::time::Instant::now();
            // let mut occurrences: BTreeMap<i32, usize> = BTreeMap::default();

            // let percent_count = ($srchdata.len() as f32 * 0.4).floor() as usize;
            // let percent_iteration = ($srchdata.len() / percent_count);
            // // get occurences map to get global ordering
            // for i in (0..true_filter_chunks.len()).step_by(percent_iteration) {
            //     for (chunk, _) in &true_filter_chunks[i].chunks {
            //         occurrences
            //             .entry(*chunk)
            //             .and_modify(|e| *e += 1)
            //             .or_insert(1);
            //     }
            // }

            // sort by occurence
            // true_filter_chunks
            //     .par_iter_mut()
            //     .for_each(|fchunk| fchunk.index_chunks(&occurrences));
            // println!("Sorting for indexes took {}ms", start.elapsed().as_millis());

            // indexes.par_iter_mut().enumerate().for_each(|(ct, index)| {
            //     for (id, record) in true_filter_chunks.iter().enumerate() {
            //         if let Some((chunk, chunk_pos)) = record.chunks.get(ct) {
            //             index
            //             .entry(*chunk)
            //             .and_modify(|listings| listings.push((id, *chunk_pos)))
            //             .or_insert(vec![(id, *chunk_pos)]);
            //         }
            //     }
            // });

            let tset_map = tset.clone();

            let mut partial_indexes: Vec<HashMap<i32, Vec<(usize, usize)>>> = tset
                .par_iter()
                .enumerate()
                .map(|(i, t)| {
                    let mut previous_t = 0;
                    if i > 0 {
                        previous_t = tset_map.get(i.saturating_sub(1)).unwrap_or(&0) + i;
                    }
                    let df = t.saturating_sub(previous_t) + 1;
                    let mut index: HashMap<i32, Vec<(usize, usize)>> = HashMap::default();
                    for (id, record) in true_filter_chunks.iter().enumerate() {
                        for (chunk, chunk_pos) in record.chunks.iter().skip(previous_t).take(df) {
                            index
                                .entry(*chunk)
                                .and_modify(|listings| listings.push((id, *chunk_pos)))
                                .or_insert(vec![(id, *chunk_pos)]);
                        }
                    }
                    return index;
                })
                .collect();
            // println!("Building indexes took {}ms", start.elapsed().as_millis());

            for t in tset_map.iter() {
                indexes[*t] = partial_indexes.remove(0);
            }

            // for (id, record) in true_filter_chunks.iter().enumerate() {
            //     let mut t = 0;
            //     for (chunk, chunk_pos) in record.chunks.iter().take(*max_threshold) {
            //         indexes[t]
            //             .entry(*chunk)
            //             .and_modify(|listings| listings.push((id, *chunk_pos)))
            //             .or_insert(vec![(id, *chunk_pos)]);
            //         t += 1;
            //     }
            // }

            indexes.par_iter_mut().for_each(|ivx| {
                for listings in ivx.values_mut() {
                    listings.sort_by_key(|&(_, a)| a)
                }
            });
            // dbg!(&indexes[0]);
            // println!("Building indexes took {}ms", start.elapsed().as_millis());

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
