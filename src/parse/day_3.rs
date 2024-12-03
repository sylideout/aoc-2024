pub fn d_3_1(data: String) {
    let mul_removed_substrings: Vec<&str> = data.split("mul").collect();
    let res: i32 = mul_removed_substrings
        .iter()
        .map(|x| compute_value_from_substring(x))
        .sum();
    println!("{:?}", res);
}

fn compute_value_from_substring(ss: &str) -> i32 {
    if ss.is_empty() {
        return 0;
    }

    let mut first_val = String::new();
    let mut second_val= String::new();
    let mut substring_chars = ss.chars();

    if substring_chars.next().unwrap() != '(' {
        return 0;
    }

    let next_char: char = substring_chars.next().unwrap();
    if ('0'..='9').contains(&next_char) {
        first_val.push(next_char);
    } else {
        return 0;
    }

    for _ in 0..3 {
        let next_char = substring_chars.next();
        match next_char {
            Some(x) if ('0'..='9').contains(&x) => first_val.push(x),
            Some(',') => break,
            None => return 0,
            Some(_) => return 0,
        }
    }

    let next_char: char = substring_chars.next().unwrap();
    if ('0'..='9').contains(&next_char) {
        second_val.push(next_char);
    } else {
        return 0;
    }

    for _ in 0..3 {
        let next_char = substring_chars.next();
        match next_char {
            Some(x) if ('0'..='9').contains(&x) => second_val.push(x),
            Some(')') => break,
            None => return 0,
            Some(_) => return 0,
        }
    }

    if first_val.chars().count() <= 3 && second_val.chars().count() <= 3 {
        first_val.parse::<i32>().unwrap() * second_val.parse::<i32>().unwrap()
    } else {
        return 0;
    }

}


pub fn d_3_2(data: String) {
    let mut res = 0;
    let dont_split_string: Vec<&str> = data.split("don't()").collect();

    if data.len() == 1 {
        res += get_sum_from_eligible_string(dont_split_string[0])
    } else {
        res += get_sum_from_eligible_string(dont_split_string[0]);
        for i in 1..dont_split_string.len() {
            let curr_string = dont_split_string[i];
            let do_split_string: Vec<&str> = curr_string.split("do()").collect();
            if do_split_string.len() > 1 {
                for j in 1..do_split_string.len() {
                    res += get_sum_from_eligible_string(do_split_string[j]);
                }
            }
        };
    }
    println!("{}", res);
}

fn get_sum_from_eligible_string(ss: &str) -> i32 {
    let mul_removed_substrings: Vec<&str> = ss.split("mul").collect();
    let total: i32 = mul_removed_substrings
        .iter()
        .map(|x| compute_value_from_substring(x))
        .sum();
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d_3_1() {
        let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            .to_string();
        d_3_1(data);
    }

    #[test]
    fn test_d_3_2() {
        let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"
            .to_string();
        d_3_2(data);
    }
}
