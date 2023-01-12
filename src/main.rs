mod filters;
mod ukkonen;

use linereader::LineReader;
use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::{prelude::*, stdin, stdout, BufReader};
use std::time::Instant;

use crossbeam_channel::unbounded;
use ukkonen::ukkonen;

use crate::filters::Qgram;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn read<R: std::io::Read>(file: R) {
    // let start = Instant::now();
    let mut srchdata: Vec<Vec<u8>> = Vec::<Vec<u8>>::with_capacity(1024 * 1024 * 64);
    let mut querydata: Vec<(String, usize)> =
        Vec::<(String, usize)>::with_capacity(1024 * 1024 * 64);
    let srch_line = "[SEARCH]";
    let srch_line_bytes = srch_line.as_bytes();
    let mut reader = LineReader::with_capacity(1024 * 1024, file);

    while let Some(line) = reader.next_line() {
        let line = line.expect("read error");
        let line = &line[0..line.len() - 1];
        if line.eq_ignore_ascii_case(srch_line_bytes) {
            break;
        }
        srchdata.push(line.to_owned());
    }

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

    let srchgrams: Vec<Qgram> = srchdata.par_iter().map(|data| Qgram::new(data)).collect();

    let sum: usize = querydata
        .par_iter()
        .fold(
            || 0usize,
            |acc, (query_word, t)| {
                // let mut sum = 0;
                let qwlen = query_word.len();
                let qwbytes = query_word.as_bytes();
                let query_qgram = Qgram::new(qwbytes);
                let t2 = *t * 2;

                let sum: usize = srchdata
                    .iter()
                    .enumerate()
                    .filter(|(wid, _)| Qgram::dist(&srchgrams[*wid], &query_qgram) <= t2)
                    .map(|(id, word)| {
                        if word.len() > qwlen {
                            ukkonen(qwbytes, word, t + 1, id + 1)
                        } else {
                            ukkonen(word, qwbytes, t + 1, id + 1)
                        }
                        // id + 1
                    })
                    .sum();
                acc + sum
            },
        )
        .sum();
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
