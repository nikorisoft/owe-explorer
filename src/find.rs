use std::collections::{HashMap, HashSet};
use super::data::{AreaCode, CityData, City};

fn city(city_data: &CityData, index: usize) -> &City {
    &city_data.cities[index]
}

fn prepare_area_data(city_data: &CityData) -> (HashMap<String, &City>, HashSet<usize>, HashSet<usize>, HashSet<usize>) {
    let mut city_map = HashMap::new();
    let mut cities_asia = HashSet::new();
    let mut cities_europe = HashSet::new();
    let mut cities_na = HashSet::new();

    for i in 0..city_data.cities.len() {
        city_map.insert(city_data.cities[i].code.clone(), &city_data.cities[i]);

        match city_data.cities[i].area {
            AreaCode::Asia | AreaCode::Japan => cities_asia.insert(i),
            AreaCode::EuropeMiddleEast => cities_europe.insert(i),
            AreaCode::NorthAmerica => cities_na.insert(i),
            _ => false
        };
    }

    (city_map, cities_asia, cities_europe, cities_na)
}

fn find_route_in_continent(city_data: &CityData, cities: &HashSet<usize>, num: usize, from: &City, to: &City) -> Option<(u32, Vec<usize>)> {
    match find_route_in_continent_hub_core(city_data, cities, num, from, to, &[from]) {
        Some((max_distance, mut rev_route)) => {
            rev_route.reverse();

            Some((max_distance, rev_route))
        },
        None => None
    }
}

fn check_rule(history: &[&City], current: usize, next: usize) -> bool {
    let mut hawaii = false;

    for i in 0..history.len() {
        if i + 1 < history.len() {
            if history[i].index == current && history[i + 1].index == next {
                return false;
            }
        }
        if history[i].code == "HNL" || history[i].code == "OGG" {
            hawaii = true;
        } else if hawaii {
            return false;
        }
    }

    true
}

fn find_route_in_continent_hub_core(city_data: &CityData, cities: &HashSet<usize>, num: usize, from: &City, to: &City, hist: &[&City]) -> Option<(u32, Vec<usize>)> {
    let mut max_distance = 0;
    let mut max_route = Vec::new();

    if num == 0 {
        return if check_rule(hist, from.index, to.index) {
            Some((from.distance(to), Vec::from([to.index, from.index])))
        } else {
            None
        };
    }

    for i in cities {
        let c = city(city_data, *i);

        if from.distance(c) > 0 && cities.contains(i) {
            if check_rule(hist, from.index, *i) {

                let result = find_route_in_continent_hub_core(city_data, cities, num - 1, c, to,
                    &[hist, &[c]].concat());

                if let Some((distance, route)) = result {
                    let total = from.distance(c) + distance;
                    if total > max_distance {
                        max_distance = total;
                        max_route = route;
                    }
                }
            }
        }
    }

    if max_route.len() > 0 {
        max_route.push(from.index);

        Some((max_distance, max_route))
    } else {
        None
    }
}

fn find_boundary(city_data: &CityData, set_from: &HashSet<usize>, set_to: &HashSet<usize>) -> HashSet<usize> {
    let mut boundary_set = HashSet::new();

    for i in set_from {
        for j in set_to {
            let from = city(city_data, *i);
            let to = city(city_data, *j);

            if from.distance(to) > 0 {
                boundary_set.insert(*i);
                break;
            }
        }
    }

    boundary_set
}

fn find_longest_intercontinental_routes(city_data: &CityData, set_from: &HashSet<usize>, set_to: &HashSet<usize>) -> Vec<(u32, usize, usize)> {
    let mut result = Vec::new();

    for i in set_from {
        for j in set_to {
            let from = city(city_data, *i);
            let to = city(city_data, *j);

            let d = from.distance(to);
            if d > 0 {
                result.push((d, *i, *j));
            }
        }
    }

    result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    result
}


