extern crate core;

use std::ops::Mul;
use core::utils::donwload_puzzle;
use std::slice::Iter;
use itertools::Itertools;


pub async fn solve_part2() {
    let input: String = donwload_puzzle(1).await.unwrap();
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];

    input
        .lines()
        .enumerate()
        .for_each(|(pos, line)| {
            let mut iter = line.split_whitespace();

            let v1 : usize = iter.next().unwrap().parse().unwrap();
            let v2 : usize = iter.next().unwrap().parse().unwrap();

            left.push(v1);
            right.push(v2);
        });


    let frequencies = right.iter().counts();


    let res = left.iter().map(|v| {
        frequencies.get(v).unwrap_or(&0).mul(v)
    }).sum::<usize>();

    // println!("{:?}", left);
    // println!("{:?}", right);

    println!("{:?}", res);
}
