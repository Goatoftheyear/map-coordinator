use std::collections::HashMap;
use std::f32::INFINITY;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
struct Node {
    owner: String,
    x: f32,
    y: f32,
}
#[derive(Clone, Debug, PartialEq)]
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
        .sqrt()
            * 2.6;
    }
    edge_list.push(Edge {
        node1: starting_node,
        node2: ending_node,
        weight: distance,
    })
}

fn permutation(players: &mut Vec<String>, permutations: &mut Vec<Vec<String>>) -> () {
    if players.len() == 1 {
        permutations.push(players.clone());
        return;
    }
    let mut temp_players = players.to_vec();
    let mut permutation_amount = 1;
    for i in 1..(players.len() + 1) {
        permutation_amount = permutation_amount * i;
    }
    let mut iterator = 0;
    for _i in 1..permutation_amount {
        temp_players.swap(iterator, iterator + 1);
        permutations.push(temp_players.clone());
        iterator += 1;
        if iterator == temp_players.len() - 1 {
            iterator = 0;
        }
    }
}

fn find_permutation(mut players: Vec<String>) -> Vec<Vec<String>> {
    let mut permutations = Vec::<Vec<String>>::new();
    permutation(&mut players, &mut permutations);

    permutations
}

fn help_text() -> () {
    println!("You can right click to the prompt to paste your latest copy entry");
    println!("Add person's coordinates. E.g. WoL Urqopacha(17.5 , 46.3)");
    println!("Delete by typing just the name. E.g. WoL");
    println!("Entries are case sensitive");
    println!("Commands: ");
    println!("help: send help message");
    println!("reset: remove all current entry");
    println!("end: end the program");
}

fn remove_existing_entry(opened_maps: &mut HashMap<String, Vec<Node>>, owner_name: &String) {
    let temp_opened_maps: HashMap<String, Vec<Node>> = opened_maps.clone();
    for (map_name, nodes) in temp_opened_maps {
        let mut temp_nodes = nodes.clone();
        temp_nodes.retain(|x| x.owner != owner_name.clone());
        opened_maps.insert(map_name.to_string(), temp_nodes);
    }
}

