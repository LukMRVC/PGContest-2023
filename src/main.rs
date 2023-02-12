mod filters;
mod macros;
mod statistics;
mod ukkonen;

use fxhash::{FxHashMap, FxHashSet};
use linereader::LineReader;
use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use std::fs::File;
use std::io::{prelude::*, stdin, stdout};

use ukkonen::ukkonen;

use crate::filters::{
    dnaqgram_filter::DNAQgram,
    qgram_filter::Qgram,
    true_match_filter::{ngrams, TrueMatchFilter},
    NgramFilter,
};

#[inline(always)]
fn ukkonen_map(id: usize, word: &[u8], qwlen: usize, qwbytes: &[u8], threshold: usize) -> usize {
    if word.len() > qwlen {
        ukkonen(qwbytes, word, threshold, id)
    } else {
        ukkonen(word, qwbytes, threshold, id)
    }
}

#[inline(always)]
fn record_to_chunk_filter(record: &Vec<u8>, n: usize) -> TrueMatchFilter {
    TrueMatchFilter::new(record, n)
}

#[inline(always)]
fn record_to_ngrams(record: &mut String, n: usize) -> Vec<(i32, usize)> {
    let padding = String::from_utf8(vec![b'$'; n - 1]).unwrap();
    record.push_str(&padding);
    ngrams(record.as_bytes(), n)
}

