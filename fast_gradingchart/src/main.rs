//use rustup 2024 ; cargo run 
const OUTPUT_FILE: &str = "./src/output.txt";
const INPUT_FILE: &str = "./src/input.txt";
use std::io::{BufRead, BufWriter, Write};

fn min_rows_cols(amount: usize) -> (usize, usize) {
    let sqrt = (amount as f64).sqrt();
    let rows = sqrt.ceil() as usize;
    let cols = if rows * (rows - 1) >= amount {
        rows - 1
    } else {
        rows
    };
    (rows, cols)
}
fn code_slow(){
        //4044 
        //file input
        let input: String = std::fs::read_to_string(INPUT_FILE).expect("Unable to read file");
        let mut output: std::fs::File = std::fs::File::create(OUTPUT_FILE).expect("Unable to create file");
        let mut grades: Vec<i8> = Vec::new();
        let lines: std::str::Lines<'_> = input.lines();
        for line in lines {
            //parse grade split by space
            let grade: i8 = line.split_whitespace().collect::<Vec<&str>>()[1]
                .parse()
                .expect("Unable to parse grade");
            grades.push(grade);
        }
        //get the avg of the grades
        let avg: f64 = grades.iter().map(|&x| x as i32).sum::<i32>() as f64 / grades.len() as f64;
        println!("Average: {}", avg);
        let (rows, cols) = min_rows_cols(grades.len());
        for i in 0..rows {
            for j in 0..cols {
                if let Some(grade) = grades.get(i * cols + j) {
                    write!(output, "{} ", grade).expect("Unable to write to file");
                }
            }
            write!(output, "\n").unwrap();
        }
    
}



fn code_fast() {
    let input: std::fs::File = std::fs::File::open(INPUT_FILE).expect("Unable to read file");
    let input: std::io::BufReader<std::fs::File> = std::io::BufReader::new(input);

    let output: std::fs::File = std::fs::File::create(OUTPUT_FILE).expect("Unable to create file");
    let mut output: BufWriter<std::fs::File> = BufWriter::new(output);

    let mut grades: Vec<i8> = Vec::new();

    for line in input.lines() {
        let line: String = line.expect("Unable to read line");
        let grade: i8 = line.split_whitespace().nth(1)
            .and_then(|s| s.parse().ok())
            .expect("Unable to parse grade");
        grades.push(grade);
    }

    let avg: f64 = grades.iter().map(|&x| x as i32).sum::<i32>() as f64 / grades.len() as f64;
    println!("Average: {}", avg);

    let (rows, cols) = min_rows_cols(grades.len());
    for i in 0..rows {
        for j in 0..cols {
            if let Some(grade) = grades.get(i * cols + j) {
                write!(output, "{} ", grade).expect("Unable to write to file");
            }
        }
        writeln!(output).unwrap();
    }
}
fn main() {
    let start = std::time::Instant::now();
    code_fast();
    println!("Time: {}ms", start.elapsed().as_millis());
}

