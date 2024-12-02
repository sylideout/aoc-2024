pub fn d_2_1(data: String) {
    let mut res = 0;
    data
        .lines()
        .for_each(
            |report| {
                if validate_order(report
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect()) {
                        res += 1;
                    };
            });
    println!("{}", res);
}

fn validate_order(report: Vec<i32>) -> bool {
    let mut index = 1;
    match (report[0], report[1]) {
        (x, y) if x > y => {
            while index < report.len() {
                let a = report[index-1];
                let b = report[index];
                if b >= a || (a - b) > 3 {
                    return false;
                }
                index += 1;
            };
            true
        }
        (x, y) if x < y => {
            while index < report.len() {
                let a = report[index-1];
                let b = report[index];
                if b <= a || (b - a) > 3 {
                    return false;
                }
                index += 1;
            };
            true
        }
        (_, _) => {
            false
        }
    }
}

pub fn d_2_2(data: String) {
    let mut res = 0;
    data
        .lines()
        .for_each(
            |report| {
                if validate_order_with_dampener(report
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect()) {
                        res += 1;
                    };
            });
    println!("{}", res);
}

fn validate_order_with_dampener(report: Vec<i32>) -> bool {
    let mut index = 0;
    while index < report.len() {
        let mut x = report.clone();
        x.remove(index);
        if validate_order(x) {
            return true;
        }
        index += 1;
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d_2_1() {
        let data = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string();
        d_2_1(data);
    }

    #[test]
    fn test_d_2_2() {
        let data = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string();
        d_2_2(data);
    }
}
