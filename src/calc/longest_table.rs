use super::super::data::MapData;
use super::RouteResult;
use std::collections::HashSet;

pub struct LongestPathEntry {
    pub mileage: u32,
    pub next: usize
}

pub fn find_longest_route(map_data: &MapData, cities: &HashSet<usize>, num_cities: usize) -> RouteResult {
    let mut table = Vec::with_capacity(cities.len());

    let cities_array: Vec<&usize> = cities.iter().collect();

    for i in 0..cities_array.len() {
        let mut max_mileage = 0;
        let mut max_index = i;
        for j in 0..cities_array.len() {
            let mileage = map_data.mileage(*cities_array[i], *cities_array[j]);

            if mileage > max_mileage {
                max_mileage = mileage;
                max_index = j;
            }
        }

        table.push(LongestPathEntry {
            mileage: max_mileage,
            next: max_index
        });
    }

    let mut max_mileage = 0;
    let mut max_index = 0;
    for i in 0..cities_array.len() {
        let mut total = 0;
        let mut cur_index = i;

        for _ in 0..num_cities - 1 {
            total += table[cur_index].mileage;
            cur_index = table[cur_index].next;
        }

        if total > max_mileage {
            max_mileage = total;
            max_index = i;
        }
    }

    let mut longest_path = Vec::new();
    longest_path.push(*cities_array[max_index]);
    let mut cur_index = max_index;
    for _ in 0..num_cities - 1 {
        cur_index = table[cur_index].next;
        longest_path.push(*cities_array[cur_index]);
    }

    RouteResult {
        mileage: max_mileage,
        index: longest_path
    }
}
