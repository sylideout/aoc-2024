use std::collections::{HashSet, HashMap};
use std::convert::TryInto;

pub fn d_5_1(data: String) {
    let mut res: usize = 0;
    let split_data: [&str; 2] = data
        .split("\n\n")
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let page_order_map = split_create_map(split_data[0]);
    let updates: Vec<&str> = split_data[1].split("\n").collect();

    updates
        .into_iter()
        .for_each(
            |f| {
                let mut valid_update = true;
                let update: Vec<&str> = f.split(",").collect();
                let mut parsed = HashSet::new();
                for i in 0..update.len() {
                    if parsed.intersection(
                        &page_order_map
                            .get(update[i])
                            .unwrap_or(&HashSet::new())
                        )
                        .next()
                        .is_some() {
                                valid_update = false;
                                break;
                            }
                    parsed.insert(update[i]);
                }
                if valid_update {
                    res += (update[(update.len()-1)/2]).parse::<usize>().unwrap();
                }
            }
        );
    println!("The result is: {}", res);
}

pub fn d_5_2(data: String) {
    let mut res: usize = 0;
    let split_data: [&str; 2] = data
        .split("\n\n")
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let page_order_map = split_create_map(split_data[0]);
    let updates: Vec<&str> = split_data[1].split("\n").collect();

    updates
        .into_iter()
        .for_each(
            |f| {
                let mut is_valid = true;
                let update: Vec<&str> = f.split(",").collect();
                let mut parsed = HashSet::new();
                for i in 0..update.len() {
                    if parsed.intersection(
                        &page_order_map
                            .get(update[i])
                            .unwrap_or(&HashSet::new())
                        )
                        .next()
                        .is_some() {
                                is_valid = false;
                                break;
                            }
                    parsed.insert(update[i]);
                }
                if !is_valid {
                    let mut count_map: Vec<(&str, usize)> = Vec::new();
                    let incorrect_update: HashSet<&str> = update.clone().into_iter().collect();
                    for i in 0..update.len() {
                        count_map.push((
                            update[i],
                            incorrect_update.intersection(
                                &page_order_map
                                    .get(update[i])
                                    .unwrap_or(&HashSet::new())
                            ).count())
                        )
                    }
                    count_map.sort_by(|a, b| b.1.cmp(&a.1));
                    println!("DBG {:?}", count_map);
                    res += (count_map[(count_map.len()-1)/2].0).parse::<usize>().unwrap();
                }
            }
        );
    println!("The result is: {}", res);
}


fn split_create_map(s: &str) -> HashMap<&str, HashSet<&str>> {
    let lines: Vec<&str> = s.split("\n").collect();
    let mut page_order_map = HashMap::new();
    lines
        .into_iter()
        .for_each(
            |pair| {
                let parts: [&str; 2] = pair
                    .split('|')
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                page_order_map
                    .entry(parts[0])
                    .or_insert_with(HashSet::new)
                    .insert(parts[1]);
            }
        );
    page_order_map
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d_5_1() {
        let data = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .to_string();
        d_5_1(data);
    }

    #[test]
    fn test_d_5_2() {
        let data = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .to_string();
        d_5_2(data);
    }
}
