struct MapPosition {
    position: (isize, isize),
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
    let unique_visitations = run_part_one(player_position, (0, -1), map);
    println!(
        "(Part 1) The player visited {:?} unique places",
        unique_visitations
    );
}

fn run_part_one(
    player_position: (isize, isize),
    direction_vec: (isize, isize),
    mut map: Vec<Vec<MapPosition>>,
) -> usize {
    let mut player_position = player_position.clone();
    map[player_position.1 as usize][player_position.0 as usize].visit();
    let mut direction_vec = direction_vec.clone();
    let height = map.len() as isize;
    let width = map[0].len() as isize;
    loop {
        let next_pos = (
            player_position.0 + direction_vec.0,
            player_position.1 + direction_vec.1,
        );
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= width || next_pos.1 >= height {
            break;
        }
        if map[next_pos.1 as usize][next_pos.0 as usize].is_obstacle {
            direction_vec = match direction_vec {
                (0, 1) => (-1, 0),
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (-1, 0) => (0, -1),
                _ => panic!("Unknown direction vector {:?}", direction_vec),
            }
        } else {
            map[next_pos.1 as usize][next_pos.0 as usize].visit();
            player_position = next_pos;
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
fn parse_input(input: &Vec<String>) -> ((isize, isize), Vec<Vec<MapPosition>>) {
    let mut result: Vec<Vec<MapPosition>> = Vec::new();
    let mut player_position: (isize, isize) = (0, 0);
    let mut y: isize = 0;
    for line in input {
        let mut x: isize = 0;
        let mut positions: Vec<MapPosition> = Vec::new();
        for char in line.chars() {
            let is_obstacle = match char {
                '^' => {
                    player_position = (x.try_into().unwrap(), y.try_into().unwrap());
                    false
                }
                '.' => false,
                '#' => true,
                _ => panic!("Unknown char in input {}", char),
            };
            positions.push(MapPosition {
                position: (x, y),
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
