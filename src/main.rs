use std::env;
mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day  = &args[1];
    let part = &args[2];
    parse::parse_input(day, part); 
}

// RUN COMMAND cargo run 1 1
// TEST COMMAND cargo test test_d_1_1 -- --nocapture