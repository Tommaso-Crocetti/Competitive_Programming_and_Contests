use clap::Parser;
use handson_2_solution_tommaso_crocetti::lib::lib_minandmax::Minandmax;
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

    // Raccogli i valori in un Vec<u32> prima di accedere per indice
    let mut values: Vec<u32> = current
        .split_whitespace()
        .map(|x| match x.parse::<u32>() {
            Ok(val) => val,
            Err(_) => {
                println!("Errore nel parsing del valore: {}", x);
                0 // In questo caso restituiamo 0 per continuare
            }
        })
        .collect();

    let n = values[0];

    current = match lines.next() {
        Some(Ok(line)) => line,
        _ => {
            println!("Errore nella lettura della prima riga.");
            return;
        }
    };

    // Raccogli i valori in un Vec<u32> prima di accedere per indice
    values = current
        .split_whitespace()
        .map(|x| match x.parse::<u32>() {
            Ok(val) => val,
            Err(_) => {
                println!("Errore nel parsing del valore: {}", x);
                0 // In questo caso restituiamo 0 per continuare
            }
        })
        .collect();

    if values.len() != n as usize {
        println!("Dimensione errata dell'array");
        return;
    }
    // Crea un Segment Tree
    let mut seg_tree = Minandmax::new(&values);

    // Leggi tutte le righe rimanenti e processa ciascuna
    for line in lines {
        match line {
            Ok(line_content) => {
                // Processa la riga (es. separazione e parsing dei valori)
                let query_values: Vec<u32> = line_content
                    .split_whitespace()
                    .map(|x| match x.parse::<u32>() {
                        Ok(val) => val,
                        Err(_) => {
                            println!("Errore nel parsing del valore: {}", x);
                            0 // In questo caso restituiamo 0 per continuare
                        }
                    })
                    .collect();
                if query_values[0] == 0 {
                    seg_tree.update(
                        (query_values[1] - 1) as usize,
                        (query_values[2] - 1) as usize,
                        query_values[3],
                    );
                } else if let Some(res) = seg_tree.max(
                    (query_values[1] - 1) as usize,
                    (query_values[2] - 1) as usize,
                ) {
                    println!("{}", res)
                }
            }
            Err(_) => {
                println!("Errore nella lettura di una riga.");
                return; // Se c'è un errore, termina la funzione
            }
        }
    }
}
