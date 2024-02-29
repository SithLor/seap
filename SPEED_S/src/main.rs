//use rustup 2023 
const INPUT_FILE: &str = "./src/people.m.txt";
const OUTPUT_FILE: &str = "./src/seating_chart.txt";


use std::io::{BufWriter, Write, BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;
use std::cell::RefCell;
thread_local! {
    static MEMO: RefCell<HashMap<usize, (usize, usize)>> = RefCell::new(HashMap::new());
}
#[inline(always)]
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
    code_faster();
    println!("Time: {}ms", start.elapsed().as_millis());
    
}