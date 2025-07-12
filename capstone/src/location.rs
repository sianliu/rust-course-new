/// Location module for handling countries and continents
///
/// This module defines Country and Continent enums with proper parsing
/// and geographical mapping functionality.
use std::str::FromStr;

/// Represents supported countries for transaction processing
///
/// Each variant represents a specific country that can be parsed
/// from CSV data and mapped to its corresponding continent.
#[derive(Debug, Clone, PartialEq)]
pub enum Country {
    UnitedStates,
    Canada,
    UnitedKingdom,
    Germany,
    France,
    Japan,
    Australia,
    China,
    Brazil,
    SouthKorea,
    Ireland,
    Spain,
    India,
    Switzerland,
}

/// Represents geographical continents for transaction classification
///
/// Used for grouping and analyzing transactions by geographical region.
#[derive(Debug, Clone, PartialEq)]
pub enum Continent {
    NorthAmerica,
    Europe,
    Asia,
    Oceania,
    SouthAmerica,
}

impl Country {
    /// Maps a country to its corresponding continent
    ///
    /// # Returns
    /// * `Continent` - The continent where this country is located
    ///
    /// # Examples
    /// ```
    /// let country = Country::Germany;
    /// assert_eq!(country.country_to_continent(), Continent::Europe);
    /// ```
    pub fn country_to_continent(&self) -> Continent {
        match self {
            // North American countries
            Country::UnitedStates | Country::Canada => Continent::NorthAmerica,

            // European countries
            Country::UnitedKingdom
            | Country::Germany
            | Country::France
            | Country::Spain
            | Country::Switzerland
            | Country::Ireland => Continent::Europe,

            // Asian countries
            Country::Japan | Country::China | Country::SouthKorea | Country::India => {
                Continent::Asia
            }

            // Oceanian countries
            Country::Australia => Continent::Oceania,

            // South American countries
            Country::Brazil => Continent::SouthAmerica,
        }
    }

    /// Gets the full country name as a string
    ///
    /// # Returns
    /// * `&str` - The full country name
    pub fn full_name(&self) -> &str {
        match self {
            Country::UnitedStates => "United States",
            Country::Canada => "Canada",
            Country::UnitedKingdom => "United Kingdom",
            Country::Germany => "Germany",
            Country::France => "France",
            Country::Japan => "Japan",
            Country::Australia => "Australia",
            Country::China => "China",
            Country::Brazil => "Brazil",
            Country::SouthKorea => "South Korea",
            Country::Ireland => "Ireland",
            Country::Spain => "Spain",
            Country::India => "India",
            Country::Switzerland => "Switzerland",
        }
    }
}

/// Custom error type for country parsing failures
#[derive(Debug, Clone)]
pub struct CountryParseError {
    pub input: String,
}

impl std::fmt::Display for CountryParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to parse '{}' as a valid country", self.input)
    }
}

impl std::error::Error for CountryParseError {}

/// Implements string parsing for Country enum with comprehensive alias support
///
/// This implementation handles various common representations of country names
/// including abbreviations, full names, and alternative spellings.
impl FromStr for Country {
    type Err = CountryParseError;

    /// Parses a string into a Country enum variant
    ///
    /// # Arguments
    /// * `s` - The string to parse (case-insensitive)
    ///
    /// # Returns
    /// * `Result<Country, CountryParseError>` - The parsed country or error
    ///
    /// # Examples
    /// ```
    /// let country: Country = "USA".parse()?;
    /// let country: Country = "United Kingdom".parse()?;
    /// ```
    fn from_str(s: &str) -> Result<Country, Self::Err> {
        let normalized = s.trim().to_lowercase();

        match normalized.as_str() {
            // United States variations
            "usa" | "united states" | "unitedstates" | "us" | "america" => {
                Ok(Country::UnitedStates)
            }

            // Canada variations
            "canada" | "can" => Ok(Country::Canada),

            // United Kingdom variations
            "uk" | "united kingdom" | "unitedkingdom" | "britain" | "great britain" => {
                Ok(Country::UnitedKingdom)
            }

            // Germany variations
            "germany" | "deutschland" | "de" => Ok(Country::Germany),

            // France variations
            "france" | "fr" => Ok(Country::France),

            // Japan variations
            "japan" | "jp" => Ok(Country::Japan),

            // Australia variations
            "australia" | "au" | "aus" => Ok(Country::Australia),

            // China variations
            "china" | "cn" | "prc" => Ok(Country::China),

            // Brazil variations
            "brazil" | "brasil" | "br" => Ok(Country::Brazil),

            // South Korea variations
            "south korea" | "southkorea" | "korea" | "kr" | "republic of korea" => {
                Ok(Country::SouthKorea)
            }

            // Ireland variations
            "ireland" | "ie" | "eire" => Ok(Country::Ireland),

            // Spain variations
            "spain" | "es" | "españa" => Ok(Country::Spain),

            // India variations
            "india" | "in" => Ok(Country::India),

            // Switzerland variations
            "switzerland" | "ch" | "schweiz" => Ok(Country::Switzerland),

            // Handle empty string explicitly
            "" => Err(CountryParseError {
                input: "Empty string".to_string(),
            }),

            // Unrecognized country
            _ => Err(CountryParseError {
                input: s.to_string(),
            }),
        }
    }
}
