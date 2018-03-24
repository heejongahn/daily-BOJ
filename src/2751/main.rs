use std::io;
use std::io::BufRead;

fn expect_i32() -> i32 {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();

    handle.read_line(&mut buf);

    return buf.trim_right().parse().expect("Failed to expect i_32");
}

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

    while (left_idx < left_len) || (right_idx < right_len) {
        // Only right elems are left
        if left_idx == left_len {
            sorted.push(right[right_idx]);
            right_idx += 1;
            continue;
        }

        // Only left elems are left
        if right_idx == right_len {
            sorted.push(left[left_idx]);
            left_idx += 1;
            continue;
        }

        if left[left_idx] < right[right_idx] {
            sorted.push(left[left_idx]);
            left_idx += 1;
        } else {
            sorted.push(right[right_idx]);
            right_idx += 1;
        }
    }

    return sorted;
}

fn main() {
    let count = expect_i32();
    let numbers: Vec<_> = (0..count).map(|_| { expect_i32() }).collect();

    let sorted = merge_sort(&numbers[..]);
    for number in sorted.iter() {
        println!("{}", number);
    }
}
