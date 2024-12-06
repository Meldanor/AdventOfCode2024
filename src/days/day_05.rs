use std::collections::{HashMap, HashSet};

pub fn run(input: &Vec<String>) {
    let input_separator_index = input.iter().position(String::is_empty).unwrap();
    let (rules, pages) = input.split_at(input_separator_index);
    // Remove empty line
    let rules = parse_page_ordering_rules(rules);
    let pages: Vec<Vec<usize>> = parse_update(&pages[1..]);

    let part_one_sum = sum_middle_page_number_of_valid_pages(&pages, &rules);
    println!(
        "(Part 1) The sum of the middle page number of valid updates is {}",
        part_one_sum
    );

    let mut pages = pages.clone();
    let part_two_sum = sum_middle_page_number_of_fixed_update(&mut pages, &rules);
    println!(
        "(Part 2) The sum of the middle page number of fixed updates is {}",
        part_two_sum
    );
}

fn parse_page_ordering_rules(input: &[String]) -> HashMap<usize, HashSet<usize>> {
    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();

    for line in input {
        let (first, second) = line.split_once("|").unwrap();
        let first: usize = first.parse().unwrap();
        let second: usize = second.parse().unwrap();
        rules
            .entry(first)
            .and_modify(|e| {
                e.insert(second);
            })
            .or_insert_with(|| {
                let mut pages_after: HashSet<usize> = HashSet::new();
                pages_after.insert(second);
                return pages_after;
            });
    }

    return rules;
}

fn parse_update(input: &[String]) -> Vec<Vec<usize>> {
    return input
        .iter()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
}

fn sum_middle_page_number_of_valid_pages(
    updates: &Vec<Vec<usize>>,
    rules: &HashMap<usize, HashSet<usize>>,
) -> usize {
    let mut sum: usize = 0;
    for update in updates {
        if is_update_valid(update, rules) {
            let middle_page = update[update.len() / 2];
            sum += middle_page;
        }
    }
    return sum;
}

fn sum_middle_page_number_of_fixed_update(
    updates: &mut Vec<Vec<usize>>,
    rules: &HashMap<usize, HashSet<usize>>,
) -> usize {
    let mut sum: usize = 0;
    for update in updates.iter_mut() {
        if is_update_valid(update, rules) {
            continue;
        }
        fix_update(update, rules);
        let middle_page = update[update.len() / 2];
        sum += middle_page;
    }
    return sum;
}

fn is_update_valid(update: &Vec<usize>, rules: &HashMap<usize, HashSet<usize>>) -> bool {
    let mut i: usize = 0;
    while i < update.len() - 1 {
        let (current, next) = (update[i], update[i + 1]);
        let next_is_after_current: bool = match rules.get(&current) {
            Some(pages_after) => pages_after.get(&next).is_some(),
            None => false,
        };
        if !next_is_after_current {
            return false;
        }
        i += 1;
    }
    return true;
}

fn fix_update(invalid_update: &mut Vec<usize>, rules: &HashMap<usize, HashSet<usize>>) -> bool {
    // Find bad position
    let mut i: usize = 0;
    while i < invalid_update.len() - 1 {
        let (current, next) = (invalid_update[i], invalid_update[i + 1]);
        let next_is_after_current: bool = match rules.get(&current) {
            Some(pages_after) => pages_after.get(&next).is_some(),
            None => false,
        };
        if !next_is_after_current {
            break;
        }
        i += 1;
    }
    if i == invalid_update.len() - 1 {
        return true;
    } else {
        invalid_update.swap(i, i + 1);
        return fix_update(invalid_update, rules);
    }
}
