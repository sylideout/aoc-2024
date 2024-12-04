pub fn d_4_1(data: String) {
    let mut puzzle: Vec<Vec<char>> = Vec::new();
    let mut res: usize = 0;
    data
        .lines()
        .for_each(
            |l| {
                puzzle.push(l.chars().collect());
            }
        );
    let length = puzzle.len();
    let height = puzzle[0].len();

    for i in 0..height {
        for j in 0..length {
            let curr_char  = puzzle[i][j];
            if curr_char != 'X' { continue;}
            res += number_of_xmas(i, j, height-3, length-3, &puzzle);
        }
    }
    println!("{}", res);
}

pub fn d_4_2(data: String) {
    let mut puzzle: Vec<Vec<char>> = Vec::new();
    let mut res: usize = 0;
    data
        .lines()
        .for_each(
            |l| {
                puzzle.push(l.chars().collect());
            }
        );
    let length = puzzle.len();
    let height = puzzle[0].len();

    for i in 0..height {
        for j in 0..length {
            let curr_char  = puzzle[i][j];
            if curr_char == 'A' 
                && 0 < i && i < height-1
                && 0 < j && j < length-1 
                && is_x_mas(i, j, &puzzle) {
                println!("ONE HERE: {:?}", (i, j));
                res += 1;
            }
        }
    }
    println!("{}", res);
}

fn number_of_xmas(
    idxh: usize,
    idxl: usize,
    h: usize,
    l: usize,
    mref: &Vec<Vec<char>>
) -> usize {
    let mut total = 0; 
    if idxl > 2 {
        if mref[idxh][idxl-1] == 'M' && mref[idxh][idxl-2] == 'A' && mref[idxh][idxl-3] == 'S' {
            total += 1;
        }
    }
    if idxh > 2 {
        if mref[idxh-1][idxl] == 'M' && mref[idxh-2][idxl] == 'A' && mref[idxh-3][idxl] == 'S' {
            total += 1;
        }
    }
    if idxh > 2 && idxl > 2 {
        if mref[idxh-1][idxl-1] == 'M' && mref[idxh-2][idxl-2] == 'A' && mref[idxh-3][idxl-3] == 'S' {
            total += 1;
        }
    }
    if idxh > 2 && idxl < l{
        if mref[idxh-1][idxl+1] == 'M' && mref[idxh-2][idxl+2] == 'A' && mref[idxh-3][idxl+3] == 'S' {
            total += 1;
        }
    }
    if idxl < l {
        if mref[idxh][idxl+1] == 'M' && mref[idxh][idxl+2] == 'A' && mref[idxh][idxl+3] == 'S' {
            total += 1;
        }
    }
    if idxh < h && idxl < l {
        if mref[idxh+1][idxl+1] == 'M' && mref[idxh+2][idxl+2] == 'A' && mref[idxh+3][idxl+3] == 'S' {
            total += 1;
        }
    }
    if idxh < h {
        if mref[idxh+1][idxl] == 'M' && mref[idxh+2][idxl] == 'A' && mref[idxh+3][idxl] == 'S' {
            total += 1;
        }
    }
    if idxh < h && idxl > 2 {
        if mref[idxh+1][idxl-1] == 'M' && mref[idxh+2][idxl-2] == 'A' && mref[idxh+3][idxl-3] == 'S' {
            total += 1;
        }
    }
    total
}


fn is_x_mas(
    idxh: usize,
    idxl: usize,
    mref: &Vec<Vec<char>>
) -> bool {
    let mut is_valid_left_diagonal = false;
    let mut is_valid_right_diagonal = false;

    if mref[idxh-1][idxl-1] == 'M' && mref[idxh+1][idxl+1] == 'S' {
        is_valid_left_diagonal = true;
    } else if mref[idxh-1][idxl-1] == 'S' && mref[idxh+1][idxl+1] == 'M' {
        is_valid_left_diagonal = true;
    }

    if mref[idxh-1][idxl+1] == 'M' && mref[idxh+1][idxl-1] == 'S' {
        is_valid_right_diagonal = true;
    } else if mref[idxh-1][idxl+1] == 'S' && mref[idxh+1][idxl-1] == 'M' {
        is_valid_right_diagonal = true;
    }

    is_valid_left_diagonal && is_valid_right_diagonal
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d_4_1() {
        let data = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string();
        d_4_1(data);
    }

    #[test]
    fn test_d_4_2() {
        let data = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string();
        d_4_2(data);
    }
}
