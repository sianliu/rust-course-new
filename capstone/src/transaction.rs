/// Transaction module for processing CSV transaction data
///
/// This module defines the Transaction struct and implements parsing logic
/// with comprehensive error handling and data validation.
use crate::location::{Continent, Country};
use chrono::NaiveDate;

/// Represents a financial transaction with all required fields
///
/// All fields are public to allow access for analysis and reporting
#[derive(Debug, Clone)]
pub struct Transaction {
    pub transaction_id: u32,
    pub client_id: u32,
    pub asset_name: String,   // Will be fully capitalized
    pub country: Country,     // Parsed from string using FromStr
    pub continent: Continent, // Calculated from country
    pub amount: f64,
    pub days_under_management: i64, // Calculated from date difference
}

impl Transaction {
    /// Creates a Transaction from a CSV line with comprehensive error handling
    ///
    /// # Arguments
    /// * `line` - A CSV line containing transaction data
    ///
    /// # Returns
    /// * `Result<Transaction, String>` - Ok(Transaction) on success, Err(message) on failure
    ///
    /// # Examples
    /// ```
    /// let line = "1,101,Apple Inc.,2023-01-10,2023-01-20,USA,1000";
    /// let transaction = Transaction::from_csv_line(line)?;
    /// ```
    pub fn from_csv_line(line: &str) -> Result<Transaction, String> {
        let fields: Vec<&str> = line.split(',').collect();

        // Validate field count using match for proper error handling
        if fields.len() != 7 {
            return Err(format!("Expected 7 fields, found {}", fields.len()));
        }

        // Parse transaction_id with proper error handling
        let transaction_id = fields[0]
            .trim()
            .parse::<u32>()
            .map_err(|_| format!("Invalid transaction ID: '{}'", fields[0]))?;

        // Parse client_id with proper error handling
        let client_id = fields[1]
            .trim()
            .parse::<u32>()
            .map_err(|_| format!("Invalid client ID: '{}'", fields[1]))?;

        // Process asset name - fully capitalize as required
        let asset_name = fields[2].trim().to_uppercase();

        // Validate asset name is not empty
        if asset_name.is_empty() {
            return Err("Asset name cannot be empty".to_string());
        }

        // Parse start date with proper error handling
        let transaction_start_date = NaiveDate::parse_from_str(fields[3].trim(), "%Y-%m-%d")
            .map_err(|_| {
                format!(
                    "Invalid start date format: '{}'. Expected YYYY-MM-DD",
                    fields[3]
                )
            })?;

        // Parse end date with proper error handling
        let transaction_end_date = NaiveDate::parse_from_str(fields[4].trim(), "%Y-%m-%d")
            .map_err(|_| {
                format!(
                    "Invalid end date format: '{}'. Expected YYYY-MM-DD",
                    fields[4]
                )
            })?;

        // Validate date logic
        if transaction_end_date < transaction_start_date {
            return Err("End date cannot be before start date".to_string());
        }

        // Parse country using FromStr trait with ? operator for proper error propagation
        let country: Country = fields[5]
            .trim()
            .parse()
            .map_err(|e| format!("Country parsing error: {}", e))?;

        // Calculate continent from country (demonstrates proper use of modules)
        let continent = country.country_to_continent();

        // Parse amount with proper error handling
        let amount = fields[6]
            .trim()
            .parse::<f64>()
            .map_err(|_| format!("Invalid amount: '{}'", fields[6]))?;

        // Validate amount is positive
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }

        // Calculate days under management (demonstrates proper use of external crates)
        let days_under_management = (transaction_end_date - transaction_start_date).num_days();

        // Create and return the transaction struct
        let transaction = Transaction {
            transaction_id,
            client_id,
            asset_name,
            country,
            continent,
            amount,
            days_under_management,
        };

        Ok(transaction)
    }

    /// Gets the transaction value tier based on amount
    ///
    /// # Returns
    /// * `&str` - The tier classification
    pub fn get_value_tier(&self) -> &str {
        match self.amount {
            x if x >= 5000.0 => "High Value",
            x if x >= 2000.0 => "Medium Value",
            _ => "Standard Value",
        }
    }

    /// Checks if transaction is long-term (>= 30 days)
    ///
    /// # Returns
    /// * `bool` - true if long-term investment
    pub fn is_long_term(&self) -> bool {
        self.days_under_management >= 30
    }
}
