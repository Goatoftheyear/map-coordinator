use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Clone, Debug)]

struct Node {
    owner: String,
    x: f64,
    y: f64,
}

struct Connection {
    node1: Node,
    node2: Node,
    edge: i8,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_to_node(mut coordinates: String) -> Node {
    coordinates = coordinates.replace("(", "");
    coordinates = coordinates.replace(")", "");
    let coordinates: Vec<String> = coordinates.split(",").map(|s| s.to_string()).collect();
    let node = Node {
        owner: String::from("default"),
        x: coordinates[0]
            .parse()
            .expect("error parsing x coordinates from map"),
        y: coordinates[1]
            .parse()
            .expect("error parsing y coordinates from map"),
    };
    return node;
}

fn main() {
    println!("Hello!");
    let mut opened_maps: HashMap<String, String> = HashMap::new();
    let mut teleport_locations: HashMap<String, Vec<String>> = HashMap::new();

    //TODO: set up fix tp location
    if let Ok(lines) = read_lines("./config/test.txt") {
        for line in lines.map_while(Result::ok) {
            if line.is_empty() == false {
                let mut line_splitted: Vec<String> =
                    line.splitn(2, " ").map(|s| s.to_string()).collect();
                let mut chars = line_splitted[0].chars();
                for char in line_splitted[0].chars() {
                    if char.is_alphabetic() == false {
                        chars.next();
                        continue;
                    }
                    line_splitted[0] = chars.as_str().to_string();
                    break;
                }
                let mut tp_coord = Vec::new();
                if let Some(location_list) = teleport_locations.get(&line_splitted[0]) {
                    tp_coord = location_list.to_vec();
                }
                let mut test = line_splitted[1].replace(" ", "");
                let node = parse_to_node(test);
                tp_coord.push(node);
                teleport_locations.insert(line_splitted[0].clone(), tp_coord);
                println!("{:?}", teleport_locations);
            }
        }
    }
    loop {
        println!("You can right click to the prompt to paste your latest copy entry");
        println!("Add person's coordinates. E.g. Kay Memoryland(17.5 , 46.3)");
        println!("Delete by typing just the name. E.g. Kay");
        println!("Entries are case sensitive");
        //TODO: decide when and where the node calculation should be
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
        .expect("Error reading input");
        input = input.trim_end().to_string();
        println!("{:?}", input);
        //TODO: add entries in Map/dictionary
        let _splitted: &Vec<String> = &input.splitn(2, " ").map(|s| s.to_string()).collect();
        println!("{:?}", _splitted);
        if _splitted.len() == 1 {
            opened_maps.remove(&input);
        } else {
        opened_maps.insert(_splitted[0].clone(), _splitted[1].clone());
    }
        println!("{opened_maps:?}");
        if input.trim() == "end" {
            println!("See ya!");
            break;
        }
    }
}
// Yak T'el ( 16.6  , 12.4 )
// Yak T'el ( 12.3  , 12.3 )
//assumption after testing is 2.6 per 1.0 grid