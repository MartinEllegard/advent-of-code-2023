#![feature(test)]

extern crate test;

#[bench]
fn name(b: &mut test::Bencher) {
    let input = include_str!("input1.txt");
    b.iter(|| part2(input))
}

fn main() {
    let input = include_str!("input1.txt");
    let result = part2(input);
    println!("result is: {:?}", result);
}

fn part2(input: &str) -> u32 {
    input.lines().map(|line| extract(line)).sum()
}

fn extract(line: &str) -> u32 {
    let mut fixed_line = (0..line.len()).filter_map(|index| {
        let line_from_index = &line[index..];
        let result = if line_from_index.starts_with("one") {
            '1'
        } else if line_from_index.starts_with("two") {
            '2'
        } else if line_from_index.starts_with("three") {
            '3'
        } else if line_from_index.starts_with("four") {
            '4'
        } else if line_from_index.starts_with("five") {
            '5'
        } else if line_from_index.starts_with("six") {
            '6'
        } else if line_from_index.starts_with("seven") {
            '7'
        } else if line_from_index.starts_with("eight") {
            '8'
        } else if line_from_index.starts_with("nine") {
            '9'
        } else {
            line_from_index.chars().next().unwrap()
        };

        result.to_digit(10)
    });

    let first = fixed_line.next().expect("Must be a number");

    match fixed_line.last() {
        Some(last) => format!("{0}{1}", first, last),
        None => format!("{0}{1}", first, first),
    }
    .parse::<u32>()
    .unwrap()
}
