pub fn ukkonen(s1: &[u8], s2: &[u8], threshold: usize, record_id: usize) -> usize {
    let mut s1len = s1.len();
    let mut s2len = s2.len();

    // if cfg!(debug_assertions) {
    //     println!(
    //         "S1:{}\tS2:{}",
    //         std::str::from_utf8(s1).unwrap(),
    //         std::str::from_utf8(s2).unwrap(),
    //     )
    // }

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

    let s1 = &s1[t_start..s1len];
    let s2 = &s2[t_start..s2len];

    s1len -= t_start;
    s2len -= t_start;

    if s1len == 0 {
        return if s2len < threshold { record_id } else { 0 };
    }

    let new_threshold = std::cmp::min(s2len, threshold);
    let diff_len = s2len - s1len;

    if new_threshold < diff_len {
        return 0;
    }

    // initialize zero_k
    let zero_k: i32 = ((std::cmp::min(s1len, new_threshold) >> 1) + 2) as i32;

    let arr_len = diff_len + (zero_k as usize) * 2 + 2;
    let mut current_row = vec![-1i32; arr_len];
    let mut next_row = vec![-1i32; arr_len];

    let mut i = 0;
    let condition_row = diff_len as i32 + zero_k;
    let end_max = condition_row << 1;
    loop {
        i += 1;
        std::mem::swap(&mut next_row, &mut current_row);

        let start: i32;
        let mut next_cell: i32;
        let mut previous_cell: i32;
        let mut current_cell: i32 = -1;

        if i <= zero_k {
            start = -(i as i32) + 1;
            next_cell = i as i32 - 2i32;
        } else {
            start = i - (zero_k << 1) + 1;
            next_cell = current_row[(zero_k + start) as usize];
        }

        let end: i32;
        if i <= condition_row {
            end = i;
            next_row[(zero_k + i) as usize] = -1;
        } else {
            end = end_max - i;
        }

        let mut row_index = (start + zero_k) as usize;

        let mut t = 0i32;

        for k in start..end {
            previous_cell = current_cell;
            current_cell = next_cell;
            next_cell = current_row[row_index + 1];

            // max()
            t = std::cmp::max(
                std::cmp::max(current_cell + 1, previous_cell),
                next_cell + 1,
            );

            while (t as usize) < s1len
                && ((t + k) as usize) < s2len
                && s1[t as usize] == s2[(t + k) as usize]
            {
                t += 1;
            }

            next_row[row_index] = t;
            row_index += 1;
        }

        if !(next_row[condition_row as usize] < (s1len as i32) && i <= (new_threshold as i32)) {
            break if i as usize - 1 <= (threshold - 1) {
                record_id
            } else {
                0
            };
        }
    }
}
