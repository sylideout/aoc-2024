pub fn d_7_1(data: String) {
    let mut output:  Vec<usize> = Vec::new();
    let mut input: Vec<Vec<usize>> = Vec::new();
    let mut total: usize = 0;

    data
        .lines()
        .for_each(
            |line| {
                let io: Vec<&str> = line.split(": ").collect();
                output.push(io[0].parse().unwrap());
                input.push(io[1].split(" ").map(|s| s.parse::<usize>().unwrap()).collect());
            }
        );

    for i in 0..output.len() {
        if can_operate(input[i][0], output[i], 1, 'x', &input[i]) 
        || can_operate(input[i][0], output[i], 1, '+', &input[i]) {
            total += output[i];
        }
    }
    println!("Result is {total}");
}

pub fn d_7_2(data: String) {
    let mut output:  Vec<usize> = Vec::new();
    let mut input: Vec<Vec<usize>> = Vec::new();
    let mut total: usize = 0;

    data
        .lines()
        .for_each(
            |line| {
                let io: Vec<&str> = line.split(": ").collect();
                output.push(io[0].parse().unwrap());
                input.push(io[1].split(" ").map(|s| s.parse::<usize>().unwrap()).collect());
            }
        );

    for i in 0..output.len() {
        if can_operate_2(input[i][0], output[i], 1, 'x', &input[i]) 
        || can_operate_2(input[i][0], output[i], 1, '+', &input[i])
        || can_operate_2(input[i][0], output[i], 1, '|', &input[i]) {
            total += output[i];
        }
    }
    println!("Result is {total}");
}

fn can_operate(
    curr: usize,
    target: usize,
    idx: usize,
    operator: char,
    ref_input: &Vec<usize>
) -> bool {

    if idx == ref_input.len() {
        return curr == target;
    }
    if curr > target {
        return false;
    }

    let mut next_value = curr;

    if operator == 'x' {
        next_value *= ref_input[idx]
    } else if operator == '+' {
        next_value += ref_input[idx]
    }

    return can_operate(next_value, target, idx+1, 'x', ref_input) 
        || can_operate(next_value, target, idx+1, '+', ref_input);
}

fn can_operate_2(
    curr: usize,
    target: usize,
    idx: usize,
    operator: char,
    ref_input: &Vec<usize>
) -> bool {

    if idx == ref_input.len() {
        return curr == target;
    }
    if curr > target {
        return false;
    }

    let mut next_value = curr;

    if operator == 'x' {
        next_value *= ref_input[idx]
    } else if operator == '+' {
        next_value += ref_input[idx]
    } else if operator == '|' {
        let combined_str = format!("{}{}", next_value, ref_input[idx]);
        next_value = combined_str.parse::<usize>().unwrap();
    }

    return can_operate_2(next_value, target, idx+1, 'x', ref_input) 
        || can_operate_2(next_value, target, idx+1, '+', ref_input)
        || can_operate_2(next_value, target, idx+1, '|', ref_input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d_7_1() {
        let data = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
            .to_string();
        d_7_1(data);
    }

    #[test]
    fn test_d_7_2() {
        let data = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
            .to_string();
        d_7_2(data);
    }
}