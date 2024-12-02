pub fn run(input: &Vec<String>) {
    println!("Hello, this is Day 01");

    let mut distance_sum: u32 = 0;
    let (left_list, right_list) = parse_lists(input);
    for (i, left_location_id) in left_list.iter().enumerate() {
        let right_location_id = right_list[i];
        distance_sum += left_location_id.abs_diff(right_location_id);
    }
    println!("The sum of distances is {}", distance_sum);
}

fn parse_lists(input: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in input.iter() {
        let values: Vec<&str> = line.split_whitespace().collect();
        left_list.push(values[0].parse().unwrap());
        right_list.push(values[1].parse().unwrap());
    }
    left_list.sort();
    right_list.sort();
    return (left_list, right_list);
}
