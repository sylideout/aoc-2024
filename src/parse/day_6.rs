use std::collections::{HashMap, HashSet};

pub fn d_6_1(data: String) {
    let mut lab: Vec<Vec<char>> = Vec::new();
    let mut unique_positions: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let (mut h, mut l, mut dir) = (0, 0, 0);

    data.lines()
        .for_each(
            |line| {
                let row: Vec<char> = line.chars().collect();
                lab.push(row);
            }
        );
    
    for i in 0..lab.len() {
        for j in 0..lab[0].len() {
            if lab[i][j] == '^' {
                (h, l) = (i, j);
                unique_positions.insert((h, l, dir));
                visited.insert((h, l));
            }
        }
    }

    loop {
        let (n_h, n_l, n_d) = trace(&mut lab, (h, l), dir);
        if unique_positions.contains(&(n_h, n_l, n_d)) {
            break
        } else {
            (h, l, dir) = (n_h, n_l, n_d);
            unique_positions.insert((n_h, n_l, n_d));
            visited.insert((n_h, n_l));
        }
    }

    println!("{:?}", visited.len());
}

pub fn d_6_2(data: String) {
    let mut lab: Vec<Vec<char>> = Vec::new();
    let mut unique_positions: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    let mut historical_routes
        : HashMap<(usize, usize, usize), HashSet<(usize, usize, usize)>> = HashMap::new();
    let mut combinations = 0;
    let (mut h, mut l, mut dir) = (0, 0, 0);

    data.lines()
        .for_each(
            |line| {
                let row: Vec<char> = line.chars().collect();
                lab.push(row);
            }
        );
    
    for i in 0..lab.len() {
        for j in 0..lab[0].len() {
            if lab[i][j] == '^' {
                (h, l) = (i, j);
                unique_positions.insert((h, l, dir));
                visited.insert((h, l));
            } else if lab[i][j] == '#' {
                obstacles.insert((i, j));
            }
        }
    }

    loop {
        let (n_h, n_l, n_d) = trace(&mut lab, (h, l), dir);
        if unique_positions.contains(&(n_h, n_l, n_d)) {
            break
        } else {
            (h, l, dir) = (n_h, n_l, n_d);

            if unique_positions.contains(&(n_h, n_l, (n_d+3)%4)) {
                historical_routes
                    .entry((n_h, n_l, (n_d+3)%4))
                    .or_insert_with(HashSet::new)
                    .insert((n_h, n_l, n_d));
            }
            unique_positions.insert((n_h, n_l, n_d));
            visited.insert((n_h, n_l));
        }
    }
    unique_positions.iter().for_each(
        |p| {
        if valid_loop(*p, &unique_positions, &visited, &obstacles, &historical_routes) {combinations+=1}
        }
    );

    println!("{:?}", combinations);
}

fn trace(
    map: &mut Vec<Vec<char>>,
    idx: (usize, usize),
    dir: usize
) -> (usize, usize, usize) {
    let (h, l, _) = get_next_dir((idx.0, idx.1, dir));

    if h < map.len() && l < map[0].len() {
        if map[h][l] == '#' {
            return (idx.0, idx.1, (dir+1)%4);
        } else {
            return (h, l, dir);
        }
    } else {
        return (idx.0, idx.1, dir);
    }
}

fn valid_loop(
    p: (usize, usize, usize),
    unique_positions: &HashSet<(usize, usize, usize)>,
    visited: &HashSet<(usize, usize)>,
    obstacles: &HashSet<(usize, usize)>,
    historical_routes: &HashMap<(usize, usize, usize), HashSet<(usize, usize, usize)>>
) -> bool {
    let (o_h, o_l, _) = get_next_dir((p.0, p.1, p.2));
    let mut x = obstacles.clone();
    if obstacles.contains(&(o_h, o_l)) {
        return false;
    } else {
        x.insert((o_h, o_l));
    }

    if historical_routes.contains_key(&(p.0, p.1, p.2)) 
        && historical_routes.get(&(p.0, p.1, p.2)).unwrap().contains(&(p.0, p.1, (p.2+1)%4)){
        return false;
    }

    if !can_go_next((p.0, p.1, p.2)) {
        return false;
    }

    let (mut n_h, mut n_l, mut n_d) = get_next_dir((p.0, p.1, (p.2+1)%4));
    loop {
        if !can_go_next((n_h, n_l, n_d)) {
            return false;
        }
        if x.contains(&(n_h, n_l)) {
            // println!("OBSTACLE{:?}", (n_h, n_l, n_d));
            (n_h, n_l, n_d) = get_prev_dir((n_h, n_l, (n_d+1)%4));
            // println!("WRONG{:?}", (n_h, n_l, n_d));
            continue
        }
        if unique_positions.contains(&(n_h, n_l, n_d)) {
            return true;
        }
        if visited.contains(&(n_h, n_l)) {
            (n_h, n_l, n_d) = get_next_dir((n_h, n_l, n_d));
            continue
        }
        return false;
    }
}

fn get_next_dir(curr: (usize, usize, usize)) -> (usize, usize, usize) {
    let (mut r_h, mut r_l, r_d) = curr;
    if curr.2 == 0 {
        r_h -= 1;
    } else if curr.2 == 2 {
        r_h += 1;
    } else if curr.2 == 3 {
        r_l -= 1;
    } else if curr.2 == 1 {
        r_l += 1;
    }
    (r_h, r_l, r_d)
}

fn can_go_next(curr: (usize, usize, usize)) -> bool {
    if curr.2 == 0 && 0 < curr.0 {
        return true;
    } else if curr.2 == 2 && curr.0 < 130 {
        return true;
    } else if curr.2 == 3 && 0 < curr.1 {
        return true;
    } else if curr.2 == 1 && curr.1 < 130 {
        return true;
    }
    false
}

fn get_prev_dir(curr: (usize, usize, usize)) -> (usize, usize, usize) {
    let (mut r_h, mut r_l, r_d) = curr;
    if curr.2 == 0 {
        r_h -= 1;
        r_l += 1;
    } else if curr.2 == 2 {
        r_h += 1;
        r_l -= 1;
    } else if curr.2 == 3 {
        r_h -= 1;
        r_l -= 1;
    } else if curr.2 == 1 {
        r_h += 1;
        r_l += 1;
    }
    (r_h, r_l, r_d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d_6_1() {
        let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string();
        d_6_1(data);
    }

    #[test]
    fn test_d_6_2() {
        let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string();
        d_6_2(data);
    }
}