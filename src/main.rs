mod filters;
mod macros;
mod statistics;
mod ukkonen;

use linereader::LineReader;
use rayon::prelude::*;
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

// ngrams and nchunks size
const N: usize = 3;

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
    let mut srchdata: Vec<Vec<u8>> = Vec::<Vec<u8>>::with_capacity(1024 * 1024 * 64);
    let mut querydata: Vec<(String, usize)> =
        Vec::<(String, usize)>::with_capacity(1024 * 1024 * 64);
    let srch_line = "[SEARCH]";
    let srch_line_bytes = srch_line.as_bytes();
    let mut reader = LineReader::with_capacity(1024 * 1024, file);
    let mut is_dna: bool = true;
    let mut len_sum = 0;

    let start = std::time::Instant::now();
    while let Some(line) = reader.next_line() {
        let line = line.expect("read error");
        let line = &line[0..line.len() - 1];
        if line.eq_ignore_ascii_case(srch_line_bytes) {
            break;
        }
        srchdata.push(line.to_owned());
        len_sum += line.len();
    }

    // println!("Reading data took: {} ms", start.elapsed().as_millis());

    'outer: for srchline in srchdata.iter().take(5) {
        for char_byte in srchline.iter() {
            if char_byte != &65 && char_byte != &67 && char_byte != &71 && char_byte != &84 {
                is_dna = false;
                break 'outer;
            }
        }
    }

    // let mean_record_length = (len_sum as f32) / (srchdata.len() as f32);
    // println!("Mean rec len is {}", mean_record_length);
    // let srchdata_lenghts: Vec<usize> = srchdata.par_iter().map(|line| line.len()).collect();
    // let deviation = statistics::std_dev(&srchdata_lenghts, data_mean);

    while let Some(line) = reader.next_line() {
        let line = line.expect("read error");
        let line = &line[0..line.len() - 1];
        let simd_line = simdutf8::basic::from_utf8(line).unwrap();
        let Some((query_word, t)) = simd_line.split_once(',') else {
            panic!("Cannot split!");
        };
        let t: usize = t.parse().unwrap();
        querydata.push((query_word.to_owned(), t));
    }

    let mut sum: usize = 0;

    if is_dna {
        sum = macros::filtering!(
            querydata,
            srchdata,
            DNAQgram,
            srchdata.len() >= 250_000 || querydata.len() > 150,
            false,
            5
        );
    } else {
        sum = macros::filtering!(querydata, srchdata, Qgram, true, true, 3);
    }

    println!("{}", sum);
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
