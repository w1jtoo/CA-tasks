use std::collections::HashSet;
use std::io::{BufRead};

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Square {
    raw: char,
    column: u8,
}

impl Square {
    fn new(raw: char, column: u8) -> Square {
        if raw > 'H' || raw < 'A' || column > 8 {
            panic!("Wrong raw or column");
        }
        Square {
            raw: raw,
            column: column,
        }
    }
    fn is_neighbour(&self, other: &Square) -> bool {
        let neighbuors = get_neighbours(&self);
        neighbuors.contains(other)
    }
    fn from_string(line: &str) -> Square {
        let chars: Vec<char> = line.to_uppercase().chars().collect();

        Square::new(chars[0], chars[1].to_digit(10).unwrap() as u8)
    }
    fn to_string(&self) -> String {
        format!("{}{}", self.raw, self.column)
    }
}
use std::fs;
fn main() {
    // let mut s = String::new();
    // let stdin = io::stdin();

    // stdin.lock().read_line(&mut s).unwrap();
    // let start_position = Square::from_string(&s);

    // let mut s = String::new();
    // stdin.lock().read_line(&mut s).unwrap();
    // let result_position = Square::from_string(&s);
    println!("Read file...");
    let lines = lines_from_file("input.txt");
    let start_position = Square::from_string(&lines[0]);
    let result_position = Square::from_string(&lines[1]);

    println!("Find path from {} to {}...", lines[0], lines[1]);
    let result: Vec<String> = find_path(start_position, result_position)
        .iter()
        .map(|el| el.to_string().to_lowercase())
        .collect();
    println!("Found path:");
    println!("{}", result.join(" -> "));
    println!("Writing to file...");
    fs::write("output.txt", result.join("\n")).expect("Unable to write file");
    println!("Successfully!");
}

fn find_path(start_position: Square, result_position: Square) -> Vec<Square> {
    let mut path = Vec::new();
    let mut possible_ways = Vec::new();
    let mut visited = HashSet::new();
    possible_ways.push(start_position);

    {
        let column = result_position.column as i8;
        if exists(char_shift(result_position.raw, -1), column - 1) {
            let dangeos = Square::new(
                char_shift(result_position.raw, -1),
                result_position.column - 1,
            );
            if dangeos == start_position {
                panic!("Knight have already dead! ")
            }
            visited.insert(dangeos);
        }
        if exists(char_shift(result_position.raw, 1), column - 1) {
            let dangeos = Square::new(
                char_shift(result_position.raw, 1),
                result_position.column - 1,
            );
            if dangeos == start_position {
                panic!("Knight have already dead! ")
            }
            visited.insert(dangeos);
        }
    }

    loop {
        if possible_ways.len() == 0 {
            break;
        }

        let position = possible_ways.pop().unwrap();
        if visited.contains(&position) {
            continue;
        }
        visited.insert(position);
        path.push(position);
        possible_ways.append(&mut get_neighbours(&position));

        if position == result_position {
            break;
        }
    }
    let result = distinct(&path);
    return result.into_iter().map(|x| *x).collect();
}

fn distinct(path: &Vec<Square>) -> Vec<&Square> {
    if path.len() == 0 {
        panic!("distinct empty path")
    }
    let mut result: Vec<&Square> = Vec::new();
    let mut last: &Square = path.last().unwrap();
    result.push(path.last().unwrap());
    for square in path.iter().rev() {
        if last.is_neighbour(square) {
            result.push(square);
            last = result.last().unwrap();
        }
    }

    result.reverse();
    result
}

