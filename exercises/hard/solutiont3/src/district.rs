use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;


pub fn count_provinces() -> String {
    // Read JSON file
    let mut file = File::open("/Users/terry/RustroverProjects/Rust-Professional/exercises/hard/solutiont3/district.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse JSON
    let json: Value = serde_json::from_str(&contents).unwrap();

    // Store province count for each batch
    let mut result = Vec::new();

    // Iterate through each batch
    for (_, batch) in json.as_object().unwrap() {
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

        // Build graph
        for (city, neighbors) in batch.as_object().unwrap() {
            let city = city.to_string();
            graph.entry(city.clone()).or_insert_with(HashSet::new);
            for neighbor in neighbors.as_array().unwrap() {
                let neighbor = neighbor.as_str().unwrap().to_string();
                graph.entry(city.clone()).and_modify(|e| { e.insert(neighbor.clone()); });
                graph.entry(neighbor).or_insert_with(HashSet::new).insert(city.clone());
            }
        }

        // Count connected components
        let province_count = count_connected_components(&graph);
        result.push(province_count.to_string());
    }

    // Return result
    result.join(",")
}


fn count_connected_components(graph: &HashMap<String, HashSet<String>>) -> usize {
    let mut visited = HashSet::new();
    let mut count = 0;

    for node in graph.keys() {
        if !visited.contains(node) {
            dfs(node, graph, &mut visited);
            count += 1;
        }
    }

    count
}


fn dfs(node: &str, graph: &HashMap<String, HashSet<String>>, visited: &mut HashSet<String>) {
    visited.insert(node.to_string());

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs(neighbor, graph, visited);
            }
        }
    }
}
