use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    fn read_str() -> String {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        buffer.trim_right().to_string()
    }

    let mut first_line = read_str();
    let mut first_line_iter = first_line.split_whitespace();
    let pokemon_count: i32 = first_line_iter.next().expect("pokemon_count").parse().expect("String -> i32");
    let problem_count: i32 = first_line_iter.next().expect("problem_count").parse().expect("String -> i32");

    let mut pokemon_arr = Vec::<String>::with_capacity((pokemon_count + 1) as usize);
    pokemon_arr.push(String::new());
    let mut name_to_num_hash = HashMap::<String, i32>::new();

    for i in 0..pokemon_count {
        let name = read_str();
        pokemon_arr.push(name.clone());
        name_to_num_hash.insert(name, i + 1);
    }

    for i in 0..problem_count {
        let problem = read_str();
        match problem.parse::<i32>() {
            Ok (number) => {
                println!("{}", pokemon_arr[number as usize]);
            },
            _ => {
                println!("{}", name_to_num_hash.get(&problem).expect("No such pokemon"));
            }
        }
    }
}
