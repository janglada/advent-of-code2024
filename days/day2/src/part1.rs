extern crate core;

use core::utils::donwload_puzzle;

pub async fn solve_part1() {
    let input: String = donwload_puzzle(2).await.unwrap();
//     let input: &str = r#"7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9"#;
    let res = input
        .lines()

        .map(| line| {
            let mut iter = line.split_whitespace();

            let mut  previous: u32   = iter.next().unwrap().parse().unwrap();
            let mut next: u32       = iter.next().unwrap().parse().unwrap();
            let mut diff = next.abs_diff(previous);
            if diff > 3 || diff == 0 {
                0
            } else {
                let is_incrementing = previous < next;
                while let Some(level_str) = iter.next() {
                    previous = next;
                    next = level_str.parse().unwrap();

                    if is_incrementing && next < previous {
                        return 0;
                    } else if !is_incrementing && next > previous {
                        return 0;
                    } else {
                        diff = next.abs_diff(previous);
                        if diff > 3 || diff == 0 {
                            return 0;
                        }
                    }
                }
                1
            }
        })
        .sum::<u32>();

    println!("{:?}", res);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;
    use std::collections::HashMap;

    #[test]
    fn masks1() {



    }
}
