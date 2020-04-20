use std::collections::VecDeque;
use std::{fs::File, io::BufReader, path::Path};
use std::io::{BufRead};
use std::fs;

static INPUT_FILE_NAME: &str = "in.txt";
static OUTPUT_FILE_NAME: &str = "out.txt";

fn main() {
    // let graph: Vec<_> = vec![
    //     vec![0, 1, 1, 0, 0, 0],
    //     vec![1, 0, 1, 0, 0, 0],
    //     vec![1, 1, 0, 1, 0, 0],
    //     vec![0, 0, 1, 0, 0, 0],
    //     vec![0, 0, 0, 0, 0, 1],
    //     vec![0, 0, 0, 0, 1, 0],
    // ];
    println!("Reading file \"{}\"...", INPUT_FILE_NAME);

    let lines = get_lines_from_file(INPUT_FILE_NAME);
    let size = lines[0].parse::<usize>().expect("Expected that first line will be demension");
    let matrix_raws: Vec<String> = lines.into_iter().skip(1).take(size).collect();
    let graph = parse_matrix(&matrix_raws); 

    println!("Recongnized matrix with size {}:", size);
    println!("{}", matrix_raws.join("\n"));

    println!("Finding connected components...");
    let mut components = get_connected_components(&graph);
    
    components.sort_by(|a, b| a.iter().min().unwrap().cmp(&b.iter().min().unwrap())); 
    // sort by max element in component 
    

    let mut r = Vec::new();
    for component in components {
        let mut sorted = component.clone();
        sorted.sort(); // sort 
        let mut b: Vec<String> = sorted.iter().map(|x| (x + 1).to_string()).collect();
        b.push("0".to_string()); // adding zero to line end
        r.push(b.join(" ")); // concat it to single string 
    }
    
    let result = r.join("\n");
    println!("Found components:");
    println!("{}", result);

    println!("Writing to file \"{}\"", OUTPUT_FILE_NAME);
    fs::write("out.txt", result).expect("Unable to write file");

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
    let mut result: Vec<_> = Vec::new();
    for vertex in 0..(grapth.len()) {
        let mut visited: Vec<_> = Vec::new();
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

fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse_matrix(lines: &Vec<String>) -> Vec<Vec<u16>> {
    lines.iter().map(|l| parse_raw(&l)).collect()
}

fn parse_raw(raw: &String) -> Vec<u16> {
    raw.split(" ").map(|n| n.trim().parse::<u16>().expect("can't parse line")).collect()
}
