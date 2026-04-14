use std::fs::File;
use std::io::{BufReader, BufRead};
use rand::seq::SliceRandom; 
use rand::thread_rng;
use rand::Rng;

#[derive(Debug, Clone)]
struct City {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Permutation {
    route: Vec<usize>, 
}

impl Permutation {
    fn distance(&self, dist_matrix: &[Vec<u64>]) -> u64 {
        let mut total: u64 = 0;
        let n = self.route.len();

        for i in 0..n {
            let city_a = self.route[i];
            let city_b = self.route[(i + 1) % n];
            total += dist_matrix[city_a][city_b];
        }

        total
    }

    fn invert(&mut self, i: usize, j: usize) {
        self.route[i..=j].reverse(); 
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

fn draw_pair(num: usize) -> (usize, usize) { 
    let mut rng = thread_rng();
    let i = rng.gen_range(1..num - 1);
    let j = rng.gen_range(i + 1..num);
    (i, j)
}

fn count_dist_matrix(cities: &[City]) -> Vec<Vec<u64>> {
    let mut dist_matrix = vec![vec![0; cities.len()]; cities.len()];

    for i in 0..cities.len() {
        for j in 0..cities.len() {
            let city_a = &cities[i];
            let city_b = &cities[j];
            let dx = city_b.x - city_a.x;
            let dy = city_b.y - city_a.y;
            let d = (dx * dx + dy * dy).sqrt();
            dist_matrix[i][j] = (d + 0.5).floor() as u64;
        }
    }
    dist_matrix
}

fn find_best_neighbour(permutation: &Permutation, dist_matrix: &[Vec<u64>]) -> Option<(usize, usize)> {
    let mut best_move: Option<(usize, usize)> = None;
    let mut best_delta: i64 = 0;
    let n = permutation.route.len();

   for _ in 0..n {
        let (i, j) = draw_pair(n);
        let city_before_i = permutation.route[i - 1];
        let city_i = permutation.route[i];
        let city_j = permutation.route[j];
        let city_after_j = permutation.route[(j + 1) % n];

        let old_edges = dist_matrix[city_before_i][city_i] + dist_matrix[city_j][city_after_j];
        let new_edges = dist_matrix[city_before_i][city_j] + dist_matrix[city_i][city_after_j];
        let delta = (new_edges as i64) - (old_edges as i64);    
        if delta < best_delta {
            best_delta = delta;
            best_move = Some((i, j));
        }
   }
    best_move
}

fn main() {
    let cities = load_data("../../data/ireland.tsp"); 
    let n = cities.len();
    
    let dist_matrix = count_dist_matrix(&cities);
    let mut rng = thread_rng();
    
    let num_runs = 5;
    let mut sum_of_distances: u64 = 0;
    let mut sum_of_iterations: u64 = 0;
    let mut global_best_distance: u64 = u64::MAX;

    for _ in 0..num_runs {
        let mut route: Vec<usize> = (0..n).collect();
        route.shuffle(&mut rng);
        let mut permutation = Permutation { route };
        let mut iteration = 0;
        
        loop {
            if let Some((i, j)) = find_best_neighbour(&permutation, &dist_matrix) {
                permutation.invert(i, j);
                iteration += 1;
            } else {
                break; 
            }
        }
        
        let final_dist = permutation.distance(&dist_matrix);
        sum_of_distances += final_dist;
        sum_of_iterations += iteration;
        
        if final_dist < global_best_distance {
            global_best_distance = final_dist;
        }
    }
    
    let avg_distance = sum_of_distances as f64 / num_runs as f64;
    let avg_iterations = sum_of_iterations as f64 / num_runs as f64;

    println!("Srednia wartosc rozwiazania: {:.2}", avg_distance);
    println!("Srednia liczba krokow poprawy: {:.2}", avg_iterations);
    println!("Najlepsze uzyskane rozwiazanie: {}", global_best_distance);
}