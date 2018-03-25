use std::io;
use std::io::BufRead;

fn merge_sort(v: &[i32]) -> Vec<i32> {
    let len = v.len();
    if len < 2 {
        return v.to_vec();
    }

    let half = len / 2;

    let left = merge_sort(&v[0..half]);
    let right = merge_sort(&v[half..len]);

    return merge(&left, &right);
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let left_len = left.len();
    let right_len = right.len();

    let mut left_idx = 0;
    let mut right_idx = 0;

    let mut sorted = Vec::with_capacity(left_len + right_len);

    while (left_idx < left_len) && (right_idx < right_len) {
        if left[left_idx] < right[right_idx] {
            sorted.push(left[left_idx]);
            left_idx += 1;
        } else {
            sorted.push(right[right_idx]);
            right_idx += 1;
        }
    }

    if left_idx == left_len {
        sorted.extend_from_slice(&right[right_idx..right_len]);
    } else {
        sorted.extend_from_slice(&left[left_idx..left_len]);
    }

    return sorted;
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut expect_i32 = || {
        let mut buffer = String::new();
        handle.read_line(&mut buffer).unwrap();
        buffer[0..(buffer.len() - 1)].parse().expect("Failed to expect i_32")
    };

    let count = expect_i32();
    let numbers: Vec<_> = (0..count).map(|_| { expect_i32() }).collect();

    let sorted = merge_sort(&numbers[..]);
    for number in sorted.iter() {
        println!("{}", number);
    }
}
