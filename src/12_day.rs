use std::fs;
use dict::{ Dict, DictIface, DictEntry };

fn create_graph() -> u32 {  // Vec<DictEntry<Vec<&'static str>>>
    let filename: &str = "./src/12_data.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut graph = Dict::<Vec<&str>>::new();

    for link in content.split("\n") {
        let mut s = link.split("-");
        let a = s.next().unwrap();
        let b = s.next().unwrap();

        if !graph.contains_key(a) {
            graph.add(a.to_string(), vec![] as Vec<&str>);
        }

        if graph.contains_key(b) {
            graph.add(b.to_string(), vec![] as Vec<&str>);
        }
    }

    for link in content.split("\n") {
        let mut s = link.split("-");
        let a = s.next().unwrap();
        let b = s.next().unwrap();

        let mut neighbours_a: Vec<&str> = graph.get(a).unwrap().to_vec();
        let mut neighbours_b: Vec<&str> = graph.get(b).unwrap().to_vec();

        neighbours_a.push(b);
        neighbours_b.push(a);

        graph.add(a.to_string(), neighbours_a);
        graph.add(b.to_string(), neighbours_b);
    }

    for o in &graph {
        println!("{} - {:?}", o.key, o.val);
    }

    return 0
}

fn main() {
    create_graph();

    // println!("Part 1 total: {:?}", create_graph());  // Part 1 total: 1691
    // println!("Part 2 total: {:?}", create_graph());  // Part 2 total: 216

}
