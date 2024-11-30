use std::{collections::HashMap, fs};
mod day_1;

fn generate_id_fn_mapping() -> HashMap<String, fn(String)> {
    HashMap::from([
        ("11".to_string(), day_1::d_1_1 as fn(String)),
        ("12".to_string(), day_1::d_1_2 as fn(String)),
        // ("21".to_string(), day_2::d_2_1 as fn(String)),
        // ("22".to_string(), day_2::d_2_2 as fn(String)),
        // ("31".to_string(), day_3::d_3_1 as fn(String)),
    ])
}

pub fn parse_input(day: &str, part:&str) {
    let day_part_fn_map: HashMap<String, fn(String)> = generate_id_fn_mapping();
    let unique_qn_id = format!("{day}{part}");
    let file_path = format!("src/input/day_1_1.txt");
    let data = fs::read_to_string(file_path)
        .expect("Wrong input");
    day_part_fn_map[&unique_qn_id[..]](data);
}