use aoc_utils::read_file;

fn main() {
    if let Ok(contents) = read_file("./input") {
        let numbers: Vec<i32> = contents
            .split("\n")
            .filter_map(|n| n.parse::<i32>().ok())
            .collect();

        println!("The sum of the fuel requirements is: {}", part1(&numbers));
    }
}

fn calc_fuel_requirement(n: &i32) -> i32 {
    return n / 3 - 2;
}

fn part1(numbers: &[i32]) -> i32 {
    return numbers.iter().map(calc_fuel_requirement).sum();
}

#[test]
fn test_part1() {
    assert_eq!(part1(&vec![12]), 2);
    assert_eq!(part1(&vec![14]), 2);
    assert_eq!(part1(&vec![1969]), 654);
    assert_eq!(part1(&vec![100756]), 33583);
}
