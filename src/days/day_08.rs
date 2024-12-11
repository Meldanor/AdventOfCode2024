use std::collections::{HashMap, HashSet};

use crate::point_2d::Point2D;

pub fn run(input: &Vec<String>) {
    let height: isize = input.len() as isize;
    let width: isize = input[0].len() as isize;
    let antennas = parse_input(input);

    let unique_antinode_locations = find_unique_antinode_locations(&antennas, width, height);

    println!(
        "(Part 1) There are {} unique antinode positions",
        unique_antinode_locations
    );

    let unique_antinode_harmoic_locations =
        find_unique_antinode_locations_with_harmonics(&antennas, width, height);

    println!(
        "(Part 2) There are {} unique antinode positions with harmonics",
        unique_antinode_harmoic_locations
    );
}

fn find_unique_antinode_locations(
    antennas: &HashMap<char, Vec<Point2D>>,
    width: isize,
    height: isize,
) -> usize {
    let mut unique_antinode_locations: HashSet<Point2D> = HashSet::new();
    for (_, antenna_group) in antennas {
        for antenna_a in antenna_group.iter() {
            for antenna_b in antenna_group.iter() {
                if antenna_a.eq(antenna_b) {
                    continue;
                }
                let distance = *antenna_b - *antenna_a;
                let point_a = *antenna_a - distance;
                if is_point_inside_map(&point_a, width, height) {
                    unique_antinode_locations.insert(point_a);
                }
                let point_b = *antenna_b + distance;
                if is_point_inside_map(&point_b, width, height) {
                    unique_antinode_locations.insert(point_b);
                }
            }
        }
    }
    return unique_antinode_locations.len();
}

fn find_unique_antinode_locations_with_harmonics(
    antennas: &HashMap<char, Vec<Point2D>>,
    width: isize,
    height: isize,
) -> usize {
    let mut unique_antinode_locations: HashSet<Point2D> = HashSet::new();
    for (_, antenna_group) in antennas {
        for antenna_a in antenna_group.iter() {
            for antenna_b in antenna_group.iter() {
                if antenna_a.eq(antenna_b) {
                    continue;
                }
                let distance = *antenna_b - *antenna_a;
                let point_a = *antenna_a - distance;
                if is_point_inside_map(&point_a, width, height) {
                    unique_antinode_locations.insert(point_a);
                }
                let point_b = *antenna_b + distance;
                if is_point_inside_map(&point_b, width, height) {
                    unique_antinode_locations.insert(point_b);
                }
                add_antinodes_along_line(
                    &mut unique_antinode_locations,
                    &point_a,
                    &(distance * 1_isize),
                    width,
                    height,
                );
                add_antinodes_along_line(
                    &mut unique_antinode_locations,
                    &point_b,
                    &distance,
                    width,
                    height,
                );
            }
        }
    }
    return unique_antinode_locations.len();
}

fn add_antinodes_along_line(
    antinodes: &mut HashSet<Point2D>,
    point: &Point2D,
    direction: &Point2D,
    width: isize,
    height: isize,
) {
    let mut antinode = *point;
    loop {
        antinode = antinode + *direction;
        if is_point_inside_map(&antinode, width, height) {
            antinodes.insert(antinode);
        } else {
            break;
        }
    }
}

fn is_point_inside_map(point: &Point2D, width: isize, height: isize) -> bool {
    point.x >= 0 && point.y >= 0 && point.x < width && point.y < height
}

fn parse_input(input: &Vec<String>) -> HashMap<char, Vec<Point2D>> {
    let mut antennas: HashMap<char, Vec<Point2D>> = HashMap::new();
    for (y, line) in input.iter().enumerate() {
        for (x, char) in line.chars().into_iter().enumerate() {
            match char {
                '.' => (),
                c => {
                    let point = Point2D {
                        x: x as isize,
                        y: y as isize,
                    };
                    antennas
                        .entry(c)
                        .and_modify(|e| {
                            e.push(point);
                        })
                        .or_insert(vec![point]);
                }
            };
        }
    }
    return antennas;
}
