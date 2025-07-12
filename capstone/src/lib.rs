//! # Capstone Project - Transaction Data Analysis
//!
//! This crate provides comprehensive transaction data analysis capabilities
//! for processing CSV files containing financial transaction records.
//!
//! ## Features
//!
//! - **Data Validation**: Excludes records with missing values
//! - **Data Normalization**: Capitalizes all asset names
//! - **Geographic Analysis**: Maps countries to continents
//! - **Time Analysis**: Calculates days under management
//! - **Error Handling**: Comprehensive error handling with detailed messages
//! - **Statistical Analysis**: Provides summary statistics by continent
//!
//! ## Usage
//!
//! ```rust
//! use capstone::transaction::Transaction;
//!
//! let csv_line = "1,101,APPLE INC.,2023-01-10,2023-01-20,USA,1000";
//! let transaction = Transaction::from_csv_line(csv_line)?;
//!
//! println!("Country: {:?}", transaction.country);
//! println!("Continent: {:?}", transaction.continent);
//! println!("Days under management: {}", transaction.days_under_management);
//! ```
//!
//! ## Modules
//!
//! - [`transaction`] - Core transaction processing functionality
//! - [`location`] - Country and continent definitions and mappings

pub mod location;
pub mod transaction;
