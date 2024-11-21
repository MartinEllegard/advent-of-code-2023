use day_01::*;

fn main() -> miette::Result<()> {
    let input1 = include_str!("../../input1.txt");
    let input2 = include_str!("../../input2.txt");

    let result1 = part1::process(input1)?;
    let result2 = part2::process(input2)?;

    println!("part 1: {}", result1);
    println!("part 2: {}", result2);

    Ok(())
}