fn read<R: std::io::Read>(file: R) {
    // let start = Instant::now();
    let mut srchdata: Vec<(Vec<u8>, usize)> = Vec::with_capacity(1024 * 1024 * 64);
    let mut querydata: Vec<(String, usize)> =
        Vec::<(String, usize)>::with_capacity(1024 * 1024 * 64);
    let srch_line = "[SEARCH]";
    let srch_line_bytes = srch_line.as_bytes();
    let mut reader = LineReader::with_capacity(1024 * 1024, file);
    let mut is_dna: bool = true;

    let mut str_id = 1;
    let mut min_line_len = usize::MAX;
    // let mut len_distribution: BTreeMap<usize, usize> = BTreeMap::new();

    while let Some(line) = reader.next_line() {
        let line = line.expect("read error");
        let line = &line[0..line.len() - 1];
        if line.eq_ignore_ascii_case(srch_line_bytes) {
            break;
        }
        srchdata.push((line.to_owned(), str_id));
        str_id += 1;
        // len_sum += line.len();
        if line.len() < min_line_len {
            min_line_len = line.len();
        }

        // if let Some(count) = len_distribution.get_mut(&line.len()) {
        //     *count += 1;
        // } else {
        //     len_distribution.insert(line.len(), 1);
        // }
    }
    // start.elapsed().as_millis()

    // println!("Reading data took: {} ms", start.elapsed().as_millis());

    'outer: for (srchline, _) in srchdata.iter().take(5) {
        for char_byte in srchline.iter() {
            if char_byte != &65 && char_byte != &67 && char_byte != &71 && char_byte != &84 {
                is_dna = false;
                break 'outer;
            }
        }
    }

    let mut len_map = BTreeMap::<usize, usize>::new();

    // let mut cummulative_count = 0;
    // for l in 0..=(*len_distribution.keys().max().unwrap()) {
    //     len_map.insert(l, cummulative_count);
    //     cummulative_count += len_distribution.get(&l).unwrap_or(&0);
    // }

    // perform jump search

    // dbg!(len_distribution);
    // dbg!(&len_map);

    // let mean_record_length = (len_sum as f32) / (srchdata.len() as f32);
    // println!("Mean rec len is {}", mean_record_length);
    // let srchdata_lenghts: Vec<usize> = srchdata.par_iter().map(|line| line.len()).collect();
    // let deviation = statistics::std_dev(&srchdata_lenghts, data_mean);
    // let mut max_threshold = usize::MIN;
    // threshold set
    let mut tset: BTreeSet<usize> = BTreeSet::default();

    while let Some(line) = reader.next_line() {
        let line = line.expect("read error");
        let line = &line[0..line.len() - 1];
        let simd_line = simdutf8::basic::from_utf8(line).unwrap();
        let Some((query_word, t)) = simd_line.split_once(',') else {
            panic!("Cannot split!");
        };
        let t: usize = t.parse().unwrap();
        tset.insert(t);
        // if t > max_threshold {
        //     max_threshold = t;
        // }
        querydata.push((query_word.to_owned(), t));
    }
    let mut sum: usize = 0;

    if is_dna {
        sum = macros::filtering!(
            querydata,
            srchdata,
            len_map,
            DNAQgram,
            tset,
            srchdata.len() >= 250_000 || querydata.len() > 150,
            false,
            5
        );
    } else {
        srchdata.par_sort_unstable_by(|a, b| a.0.len().cmp(&b.0.len()));
        let mut last_len = srchdata[0].0.len();
        len_map.insert(last_len, 0);
        for i in (0..srchdata.len()).step_by(8) {
            if srchdata[i].0.len() > last_len {
                let mut j = 1;
                while srchdata[i - j].0.len() != last_len {
                    j += 1;
                }
                last_len = srchdata[i - j + 1].0.len();
                len_map.insert(last_len, i - j + 1);
            }
        }
        sum = macros::filtering!(querydata, srchdata, len_map, Qgram, tset, false, true, 2);
    }

    println!("{sum}");
    stdout().flush().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str;
    if args.len() > 1 {
        filename = &args[1];
        let file = File::open(filename).unwrap();
        read(file);
    } else {
        read(stdin().lock());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filtering_skip_and_take_is_ok() {
        let mut srchdata: Vec<(&str, usize)> = vec![
            ("karel", 1),
            ("robot", 2),
            ("kafe", 3),
            ("otoman", 4),
            ("stul", 5),
            ("sysel", 6),
            ("hubert", 7),
            ("kanape", 8),
            ("jurta", 9),
            ("nhl", 10),
            ("ork", 11),
            ("vlak", 12),
            ("zidle", 13),
            ("afrika", 14),
            ("evropa", 15),
            ("slon", 16),
            ("zebra", 17),
            ("saty", 18),
            ("auto", 19),
            ("autobus", 20),
            ("kolo", 21),
            ("fontana", 22),
            ("opera", 23),
            ("rakousko", 24),
            ("hora", 25),
            ("beh", 26),
            ("touha", 27),
            ("kamarad", 28),
            ("pocitac", 29),
            ("procesor", 30),
            ("klavesnice", 31),
            ("mys", 32),
            ("parek", 33),
        ];

        srchdata.sort_by(|a, b| a.0.len().cmp(&b.0.len()));
        let qdata = vec![
            ("kryl", 2),
            ("dale", 2),
            ("chobot", 2),
            ("lak", 2),
            ("kanar", 2),
            ("obraz", 2),
            ("hul", 2),
            ("panak", 3),
        ];

        let mut len_map: BTreeMap<usize, usize> = BTreeMap::new();
        len_map.insert(0, 0);
        len_map.insert(1, 0);
        len_map.insert(2, 0);
        len_map.insert(3, 0);
        len_map.insert(4, 4);
        len_map.insert(5, 12);
        len_map.insert(6, 21);
        len_map.insert(7, 26);
        len_map.insert(8, 30);
        len_map.insert(9, 32);
        len_map.insert(10, 32);

        for (query_word, t) in qdata {
            let qwlen = query_word.len();

            let srchdata_len = srchdata.len();
            let start_idx = *len_map.get(&qwlen.saturating_sub(t)).unwrap_or(&0);
            let end_idx = len_map.get(&(qwlen + t + 1)).unwrap_or(&srchdata_len);
            let idx_diff = *end_idx - start_idx;

            let it_ski_take: Vec<&(&str, usize)> =
                srchdata.iter().skip(start_idx).take(idx_diff).collect();
            let it_filt: Vec<&(&str, usize)> = srchdata
                .iter()
                .filter(|a| a.0.len().abs_diff(qwlen) <= t)
                .collect();

            assert_eq!(it_ski_take, it_filt);

            let sm1: usize = it_ski_take
                .iter()
                .map(|word| {
                    ukkonen_map(
                        word.1,
                        word.0.as_bytes(),
                        qwlen,
                        query_word.as_bytes(),
                        t + 1,
                    )
                })
                .sum();

            let sm2: usize = it_filt
                .iter()
                .map(|word| {
                    ukkonen_map(
                        word.1,
                        word.0.as_bytes(),
                        qwlen,
                        query_word.as_bytes(),
                        t + 1,
                    )
                })
                .sum();

            assert_eq!(sm1, sm2);
        }
    }
}
