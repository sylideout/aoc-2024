use std::{collections::HashMap, fs};
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

fn generate_id_fn_mapping() -> HashMap<String, fn(String)> {
    HashMap::from([
        ("11".to_string(), day_1::d_1_1 as fn(String)),
        ("12".to_string(), day_1::d_1_2 as fn(String)),
        ("21".to_string(), day_2::d_2_1 as fn(String)),
        ("22".to_string(), day_2::d_2_2 as fn(String)),
        ("31".to_string(), day_3::d_3_1 as fn(String)),
        ("32".to_string(), day_3::d_3_2 as fn(String)),
        ("41".to_string(), day_4::d_4_1 as fn(String)),
        ("42".to_string(), day_4::d_4_2 as fn(String)),
        ("51".to_string(), day_5::d_5_1 as fn(String)),
        ("52".to_string(), day_5::d_5_2 as fn(String)),
        ("61".to_string(), day_6::d_6_1 as fn(String)),
        ("62".to_string(), day_6::d_6_2 as fn(String)),
        ("71".to_string(), day_7::d_7_1 as fn(String)),
        ("72".to_string(), day_7::d_7_2 as fn(String)),
        ("81".to_string(), day_8::d_8_1 as fn(String)),
        // ("31".to_string(), day_3::d_3_1 as fn(String)),
    ])
}

pub fn parse_input(day: &str, part:&str) {
    let day_part_fn_map: HashMap<String, fn(String)> = generate_id_fn_mapping();
    let unique_qn_id = format!("{day}{part}");
    let file_path = format!("src/input/day_{day}.txt");
    let data = fs::read_to_string(file_path)
        .expect("Wrong input");
    day_part_fn_map[&unique_qn_id](data);
}