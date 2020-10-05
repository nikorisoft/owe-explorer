use super::data::MapData;
use std::collections::HashSet;

pub struct RouteResult {
    pub mileage: u32,
    pub index: Vec<usize>
}

fn find_longest_route_bruteforce_internal(map_data: &MapData, cities: &HashSet<usize>, num_cities: usize, prev_index: usize, current: usize) -> RouteResult {
    let mut max_mileage = 0u32;
    let mut max_index = 0;
    let mut max_result = RouteResult {
        mileage: 0,
        index: [].to_vec()
    };

    if current == num_cities {
        for index in cities {
            let m = map_data.mileage(prev_index, *index);
            if m > max_mileage {
                max_index = *index;
                max_mileage = m;
            }
        }
        RouteResult {
            mileage: max_mileage,
            index: [max_index].to_vec()
        }
    } else {
        for index in cities {
            let m = map_data.mileage(prev_index, *index);
            let result = find_longest_route_bruteforce_internal(map_data, cities, num_cities, *index, current + 1);

            if m + result.mileage > max_mileage {
                max_index = *index;
                max_mileage = m + result.mileage;
                max_result = result;
            }
        }

        let mut indices = max_result.index;
        indices.push(max_index);

        RouteResult {
            mileage: max_mileage,
            index: indices
        }
    }
}

pub fn find_longest_route(map_data: &MapData, cities: &HashSet<usize>, num_cities: usize) -> RouteResult {
    let mut max_mileage = 0u32;
    let mut max_index = 0;
    let mut max_result = RouteResult {
        mileage: 0,
        index: [].to_vec()
    };

    for index in cities {
        let result = find_longest_route_bruteforce_internal(map_data, cities, num_cities, *index, 2);
        if result.mileage > max_mileage {
            max_index = *index;
            max_mileage = result.mileage;
            max_result = result;
        }
    }

    let mut indices = max_result.index;
    indices.push(max_index);
    indices.reverse();

    RouteResult {
        mileage: max_mileage,
        index: indices
    }
}
