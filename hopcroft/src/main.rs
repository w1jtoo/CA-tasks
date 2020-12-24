struct Graph<'a> {
    count: usize,
    sets: &'a Vec<&'a Vec<char>>,
    items: Vec<char>
}

impl<'a> Graph<'a> {
    fn new(sets: &'a Vec<&'a Vec<char>>) -> Graph<'a> {
        let mut items = Vec::new();
        for list in sets {
            for item in list.iter() {
                if !items.contains(item) {
                    items.push(*item);
                }
            }
        }

        Graph {
            sets: sets,
            count: sets.len(),
            items: items
        }
    }
}

fn bfs(graph: &Graph) -> &Vec<>{

}

fn main() {
    println!("Hello, world!");
}
