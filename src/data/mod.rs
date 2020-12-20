mod json;

use std::path::Path;
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;

// Radius of the earth in mile
const RADIUS_MILE: f64 = 3958.756;
// Multiplier to calculate frequent flyer point
const CLASS_MUL: f64 = 1.25;

#[derive(PartialEq)]
pub enum AreaCode {
    Japan,
    Asia,
    EuropeMiddleEast,
    Oceania,
    Africa,
    NorthAmerica,
    SouthAmerica
}

pub struct City {
    pub area: AreaCode,
    lon: f64,
    lat: f64,
    pub code: String,
    pub country: String,
    pub hub: bool,
    distances: Vec<u32>,
    pub index: usize
}
impl City {
    fn from_json_data(city: &json::JsonCityData, is_hub: bool, country_map: &HashMap<String, &json::JsonCountryCode>, index: usize) -> City {
        let area = CityData::find_area(&city.countryCode, &city.timeZone, country_map);

        City {
            area,
            lon: city.lon.parse::<f64>().unwrap(),
            lat: city.lat.parse::<f64>().unwrap(),
            code: city.cityCode.clone(),
            country: city.countryCode.clone(),
            hub: is_hub,
            distances: Vec::new(),
            index
        }
    }
    fn calc_distance(from: &City, to: &City) -> f64 {
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
    pub fn calc_point(from: &City, to: &City) -> u32 {
        let mul = if from.area == AreaCode::Japan && from.area == to.area {
            // Japan domestic
            2.0
        } else if (from.area == AreaCode::Japan && (to.area == AreaCode::Asia || to.area == AreaCode::Oceania)) ||
            (to.area == AreaCode::Japan && (from.area == AreaCode::Asia || from.area == AreaCode::Oceania)) {
            if from.country == "RU" || to.country == "RU" {
                // Russia (East of Ural) is considered as Europe
                1.0
            } else {
                1.5
            }
        } else {
            1.0
        };

        let extra = if from.area == AreaCode::Japan || to.area == AreaCode::Japan {
            400.0
        } else {
            0.0
        };

        (City::calc_distance(from, to) * mul * CLASS_MUL + extra).floor() as u32
    }

    pub fn distance(&self, to: &City) -> u32 {
        self.distances[to.index]
    }
}

pub struct CityData {
    pub cities: Vec<City>,
}
impl CityData {
    fn find_area(country_code: &str, time_zone: &str, map: &HashMap<String, &json::JsonCountryCode>) -> AreaCode {
        let country = map[country_code];

        if country.code == "JP" {
            AreaCode::Japan
        } else if country.region == "Asia" {
            if country.sub_region == "Western Asia" {
                AreaCode::EuropeMiddleEast
            } else if country.code == "IR" { // Iran is in Middle-East
                AreaCode::EuropeMiddleEast
            } else {
                AreaCode::Asia
            }
        } else if country.region == "Oceania" {
            AreaCode::Oceania
        } else if country.region == "Europe" {
            if country.code == "RU" {
                match time_zone {
                    "6" | "7" | "8" | "9" | "10" | "11" => AreaCode::Asia,
                    _ => AreaCode::EuropeMiddleEast
                }
            } else {
                AreaCode::EuropeMiddleEast
            }
        } else if country.region == "Africa" {
            if country.code == "DZ" || country.code == "MA" { // Algeria and Morocco are in Europe
                AreaCode::EuropeMiddleEast
            } else if country.code == "EG" || country.code == "LY" || country.code == "SD" { // Egypt, Lybia and Sudan are in Middle East
                AreaCode::EuropeMiddleEast
            } else {
                AreaCode::Africa
            }
        } else if country.region == "Americas" {
            if country.sub_region == "Northern America" {
                AreaCode::NorthAmerica
            } else {
                if country.intermediate_region == "Caribbean" || country.intermediate_region == "Central America" {
                    AreaCode::NorthAmerica
                } else {
                    AreaCode::SouthAmerica
                }
            }
        } else {
            panic!("Cannot determine region for {}", country.code);
        }
    }

    pub fn load_all_data(data_dir: &str) -> CityData {
        // Load JSON files
        let country_code = json::JsonCountryCode::load_from_file(Path::new(data_dir).join("country-code.json").to_str().unwrap());
        let map_data: json::JsonMapData = json::load_from_json_file(Path::new(data_dir).join("owe-map-data.json").to_str().unwrap());
        let hubs: Vec<json::JsonHubInfo> = json::load_from_json_file(Path::new(data_dir).join("hub-info.json").to_str().unwrap());

        // Composite all the map data
        let mut airports_set = HashSet::new();
        let mut hubs_map = HashMap::new();
        let mut airport_to_city_map = HashMap::new();

        for hub in hubs {
            let mut reachable_airport_set = HashSet::new();
            for airport in hub.airports {
                let route: Vec<json::JsonFr24Route> = 
                    json::load_from_json_file(Path::new(data_dir).join("routes").join({
                        let mut s = airport.to_lowercase(); s.push_str(".json"); s
                    }).to_str().unwrap());
                
                airport_to_city_map.insert(airport.clone(), hub.city_code.clone());

                for r in route {
                    if let Some(ap) = r.iata {
                        airports_set.insert(ap.clone());
                        reachable_airport_set.insert(ap.clone());
                    }
                }
            }
            hubs_map.insert(hub.city_code, reachable_airport_set);
        }

        let mut country_map = HashMap::new();
        for i in 0..country_code.len() {
            country_map.insert(String::from(&country_code[i].code), &country_code[i]);
        }

        let mut cities = Vec::new();
        let mut index = 0usize;
        for city in map_data.mapData.cities.city {
            let is_hub = hubs_map.contains_key(&city.cityCode);
            let is_reachable_from_hub = airports_set.contains(&city.cityCode);

            if is_hub || is_reachable_from_hub {
                let city_data = City::from_json_data(&city, is_hub, &country_map, index);

                cities.push(city_data);
                index += 1;
            }
        }
        // Calculate distance
        for i in 0..cities.len() {
            for j in 0..cities.len() {
                // Is there any route?
                let reachable =
                    if cities[i].hub {
                        let reachable_set = &hubs_map[&cities[i].code];
                        if airport_to_city_map.contains_key(&cities[j].code) {
                            let city_code = &airport_to_city_map[&cities[j].code];
                            reachable_set.contains(city_code)
                        } else {
                            reachable_set.contains(&cities[j].code)
                        }
                    } else if cities[j].hub {
                        let reachable_set = &hubs_map[&cities[j].code];
                        reachable_set.contains(&cities[i].code)
                    } else {
                        false
                    };

                if reachable {
                    let c = City::calc_point(&cities[i], &cities[j]);
                    cities[i].distances.push(c);
                } else {
                    cities[i].distances.push(0);
                }
            }
        }

        CityData {
            cities,
        }
    }
}
