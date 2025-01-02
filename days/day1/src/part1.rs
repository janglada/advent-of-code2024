extern crate core;

use core::utils::donwload_puzzle;

#[derive(Debug, Clone, Copy, Default)]
struct Item {
    position: usize,
    value: u32,
}


pub async fn solve_part1()
{
    let input: String = donwload_puzzle(1).await.unwrap();

//     let input = r#"3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3"#;

    let mut left: Vec<Item> = vec![];
    let mut right: Vec<Item> = vec![];

    input
        .lines()
        .enumerate()
        .for_each(|(pos, line)| {
            let mut iter = line.split_whitespace();

            let v1 : u32 = iter.next().unwrap().parse().unwrap();
            let v2 : u32 = iter.next().unwrap().parse().unwrap();

            left.push(Item { position: pos , value: v1 });
            right.push(Item { position: pos , value: v2 });
        });


    left.sort_unstable_by_key(|item| (item.value, item.position));
    right.sort_unstable_by_key(|item| (item.value, item.position));



    let iter = left.iter().zip(right.iter());

    let res = iter.map(|(l,r)| {

         println!("{:?} {:?} --> {}" , l, r, l.position.abs_diff(r.position));
        (l.value.abs_diff(r.value))

    }).sum::<u32>();
    // println!("{:?}", left);
    // println!("{:?}", right);

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
