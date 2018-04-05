use std::io;
use std::f32;

fn is_prime(n: i32) -> bool {
    for i in 2..n {
        if ((n % i) == 0) {
            return false;
        }
    }

    return true;
}

fn main() {
    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("Failed to read line");

    let mut numbers_str = String::new();
    io::stdin().read_line(&mut numbers_str).expect("Failed to read line");

    let strings = numbers_str.trim().split(" ");
    let numbers: Vec<i32> = Vec::new();

    let mut primes: Vec<i32> = Vec::new();
    for string in strings {
        match (str::parse::<i32>(string)) {
            Ok(n) if n > 1 && is_prime(n) => { primes.push(n) },
            _ => {}
        }
    }

    print!("{}", primes.len())
}
