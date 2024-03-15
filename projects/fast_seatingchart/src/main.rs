//use rustup 2023 
const INPUT_FILE: &str = "./src/people.m.txt";
const OUTPUT_FILE: &str = "./src/seating_chart.txt";

use rayon::prelude::*;
use std::io::{BufWriter, Write, BufReader, BufRead};
use std::fs::File;
use std::sync::Mutex;
use std::collections::HashMap;
use std::cell::RefCell;

thread_local! {
    static MEMO: RefCell<HashMap<usize, (usize, usize)>> = RefCell::new(HashMap::new());
}
fn min_rows_cols(amount: usize) -> (usize, usize) {
    MEMO.with(|memo| {
        if let Some(&result) = memo.borrow().get(&amount) {
            return result;
        }
        let sqrt: f64 = (amount as f64).sqrt();
        let rows: usize = sqrt.ceil() as usize;
        let cols: usize = if rows * (rows - 1) >= amount {
            rows - 1
        } else {
            rows
        };
        let result = (rows, cols);
        memo.borrow_mut().insert(amount, result);
        result
    })
}

//fn min_rows_cols(amount: usize) -> (usize, usize) {
//    let sqrt = (amount as f64).sqrt();
//    let rows = sqrt.ceil() as usize;
//    let cols = if rows * (rows - 1) >= amount { rows - 1 } else { rows };
//    (rows, cols)
//}
fn code_slow(){
    let file: std::fs::File = std::fs::File::open(INPUT_FILE).unwrap();
    let reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(file);
    let mut people: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        people.push(line);
    }
    let (rows, cols) = min_rows_cols(people.len());
    let mut seating_chart = vec![vec!["".to_string(); cols]; rows];
    for (i, person) in people.iter().enumerate() {
        let row: usize = i / cols;
        let col: usize = i % cols;
        seating_chart[row][col] = person.to_string();
    }
    let mut file: std::fs::File = std::fs::File::create(OUTPUT_FILE).unwrap();
    for row in seating_chart {
        for person in row {
            write!(file, "{:20}", person).unwrap();
        }
        writeln!(file).unwrap();
    }
    println!("Seating chart written to {}", OUTPUT_FILE);
}

fn code_fast() {
    let input: std::fs::File = std::fs::File::open(INPUT_FILE).expect("Unable to read file");
    let input: std::io::BufReader<std::fs::File> = std::io::BufReader::new(input);

    let output: std::fs::File = std::fs::File::create(OUTPUT_FILE).expect("Unable to create file");
    let mut output: BufWriter<std::fs::File> = BufWriter::new(output);

    let mut people: Vec<String> = Vec::new();

    for line in input.lines() {
        let line: String = line.expect("Unable to read line");
        people.push(line);
    }

    let (rows, cols) = min_rows_cols(people.len());
    let mut seating_chart: Vec<Vec<String>> = vec![vec!["".to_string(); cols]; rows];
    for (i, person) in people.iter().enumerate() {
        let row = i / cols;
        let col = i % cols;
        seating_chart[row][col] = person.to_string();
    }

    for row in &seating_chart {
        for person in row {
            write!(output, "{:20}", person).expect("Unable to write to file");
        }
        writeln!(output).unwrap();
    }
    println!("Seating chart written to {}", OUTPUT_FILE);
}

fn code_rayon() {
    let input: File = File::open(INPUT_FILE).expect("Unable to read file");
    let input: BufReader<File> = BufReader::new(input);

    let output: File = File::create(OUTPUT_FILE).expect("Unable to create file");
    let mut output: BufWriter<File> = BufWriter::new(output);

    let people: Vec<String> = input.lines()
        .filter_map(|line| line.ok())
        .collect();

    let (rows, cols) = min_rows_cols(people.len());
    let mut seating_chart: Vec<Vec<Mutex<String>>> = Vec::with_capacity(rows);
    for _ in 0..rows {
        let mut row: Vec<Mutex<String>> = Vec::with_capacity(cols);
        for _ in 0..cols {
            row.push(Mutex::new("".to_string()));
        }
        seating_chart.push(row);
    }

    people.par_iter().enumerate().for_each(|(i, person)| {
        let row = i / cols;
        let col = i % cols;
        *seating_chart[row][col].lock().unwrap() = person.to_string();
    });

    for row in &seating_chart {
        for person in row {
            write!(output, "{:20}", person.lock().unwrap()).expect("Unable to write to file");
        }
        writeln!(output).unwrap();
    }
    println!("Seating chart written to {}", OUTPUT_FILE);
}


fn code_faster() {
    let input: File = File::open(INPUT_FILE).expect("Unable to read file");
    let input: BufReader<File> = BufReader::new(input);

    let output: File = File::create(OUTPUT_FILE).expect("Unable to create file");
    let mut output: BufWriter<File> = BufWriter::new(output);

    let people: Vec<String> = input.lines()
        .filter_map(|line| line.ok())
        .collect();

    let (rows, cols) = min_rows_cols(people.len());
    let mut seating_chart: Vec<Vec<String>> = vec![vec!["".to_string(); cols]; rows];

    for (i, person) in people.iter().enumerate() {
        let row = i / cols;
        let col = i % cols;
        seating_chart[row][col] = person.to_string();
    }

    for row in &seating_chart {
        let line: String = row.join(" ");
        writeln!(output, "{:20}", line).expect("Unable to write to file");
    }
    println!("Seating chart written to {}", OUTPUT_FILE);
}
fn main() {
    //Get the time it takes to run the code
    let start: std::time::Instant = std::time::Instant::now();
    code_rayon();
    println!("Time: {}ms", start.elapsed().as_millis());
    
}
