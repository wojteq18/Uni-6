use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Clone)]
struct City {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Edge {
    idx1: usize,
    idx2: usize,
    weight: u64,
}

struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    fn new(size: usize) -> Self {
        DSU { parent: (0..size).collect() }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        self.parent[i] = self.find(self.parent[i]);
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

fn distance(a: &City, b: &City) -> u64 { 
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let d = (dx * dx + dy * dy).sqrt();
    (d + 0.5).floor() as u64
}

fn all_edges(cities: &Vec<City>) -> Vec<Edge> {
    let mut edges = Vec::new();
    for i in 0..cities.len() {
        for j in (i + 1)..cities.len() {
            let a = &cities[i];
            let b = &cities[j];
            let w = distance(a, b);
            edges.push(Edge { idx1: i, idx2: j, weight: w });
        }
    }
    edges
}

fn sort_by_weight(edges: Vec<Edge>) -> Vec<Edge> {
    let mut sorted_edges = edges;
    sorted_edges.sort_by_key(|e| e.weight);
    sorted_edges
}

fn kruskal(num_cities: usize, sorted_edges: Vec<Edge>) -> Vec<Edge> {
    let mut mst = Vec::new();
    let mut dsu = DSU::new(num_cities);

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

fn main() {
    let all_cities = load_data("../../data/western_sahara.tsp");
    let all_edges = all_edges(&all_cities);
    let sorted_edges = sort_by_weight(all_edges);
    let kruskal_edges = kruskal(all_cities.len(), sorted_edges);
    let mst = kruskal(all_cities.len(), kruskal_edges);
    println!("{:?}", mst);
    println!("{}", mst.len());
}
