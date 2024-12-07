use std::collections::HashSet;

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
        let (n_h, n_l, n_d) = trace(&lab, (h, l), dir);
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
    let mut obstacles_placed: HashSet<(usize, usize)> = HashSet::new();
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
        let (o_h, o_l, _) = get_next_dir((h, l, dir));
        if !visited.contains(&(o_h, o_l)) && o_h < lab.len() && o_l < lab[0].len() 
            && lab[o_h][o_l] == '.' {
            lab[o_h][o_l] = '#';
            let (mut c_h, mut c_l, mut c_d) = (h, l, dir);
            let mut loop_visited: HashSet<(usize, usize, usize)> = HashSet::new();
            loop {
                let (n_h, n_l, n_d) = trace(&lab, (c_h, c_l), c_d);
                if (n_h, n_l, n_d) == (c_h, c_l, c_d) {
                    break;
                }
                if loop_visited.contains(&(n_h, n_l, n_d)) 
                    || unique_positions.contains(&(n_h, n_l, n_d)) {
                    obstacles_placed.insert((o_h, o_l));
                    break;
                } else {
                    (c_h, c_l, c_d) = (n_h, n_l, n_d);
                    loop_visited.insert((c_h, c_l, c_d));
                }
            }
            lab[o_h][o_l] = '.';
        }
        let (n_h, n_l, n_d) = trace(&lab, (h, l), dir);
        if unique_positions.contains(&(n_h, n_l, n_d)) {
            break
        } else {
            (h, l, dir) = (n_h, n_l, n_d);
            unique_positions.insert((n_h, n_l, n_d));
            visited.insert((h, l));
        }
    }
    println!("{:?}", obstacles_placed.len());
}

fn trace(
    map: &Vec<Vec<char>>,
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

fn get_next_dir(curr: (usize, usize, usize)) -> (usize, usize, usize) {
    let (mut r_h, mut r_l, r_d) = curr;
    if curr.2 == 0 && r_h > 0 {
        r_h -= 1;
    } else if curr.2 == 2 {
        r_h += 1;
    } else if curr.2 == 3 && r_l > 0 {
        r_l -= 1;
    } else if curr.2 == 1 {
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