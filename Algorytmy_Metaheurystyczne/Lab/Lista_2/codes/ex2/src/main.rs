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

fn find_best_neighbour(permutation: &Permutation, dist_matrix: &[Vec<u64>], tabu_list: &[(usize, usize)]) -> Option<(usize, usize)> {
    let mut best_move: Option<(usize, usize)> = None;
    let mut best_delta: i64 = i64::MAX;
    let n = permutation.route.len();

    for i in 1..n - 1 {
        for j in i + 1..n {
            if tabu_list.contains(&(i, j)) {
                continue;
            }

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
    }
    best_move
}

fn main() {
    let cities = load_data("../../data/zimbabwe.tsp");
    let n = cities.len();
    
    let dist_matrix = count_dist_matrix(&cities);
    let mut rng = thread_rng();
    
    let num_runs = 100;
    let stagnations = [50, 100, 150, 200];
    let tenures = [10, 20, 30, 40, 50];

    for &max_stagnation in &stagnations {
        for &tabu_tenure in &tenures {
            let mut best_param_distance = u64::MAX;

            for _ in 0..num_runs {
                let mut tabu_list: Vec<(usize, usize)> = Vec::new();
                let mut route: Vec<usize> = (0..n).collect();
                route.shuffle(&mut rng);
                let mut permutation = Permutation { route };
                
                let mut current_dist = permutation.distance(&dist_matrix);
                let mut best_run_distance = current_dist;
                
                let mut stagnation_counter = 0;
                
                loop {
                    if let Some((i, j)) = find_best_neighbour(&permutation, &dist_matrix, &tabu_list) {
                        tabu_list.push((i, j));
                        if tabu_list.len() > tabu_tenure {
                            tabu_list.remove(0);
                        }

                        permutation.invert(i, j);
                        current_dist = permutation.distance(&dist_matrix);

                        if current_dist < best_run_distance {
                            best_run_distance = current_dist;
                            stagnation_counter = 0;
                        } else {
                            stagnation_counter += 1;
                        }

                        if stagnation_counter >= max_stagnation {
                            break;
                        }
                    } else {
                        break; 
                    }
                }
                
                if best_run_distance < best_param_distance {
                    best_param_distance = best_run_distance;
                }
            }
            
            println!("Tabu tenure: {:>2}, max_stagnation: {:>3} => Dystans: {}", tabu_tenure, max_stagnation, best_param_distance);
        }
    }
}