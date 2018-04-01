use std::io;
use std::io::{Read, Error};
use std::collections::BTreeSet;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error occured: {}", e);
    }
}

fn run() -> Result<(), Error> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("read_to_string failed");

    let lines: Vec<String> = buf.lines().map(|s| s.to_string()).collect();
    let mut workers = BTreeSet::<String>::new();

    let workers_vec = &lines[1..];

    for line in workers_vec {
        if line.len() == 0 {
            break;
        }

        let worker = line.split(' ').next().expect("worker name");
        if workers.contains(worker) {
            workers.remove(worker);
        } else {
            workers.insert(worker.to_string());
        }
    }

    let mut workers: Vec<&String> = workers.iter().collect();
    workers.reverse();

    for worker in workers {
        println!("{}", worker);
    }

    Ok(())
}
