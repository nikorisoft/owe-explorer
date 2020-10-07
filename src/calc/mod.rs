use std::time;
use std::collections::HashSet;
use super::data::MapData;

pub mod brute_force;
pub mod longest_table;

pub struct RouteResult {
    pub mileage: u32,
    pub index: Vec<usize>
}

pub struct FindResult {
    pub route: RouteResult,
    pub elapsed_time: time::Duration
}

pub enum Methods {
    BruteForce,
    LongestTable1
}

pub fn find_route(method: Methods, map_data: &MapData, cities: &HashSet<usize>, num_cities: usize) -> FindResult {
    let time_start = time::SystemTime::now();

    let result =
        match method {
            Methods::BruteForce => brute_force::find_longest_route(map_data, cities, num_cities),
            Methods::LongestTable1 => longest_table::find_longest_route(map_data, cities, num_cities)
        };

    let time_end = time::SystemTime::now();

    FindResult {
        route: result,
        elapsed_time: time_end.duration_since(time_start).unwrap()
    }
}
