fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Missing input file");

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let matcher = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Couldn't compile regex");

    let result = matcher
        .captures_iter(&input)
        .map(|m| m[1].parse::<i64>().expect("first match not numeric") * m[2].parse::<i64>().expect("second match not numeric"))
        .reduce(|m,n| m + n)
        .expect("no results");

    println!("{}", result);
}

fn part2(input: &str) {
    let matcher = regex::Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|((do|don't)\(\))").expect("Couldn't compile regex");

    let result = matcher
        .captures_iter(&input)
        .fold((0,true), |(sum, enabled), c| match (c.get(5).map(|m| m.as_str()), enabled) {
            (Some("do"), _) => (sum, true),
            (Some("don't"), _) => (sum, false),
            (None, true) => (sum + (c[2].parse::<i64>().expect("first match not numeric") * c[3].parse::<i64>().expect("second match not numeric")), enabled),
            _ => (sum, enabled),
        }).0;

    println!("{}", result);
}
