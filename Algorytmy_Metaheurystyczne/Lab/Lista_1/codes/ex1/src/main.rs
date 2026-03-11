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
    fn distance(&self) -> u64 {
        let mut total: u64 = 0;

        for i in 0..self.cities.len() {
            let a = &self.cities[i];
            let b = &self.cities[(i + 1) % self.cities.len()];

            let dx = b.x - a.x;
            let dy = b.y - a.y;
            let d = (dx * dx + dy * dy).sqrt();

            let w = (d + 0.5).floor() as u64;

            total += w;
        }

        total
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

fn avg_result(how_many_draws: usize, permutations: &Vec<u64>) -> u64 { //how many draws is a parameter that tells us how many draws there are in one group
    if how_many_draws == 1 {
        return *permutations.iter().min().unwrap_or(&0);
    } else {
        let mut group_avg = 0u64;
        for group in permutations.chunks(how_many_draws) {
            let group_min = group.iter().min().unwrap_or(&0);
            group_avg += group_min;
        }
        group_avg / (permutations.len() as u64 / how_many_draws as u64)
    }
}

fn main() {
    let cities_coords = load_data("../../data/zimbabwe.tsp"); //example for western_sahara file

    let mut rng = thread_rng();
    let mut all_permutations = Vec::new();
    for _ in 0..1000 {
        let mut perm = cities_coords.clone();
        perm.shuffle(&mut rng);
        all_permutations.push(Permutation { cities: perm });
    }

    let distances: Vec<u64> = all_permutations.iter().map(|p| p.distance()).collect();

    let result100 = avg_result(10, &distances);
    let result50 = avg_result(50, &distances);
    let result1 = avg_result(1, &distances);

    println!("Average result with 100 groups: {}", result100);
    println!("Average result with 20 groups: {}", result50);
    println!("Average result with 1 group: {}", result1);
}