pub fn find_intercontinental_route(city_data: CityData, area1: &str, area2: &str){
    let (_, cities_asia, cities_europe, cities_na) = prepare_area_data(&city_data);

    let area_set1 = match area1 {
        "asia" => &cities_asia,
        "europe" => &cities_europe,
        "na" => &cities_na,
        _ => panic!("Unrecognized area: {}", area1)
    };
    let area_set2 = match area2 {
        "asia" => &cities_asia,
        "europe" => &cities_europe,
        "na" => &cities_na,
        _ => panic!("Unrecognized area: {}", area2)
    };

    let boundaries = find_boundary(&city_data, area_set1, area_set2);
    let mut routes = find_longest_intercontinental_routes(&city_data, &boundaries, area_set2);
    routes.reverse();
    
    for (distance, city1, city2) in routes.iter().take(10) {
        println!("{} -> {}: {}", city(&city_data, *city1).code, city(&city_data, *city2).code, distance);
    }
}

pub fn find_continental_route(city_data: CityData, from: &str, to: &str, num: usize) {
    let (city_map, cities_asia, cities_europe, cities_na) = prepare_area_data(&city_data);
    let from = city_map[from];
    let to = city_map[to];

    if from.area != to.area && (from.area != AreaCode::Asia || to.area != AreaCode::Japan) &&
        (from.area != AreaCode::Japan || to.area != AreaCode::Asia) {
        panic!("Origin and destination are not in the same continent");
    }

    let area = match from.area {
        AreaCode::Asia | AreaCode::Japan => &cities_asia,
        AreaCode::EuropeMiddleEast => &cities_europe,
        AreaCode::NorthAmerica => &cities_na,
        _ => panic!("Unsupported area")
    };

    let result = find_route_in_continent(&city_data, area, num, from, to);

    if let Some((distance, route)) = result {
        println!("Distance = {}", distance);

        for p in route {
            print!("{} - ", city(&city_data, p).code);
        }
        println!("");
    } else {
        println!("Cannot find any route for the combination");
    }
}

fn find_route_continents(city_data: &CityData, area_sets: &[&HashSet<usize>], boundaries: &[&City], limits: &[usize]) -> (u32, Vec<usize>) {
    let mut final_routes = Vec::new();

    for i in 0..area_sets.len() {
        let result = find_route_in_continent(&city_data, area_sets[i], limits[i] - 1, boundaries[i * 2], boundaries[i * 2 + 1]);

        if let Some((distance, mut routes)) = result {
            final_routes.append(&mut routes);
        } else {
            panic!("No route found inside the continent");
        }
    }

    final_routes.push(boundaries[0].index);

    let mut distance = 0;
    for i in 0..final_routes.len() - 1 {
        distance += city(city_data, final_routes[i]).distance(city(city_data, final_routes[i + 1]));
    }

    (distance, final_routes)
}

pub fn find_route(city_data: CityData, _mode: &str, boundaries: &[&str]) {
    let (city_map, cities_asia, cities_europe, cities_na) = prepare_area_data(&city_data);

    let mut boundary_cities = Vec::new();
    for b in boundaries {
        boundary_cities.push(city_map[*b]);
    }

    let results = [
        find_route_continents(&city_data, &[&cities_asia, &cities_europe, &cities_na], &boundary_cities, &[4, 4, 5]),
        find_route_continents(&city_data, &[&cities_asia, &cities_europe, &cities_na], &boundary_cities, &[4, 3, 6]),
        find_route_continents(&city_data, &[&cities_asia, &cities_europe, &cities_na], &boundary_cities, &[3, 4, 6]),
    ];

    let mut max_distance = 0;
    let mut longest_route = &Vec::new();
    for r in &results {
        if r.0 > max_distance {
            max_distance = r.0;
            longest_route = &r.1;
        }
    }

    println!("Distance = {}", max_distance);

    for p in longest_route {
        print!("{} - ", city(&city_data, *p).code);
    }
    println!("");
}
