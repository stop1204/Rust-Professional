use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;


pub fn count_provinces() -> String {
    // 读取JSON文件
    let mut file = File::open("/Users/terry/RustroverProjects/Rust-Professional/exercises/hard/solutiont3/district.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // 解析JSON
    let json: Value = serde_json::from_str(&contents).unwrap();

    // 存储每个批次的省份数量
    let mut result = Vec::new();

    // 遍历每个批次
    for (_, batch) in json.as_object().unwrap() {
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

        // 构建图
        for (city, neighbors) in batch.as_object().unwrap() {
            let city = city.to_string();
            graph.entry(city.clone()).or_insert_with(HashSet::new);
            for neighbor in neighbors.as_array().unwrap() {
                let neighbor = neighbor.as_str().unwrap().to_string();
                graph.entry(city.clone()).and_modify(|e| { e.insert(neighbor.clone()); });
                graph.entry(neighbor).or_insert_with(HashSet::new).insert(city.clone());
            }
        }

        // 计算连通分量数量
        let province_count = count_connected_components(&graph);
        result.push(province_count.to_string());
    }

    // 返回结果
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
    if visited.insert(node.to_string()) {
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                dfs(neighbor, graph, visited);
            }
        }
    }
}
