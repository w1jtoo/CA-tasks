use std::collections::{HashSet, VecDeque};

fn main() {
    let grapth: Vec<Vec<u16>> = vec![
        vec![0, 1, 1, 0, 0, 0],
        vec![1, 0, 1, 0, 0, 0],
        vec![1, 1, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 1, 0]
    ];
    let components = get_connected_components(&grapth);
    let mut r = Vec::new();
    for c in components {
        let b: Vec<String> = c.iter().map(|x| x.to_string()).collect();
        r.push(b.join(" "));
    }
    println!("{}", r.join("\n"));
}

fn get_neighbours(edges: &Vec<u16>) -> Vec<u16> {
    let mut result = Vec::new();
    for (index, edge) in edges.iter().enumerate() {
        if edge == &1 {
            result.push(index as u16);
        }
    }

    result
}

fn get_connected_components(grapth: &Vec<Vec<u16>>) -> Vec<Vec<u16>> {
    let mut result: Vec<Vec<u16>> = Vec::new();
    for vertex in 0..(grapth.len()) {
        let mut visited: Vec<u16> = Vec::new();
        let mut vertexies = result.iter().flatten();
        if vertexies.any(|v| v == &(vertex as u16)) {
            continue;
        }
        // bfs start
        let current_neighbours = get_neighbours(&grapth[vertex]);
        let mut neighbours = VecDeque::<u16>::new();
        neighbours.extend(&current_neighbours);
        visited.push(vertex as u16);
        
        while neighbours.len() != 0 {
            let current_vertex = neighbours.pop_front().unwrap();
            if visited.contains(&current_vertex) {
                continue;
            }
            let current_neighbours = get_neighbours(&grapth[current_vertex as usize]);
            neighbours.extend(&current_neighbours);
            visited.push(current_vertex as u16);
        }

        result.push(visited);
    }

    result
}
