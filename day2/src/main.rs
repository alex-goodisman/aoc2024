fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Missing input file");
    let lines = input.trim();

    let mut data: Vec<Vec<i64>> = Vec::new();

    for line in lines.split('\n') {
        let mut report: Vec<i64> = Vec::new();
        for level in line.trim().split(' ') {
            report.push(level.parse().expect("could not parse as integer"));
        }
        data.push(report);
    }

    part1(&data);
    part2(&data);
}

fn is_safe(report: &Vec<i64>) -> bool {
    let mut last_delta: i64 = 0;
    for (idx, level) in report.iter().enumerate() {
        if idx > 0 {
            let delta = level - report[idx - 1];
            let mag = delta.abs();
            if mag < 1 || mag > 3 || (idx > 1 && delta.signum() != last_delta.signum()){
                return false;
            }
            last_delta = delta;
        }
    }
    return true;
}

fn part1(data: &Vec<Vec<i64>>) {
    let mut count = 0;
    for report in data.iter() {
        if is_safe(report) {
            count = count + 1;
        }
    }
    println!("{}", count);
}


fn part2(data: &Vec<Vec<i64>>) {
    let mut count = 0;
    'counter: for report in data.iter() {
        if is_safe(report) {
            count = count + 1;
            continue;
        }
        // I tried for a while to come up with a closed-form (linear) solution
        // but the lists are short and there are a lot of edge cases.
        // e.g. if the first or last delta is in the wrong direction.
        for i in 0..report.len() {
            let mut spliced = report.clone();
            spliced.remove(i);
            if is_safe(&spliced) {
                count = count + 1;
                continue 'counter;
            }
        }
    }
    println!("{}", count);
}