fn main() {
    println!("Hello!");
    help_text();
    let mut opened_maps: HashMap<String, Vec<Node>> = HashMap::new();
    let mut teleport_locations: HashMap<String, Vec<Node>> = HashMap::new();
    let mut player_maps: HashMap<String, String> = HashMap::new();
    // read file
    if let Ok(lines) = read_lines("./config/default_location.txt") {
        for line in lines.map_while(Result::ok) {
            if line.is_empty() == false {
                let line_splitted: Vec<String> = line.split("(").map(|s| s.to_string()).collect();
                let name_with_location_name: Vec<String> = line_splitted[0]
                    .trim()
                    .split("-")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let mut tp_coord = Vec::new();
                if let Some(location_list) =
                    teleport_locations.get(&name_with_location_name[1].clone())
                {
                    tp_coord = location_list.to_vec();
                }
                let no_space_coordinates = line_splitted[1].replace(" ", "");
                let node = parse_to_node(
                    no_space_coordinates,
                    String::from(&name_with_location_name[0]),
                );
                tp_coord.push(node);
                teleport_locations.insert(name_with_location_name[1].clone(), tp_coord);
            }
        }
    }
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        input = input.trim_end().to_string();
        if input == "" {
            continue;
        }
        if input.trim() == "end" {
            println!("See ya!");
            break;
        }
        if input.trim() == "help" {
            help_text();
            continue;
        }
        if input.trim() == "reset" {
            player_maps = HashMap::new();
            opened_maps = HashMap::new();
            continue;
        }
        let _splitted: &Vec<String> = &input.splitn(2, " ").map(|s| s.to_string()).collect();
        // Assume if len 1 is delete
        if _splitted.len() == 1 {
            remove_existing_entry(&mut opened_maps, &input);
            player_maps.remove(&input);
        } else {
            remove_existing_entry(&mut opened_maps, &input);

            player_maps.insert(_splitted[0].clone(), _splitted[1].clone());
            let mut coordinates_with_name_temp: Vec<String> =
                _splitted[1].split("(").map(|s| s.to_string()).collect();
            let allowed_characters = "' -";
            coordinates_with_name_temp[0] = coordinates_with_name_temp[0]
                .trim_end()
                .to_string()
                .chars()
                .filter(|s| s.is_alphanumeric() || allowed_characters.contains(*s))
                .collect();
            let coordinates_with_name: Vec<String> = coordinates_with_name_temp.to_vec();
            // coordinates_with_name
            let node = parse_to_node(
                coordinates_with_name[1].replace(" ", "").clone(),
                _splitted[0].clone(),
            );
            let mut tp_coord = Vec::new();
            if let Some(location_list) = opened_maps.get(&coordinates_with_name[0].clone()) {
                tp_coord = location_list.to_vec();
            }
            tp_coord.retain(|x| x.owner != node.owner);
            tp_coord.push(node);
            opened_maps.insert(coordinates_with_name[0].clone(), tp_coord);
        }

        for place in opened_maps.keys().collect::<Vec<&String>>() {
            // reset for each map
            let mut total_weight: Vec<f32> = Vec::new();
            let mut lowest_weight: f32 = INFINITY;
            let mut edge_list: Vec<Edge> = Vec::new();
            let mut starting_edge_list: Vec<Edge> = Vec::new();
            if let (Some(default_nodes), Some(maps)) =
                (teleport_locations.get(place), opened_maps.get(place))
            {
                if maps.len() == 0 {
                    println!("There is no opened map currently");
                    continue;
                }
                let mut current_player_map = Vec::new();
                for map in maps.clone() {
                    current_player_map.push(map.owner);
                }
                // create edges
                for map in maps {
                    for default_node in default_nodes {
                        creating_edge(default_node, map, false, &mut starting_edge_list);
                        creating_edge(map, default_node, true, &mut edge_list);
                    }
                    for other_map in maps {
                        if map.owner != other_map.owner {
                            creating_edge(map, other_map, false, &mut edge_list);
                            creating_edge(other_map, map, false, &mut edge_list);
                        }
                    }
                }
                let all_permutation = find_permutation(current_player_map);
                // idea default -> 1st -> 2nd -> 3rd -> 4th
                // e.g. start default to 1st -> start 1st to 2nd ... -> from 2nd last -> last
                // reason for index equal -1 due to have to start without the player and
                // and use index + 1 to cancel the loop
                let mut fastest_path: &Vec<String> = &all_permutation[0];
                let mut final_tp_location = HashMap::new();

                for node in default_nodes {
                    for (_i, entry) in all_permutation.iter().enumerate() {
                        let mut index = 0;
                        let mut weight: f32 = 0.0;
                        let mut required_tp = HashMap::new();

                        while index < entry.len() {
                            let end = entry[index].clone();
                            if index == 0 {
                                for edge in &starting_edge_list {
                                    if edge.node1 == node && edge.node2.owner == end {
                                        weight += edge.weight;
                                        required_tp.insert(index, node);
                                        break;
                                    }
                                }
                            } else {
                                //TODO: check here
                                let mut tp_weight = INFINITY;
                                let mut back_to_being_node = &default_nodes[0];
                                for starting_node in default_nodes {
                                    for edge in &starting_edge_list {
                                        if edge.node1 == starting_node
                                            && edge.node2.owner == end
                                            && edge.weight + 5.0 < tp_weight
                                        {
                                            tp_weight = edge.weight;
                                            back_to_being_node = edge.node1;
                                            break;
                                        }
                                    }
                                }
                                let start = entry[index - 1].clone();
                                for edge in &edge_list {
                                    if edge.node1.owner == start && edge.node2.owner == end {
                                        if tp_weight < edge.weight {
                                            weight += tp_weight;
                                            required_tp.insert(index, back_to_being_node);
                                        } else {
                                            weight += edge.weight;
                                        }
                                        break;
                                    }
                                }
                            }

                            index += 1;
                        }
                        if weight < lowest_weight {
                            lowest_weight = weight;
                            final_tp_location = required_tp.clone();
                            fastest_path = entry;
                        }
                        total_weight.push(weight);
                    }
                }
                println!("****************");
                println!("order to go for {}", place);
                println!("start at {}", final_tp_location[&0].owner);
                final_tp_location.remove(&0);
                for (i, player) in fastest_path.iter().enumerate() {
                    if let Some(tp_location) = final_tp_location.get(&i) {
                        println!("Teleport back to {}", tp_location.owner);
                    }
                    println!("{}", player);
                }
                println!("****************");
            } else {
                println!("Error calculating due to entry's does not exist.");
                println!("Please try again.");
            }
        }
    }
}
// Yak T'el ( 16.6  , 12.4 )
// Yak T'el ( 12.3  , 12.3 )
