use std::collections::{HashMap, HashSet};

pub fn d_8_1(data: String) {
    let mut antenna_spots = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    let lines:  Vec<&str> = data.split("\n").collect();
    let height = lines.len();
    let length = lines[0].chars().count();

    for i in 0..height {
        let row_chars: Vec<char> = lines[i].chars().collect();
        for j in 0..length {
            if row_chars[j] == '.' {continue};
            antenna_spots
                .entry(row_chars[j])
                .or_insert_with(Vec::new)
                .push((i, j));
        }
    }

    antenna_spots.values().for_each(
        |positions| {
            antinodes.extend(get_antinode_from_positions(positions, height, length));
        }
    );

    println!("Number of antinodes: {:?}", antinodes.len());
}

fn get_antinode_from_positions(
    positions: &Vec<(usize, usize)>,
    height: usize,
    length: usize
) -> HashSet<(usize, usize)> {

    let mut result = HashSet::new();

    for i in 0..positions.len() {
        for j in i+1..positions.len() {
            if 2*(positions[i].0) >= positions[j].0
                && 2*(positions[i].1) >= positions[j].1
                && 2*(positions[i].1) - positions[j].1 < length
                {
                    result.insert(
                        (2*(positions[i].0)-positions[j].0,
                            2*(positions[i].1)-positions[j].1)
                    );
                } 
                if 2*(positions[j].0) - positions[i].0 < height
                && 2*(positions[j].1) >= positions[i].1
                && 2*(positions[j].1) - positions[i].1 < length
                {
                    result.insert(
                        (2*(positions[j].0)-positions[i].0,
                            2*(positions[j].1)-positions[i].1)
                    );
                } 
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> String {
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............".to_string()
    }

    #[test]
    fn test_d_8_1() {
        let data = get_data();
        d_8_1(data);
    }

    // #[test]
    // fn test_d_8_2() {
    //     let data = get_data().to_string();
    //     d_8_2(data);
    // }
}