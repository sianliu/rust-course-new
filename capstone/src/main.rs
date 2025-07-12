/// Capstone Project - Transaction Data Analysis
///
/// This program processes transaction data from a CSV file and performs various analyses:
/// - Filters out records with missing values
/// - Capitalizes all asset names
/// - Calculates geographical continents and days under management
/// - Provides comprehensive error handling
/// - Generates summary statistics by continent
mod location;
mod transaction;

use location::Continent;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use transaction::Transaction;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CAPSTONE PROJECT - TRANSACTION DATA ANALYSIS ===\n");

    // Open and read the CSV file with proper error handling
    let file = File::open("./transaction.csv")
        .map_err(|e| format!("Failed to open transaction.csv: {}", e))?;

    let reader = BufReader::new(file);
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut skipped_records: Vec<(usize, String, String)> = Vec::new();

    // Process each line with proper error handling
    for (line_number, line_result) in reader.lines().enumerate() {
        // Skip header row
        if line_number == 0 {
            continue;
        }

        match line_result {
            Ok(line_content) => {
                // Check for missing values (empty fields or consecutive commas)
                if has_missing_values(&line_content) {
                    skipped_records.push((
                        line_number + 1,
                        "Missing required field(s)".to_string(),
                        line_content,
                    ));
                    continue;
                }

                // Attempt to parse the transaction
                match Transaction::from_csv_line(&line_content) {
                    Ok(transaction) => transactions.push(transaction),
                    Err(error_msg) => {
                        skipped_records.push((line_number + 1, error_msg, line_content));
                    }
                }
            }
            Err(io_error) => {
                eprintln!("Error reading line {}: {}", line_number + 1, io_error);
            }
        }
    }

    // Report processing results
    println!("📊 PROCESSING SUMMARY:");
    println!(
        "✅ Successfully processed: {} transactions",
        transactions.len()
    );
    println!(
        "❌ Skipped records: {} (due to errors or missing data)",
        skipped_records.len()
    );
    println!("{}", "=".repeat(60));

    // Display skipped records if any
    if !skipped_records.is_empty() {
        println!("\n🚫 EXCLUDED RECORDS (Missing Values or Errors):");
        for (line_num, reason, content) in &skipped_records {
            println!("Line {}: {} - {}", line_num, reason, content);
        }
        println!("{}", "=".repeat(60));
    }

    // Calculate and display continent totals using HashMap
    let continent_analysis = analyze_by_continent(&transactions);
    display_continent_analysis(&continent_analysis);

    // Filter and display European transactions
    println!("\n🇪🇺 EUROPEAN TRANSACTIONS:");
    filter_and_display_by_continent(&transactions, &Continent::Europe);

    // Display all valid transactions with calculated fields
    println!("\n📋 ALL PROCESSED TRANSACTIONS:");
    display_all_transactions(&transactions);

    Ok(())
}

/// Checks if a CSV line has missing values (empty fields between commas)
///
/// # Arguments
/// * `line` - The CSV line to check
///
/// # Returns
/// * `bool` - true if missing values are detected
fn has_missing_values(line: &str) -> bool {
    let fields: Vec<&str> = line.split(',').collect();

    // Check if we have the expected number of fields (7)
    if fields.len() != 7 {
        return true;
    }

    // Check for empty fields (except we allow empty country field to test error handling)
    for (index, field) in fields.iter().enumerate() {
        if field.trim().is_empty() && index != 5 {
            // Allow empty country for demonstration
            return true;
        }
    }

    false
}

/// Analyzes transactions by continent using HashMap to track totals
///
/// # Arguments
/// * `transactions` - Slice of Transaction structs
///
/// # Returns
/// * `HashMap<String, (f64, usize)>` - Continent name mapped to (total_amount, count)
fn analyze_by_continent(transactions: &[Transaction]) -> HashMap<String, (f64, usize)> {
    let mut continent_data: HashMap<String, (f64, usize)> = HashMap::new();

    for transaction in transactions {
        let continent_name = format!("{:?}", transaction.continent);
        let entry = continent_data.entry(continent_name).or_insert((0.0, 0));
        entry.0 += transaction.amount;
        entry.1 += 1;
    }

    continent_data
}

/// Displays comprehensive continent analysis
///
/// # Arguments
/// * `analysis` - HashMap containing continent analysis data
fn display_continent_analysis(analysis: &HashMap<String, (f64, usize)>) {
    println!("\n💰 INVESTMENT ANALYSIS BY CONTINENT:");
    println!("{:-<80}", "");
    println!(
        "{:<15} {:<15} {:<15} {:<15}",
        "Continent", "Total Amount", "Count", "Average"
    );
    println!("{:-<80}", "");

    let mut total_amount = 0.0;
    let mut total_count = 0;

    for (continent, (amount, count)) in analysis {
        let average = amount / *count as f64;
        println!(
            "{:<15} ${:<14.2} {:<15} ${:<14.2}",
            continent, amount, count, average
        );
        total_amount += amount;
        total_count += count;
    }

    println!("{:-<80}", "");
    println!(
        "{:<15} ${:<14.2} {:<15} ${:<14.2}",
        "TOTAL",
        total_amount,
        total_count,
        total_amount / total_count as f64
    );
    println!("{}", "=".repeat(60));
}

/// Filters transactions by continent and displays them
///
/// # Arguments
/// * `transactions` - Slice of Transaction structs
/// * `target_continent` - The continent to filter by
fn filter_and_display_by_continent(transactions: &[Transaction], target_continent: &Continent) {
    let filtered: Vec<&Transaction> = transactions
        .iter()
        .filter(|transaction| {
            std::mem::discriminant(&transaction.continent)
                == std::mem::discriminant(target_continent)
        })
        .collect();

    if filtered.is_empty() {
        println!("No transactions found for {:?}", target_continent);
        return;
    }

    println!("Found {} European transactions:", filtered.len());
    println!("{:-<100}", "");

    for transaction in filtered {
        println!(
            "ID: {:<4} | Client: {:<4} | Asset: {:<20} | Country: {:<15} | Amount: ${:<8.2} | Days: {}",
            transaction.transaction_id,
            transaction.client_id,
            transaction.asset_name,
            format!("{:?}", transaction.country),
            transaction.amount,
            transaction.days_under_management
        );
    }
    println!("{}", "=".repeat(60));
}

/// Displays all processed transactions in a formatted table
///
/// # Arguments
/// * `transactions` - Slice of Transaction structs
fn display_all_transactions(transactions: &[Transaction]) {
    if transactions.is_empty() {
        println!("No valid transactions to display.");
        return;
    }

    println!("{:-<120}", "");
    println!(
        "{:<4} {:<6} {:<25} {:<12} {:<12} {:<8} {:<6}",
        "ID", "Client", "Asset (Capitalized)", "Country", "Continent", "Amount", "Days"
    );
    println!("{:-<120}", "");

    for transaction in transactions {
        println!(
            "{:<4} {:<6} {:<25} {:<12} {:<12} ${:<7.2} {:<6}",
            transaction.transaction_id,
            transaction.client_id,
            transaction.asset_name,
            format!("{:?}", transaction.country),
            format!("{:?}", transaction.continent),
            transaction.amount,
            transaction.days_under_management
        );
    }
}