use std::{
    fs::File,
    io::{BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_neighbours(square: &Square) -> Vec<Square> {
    let mut result = Vec::new();
    let column = square.column as i8;
    if exists(char_shift(square.raw, -1), column + 2) {
        result.push(Square::new(char_shift(square.raw, -1), square.column + 2))
    }
    if exists(char_shift(square.raw, 1), column + 2) {
        result.push(Square::new(char_shift(square.raw, 1), square.column + 2))
    }
    if exists(char_shift(square.raw, 2), column + 1) {
        result.push(Square::new(char_shift(square.raw, 2), square.column + 1))
    }
    if exists(char_shift(square.raw, 2), column - 1) {
        result.push(Square::new(char_shift(square.raw, 2), square.column - 1)) // 
    }
    if exists(char_shift(square.raw, 1), column - 2) {
        result.push(Square::new(char_shift(square.raw, 1), square.column - 2))
    }
    if exists(char_shift(square.raw, -1), column - 2) {
        result.push(Square::new(char_shift(square.raw, -1), square.column - 2))
    } // OK
    if exists(char_shift(square.raw, -2), column - 1) {
        result.push(Square::new(char_shift(square.raw, -2), square.column - 1))
    }
    if exists(char_shift(square.raw, -2), column + 1) {
        result.push(Square::new(char_shift(square.raw, -2), square.column + 1))
    }

    result
}
fn char_shift(raw: char, shift: i8) -> char {
    let letters = "ABCDEFGHIKLMNOPQRSTVXYZ";
    let alphabet = letters.trim();
    let position = alphabet.chars().position(|letter| letter == raw).unwrap();
    let n = (position as i8) + shift;
    if n < 0 {
        return alphabet
            .chars()
            .nth((alphabet.len() as i8 + n - 1) as usize)
            .unwrap();
    }
    alphabet.chars().nth(n as usize).unwrap()
}

fn exists(raw: char, column: i8) -> bool {
    raw <= 'H' && raw >= 'A' && column <= 8 && column >= 1
}

#[test]
fn shift_ok() {
    assert_eq!(char_shift('A', 1), 'B');
    assert_eq!(char_shift('A', 2), 'C');
    assert_eq!(char_shift('D', -1), 'C');
    assert_eq!(char_shift('C', -2), 'A');
}
#[test]
fn exists_should_be_ok() {
    assert!(exists('A', 1)); //  positions
    assert!(exists('H', 1));
    assert!(exists('H', 8));
    assert!(exists('A', 8));
}

#[test]
fn exists_should_be_not_ok() {
    assert!(!exists('A', 9));
    assert!(!exists('W', 3));
    assert!(!exists('I', 12));
    assert!(!exists('J', 42));
}
#[test]
fn get_neighbours_middle() {
    // middle positions with 8 possible neighbuors
    assert_eq!(get_neighbours(&Square::new('D', 4)).len(), 8);
    assert_eq!(get_neighbours(&Square::new('D', 5)).len(), 8);
    assert_eq!(get_neighbours(&Square::new('C', 6)).len(), 8);
    assert_eq!(get_neighbours(&Square::new('F', 3)).len(), 8);
}

#[test]
fn get_neighbours_corners() {
    // corner positions with 2 possible neighbuors
    assert_eq!(get_neighbours(&Square::new('A', 1)).len(), 2);
    assert_eq!(get_neighbours(&Square::new('A', 8)).len(), 2);
    assert_eq!(get_neighbours(&Square::new('H', 1)).len(), 2);
    assert_eq!(get_neighbours(&Square::new('H', 8)).len(), 2);
    // corner positions with 3 possible neighbuors
    assert_eq!(get_neighbours(&Square::new('B', 1)).len(), 3);
    assert_eq!(get_neighbours(&Square::new('A', 2)).len(), 3);
    assert_eq!(get_neighbours(&Square::new('G', 8)).len(), 3);
    assert_eq!(get_neighbours(&Square::new('H', 7)).len(), 3);

    // near to corner  positions with 4 possible neighbuors
    assert_eq!(get_neighbours(&Square::new('B', 2)).len(), 4);
    assert_eq!(get_neighbours(&Square::new('G', 7)).len(), 4);
}

#[test]
fn distinct_should_not_copy() {
    let fist = Square::new('A', 1);
    let second = Square::new('B', 3);
    let third = Square::new('D', 2);
    let path = vec![fist, second];
    let expected = vec![&fist, &second];

    assert_eq!(expected, distinct(&path));

    assert_eq!(
        vec![&fist, &second, &third],
        distinct(&vec![fist, second, third])
    );

    assert_eq!(
        vec![&third, &second, &fist],
        distinct(&vec![third, second, fist])
    );
}

#[test]
fn distinct_tests() {
    let fist = Square::new('A', 1);
    let second = Square::new('B', 1);
    let third = Square::new('C', 2);

    assert_eq!(vec![&fist, &third], distinct(&vec![fist, second, third]));

    assert_eq!(vec![&third, &fist], distinct(&vec![third, second, fist]));
}


#[test]
fn get_neighbours_turn() {
    let pos = Square::new('C', 3);
    let result = vec![Square::new('B', 5) , Square::new('D', 5) , Square::new('E', 4), 
    Square::new('E', 2) , Square::new('D', 1), Square::new('B', 1), 
    Square::new('A', 2), Square::new('A', 4)];

    assert_eq!(get_neighbours(&pos), result); 
}