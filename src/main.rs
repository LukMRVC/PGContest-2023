use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::{prelude::*, stdin, stdout, BufReader};
mod ukkonen;
use ukkonen::ukkonen;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn read<R: Read>(reader: &mut BufReader<R>) {
    let mut srchdata: Vec<Vec<u8>> = Vec::<Vec<u8>>::with_capacity(8196usize);
    let mut querydata: Vec<(String, usize)> = Vec::<(String, usize)>::with_capacity(8196);
    let mut line = String::with_capacity(128);
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

    let sum: usize = querydata
        .par_iter()
        .map(|(query_word, t)| {
            let mut sum = 0;
            let qwlen = query_word.len();
            let qwbytes = query_word.as_bytes();
            srchdata.iter().enumerate().for_each(|(id, word)| {
                if word.len() > qwlen {
                    sum += ukkonen(qwbytes, word, t + 1, id + 1);
                } else {
                    sum += ukkonen(word, qwbytes, t + 1, id + 1);
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
