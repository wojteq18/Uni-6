use std::fs::File;
use std::io::{BufReader, BufRead};
use rand::seq::SliceRandom; 
use rand::thread_rng;


#[derive(Debug, Clone)]
struct City {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Permutation {
    cities: Vec<City>,
}

impl Permutation {
    fn distance(&self) -> f64 {
        let mut total_distance: f64 = 0.0;
        for i in 0..self.cities.len() {
            let city_a = &self.cities[i];
            let city_b = &self.cities[(i + 1) % self.cities.len()]; // Wrap around to the first city
            total_distance += ((city_b.x - city_a.x).powi(2) + (city_b.y - city_a.y).powi(2)).sqrt();
        }
        total_distance
    }
}

fn load_data(path: &str) -> Vec<City> {
    let mut cities = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut in_node_section = false;
    for line in reader.lines() {
        let line = line.expect("Could not read line!");
        let cleaned = line.trim();

        if cleaned.starts_with("NODE_COORD_SECTION") {
            in_node_section = true;
            continue;
        }

        if cleaned.starts_with("EOF") {
            break;
        }

        if in_node_section {
            let parts: Vec<&str> = cleaned.split_whitespace().collect();
            if parts.len() >= 3 {
                let x: f64 = parts[1].parse().expect("Could not parse x coordinate");
                let y: f64 = parts[2].parse().expect("Could not parse y coordinate");
                cities.push(City { x, y });
            }
        }
    }
    cities
}

fn avg_result(how_many_draws: usize, permutations: Vec<f64>) -> f64 { //how many draws is a parameter that tells us how many draws there are in one group
    if how_many_draws == 1 {
        return permutations.iter().min_by(|a, b| a.total_cmp(b)).copied().unwrap_or(0.0);
    } else {
        let mut group_avg = 0.0;
        for group in permutations.chunks(how_many_draws) {
            let group_min = group.iter().min_by(|a, b| a.total_cmp(b)).copied().unwrap_or(0.0);
            group_avg += group_min;
        }
        group_avg / (permutations.len() as f64 / how_many_draws as f64)
    }
}

fn main() {
    let cities_coords = load_data("../../data/western_sahara.tsp"); //example for western_sahara file

    let mut rng = thread_rng();
    let mut all_permutations = Vec::new();
    for _ in 0..1000 {
        let mut perm = cities_coords.clone();
        perm.shuffle(&mut rng);
        all_permutations.push(Permutation { cities: perm });
    }

    let distances: Vec<f64> = all_permutations.iter().map(|p| p.distance()).collect();

    let result = avg_result(50, distances); 
    println!("Average result: {}", result);
}
