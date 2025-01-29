use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Clone, Debug)]
struct Node {
    owner: String,
    x: f32,
    y: f32,
}
#[derive(Debug)]
struct Edge<'a> {
    node1: &'a Node,
    node2: &'a Node,
    weight: f32,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_to_node(mut coordinates: String, owner: String) -> Node {
    coordinates = coordinates.replace("(", "");
    coordinates = coordinates.replace(")", "");
    let coordinates: Vec<String> = coordinates.split(",").map(|s| s.to_string()).collect();
    let node = Node {
        owner: owner,
        x: coordinates[0]
            .parse()
            .expect("error parsing x coordinates from map"),
        y: coordinates[1]
            .parse()
            .expect("error parsing y coordinates from map"),
    };
    return node;
}

fn creating_edge<'a>(
    starting_node: &'a Node,
    ending_node: &'a Node,
    back_to_starting_point: bool,
    edge_list: &mut Vec<Edge<'a>>,
) -> () {
    let mut distance: f32 = 5.0;
    if !back_to_starting_point {
        distance = ((starting_node.x - ending_node.x).powf(2.0)
            + (starting_node.y - ending_node.y).powf(2.0))
        .sqrt();
    }
    println!("{}", distance);
    // Edge {
    //     node1: starting_node,
    //     node2: ending_node,
    //     weight: distance,
    // };
    edge_list.push(Edge {
        node1: starting_node,
        node2: ending_node,
        weight: distance,
    })
}

fn main() {
    println!("Hello!");
    let mut opened_maps: HashMap<String, Vec<Node>> = HashMap::new();
    let mut teleport_locations: HashMap<String, Vec<Node>> = HashMap::new();
    let mut player_maps: HashMap<String, String> = HashMap::new();
    if let Ok(lines) = read_lines("./config/default_location.txt") {
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
                let no_space_coordinates = line_splitted[1].replace(" ", "");
                let node = parse_to_node(no_space_coordinates, String::from("default"));
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
        if input.trim() == "end" {
            println!("See ya!");
            break;
        }
        let _splitted: &Vec<String> = &input.splitn(2, " ").map(|s| s.to_string()).collect();
        player_maps.insert(_splitted[0].clone(), _splitted[1].clone());
        if _splitted.len() == 1 {
            opened_maps.remove(&input);
        } else {
            let coordinates_with_name: Vec<String> =
                _splitted[1].split("(").map(|s| s.to_string()).collect();
            println!("{:?}", coordinates_with_name);
            let node = parse_to_node(
                coordinates_with_name[1].replace(" ", "").clone(),
                _splitted[0].clone(),
            );
            println!("{:?}", node);
            let mut tp_coord = Vec::new();
            if let Some(location_list) = opened_maps.get(&coordinates_with_name[0].clone()) {
                tp_coord = location_list.to_vec();
            }
            tp_coord.push(node);
            println!("{:?}", tp_coord);
            opened_maps.insert(coordinates_with_name[0].clone(), tp_coord);
        }
        println!("{opened_maps:?}");
        println!("{player_maps:?}");

        // create edges
        let mut edge_list: Vec<Edge> = Vec::new();
        if let (Some(default_nodes), Some(maps)) = (
            teleport_locations.get("Memoryland"),
            opened_maps.get("Memoryland"),
        ) {
            println!("{:?}", default_nodes);
            println!("{:?}", maps);
            for map in maps {
                for default_node in default_nodes {
                    creating_edge(default_node, map, false, &mut edge_list);
                    creating_edge(default_node, map, true, &mut edge_list);
                }
                for other_map in maps {
                    if map.owner != other_map.owner {
                        creating_edge(map, other_map, false, &mut edge_list);
                    }
                }
            }
            println!("{:?}", edge_list);
        }
    }
}
// Yak T'el ( 16.6  , 12.4 )
// Yak T'el ( 12.3  , 12.3 )
//assumption after testing is 2.6 per 1.0 grid
