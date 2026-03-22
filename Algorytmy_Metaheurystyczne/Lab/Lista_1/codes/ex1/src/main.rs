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

    fn invert(&mut self, i: usize, j: usize) {
        self.cities[i..=j].reverse();
    }

    fn dist_between(&self, i: usize, j: usize) -> i64 {
        let a = &self.cities[i];
        let b = &self.cities[j];
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let d = (dx * dx + dy * dy).sqrt();
        (d + 0.5).floor() as i64
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

fn find_best_neighbour(permutation: &mut Permutation) -> Option<(usize, usize)> {
    let mut best_move: Option<(usize, usize)> = None;
    let mut best_delta: i64 = 0;
    let amount_of_cities = permutation.cities.len();

    for i in 1..amount_of_cities - 1 {
        for j in i+1..amount_of_cities {
            let old_edges = permutation.dist_between(i-1, i) + permutation.dist_between(j, (j+1) % amount_of_cities);
            let new_edges = permutation.dist_between(i-1, j) + permutation.dist_between(i, (j+1) % amount_of_cities);
            let delta = (new_edges as i64) - (old_edges as i64);

            if delta < best_delta {
                best_delta = delta;
                best_move = Some((i, j));
            }
        }
    }
    best_move
}

fn main() {
    let cities = load_data("../../data/oman.tsp");
    let mut permutation = Permutation { cities };
    let mut rng = thread_rng();
    let mut iteration = 0;
    permutation.cities.shuffle(&mut rng);
    println!("Initial distance: {}", permutation.distance());
    loop {
        if let Some((i, j)) = find_best_neighbour(&mut permutation) {
            permutation.invert(i, j);
            iteration += 1;
        } else {
            break;
        }
    }
    println!("Final distance: {}", permutation.distance());
    println!("Iterations: {}", iteration);

}
