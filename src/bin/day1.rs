const FILE: &str = "inputs/day1.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let depths = std::fs::read_to_string(FILE)?
        .lines()
        .map(|line| line.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?;

    // How many measurements are larger than the previous measurement?
    let part1 = depths
        .windows(2)
        .filter(|window| window.last() > window.first())
        .count();

    println!("Part 1: {}", part1);

    // Consider sums of a three-measurement sliding window. How many sums are
    // larger than the previous sum?
    // Since the two middle elements are shared between windows, we still only
    // need to compare the first and last elements.
    let part2 = depths
        .windows(4)
        .filter(|window| window.last() > window.first())
        .count();

    println!("Part 2: {}", part2);

    Ok(())
}
