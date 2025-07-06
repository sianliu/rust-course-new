use::std::str::FromStr;

enum Country {
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

enum Continent {
    NorthAmerica,
    Europe,
    Asia,
    Oceania,
    SouthAmerica,
}

impl FromStr for Country {
    type Err = &'static str;

    // Self here refers to the type Country
    // Self::Err refers to the type of error that can be returned
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "united states" => Ok(Country::UnitedStates),
            "canada" => Ok(Country::Canada),
            "united kingdom" => Ok(Country::UnitedKingdom),
            "germany" => Ok(Country::Germany),
            "france" => Ok(Country::France),
            "japan" => Ok(Country::Japan),
            "australia" => Ok(Country::Australia),
            "china" => Ok(Country::China),
            "brazil" => Ok(Country::Brazil),
            "south korea" => Ok(Country::SouthKorea),
            "ireland" => Ok(Country::Ireland),
            "spain" => Ok(Country::Spain),
            "india" => Ok(Country::India),
            "switzerland" => Ok(Country::Switzerland),
            _ => Err("Unknown country"),
        }
    }
}
impl Country {
    fn country_to_continent(&self) -> Continent {
        match self {
            Country::United States | Country::Canada => Continent::NorthAmerica,
            Country::United Kingdom | Country::Germany | Country::France | 
            Country::Ireland | Country::Spain | Country::Switzerland => Continent::Europe,
            Country::Japan | Country::China | Country::South Korea => Continent::Asia,
            Country::Australia => Continent::Oceania,
            Country::Brazil => Continent::SouthAmerica,
            // _ => panic!("Unknown country"),
            println!("Unknown country"),
        }
    }
}