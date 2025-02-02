use clap::Parser;
use competitive_programming_and_contest_24::lib::lib_dp::design_a_course_v2;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let file = match File::open(args.file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Errore nell'apertura del file: {}", e);
            return;
        }
    };
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();

    let mut current = match lines.next() {
        Some(Ok(line)) => line,
        _ => {
            println!("Errore nella lettura della prima riga.");
            return;
        }
    };

    let mut values: Vec<u32> = current
        .split_whitespace()
        .map(|x| match x.parse::<u32>() {
            Ok(val) => val,
            Err(_) => {
                println!("Errore nel parsing del valore: {}", x);
                0
            }
        })
        .collect();

    let n = values[0] as usize;
    let mut topics: Vec<Vec<u32>> = Vec::with_capacity(n);
    for _ in 0..n {
        current = match lines.next() {
            Some(Ok(line)) => line,
            _ => {
                println!("Errore nella lettura della prima riga.");
                return;
            }
        };

        values = current
            .split_whitespace()
            .map(|x| match x.parse::<u32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Errore nel parsing del valore: {}", x);
                    0
                }
            })
            .collect();
        topics.push(values);
    }
    println!("{}", design_a_course_v2(n, topics));
}
