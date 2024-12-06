#[derive(PartialEq, Eq, Clone, Debug)]
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
    let (player_position, map) = parse_input(input);
    println!("{:?}", player_position);
    let unique_visitations = run_part_one(player_position, UP, map);
    println!(
        "(Part 1) The player visited {:?} unique places",
        unique_visitations
    );
}

fn run_part_one(
    player_position: Point2D,
    direction: Point2D,
    mut map: Vec<Vec<MapPosition>>,
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
    return count_unique_visitations(map);
}

fn count_unique_visitations(map: Vec<Vec<MapPosition>>) -> usize {
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
