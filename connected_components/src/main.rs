use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn get_connected_components(grapth: &Vec<Vec<u16>>) -> Vec<u16> { 
    let mut visited = HashSet::new();
    
    for vertex in 0..(grapth.len()) { 
        if !visited.contains(&vertex) { 
            continue;
        }
        // bfs start
        visited.insert(vertex);
    }
    Vec::new()
}

