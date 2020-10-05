use std::env;

mod data;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut indices = Vec::new();
    let map_data = data::MapData::load("data/mapdata.json", "data/all.json");

    if args.len() < 3 {
        eprintln!("Usage {} (cities...)", args[0]);
        std::process::exit(1);
    }
    for i in 1..args.len() {
        indices.push(map_data.find_city_by_code(&args[i]));
    }

    let mut total = 0;
    for i in 0..indices.len() - 1 {
        let from = &map_data.cities[indices[i]];
        let to = &map_data.cities[indices[i + 1]];
        let distance = map_data.mileage(indices[i], indices[i + 1]);

        println!("{} - {} : {}", from.code, to.code, distance);

        total += distance;
    }

    println!("Total: {}", total);
}
