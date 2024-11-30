use std::collections::BinaryHeap;

pub fn d_1_1(data: String) {
    let mut max = 0;
    data.split("\n\n")
        .for_each(|elf| {
            let curr_sum: i32 = elf
            .lines()
            .map(|cal| cal.parse::<i32>().unwrap())
            .sum();
            if curr_sum > max {
                max = curr_sum;
            }
        });
    dbg!(max);
}

pub fn d_1_2(data: String) {
    let mut max_heap = BinaryHeap::new();
    let mut res = 0;
    data.split("\n\n")
        .for_each(|elf| {
            let curr_sum: i32 = elf
            .lines()
            .map(|cal| cal.parse::<i32>().unwrap())
            .sum();
            max_heap.push(curr_sum);
        });
    for _ in 0..3 {
        if let Some(x) = max_heap.pop() {
            res += x;
        };
    }
    dbg!(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d_1_1() {
        let data = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            .to_string();
        d_1_1(data);
    }

    #[test]
    fn test_d_1_2() {
        let data = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            .to_string();
        d_1_2(data);
    }
}
