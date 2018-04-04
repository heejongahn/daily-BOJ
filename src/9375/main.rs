use std::io;
use std::io::{Read, Error};
use std::collections::HashMap;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error occured: {}", e);
    }
}

fn run() -> Result<(), Error> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("read_to_string failed");
    let mut lines = buf.lines();

    let problem_count: usize = lines.next().expect("get proeblem count").parse().expect("problem count parse failed");
    let mut answers: Vec<i32> = Vec::with_capacity(problem_count);

    for _i in 0..problem_count {
        let clothings_count: usize = lines.next().expect("get next clothing count").parse().expect("clothing count parse failed");
        let mut kind_to_count_hash = HashMap::<&str, i32>::new();

        for _j in 0..clothings_count {
            let line: Vec<&str> = lines.next().expect("get next line").split(" ").collect();
            let kind = line[1];

            let count = kind_to_count_hash.entry(&kind).or_insert(0);
            *count = *count + 1;
        }
        let answer = kind_to_count_hash.values().fold(1, |acc, &x| acc * (x + 1) ) - 1;
        answers.push(answer);
    }

    for answer in answers {
        println!("{}", answer);
    }

    Ok(())
}
