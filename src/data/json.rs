use serde;
use serde::{Deserialize};
use serde_json::{Value};
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct JsonFr24Route {
    pub iata: Option<String>,
    pub icao: Option<String>
}

pub struct JsonCountryCode {
    pub name: String,
    pub code: String,
    pub region: String,
    pub intermediate_region: String,
    pub sub_region: String
}
impl JsonCountryCode {
    pub fn load_from_file(filename: &str) -> Vec<JsonCountryCode> {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let j: Value = serde_json::from_str(&contents).unwrap();
        let mut v = Vec::new();

        if let Value::Array(countries) = j {
            for c in countries {
                v.push(JsonCountryCode {
                    name: String::from(c["name"].as_str().unwrap()),
                    code: String::from(c["alpha-2"].as_str().unwrap()),
                    region: String::from(c["region"].as_str().unwrap()),
                    intermediate_region: String::from(c["intermediate-region"].as_str().unwrap()),
                    sub_region: String::from(c["sub-region"].as_str().unwrap())
                });
            }
        }

        v
    }
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct JsonCityData {
    pub cityCode: String,
    pub countryCode: String,
    pub lat: String,
    pub lon: String,
    pub timeZone: String,
    pub name: String
}
#[derive(Deserialize)]
pub struct JsonCities {
    pub city: Vec<JsonCityData>
}

#[derive(Deserialize)]
pub struct JsonMapDataContents {
    pub cities: JsonCities
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct JsonMapData {
    pub mapData: JsonMapDataContents
}

#[derive(Deserialize)]
pub struct JsonHubInfo {
    pub city_code: String,
    pub airports: Vec<String>
}

pub fn load_from_json_file<T: for<'a> Deserialize<'a>>(filename: &str) -> T {
    let mut file = File::open(filename).unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();

    serde_json::from_str(&contents).unwrap()
}
