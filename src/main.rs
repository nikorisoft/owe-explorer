mod data;

use data::{MapData, Region};
use std::collections::HashSet;
use std::time;

// Switch module
// 1: Bruteforce
mod bruteforce;
use bruteforce::find_longest_route;

fn main() {
    let map_data = MapData::load("data/mapdata.json", "data/all.json");

    let mut asia = HashSet::new();
    let mut europe = HashSet::new();
    let mut north_america = HashSet::new();
    let mut south_america = HashSet::new();
    let mut oceania = HashSet::new();
    let mut africa = HashSet::new();

    for index in 0..map_data.cities.len() {
        let city = &map_data.cities[index];
        match MapData::region(&city, &map_data.country_map) {
            Region::Japan | Region::Asia => asia.insert(index),
            Region::Europe => europe.insert(index),
            Region::NorthAmerica => north_america.insert(index),
            Region::SouthAmerica => south_america.insert(index),
            Region::Oceania => oceania.insert(index),
            Region::Africa => africa.insert(index)
        };
    }

    println!("Cities: Asia = {}, Europe = {}, NA = {}, SA = {}, Oceania = {}, Africa = {}", 
        asia.len(), europe.len(), north_america.len(), south_america.len(), oceania.len(), africa.len());

    let time_start = time::SystemTime::now();
    let result = find_longest_route(&map_data, &oceania, 5);
    let time_end = time::SystemTime::now();

    println!("Result = {}", result.mileage);
    for index in result.index {
        print!("{} - ", map_data.cities[index].code);
    }
    println!("");
    println!("[Elapsed time = {:.3} s]", time_end.duration_since(time_start).unwrap().as_millis() as f64 / 1000.0);
}
