mod filters;
mod ukkonen;

use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::{prelude::*, stdin, stdout, BufReader};
use std::time::Instant;

use crossbeam_channel::unbounded;
use std::thread;
use ukkonen::ukkonen;

use crate::filters::Qgram;

fn read<R: Read>(reader: &mut BufReader<R>) {
    // let start = Instant::now();
    let mut srchdata: Vec<Vec<u8>> = Vec::<Vec<u8>>::with_capacity(1024 * 1024 * 64);
    let mut querydata: Vec<(String, usize)> =
        Vec::<(String, usize)>::with_capacity(1024 * 1024 * 64);
    let mut line = String::with_capacity(256);
    let srch_line = "[SEARCH]";

    let (sx, tx) = unbounded::<String>();

    let handle = std::thread::spawn(move || {
        let mut qgrams: Vec<Qgram> = Vec::with_capacity(1024 * 1024);
        loop {
            if let Ok(msg) = tx.recv() {
                if msg.eq_ignore_ascii_case(srch_line) {
                    break qgrams;
                }
                // println!("Building qgram");
                qgrams.push(Qgram::new(msg.as_bytes()));
            }
        }
    });
    // read database words
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }
        // remove newline
        line.pop();
        let line_clone = line.clone();
        sx.send(line_clone.clone())
            .expect("Failed to send line as bytes");
        if srch_line.eq_ignore_ascii_case(&line) {
            break;
        }

        srchdata.push(line_clone.into_bytes());
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
    // println!("Input finished");

    // let start = Instant::now();
    // let srchgrams: Vec<Qgram> = srchdata.iter().map(|data| Qgram::new(data)).collect();
    let srchgrams: Vec<Qgram> = handle.join().unwrap();

    // println!("Building Qgrams took: {} MS", start.elapsed().as_millis());

    let start = Instant::now();
    let sum: usize = querydata
        .par_iter()
        .fold(
            || 0usize,
            |acc, (query_word, t)| {
                let mut sum = 0;
                let qwlen = query_word.len();
                let qwbytes = query_word.as_bytes();
                let query_qgram = Qgram::new(qwbytes);
                let t2 = *t * 2;

                // let init_len = srchdata.len();

                // let filtered: Vec<(usize, &Vec<u8>)> = srchdata
                //     .iter()
                //     .enumerate()
                //     .filter(|(wid, _)| Qgram::dist(&srchgrams[*wid], &query_qgram, Some(t2)) <= t2)
                //     .collect();
                // let filtered_len = filtered.len();

                // println!("Filtered: {}", init_len - filtered_len);

                srchdata
                    .iter()
                    .enumerate()
                    .filter(|(wid, _)| Qgram::dist(&srchgrams[*wid], &query_qgram, Some(t2)) <= t2)
                    .for_each(|(id, word)| {
                        if word.len() > qwlen {
                            sum += ukkonen(qwbytes, word, t + 1, id + 1);
                        } else {
                            sum += ukkonen(word, qwbytes, t + 1, id + 1);
                        }
                    });
                acc + sum
            },
        )
        .sum();
    // println!("Querying took: {} MS", start.elapsed().as_millis());
    println!("{}", sum);
    stdout().flush().unwrap();
}

fn main() {
    // let s1 = "ACDF".to_owned();
    // let s2 = "ABCD".to_owned();

    // let q1 = Qgram::new(s1.as_bytes());
    // let q2 = Qgram::new(s2.as_bytes());

    // println!("{} {} {}", s1, s2, Qgram::dist(&q1, &q2));

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
