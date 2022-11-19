use std::env;
use std::fs::File;
use std::io::{prelude::*, stdin, stdout, BufReader};

fn lve(
    data: &[Vec<u8>],
    query_word: &str,
    threshold: usize,
    m: &mut [usize],
    msize: usize,
) -> usize {
    let mut sum = 0usize;
    let qwlen = query_word.len() + 1;
    let qw = query_word.as_bytes();

    for (id, word) in data.iter().enumerate() {
        let mut min = 0usize;
        let wlen = word.len() + 1;
        // let mut begin = 0usize;
        // let mut begin_value = 0usize;
        // let mut end = threshold;
        // let mut new_max: bool = false;

        if threshold == 0 {
            if word == qw {
                sum += id + 1;
                continue;
            }
        }
        let start_idx = 1;
        for i in 1..wlen {
            // if begin > start_idx {
            //     start_idx = begin;
            // }
            // new_max = false;
            // begin_value = m[(i - 1) * msize + begin];
            for j in start_idx..qwlen {
                let insertion = m[(i - 1) * msize + j] + 1;
                let deletion = m[i * msize + (j - 1)] + 1;
                let substitution =
                    m[(i - 1) * msize + (j - 1)] + ((word[i - 1] != qw[j - 1]) as usize);
                min = std::cmp::min(substitution, std::cmp::min(insertion, deletion));
                m[i * msize + j] = min;
            }
            let rowmin = m[(i * msize)..(i * msize + qwlen)].iter().min().unwrap();
            if *rowmin > threshold {
                break;
            }
        }

        if min <= threshold {
            // println!("LVE: {} {} = {}", query_word, sword, min);
            sum += id + 1;
        }

        // println!("QW: {}\t{} = {}", query_word, sword, min);
        // for i in 0..wlen {
        //     for j in 0..qwlen {
        //         print!("{} ", m[i * msize + j])
        //     }
        //     println!(" ")
        // }
        // println!("-------------------------------")
    }

    return sum;
}

fn read<R: Read>(reader: &mut BufReader<R>) {
    let mut srchdata: Vec<Vec<u8>> = Vec::<Vec<u8>>::with_capacity(8196usize);
    let mut line = String::with_capacity(128);
    let srch_line = "[SEARCH]";
    let mut longest_word: usize = 0;

    // let mut srchquery: Vec<Vec<u8>> = Vec::<Vec<u8>>::with_capacity(8196usize);

    let mut line_len: usize;
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }
        // remove newline
        line.pop();
        line_len = line.len();
        if srch_line.eq_ignore_ascii_case(&line) {
            break;
        }

        if line_len > longest_word {
            longest_word = line_len;
        }

        srchdata.push(line.clone().into_bytes());
        line.clear();
    }
    // for the zeroth elem
    longest_word += 1;
    let mut matrix = vec![0usize; longest_word * longest_word];
    // initialize first row
    for i in 0..longest_word {
        matrix[i] = i;
    }
    // initialize first column
    for i in 0..longest_word {
        matrix[i * longest_word] = i;
    }

    line.clear();

    let mut sum: usize = 0;
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }
        // remove newline
        line.pop();

        let Some((word, t)) = line.split_once(',') else {
            panic!("Cannot split!");
        };
        let t: usize = t.parse().unwrap();
        sum += lve(&srchdata, word, t, &mut matrix, longest_word);

        // srchquery.push(line.clone().into_bytes());
        line.clear();
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
        let mut reader = BufReader::new(file);
        read(&mut reader);
    } else {
        let mut reader = BufReader::new(stdin().lock());
        read(&mut reader);
    }
}
