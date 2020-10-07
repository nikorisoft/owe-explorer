mod data;

use data::{MapData, Region};
use std::collections::HashSet;

mod calc;

enum Area {
    All,
    Japan,
    Asia,
    Europe,
    Africa,
    NorthAmerica,
    SouthAmerica,
    Oceania
}

fn main() {
    let map_data = MapData::load("data/mapdata.json", "data/all.json");
    let args: Vec<String> = std::env::args().collect();

    let method =
        if args.len() < 2 {
            eprintln!("Usage: {} (method name) [(region name)]", args[0]);
            std::process::exit(1);
        } else {
            match args[1].as_str() {
                "bruteforce" => calc::Methods::Bruteforce,
                "table1" => calc::Methods::LongestTable1,
                _ => panic!("Unknown method name: {}", args[1])
            }
        };
    let area = 
        if args.len() < 3 {
            Area::All
        } else {
            match args[2].as_str() {
                "asia" => Area::Asia,
                "japan" => Area::Japan,
                "europe" => Area::Europe,
                "africa" => Area::Africa,
                "oceania" => Area::Oceania,
                "northamerica" | "na" => Area::NorthAmerica,
                "southamerica" | "sa" => Area::SouthAmerica,
                _ => panic!("Unknown area name: {}", args[2])
            }
        };

    let mut asia = HashSet::new();
    let mut japan = HashSet::new();
    let mut europe = HashSet::new();
    let mut north_america = HashSet::new();
    let mut south_america = HashSet::new();
    let mut oceania = HashSet::new();
    let mut africa = HashSet::new();

    for index in 0..map_data.cities.len() {
        let city = &map_data.cities[index];
        match MapData::region(&city, &map_data.country_map) {
            Region::Japan => { asia.insert(index); japan.insert(index) },
            Region::Asia => asia.insert(index),
            Region::Europe => europe.insert(index),
            Region::NorthAmerica => north_america.insert(index),
            Region::SouthAmerica => south_america.insert(index),
            Region::Oceania => oceania.insert(index),
            Region::Africa => africa.insert(index)
        };
    }

    println!("Cities: Asia = {}, Japan = {}, Europe = {}, NA = {}, SA = {}, Oceania = {}, Africa = {}", 
        asia.len(), japan.len(), europe.len(), north_america.len(), south_america.len(), oceania.len(), africa.len());

    let mut num_cities = 5;
    let city_set = match area {
        Area::All => asia,
        Area::Asia => asia,
        Area::Africa => africa,
        Area::Europe => europe,
        Area::NorthAmerica => { num_cities = 7; north_america },
        Area::SouthAmerica => south_america,
        Area::Oceania => oceania,
        Area::Japan => japan
    };

    let result = calc::find_route(method, &map_data, &city_set, num_cities);

    println!("Result = {}", result.route.mileage);
    for index in result.route.index {
        print!("{} - ", map_data.cities[index].code);
    }
    println!("");
    println!("[Elapsed time = {:.3} s]", result.elapsed_time.as_millis() as f64 / 1000.0);
}
