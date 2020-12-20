mod data;
mod find;

use std::collections::{HashMap, HashSet};
use find::{find_route, find_intercontinental_route, find_continental_route};
use data::{City, AreaCode};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let city_data = data::CityData::load_all_data("data");

    if args.len() < 2 {
        eprintln!("Usage: {} (route|intercontinental|mileage)", args[0]);
        std::process::exit(1);
    }
    match args[1].as_str() {
        "route" => find_route(city_data, "", &[&args[2], &args[3], &args[4], &args[5], &args[6], &args[7]]),
        "intercontinental" => find_intercontinental_route(city_data, args[2].as_str(), args[3].as_str()),
        "continental" => find_continental_route(city_data, args[2].as_str(), args[3].as_str(), args[4].parse::<usize>().unwrap()),
        "mileage" => calc_mileage(city_data, &args[2..]),
        "cities" => count_cities(city_data),
        _ => panic!("Unknown command: {}", args[1])
    }

    std::process::exit(0);
}

fn calc_mileage(city_data: data::CityData, cities: &[String]) {
    let mut map = HashMap::new();
    for i in 0..city_data.cities.len() {
        map.insert(&city_data.cities[i].code, &city_data.cities[i]);
    }

    let mut total = 0;
    for i in 0..cities.len() - 1 {
        let from = map[&cities[i]];
        let to = map[&cities[i + 1]];

        let mileage = City::calc_point(from, to);
        total += mileage;

        println!("{} - {}:  {}", from.code, to.code, mileage);
    }

    println!("Total: {}", total);
}

fn count_cities(city_data: data::CityData) {
    let mut set_asia = HashSet::new();
    let mut set_europe = HashSet::new();
    let mut set_oceania = HashSet::new();
    let mut set_africa = HashSet::new();
    let mut set_north_america = HashSet::new();
    let mut set_south_america = HashSet::new();

    for i in 0..city_data.cities.len() {
        let city = &city_data.cities[i];
        match city.area {
            AreaCode::Asia | AreaCode::Japan => set_asia.insert(i),
            AreaCode::Oceania => set_oceania.insert(i),
            AreaCode::EuropeMiddleEast => set_europe.insert(i),
            AreaCode::Africa => set_africa.insert(i),
            AreaCode::NorthAmerica => set_north_america.insert(i),
            AreaCode::SouthAmerica => set_south_america.insert(i)
        };
    }

    println!("Asia => {}, Europe => {}, Oceania => {}, Africa => {}, NA => {}, SA => {}",
        set_asia.len(), set_europe.len(), set_oceania.len(), set_africa.len(), set_north_america.len(), set_south_america.len()
    );
}
