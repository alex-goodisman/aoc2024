fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Missing input file");

    let rows: Vec<_> = input.lines().map(|s| s.to_string()).collect();

    let grid = rows.iter().map(|row| row.chars().map(|c| c.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>();

    // I tried for a long time to one-line this without pattern matching I don't see a way without using [1..] which is cheating anyway.
    // reduce() is too aggressive with needing the result to be a reference not an owned, Vec, and fold() requires special casing the first entry
    // and Option seemed like the idiomatic way to do that
    let cols = grid.iter().fold(None, |acc: Option<Vec<String>>, row| match acc {
        None => Some(row.clone()),
        Some(existing) => Some(existing.iter().zip(row.iter()).map(|(s1, s2)| s1.to_string() + s2).collect()), 
    }).expect("no rows to transpose");

    // somewhat annoying that this has to be mutable while we build it. 
    let major_diags = grid.iter().fold(None, |acc: Option<Vec<String>>, row| match acc {
        None => Some(row.clone()),
        Some(existing) => {
            let mut vec = Vec::new();
            vec.push(row[0].clone());
            vec.extend(row[1..].iter().zip(existing.iter()).map(|(s_new, s_old)| s_old.to_string() + s_new));
            vec.extend_from_slice(&existing[row.len() - 1..]);
            Some(vec)
        }, 
    }).expect("no rows to transpose");

    // reversible iterators means we can avoid annoying index offset math.
    let minor_diags = grid.iter().fold(None, |acc: Option<Vec<String>>, row| match acc {
        None => Some(row.iter().rev().map(|s| s.clone()).collect()),
        Some(existing) => {
            let mut vec = Vec::new();
            vec.push(row[row.len() - 1].clone());
            vec.extend(row[0..row.len() - 1].iter().rev().zip(existing.iter()).map(|(s_new, s_old)| s_old.to_string() + s_new));
            vec.extend_from_slice(&existing[row.len() - 1..]);

            Some(vec)
        }, 
    }).expect("no rows to transpose");

    part1(&[&rows, &cols, &major_diags, &minor_diags]);

    part2(rows.len(), cols.len(), &major_diags, &minor_diags);
}

fn part1(vecs: &[&Vec<String>]) {
    let regexes = [r"XMAS", r"SAMX"].iter().map(|p| regex::Regex::new(p).expect("couldn't compile regex")).collect::<Vec<_>>();

    let count = vecs.iter().map(|v| v.iter()).flatten().map(|s| regexes.iter().map(|r| r.find_iter(s).count())).flatten().reduce(|a, b| a + b).expect("no matches");
    println!("{}", count);
}

fn part2(rows: usize, cols: usize, major_diags: &Vec<String>, minor_diags: &Vec<String>) {
    let regexes = [r"MAS", r"SAM"].iter().map(|p| regex::Regex::new(p).expect("couldn't compile regex")).collect::<Vec<_>>();
    
    let hm= major_diags
        .iter()
        .enumerate()
        .map(|(idx, diag)| regexes
            .iter()
            .map(move |r| r
                .find_iter(diag)
                .map(move |m| (idx, m.start())))
            .flatten())
        .flatten()
        .fold(std::collections::HashMap::new(), |mut map, (idx, offset)| {
            // idx is which major diagonal we're on. offset is the offset within that diagonal to the start of the match.
            // major diagonals are counted up and right from the bottom left corner, and proceed downward and rightward
            // therefore, the row index is equal to the max row index minus the index plus the offset (plus one since we want the middle
            // of a three-letter word: (rows - 1) - idx + offset + 1), assuming we count downward.
            // Because we only include the portion of the diagonals that are in the grid, we clamp the lower bound at 0, since that's where all
            // diagonals in the 2nd half start.
            let r = rows - std::cmp::min(idx, rows - 1) + offset;
            // the column index counts from the left, but we might start offset to the right if we're past the half. clamp works here too.
            let c = std::cmp::max(idx, rows - 1) + offset + 2 - rows; // order fixed to prevent underflow
            map.insert((r, c), true);
            map
        });

        println!("");
    // do the same for minor diags except check against the map
    let count = minor_diags
        .iter()
        .enumerate()
        .map(|(idx, diag)| regexes
            .iter()
            .map(move |r| r
                .find_iter(diag)
                .map(move |m| (idx, m.start())))
            .flatten())
        .flatten()
        .filter(|(idx, offset)| {
            // minor diag are counted up and left from the bottom right corner and proceed downard and leftward.
            // therefore, we count row index exactly the same as the major diag.
            let r = rows - std::cmp::min(idx, &(rows - 1)) + offset;
            // column index is almost the same, except we are counting right to left from the rightmost (cols - 1) index.
            // so we subtract the major computation from that value.
            let c = cols  + rows - 3 - std::cmp::max(idx, &(rows - 1)) - offset; // order fixed to prevent underflow
            *hm.get(&(r, c)).unwrap_or(&false)
        })
        .count();

    println!("{}", count);

    
}
