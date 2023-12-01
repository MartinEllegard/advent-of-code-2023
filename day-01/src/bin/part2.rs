#![feature(test)]
extern crate test;

#[bench]
fn name(b: &mut test::Bencher) {
    let input = include_str!("input1.txt");
    b.iter(|| part1(input))
}

fn main() {
    let input = include_str!("input1.txt");
    let _result = part1(input);
}

fn part1(input: &str) -> String {
    input.to_string()
}
