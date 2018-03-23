use std::io;
use std::io::BufRead;

fn expect_i32() -> i32 {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();

    handle.read_line(&mut buf);

    return buf.trim_right().parse().expect("Failed to expect i_32");
}

fn quick_sort(v: &mut [i32]) {
    let len = v.len();
    if len >= 2 {
        let pivot_index = partition(v);
        quick_sort(&mut v[0..pivot_index]);
        quick_sort(&mut v[pivot_index + 1..len]);
    }
}

fn partition(v: &mut [i32]) -> usize {
    let len = v.len();
    let pivot_index = len / 2;

    v.swap(pivot_index, len - 1);

    let mut store_index = 0;
    for i in 0..len - 1 {
        if (&v[i] < &v[len - 1]) {
            v.swap(i, store_index);
            store_index += 1;
        }
    }

    v.swap(store_index, len - 1);
    store_index
}

fn main() {
    let count = expect_i32();
    let mut numbers: Vec<_> = (0..count).map(|_| { expect_i32() }).collect();

    quick_sort(&mut numbers);
    for number in numbers.iter() {
        println!("{}", number);
    }
}
