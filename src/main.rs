use std::io;

fn main() {
    println!("Hello, world!");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
        .expect("Error reading input");

        println!("{}", input);
        if input.trim() == "end" {
            println!("See ya!");
            break;
        }
    }
}
