use json;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::f64::consts::PI;
use std::collections::HashMap;

const RADIUS_MILE: f64 = 3958.756;
const CLASS_MUL: f64 = 1.25;

#[derive(PartialEq)]
pub enum Region {
    Japan,
    Asia,
    Oceania,
    Europe,
    Africa,
    NorthAmerica,
    SouthAmerica
}
impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Region::Japan => write!(f, "Japan"),
            Region::Asia => write!(f, "Asia"),
            Region::Oceania => write!(f, "Oceania"),
            Region::Europe => write!(f, "Europe"),
            Region::Africa => write!(f, "Africa"),
            Region::NorthAmerica => write!(f, "North America"),
            Region::SouthAmerica => write!(f, "South America")
        }
    }
}


#[derive(Clone)]
pub struct Country {
    pub code: String,
    pub name: String,
    pub region: String,
    pub intermediate_region: String,
    pub sub_region: String
}

impl Country {
    pub fn convert_from_json(country: &json::JsonValue) -> Country {
        Country {
            code: String::from(country["alpha-2"].as_str().unwrap()),
            name: String::from(country["name"].as_str().unwrap()),
            region: String::from(country["region"].as_str().unwrap()),
            intermediate_region: String::from(country["intermediate-region"].as_str().unwrap()),
            sub_region: String::from(country["sub-region"].as_str().unwrap())
        }
    }
}

#[derive(Clone)]
pub struct City {
    pub code: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub timezone: String
}

impl City {
    pub fn convert_from_json(city: &json::JsonValue) -> City {
        if let json::JsonValue::Object(o) = city {
            City {
                code: String::from(o["cityCode"].as_str().unwrap()),
                country: String::from(o["countryCode"].as_str().unwrap()),
                lat: o["lat"].as_str().unwrap().parse::<f64>().unwrap(),
                lon: o["lon"].as_str().unwrap().parse::<f64>().unwrap(),
                timezone: String::from(o["timeZone"].as_str().unwrap())
            }
        } else {
            panic!("Invalid 'city' json object");
        }
    }
    pub fn calc_distance(from: &City, to: &City) -> f64 {
        let lat1 = from.lat / 180.0 * PI;
        let lat2 = to.lat / 180.0 * PI;
        let lon1 = from.lon / 180.0 * PI;
        let lon2 = to.lon / 180.0 * PI;

        let d_lon = (lon1 - lon2).abs();

        let x = lat1.sin() * lat2.sin() + lat1.cos() * lat2.cos() * d_lon.cos();
        let y = ((lat2.cos() * d_lon.sin()).powi(2) + (lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * d_lon.cos()).powi(2))
        .sqrt();
        let d_sigma = y.atan2(x);

        d_sigma * RADIUS_MILE
    }
}

pub struct MapData {
    pub cities: Vec<City>,
    mileage_table: Vec<Vec<u32>>,
    pub country_map: HashMap<String, Country>
}

impl MapData {
    pub fn load(file_name: &str, country_file_name: &str) -> MapData {
        let mut file = File::open(file_name).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let map_json = json::parse(&contents).unwrap();

        let mut file = File::open(country_file_name).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let country_json = json::parse(&contents).unwrap();
        let mut countries = HashMap::new();
        if let json::JsonValue::Array(countries_array) = &country_json {
            for country_json in countries_array {
                let c = Country::convert_from_json(&country_json);
                countries.insert(String::from(&c.code), c);
            }
        }

        let mut cities = Vec::new();

        if let json::JsonValue::Array(cities_array) = &map_json["mapData"]["cities"]["city"] {
            for cities_json in cities_array {
                cities.push(City::convert_from_json(&cities_json));
            }
        } else {
            panic!("mapData.cities is not array");
        }

        let mut mileage_table = Vec::new();
        for i in 0..cities.len() {
            let mut row = Vec::new();
            for j in 0..cities.len() {
                let r1 = MapData::region(&cities[i], &countries);
                let r2 = MapData::region(&cities[j], &countries);

                let mul = if r1 == Region::Japan && r1 == r2 {
                    // Japan domestic
                    2.0
                } else if (r1 == Region::Japan && (r2 == Region::Asia || r2 == Region::Oceania)) ||
                    (r2 == Region::Japan && (r1 == Region::Asia || r1 == Region::Oceania)) {
                    if cities[i].country == "RU" || cities[j].country == "RU" {
                        // Russia (East of Ural) is considered as Europe
                        1.0
                    } else {
                        1.5
                    }
                } else {
                    1.0
                };

                let extra = if r1 == Region::Japan || r2 == Region::Japan {
                    400.0
                } else {
                    0.0
                };

                row.push((City::calc_distance(&cities[i], &cities[j]) * mul * CLASS_MUL + extra).floor() as u32);
            }
            mileage_table.push(row);
        }

        MapData {
            cities,
            mileage_table,
            country_map: countries
        }
    }

    pub fn mileage(&self, from: usize, to: usize) -> u32 {
        return self.mileage_table[from][to];
    }

    pub fn find_city_by_code(&self, code: &str) -> usize {
        self.cities.iter().position(|c| c.code == code).unwrap()
    }

    pub fn region(city: &City, country_map: &HashMap<String, Country>) -> Region {
        let country = &country_map[&city.country];

        if city.country == "JP" {
            Region::Japan
        } else if country.region == "Asia" {
            if country.sub_region == "Western Asia" {
                Region::Europe
            } else if city.country == "IR" { // Iran is in Middle-East
                Region::Europe
            } else {
                Region::Asia
            }
        } else if country.region == "Oceania" {
            Region::Oceania
        } else if country.region == "Europe" {
            if country.code == "RU" {
                if city.timezone == "6" || city.timezone == "7" || city.timezone == "8" || city.timezone == "9" || city.timezone == "10" || city.timezone == "11" { // East of Ural
                    Region::Asia
                } else {
                    Region::Europe
                }
            } else {
                Region::Europe
            }
        } else if country.region == "Africa" {
            if country.code == "DZ" || country.code == "MA" { // Algeria and Morocco are in Europe
                Region::Europe
            } else if country.code == "EG" || country.code == "LY" || country.code == "SD" { // Egypt, Lybia and Sudan are in Middle East
                Region::Europe
            } else {
                Region::Africa
            }
        } else if country.region == "Americas" {
            if country.sub_region == "Northern America" {
                Region::NorthAmerica
            } else {
                if country.intermediate_region == "Caribbean" || country.intermediate_region == "Central America" {
                    Region::NorthAmerica
                } else {
                    Region::SouthAmerica
                }
            }
        } else {
            println!("Unknown region for {}: {} {} {}", country.code, country.region, country.sub_region, country.intermediate_region);
            Region::SouthAmerica
        }
    }
}
