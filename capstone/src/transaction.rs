use crate::location::{Continent, Country};
use chrono::NaiveDate;

struct Transaction {
    transaction_id: u32,
    client_id: u32,
    asset_name: String,
    country: Country,
    continent: Continent,
    amount: f64,
    days_under_management: i64,
}

impl Transaction {
    pub fn from_csv_line(line: &str) -> Result<Transaction, String> {
        let fields: Vec<&str> = line.split(",").collect();
        if fields.len() != 7 {
            return Err("Fields length mismatch".to_owned());
        }
        let transaction_id = fields[0].parse::<u32>().unwrap();
        let client_id = fields[1].parse::<u32>().unwrap();
        let asset_name = fields[2].to_uppercase();
        let transaction_start_date = NaiveDate::parse_from_str(fields[3], "%Y-%m-%d").unwrap();
        let transaction_end_date = NaiveDate::parse_from_str(fields[4], "%Y-%m-%d").unwrap();
        let country: Country = fields[5].parse()?;
        let continent = country.country_to_continent();
        let amount = fields[6].parse::<f64>().unwrap();
        let days_under_management = (transaction_end_date - transaction_start_date).num_days();
        let transaction = Transaction {
            transaction_id,
            client_id,
            asset_name,
            country,
            continent,
            amount,
            days_under_management,
        };
        return Ok(transaction);
    }
}
