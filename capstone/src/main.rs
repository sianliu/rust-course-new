mod location;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
mod transaction;
use transaction::Transaction;

fn main() {
    let file = File::open("transactions.csv").unwrap();
    let reader = BufReader::new(file);
    let &mut transactions: Vec<Transaction> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            Transaction::from_csv_line(&line).unwrap()
        })
        .collect();
    let mut skipped_lines = Vec::new();


    for idx,line in reader.lines().enumerate() {
        if idx == 0 {
            continue; // Skip header
        }
        let line_str = line.unwrap();
        let parsed_transaction = Transaction::from_csv_line(&line_str);
        match parsed_transaction {
            Ok(transaction) => transactions.push(transaction),
            Err(e) => skipped_lines.push(idx, e, line_str),
        }
    }
    for tx in transactions {
        println!()
    }
}
