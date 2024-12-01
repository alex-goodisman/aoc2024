fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Missing input file");
    let lines = input.trim();


    let mut vec_l: Vec<i64> = Vec::new();
    let mut vec_r: Vec<i64> = Vec::new();

    for pair in lines.split('\n') {
        let idx = pair.find(' ').expect("space not found");
        let idx_r = pair.rfind(' ').expect("space not found at end");
        vec_l.push(pair[..idx].parse().expect("failed to parse L as string"));
        vec_r.push(pair[(idx_r + 1)..].trim().parse().expect("failed to parse R as string"));
    }

    part1(&mut vec_l, &mut vec_r);
    part2(&vec_l, &vec_r);


}

fn part1(vec_l: &mut Vec<i64>, vec_r: &mut Vec<i64>) {
    vec_l.sort();
    vec_r.sort();

    let mut total = 0;

    for (i, l) in vec_l.iter().enumerate() {
        let r = vec_r[i];
       total = total + (r - l).abs();
    }

    println!("{}", total);
    
}

// wanted to implement radix sort for part 1, didn't, and regretted it.
// would have had all the buckets already too, oh well.
fn part2(vec_l: &Vec<i64>, vec_r: &Vec<i64>) {
    let mut set_r: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();

    for r in vec_r {
        set_r.insert(*r, set_r.get(r).unwrap_or(&0) + 1);
    }

    let mut total = 0;

    for l in vec_l {
        total = total + (l * set_r.get(&l).unwrap_or(&0));
    }

    println!("{}", total);


}