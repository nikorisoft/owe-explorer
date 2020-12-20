mod data;
mod find;

use std::collections::HashMap;
use find::{find_route, find_intercontinental_route, find_continental_route};

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

        let mileage = from.distance(to);
        total += mileage;

        println!("{} - {}:  {}", from.code, to.code, mileage);
    }

    println!("Total: {}", total);
}
