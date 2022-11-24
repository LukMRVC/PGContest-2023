mod filters;
mod ukkonen;

use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::{prelude::*, stdin, stdout, BufReader};
use std::time::Instant;
use ukkonen::ukkonen;

fn read<R: Read>(reader: &mut BufReader<R>) {
    // let start = Instant::now();
    let mut srchdata: Vec<Vec<u8>> = Vec::<Vec<u8>>::with_capacity(1024 * 1024 * 64);
    let mut querydata: Vec<(String, usize)> =
        Vec::<(String, usize)>::with_capacity(1024 * 1024 * 64);
    let mut line = String::with_capacity(256);
    let srch_line = "[SEARCH]";

    // read database words
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }
        // remove newline
        line.pop();
        if srch_line.eq_ignore_ascii_case(&line) {
            break;
        }

        srchdata.push(line.clone().into_bytes());
        line.clear();
    }

    line.clear();

    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }
        line.pop();

        let Some((query_word, t)) = line.split_once(',') else {
            panic!("Cannot split!");
        };
        let t: usize = t.parse().unwrap();
        querydata.push((query_word.to_owned(), t));

        line.clear();
    }
    // let elapsed = start.elapsed();
    // println!("Reading input took: {} MS", elapsed.as_millis());

    let q = 2;
    let q2 = 2 * q;
    // let start = Instant::now();
    let srchgrams: Vec<filters::Qgram> = srchdata
        .par_iter()
        .map(|data| filters::Qgram::new(data, q))
        .collect();
    // let elapsed = start.elapsed();
    // println!("Building Qgrams took: {} MS", elapsed.as_millis());

    let sum: usize = querydata
        .par_iter()
        .map(|(query_word, t)| {
            let mut sum = 0;
            let qwlen = query_word.len();
            let qwbytes = query_word.as_bytes();
            let query_qgram = filters::Qgram::new(qwbytes, q);

            srchdata.iter().enumerate().for_each(|(id, word)| {
                if (query_qgram.dist(&srchgrams[id], *t) / q2) <= *t {
                    if word.len() > qwlen {
                        sum += ukkonen(qwbytes, word, t + 1, id + 1);
                    } else {
                        sum += ukkonen(word, qwbytes, t + 1, id + 1);
                    }
                }
            });
            sum
        })
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
        let mut reader = BufReader::new(file);
        read(&mut reader);
    } else {
        let mut reader = BufReader::new(stdin().lock());
        read(&mut reader);
    }
}
