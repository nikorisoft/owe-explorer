use std::env;

mod data;

fn main() {
    let args: Vec<String> = env::args().collect();
    let map_data = data::MapData::load("data/mapdata.json", "data/all.json");

    for i in 1..args.len() {
        let index = map_data.find_city_by_code(&args[i]);
        let city = &map_data.cities[index];
        let country = &map_data.country_map[&city.country];

        println!("{} ({}, {}) -> {} - {} - {} -> {}", city.code, city.country,
            country.region, country.sub_region, country.intermediate_region,
            country.name, data::MapData::region(city, &map_data.country_map));
    }
}
