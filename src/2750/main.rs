use std::io;

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

    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("Failed to read line");

    let count = count.trim_right().parse().expect("Failed to parse count");

    for _ in 0..count {
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");

        numbers.push(number.trim_right().parse().expect("Failed to parse interger line"));
    }

    let sorted = quick_sort(&numbers);
    for number in sorted.iter() {
        println!("{}", number);
    }
}
