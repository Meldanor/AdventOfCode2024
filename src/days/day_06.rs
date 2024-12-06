use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Debug, Hash, Copy)]
struct Point2D {
    x: isize,
    y: isize,
}

const ZERO: Point2D = Point2D { x: 0, y: 0 };
const UP: Point2D = Point2D { x: 0, y: -1 };
const RIGHT: Point2D = Point2D { x: 1, y: 0 };
const DOWN: Point2D = Point2D { x: 0, y: 1 };
const LEFT: Point2D = Point2D { x: -1, y: 0 };

impl Point2D {
    pub fn add(&self, b: &Point2D) -> Point2D {
        Point2D {
            x: self.x + b.x,
            y: self.y + b.y,
        }
    }

    pub fn rotate_clockwise(&self) -> Point2D {
        if self.eq(&UP) {
            RIGHT
        } else if self.eq(&RIGHT) {
            DOWN
        } else if self.eq(&DOWN) {
            LEFT
        } else if self.eq(&LEFT) {
            UP
        } else {
            panic!("unknown rotation vector")
        }
    }
}

struct MapPosition {
    is_obstacle: bool,
    visitations: usize,
}

impl MapPosition {
    pub fn visit(&mut self) {
        self.visitations += 1;
    }
}

pub fn run(input: &Vec<String>) {
    let (player_position, mut map) = parse_input(input);
    println!("{:?}", player_position);
    let unique_visitations = run_part_one(player_position, UP, &mut map);
    println!(
        "(Part 1) The player visited {:?} unique places",
        unique_visitations
    );

    // Part 2 sketch:
    // run part 1 and fetch the unique visitation position as a list of possible points for obstactles
    // loop through the list and modify the map at this position to turn it to an obstacle
    // exclude the start point from this list
    // run the part 1 code but also keep track of (visited locations + direction at this point) in this run
    // if a pair (location + direction) is encountered twice, there is a loop and the obstacle is valid
    let possible_loops = run_part_two(player_position, UP, &mut map);
    println!("(Part 2) There are {:?} possible loops", possible_loops);
}

fn run_part_one(
    player_position: Point2D,
    direction: Point2D,
    map: &mut Vec<Vec<MapPosition>>,
) -> usize {
    let mut player_position = player_position.clone();
    let mut direction = direction.clone();
    map[player_position.y as usize][player_position.x as usize].visit();
    let height = map.len() as isize;
    let width = map[0].len() as isize;
    loop {
        let next_position = player_position.add(&direction);

        if next_position.x < 0
            || next_position.y < 0
            || next_position.x >= width
            || next_position.y >= height
        {
            break;
        }
        if map[next_position.y as usize][next_position.x as usize].is_obstacle {
            direction = direction.rotate_clockwise();
        } else {
            map[next_position.y as usize][next_position.x as usize].visit();
            player_position = next_position;
        }
    }
    return count_unique_visitations(&map);
}

fn count_unique_visitations(map: &Vec<Vec<MapPosition>>) -> usize {
    let mut unique_visitations: usize = 0;
    let height = map.len();
    let width = map[0].len();
    for y in 0..height {
        for x in 0..width {
            if map[y][x].visitations > 0 {
                unique_visitations += 1;
            }
        }
    }
    return unique_visitations;
}

fn run_part_two(
    player_position: Point2D,
    direction: Point2D,
    map: &mut Vec<Vec<MapPosition>>,
) -> usize {
    let mut count_possible_loops = 0;
    let possible_obstacle_positions = fetch_possible_obstacle_positions(map, &player_position);
    for obstacle in possible_obstacle_positions {
        map[obstacle.y as usize][obstacle.x as usize].is_obstacle = true;
        if does_obstacle_result_in_loop(&player_position, &direction, map) {
            count_possible_loops += 1;
        }
        map[obstacle.y as usize][obstacle.x as usize].is_obstacle = false;
    }
    return count_possible_loops;
}

fn fetch_possible_obstacle_positions(
    map: &Vec<Vec<MapPosition>>,
    player_position: &Point2D,
) -> Vec<Point2D> {
    let mut result: Vec<Point2D> = Vec::new();

    let height = map.len();
    let width = map[0].len();
    for y in 0..height {
        for x in 0..width {
            if map[y][x].visitations > 0
            // Skip player start position
                && !(player_position.x == x as isize && player_position.y == y as isize)
            {
                result.push(Point2D {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }
    return result;
}

fn does_obstacle_result_in_loop(
    player_position: &Point2D,
    direction: &Point2D,
    map: &Vec<Vec<MapPosition>>,
) -> bool {
    let mut visited_positions: HashSet<(Point2D, Point2D)> = HashSet::new();
    let mut player_position = player_position.clone();
    let mut direction = direction.clone();
    let height = map.len() as isize;
    let width = map[0].len() as isize;
    loop {
        let next_position = player_position.add(&direction);
        if visited_positions.contains(&(next_position, direction)) {
            return true;
        }
        visited_positions.insert((next_position, direction));

        if next_position.x < 0
            || next_position.y < 0
            || next_position.x >= width
            || next_position.y >= height
        {
            return false;
        }
        if map[next_position.y as usize][next_position.x as usize].is_obstacle {
            direction = direction.rotate_clockwise();
        } else {
            player_position = next_position;
        }
    }
}

// Player position AND the map
fn parse_input(input: &Vec<String>) -> (Point2D, Vec<Vec<MapPosition>>) {
    let mut result: Vec<Vec<MapPosition>> = Vec::new();
    let mut player_position: Point2D = ZERO;
    let mut y: isize = 0;
    for line in input {
        let mut x: isize = 0;
        let mut positions: Vec<MapPosition> = Vec::new();
        for char in line.chars() {
            let is_obstacle = match char {
                '^' => {
                    player_position = Point2D { x, y };
                    false
                }
                '.' => false,
                '#' => true,
                _ => panic!("Unknown char in input {}", char),
            };
            positions.push(MapPosition {
                is_obstacle,
                visitations: 0,
            });
            x += 1;
        }

        result.push(positions);
        y += 1;
    }

    return (player_position, result);
}
