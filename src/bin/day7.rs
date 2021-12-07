const FILE: &str = "inputs/day7.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let positions = std::fs::read_to_string(FILE)?
        .trim()
        .split(',')
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;

    let min_position = *positions.iter().min().unwrap();
    let max_position = *positions.iter().max().unwrap();
    let minimal_fuel_cost = |f: fn(i32) -> i32| {
        (min_position..=max_position)
            .map(|target| positions.iter().map(|p| f((p - target).abs())).sum())
            .min()
            .unwrap()
    };

    // Determine the horizontal position that the crabs can align to using the
    // least fuel possible. How much fuel must they spend to align to that
    // position?
    // Alternatively, the median should give the right answer.
    let part1: i32 = minimal_fuel_cost(|d| d);
    println!("Part 1: {}", part1);

    // Determine the horizontal position that the crabs can align to using the
    // least fuel possible so they can make you an escape route! How much fuel
    // must they spend to align to that position?
    // The sum of the integers from 1 to n is given by: n * (n + 1) / 2.
    let part2: i32 = minimal_fuel_cost(|d| d * (d + 1) / 2);
    println!("Part 2: {}", part2);

    Ok(())
}
