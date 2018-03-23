use std::io;
use std::io::BufRead;

fn expect_i32() -> i32 {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();

    handle.read_line(&mut buf);

    return buf.trim_right().parse().expect("Failed to expect i_32");
}

fn quick_sort(v: &Vec<i32>) -> Vec<i32> {
    let len = v.len();

    match len {
        0 => vec![],
        1 => vec![v[0]],
        _ => {
            let pivot = v[0];
            let mut less = Vec::new();
            let mut greater = Vec::new();

            for i in 1..len {
                let item = v[i];
                if pivot <= item {
                    greater.push(item);
                } else {
                    less.push(item);
                }
            }

            let mut sorted = quick_sort(&less);
            sorted.push(pivot);
            sorted.extend(&quick_sort(&greater));
            return sorted
        }
    }
}

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    let count = expect_i32();

    for _ in 0..count {
        let number = expect_i32();
        numbers.push(number);
    }

    let sorted = quick_sort(&numbers);
    for number in sorted.iter() {
        println!("{}", number);
    }
}
