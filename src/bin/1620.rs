extern crate daily_boj;

use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines().filter_map(Result::ok);

    let (pokemon_count, problem_count) = lines
        .next()
        .map(|line| {
            let mut iter = line.split_whitespace()
                .map(|s| s.parse::<usize>().expect("String -> usize"));
            (
                iter.next().expect("pokemon_count"),
                iter.next().expect("problem_count"),
            )
        })
        .unwrap();

    let mut name_to_num_hash = HashMap::<String, usize>::new();
    let mut pokemon_arr = vec![];

    lines
        .take(pokemon_count + problem_count)
        .enumerate()
        .for_each(|(i, line)| {
            if i < pokemon_count {
                name_to_num_hash.insert(line.clone(), i);
                pokemon_arr.push(line.clone());
            } else {
                match line.parse::<usize>() {
                    Ok(number) => {
                        println!("{}", pokemon_arr[number as usize - 1]);
                    }
                    _ => {
                        println!(
                            "{}",
                            name_to_num_hash.get(&line).expect("No such pokemon") + 1
                        );
                    }
                }
            }
        });
}
