use std::fs::File;
use std::io::{BufReader, BufRead};


#[derive(Debug)]
struct City {
    x: f64,
    y: f64,
}

#[derive(Debug)]
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

fn main() {
    let cities_coords = load_data("../../data/western_sahara.tsp");

    for city in cities_coords {
        println!("{:?}", city);
    }
}
