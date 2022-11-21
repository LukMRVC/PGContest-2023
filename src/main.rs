use std::io::{prelude::*, stdin, stdout, BufReader};

fn ukkonen(s1: &[u8], s2: &[u8], threshold: usize, record_id: usize) -> usize {
    if s1 == s2 {
        return record_id;
    }

    let mut s1len = s1.len();
    let mut s2len = s2.len();

    if s1len.abs_diff(s2len) > threshold {
        return 0;
    }

    // make sure that |s1| is smaller or equal to s2
    // perform suffix trimming
    while s1len > 0 && s1[s1len - 1] == s2[s2len - 1] {
        s1len -= 1;
        s2len -= 1;
    }

    if s1len == 0 {
        return if s2len < threshold { record_id } else { 0 };
    }

    // now prefix trimming
    let mut t_start = 0;
    while t_start < s1len && s1[t_start] == s2[t_start] {
        t_start += 1;
    }

    s1len -= t_start;
    s2len -= t_start;

    if s1len == 0 {
        return if s2len < threshold { record_id } else { 0 };
    }

    let threshold = if s2len < threshold { s2len } else { threshold };
    let diff_len = s2len - s1len;

    if threshold < diff_len {
        return 0;
    }

    // initialize ZERO_K
    let ZERO_K = ((if s1len < threshold { s1len } else { threshold }) >> 1) + 2;
    let arr_len = diff_len + ZERO_K * 2 + 2;
    let mut current_row = vec![-1i32; arr_len];
    let mut next_row = vec![-1i32; arr_len];

    let mut i = 0;
    let condition_row = diff_len + ZERO_K;
    let end_max = condition_row << 1;
    loop {
        i += 1;
        let (current_row, next_row) = (&mut next_row, &mut current_row);

        let start: i32;
        let mut next_cell: i32;
        let mut previous_cell: i32;
        let mut current_cell: i32 = -1;

        if i <= ZERO_K {
            start = -(i as i32) + 1;
            next_cell = (i - 2) as i32;
        } else {
            start = (i - (ZERO_K << 1) + 1) as i32;
            next_cell = current_row[ZERO_K + start as usize];
        }

        let end: i32;
        if i <= condition_row {
            end = i as i32;
            next_row[ZERO_K + i] = -1;
        } else {
            end = (end_max - i) as i32;
        }

        let mut row_index = start as usize + ZERO_K;

        let mut t = 0i32;

        for k in start..end {
            previous_cell = current_cell;
            current_cell = next_cell;
            next_cell = current_row[row_index + 1];

            // max()
            t = std::cmp::max(
                current_cell + 1,
                std::cmp::max(previous_cell, next_cell + 1),
            );

            while (t as usize) < s1len
                && ((t + k) as usize) < s2len
                && s1[t as usize] == s2[(t + k) as usize]
            {
                t += 1;
            }

            row_index += 1;
        }

        next_row[row_index] = t;

        if !(next_row[condition_row] < (s1len as i32) && i <= threshold) {
            break;
        }
    }

    if i - 1 < threshold {
        record_id
    } else {
        0
    }
}

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

    // read database words
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
    // let mut matrix = vec![0usize; longest_word * longest_word];
    // initialize first row
    // for i in 0..longest_word {
    //     matrix[i] = i;
    // }
    // // initialize first column
    // for i in 0..longest_word {
    //     matrix[i * longest_word] = i;
    // }

    line.clear();
    // read query words
    let mut sum: usize = 0;
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }
        // remove newline
        line.pop();

        let Some((query_word, t)) = line.split_once(',') else {
            panic!("Cannot split!");
        };
        let t: usize = t.parse().unwrap();

        let qwlen = query_word.len();
        let qwbytes = query_word.as_bytes();
        for (id, word) in srchdata.iter().enumerate() {
            if (word.len() > qwlen) {
                sum += ukkonen(qwbytes, word, t, id + 1);
            } else {
                sum += ukkonen(word, qwbytes, t, id + 1);
            }
        }
        // sum += lve(&srchdata, word, t, &mut matrix, longest_word);

        // srchquery.push(line.clone().into_bytes());
        line.clear();
    }

    println!("{}", sum);
    stdout().flush().unwrap();
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let filename: &str;
    // if args.len() > 1 {
    //     filename = &args[1];
    //     let file = File::open(filename).unwrap();
    //     let mut reader = BufReader::new(file);
    //     read(&mut reader);
    // } else {
    let mut reader = BufReader::new(stdin().lock());
    read(&mut reader);
    // }
}
