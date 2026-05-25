use std::fs::File;
use std::io::{BufReader, BufRead};
use rand::Rng;
use rand::thread_rng;
use rand::prelude::*; //you can limit this import to just the shuffle function if needed

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
        let (start, end) = if i <= j { (i, j) } else { (j, i) };
        self.route[start..=end].reverse(); 
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

fn count_probability(current_distance: u64, new_distance: u64, temperature: f64) -> f64 {
    if new_distance < current_distance {
        1.0
    } else {
        let exponent = (current_distance as f64 - new_distance as f64) / temperature;
        exponent.exp()
    }
}

fn random_two_idx(n: usize) -> (usize, usize) {
    let mut rng = thread_rng();
    let i = rng.gen_range(0..n);
    let mut j = rng.gen_range(0..n);

    while j == i {
        j = rng.gen_range(0..n);
    }

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

fn simulated_annealing(cities: &[City], dist_matrix: &[Vec<u64>]) -> Permutation {
    let mut rng = thread_rng();
    let n = cities.len();
    let attempts_per_epoch = 100; 
    
    let mut global_best_perm = Permutation { route: (0..n).collect() };
    let mut global_best_distance = u64::MAX;

    let mut init_temperature = 100.0;
    let mut cooling_rate = 0.80;

    for _ in 0..20 {
        let mut current_perm = Permutation { route: (0..n).collect() };
        current_perm.route.shuffle(&mut rng);
        let mut best_perm = current_perm.clone();
        
        let mut current_distance = current_perm.distance(dist_matrix);
        let mut best_distance = current_distance;
        
        let mut temperature = init_temperature;

        while temperature > 1.0 {
            for _ in 0..attempts_per_epoch {
                let (i, j) = random_two_idx(n);
                let mut new_perm = current_perm.clone();
                new_perm.invert(i, j);

                let new_distance = new_perm.distance(dist_matrix);

                if count_probability(current_distance, new_distance, temperature) > thread_rng().r#gen() {
                    current_perm = new_perm;
                    current_distance = new_distance; 
                    
                    if current_distance < best_distance {
                        best_perm = current_perm.clone();
                        best_distance = current_distance;
                    }
                }
            }
            temperature *= cooling_rate;
        }
        
        println!("Temperatura początkowa: {:>8.1}, cooling_rate: {:.3} => Dystans: {}", init_temperature, cooling_rate, best_distance);

        if best_distance < global_best_distance {
            global_best_distance = best_distance;
            global_best_perm = best_perm;
        }

        init_temperature *= 10.0;
        if init_temperature > 100000.0 {
            init_temperature = 100.0;
            cooling_rate += 0.045;
        }
    }

    global_best_perm
}

fn main() {
    let cities = load_data("../../data/qatar.tsp");
    let dist_matrix = count_dist_matrix(&cities);
    let best_solution = simulated_annealing(&cities, &dist_matrix);
    println!("Best route: {:?}", best_solution.route);
    println!("Best distance: {}", best_solution.distance(&dist_matrix));
}
