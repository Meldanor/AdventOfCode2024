pub fn run(input: &Vec<String>) {
    let height: isize = input.len() as isize;
    let width: isize = input[0].len() as isize;
    let char_matrix: Vec<Vec<char>> = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut count_xmas: usize = 0;
    for x in 0..width {
        for y in 0..height {
            count_xmas += is_xmas(x, y, width, height, &char_matrix);
        }
    }
    println!("(Part 1) XMAS appears {} times", count_xmas);
}

fn is_xmas(x: isize, y: isize, width: isize, height: isize, data: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if word_in_direction(x, y, dx, dy, width, height, data) {
                count += 1;
            }
        }
    }

    return count;
}

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
fn word_in_direction(
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    width: isize,
    height: isize,
    data: &Vec<Vec<char>>,
) -> bool {
    let mut i: usize = 0;
    // Word length
    while i < WORD.len() {
        let pos_x = x + (dx * (i as isize));
        let pos_y = y + (dy * (i as isize));
        if pos_x < 0 || pos_y < 0 || pos_x >= width || pos_y >= height {
            return false;
        }
        let char_at_pos = data[pos_y as usize][pos_x as usize];
        if char_at_pos != WORD[i as usize] {
            return false;
        }
        i += 1;
    }
    return true;
}
