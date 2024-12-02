use std::collections::HashMap;

pub fn run(input: &Vec<String>) {
    println!("Hello, this is Day 01");

    let (left_list, right_list, frequencies_of_right_list) = parse_lists(input);

    // Part 1
    let mut distance_sum: u32 = 0;
    // Part 2
    let mut similarity_score_sum: u32 = 0;
    for (i, left_location_id) in left_list.iter().enumerate() {
        // Part 1
        let right_location_id = right_list[i];
        distance_sum += left_location_id.abs_diff(right_location_id);

        // Part 2
        let frequency_of_location_id: u32 = match frequencies_of_right_list.get(&left_location_id) {
            Some(val) => *val,
            None => 0,
        };
        similarity_score_sum += left_location_id * frequency_of_location_id;
    }
    println!("(Part 1) The sum of distances is {}", distance_sum);
    println!("(Part 2) The similarity scores is {}", similarity_score_sum);
}

fn parse_lists(input: &Vec<String>) -> (Vec<u32>, Vec<u32>, HashMap<u32, u32>) {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    let mut frequencies_of_right_list: HashMap<u32, u32> = HashMap::new();
    for line in input.iter() {
        let values: Vec<&str> = line.split_whitespace().collect();
        let left_value: u32 = values[0].parse().unwrap();
        left_list.push(left_value);
        let right_value: u32 = values[1].parse().unwrap();
        right_list.push(right_value);

        frequencies_of_right_list
            .entry(right_value)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    left_list.sort();
    right_list.sort();
    return (left_list, right_list, frequencies_of_right_list);
}
