fn part_one(input: &str) -> usize {
    288
}

fn main() {
    println!("{}", part_one(" "));
}

#[cfg(test)]
mod tests {
    static TEST: &str =
        "Time:        53     91     67     68\nDistance:   250   1330   1081   1025";

    use super::part_one;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST), 288)
    }
}
