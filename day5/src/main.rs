fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Missing input file");

    let regex = regex::Regex::new(r"\r?\n\r?\n").expect("couldn't compile regex");

    let input_parts = regex.split(&input).collect::<Vec<_>>();

    let rules_lines = input_parts[0].lines().collect::<Vec<_>>();
    let pages_lines = input_parts[1].lines().collect::<Vec<_>>();

    let rules = rules_lines.iter().fold(std::collections::HashSet::new(), |mut hm, rule_str| {
        let nums = rule_str.split('|').map(|n| n.parse().expect("couldn't parse rule as number")).collect::<Vec<_>>();
        hm.insert((nums[0], nums[1]));
        hm
    });

    let page_lists = pages_lines.iter().map(|page_line| page_line.split(',').map(|n| n.parse().expect("couldn't parse page as number")).collect()).collect();

    let not_ordered = part1(&rules, &page_lists);

    part2(&rules, &not_ordered);
}

fn part1(rules: &std::collections::HashSet<(i64, i64)>, page_lists: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let (ordered, not_ordered): (Vec<&Vec<i64>>, Vec<Vec<i64>>) = page_lists.iter().fold((Vec::new(), Vec::new()), |(mut ordered, mut not_ordered), pages| {
        let pairs = pages.iter().enumerate().map(|(idx, page)| pages[idx + 1..].iter().map(|other_page| (*other_page, *page))).flatten().collect::<Vec<_>>();

        if pairs.iter().any(|page_pair| rules.contains(page_pair)) {
            not_ordered.push(pages.to_vec());
        } else {
            ordered.push(pages);
        }

        (ordered, not_ordered)
    });

    let result: i64 = ordered.iter().map(|pages| pages[(pages.len() - 1) / 2]).sum();

    println!("{}", result);

    not_ordered
}

fn part2(rules: &std::collections::HashSet<(i64, i64)>, page_lists: &Vec<Vec<i64>>) {
    let result: i64 = page_lists
        .iter()
        .map(|pages| {
            // build graph map<node, (in-set, out-set)>
            let graph = pages
                .iter()
                .enumerate()
                .map(|(idx, page_a)| pages[idx+1..].iter().map(move |page_b| (page_a, page_b)))
                .flatten()
                .fold(std::collections::HashMap::new(), |mut graph, (page_a, page_b)| {
                    let has_edge = rules.contains(&(*page_a, *page_b));
                    let has_reverse_edge = rules.contains(&(*page_b, *page_a));
                    let entry_a = graph.entry(*page_a).or_insert((std::collections::HashSet::new(), std::collections::HashSet::new()));
                    if has_edge {
                        entry_a.1.insert(*page_b);
                    }
                    if has_reverse_edge {
                        entry_a.0.insert(*page_b);
                    }
                    
                    let entry_b = graph.entry(*page_b).or_insert((std::collections::HashSet::new(), std::collections::HashSet::new()));
                    if has_edge {
                        entry_b.0.insert(*page_a);
                    }
                    if has_reverse_edge {
                        entry_b.1.insert(*page_a);
                    }

                    graph
                });

            // find start node. because we know there must be a correct order, every node must have occurred at some point, so we know the hash sets exist
            let start = graph.iter().find(|(_, (in_set, _))| in_set.is_empty()).expect("couldn't find start node").0;

            let mut result_vec = Vec::new();
            let seen_set = std::collections::HashSet::new();
            topological_sort(&graph, start, seen_set, &mut result_vec);
            result_vec
        })
        .map(|pages| pages[(pages.len() - 1) / 2])
        .sum();

    println!("{}", result);
}

// Kahn's algorithm-ish
fn topological_sort(graph: &std::collections::HashMap<i64, (std::collections::HashSet<i64>, std::collections::HashSet<i64>)>, n: &i64, mut seen_set: std::collections::HashSet<i64>, result_vec: &mut Vec<i64>) {
    result_vec.push(*n);
    seen_set.insert(*n);

    // because the graph is immutable, instead of removing edges, we just track nodes we've already seen and ignore them
    let maybe_next = graph
        .get(&n)
        .expect("node in set was not in graph")
        .1 // out-set
        .iter()
        .filter(|m| !seen_set.contains(m)) // ignore nodes in the out-set we've already seen
        // we don't have to do any work to remove n since we have the seen-set
        // and since we expect a single correct ordering, we expect to find exactly 1 element here (or 0 if we're done)
        .find(|m| graph
            .get(m)
            .expect("node in set was not in graph")
            .0 // in-set
            .iter()
            .all(|precursor| seen_set.contains(precursor))); // if all of m's in-edges are in the seen-set, then we found it

    if let Some(next) = maybe_next {
        topological_sort(graph, next, seen_set, result_vec); // I have no idea if this tail recursion really counts
    }

    // if we didn't find it, then that's the end
}