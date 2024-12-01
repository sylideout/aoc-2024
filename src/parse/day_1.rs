use std::collections::{BinaryHeap, HashMap};

pub fn d_1_1(data: String) {
    let mut first_column = BinaryHeap::new();
    let mut second_column = BinaryHeap::new();
    let mut res = 0;
    data.split("\n")
        .for_each(|pair: &str| {
            let parts: Vec<&str> = pair
            .split_whitespace()
            .collect();
            if let Some(&first) = parts.get(0) {
                if let Ok(num) = first.parse::<i32>() {
                    first_column.push(num);
                }
            }
            if let Some(&second) = parts.get(1) {
                if let Ok(num) = second.parse::<i32>() {
                    second_column.push(num);
                }
            }
        });
    for _ in 0..first_column.len() {
        if let Some(n1) = first_column.pop() {
            if let Some(n2) = second_column.pop() {
                res += (n1 - n2).abs();
            }
        }
    };
    println!("The result is {res}");
}

pub fn d_1_2(data: String) {
    let mut item_counts = HashMap::new();
    let mut first_column: Vec<&str> = Vec::new();
    let mut res = 0;
    data.split("\n")
        .for_each(|pair: &str| {
            let parts: Vec<&str> = pair
            .split_whitespace()
            .collect();
            if let Some(&first) = parts.get(0) {
                first_column.push(first);
            }
            if let Some(&second) = parts.get(1) {
                *item_counts.entry(second).or_insert(0) += 1;
            }
        });
    first_column.into_iter().for_each(
        |item| {
            if item_counts.contains_key(item) {
                res += item_counts[item] * (item.parse::<i32>().unwrap());
            }
        }
    );
    println!("The result is {res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d_1_1() {
        let data = "3   4
4   3
2   5
1   3
3   9
3   3"
            .to_string();
        d_1_1(data);
    }

    #[test]
    fn test_d_1_2() {
        let data = "3   4
4   3
2   5
1   3
3   9
3   3"
            .to_string();
        d_1_2(data);
    }
}
