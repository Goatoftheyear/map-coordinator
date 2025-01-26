use std::io;
use std::collections::HashMap;

fn main() {
    println!("Hello!");
    let mut opened_maps:HashMap<String, String> = HashMap::new();
    //TODO: set up fix tp location
    loop {
        println!("Remember you can right click to the prompt to paste your latest copy entry");
        println!("Add person's coordinates. E.g. Kay Memoryland(17.5 , 46.3)");
        println!("Delete by typing just the name. E.g. Kay");
        //TODO: decide when and where the node calculation should be
        let mut input = String::new();
        io::stdin().read_line(&mut input)
        .expect("Error reading input");
        input = input.trim_end().to_string();
        println!("{:?}", input);
        //TODO: add entries in Map/dictionary
        let mut _splitted: &Vec<String> = &input.splitn(2," ").map(|s| s.to_string()).collect();
        println!("{:?}", _splitted);
        if _splitted.len() == 1{
            opened_maps.remove(&input);
        } else {
        opened_maps.insert(_splitted[0].clone(), _splitted[1].clone());
    }
    println!("{:?}",opened_maps);
        if input.trim() == "end" {
            println!("See ya!");
            break;
        }
    }
}
// Yak T'el ( 16.6  , 12.4 )
// Yak T'el ( 12.3  , 12.3 )
//assumption after testing is 2.6 per 1.0 grid