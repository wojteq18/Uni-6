use std::fs::File;
use std::io::{BufReader, BufRead};
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

#[derive(Debug, Clone)]
struct Edge {
    idx1: usize,
    idx2: usize,
    weight: u64,
}

struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(size: usize) -> Self {
        Dsu { parent: (0..size).collect() }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        self.parent[i] = self.find(self.parent[i]); //track compression
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parent[root_i] = root_j;
            return true; 
        }
        false 
    }
}

fn all_edges(cities: &[City], dist_matrix: &Vec<Vec<u64>>) -> Vec<Edge> {
    let mut edges = Vec::new();
    for i in 0..cities.len() {
        for j in (i + 1)..cities.len() {
            let w = dist_matrix[i][j];
            edges.push(Edge { idx1: i, idx2: j, weight: w });
        }
    }
    edges
}

fn kruskal(num_cities: usize, sorted_edges: Vec<Edge>) -> Vec<Edge> {
    let mut mst = Vec::new();
    let mut dsu = Dsu::new(num_cities);

    for edge in sorted_edges {
        if dsu.union(edge.idx1, edge.idx2) {
            mst.push(edge);
        }
        
        if mst.len() == num_cities - 1 {
            break;
        }
    }
    mst
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

fn find_best_neighbour(permutation: &Permutation, dist_matrix: &[Vec<u64>]) -> Option<(usize, usize)> {
    let mut best_move: Option<(usize, usize)> = None;
    let mut best_delta: i64 = 0;
    let n = permutation.route.len();

    for i in 1..n - 1 {
        for j in i + 1..n {
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

fn sort_by_weight(edges: Vec<Edge>) -> Vec<Edge> {
    let mut sorted_edges = edges;
    sorted_edges.sort_by_key(|e| e.weight);
    sorted_edges
}

fn get_dfs_route(start_node: usize, adj: &Vec<Vec<usize>>, n: usize) -> Vec<usize> {
    let mut route = Vec::with_capacity(n);
    let mut visited = vec![false; n];
    let mut stack = vec![start_node];

    while let Some(u) = stack.pop() {
        if !visited[u] {
            visited[u] = true;
            route.push(u); 
            
            for &v in &adj[u] {
                if !visited[v] {
                    stack.push(v);
                }
            }
        }
    }
    route
}



fn main() {
    let cities = load_data("../../data/egypt.tsp"); 
    let n = cities.len();
    let dist_matrix = count_dist_matrix(&cities);
    
    let all_edges = all_edges(&cities, &dist_matrix);
    let sorted_edges = sort_by_weight(all_edges);
    let mst = kruskal(n, sorted_edges);
    
    let mst_weight: u64 = mst.iter().map(|e| e.weight).sum();
    println!("Waga minimalnego drzewa rozpinającego (MST): {}", mst_weight);

    let mut adj = vec![vec![]; n];
    for edge in &mst {
        adj[edge.idx1].push(edge.idx2);
        adj[edge.idx2].push(edge.idx1);
    }

    use rand::Rng; 
    let mut rng = thread_rng();
    
    let loop_count = (n as f64).sqrt().ceil() as usize;
    println!("Uruchamiam Local Search {} razy...", loop_count);

    let mut sum_of_distances: u64 = 0;
    let mut sum_of_iterations: u64 = 0;
    let mut global_best_distance: u64 = u64::MAX;

    for i in 0..loop_count {
        let start_node = rng.gen_range(0..n);
        
        let route = get_dfs_route(start_node, &adj, n);
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

        println!("iteraation number: {}", i + 1);
    }
    
    let avg_distance = sum_of_distances as f64 / loop_count as f64;
    let avg_iterations = sum_of_iterations as f64 / loop_count as f64;

    println!("Average value of the solution: {:.2}", avg_distance); 
    println!("Average number of improvement steps: {:.2}", avg_iterations);
    println!("Best obtained solution: {}", global_best_distance);
}