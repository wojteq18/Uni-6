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

fn count_probability(current_distance: u64, new_distance: u64, temperature: f64) -> f64 {
    if new_distance < current_distance {
        1.0
    } else {
        let exponent = (current_distance as f64 - new_distance as f64) / temperature;
        exponent.exp()
    }
}

fn main() {
    println!("Hello, world!");
}
