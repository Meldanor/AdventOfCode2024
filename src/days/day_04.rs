pub fn run(input: &Vec<String>) {
    let height: isize = input.len() as isize;
    let width: isize = input[0].len() as isize;
    let char_matrix: Vec<Vec<char>> = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    // Part 1
    let mut count_xmas: usize = 0;
    let word = "XMAS".chars().collect::<Vec<char>>();
    for x in 0..width {
        for y in 0..height {
            count_xmas += is_xmas(&word, x, y, width, height, &char_matrix);
        }
    }
    println!("(Part 1) XMAS appears {} times", count_xmas);

    // Part 2
    let mut count_x_mas: usize = 0;
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            if is_x_mas(x, y, &char_matrix) {
                count_x_mas += 1;
            }
        }
    }
    println!("(Part 2) X-MAS appears {} times", count_x_mas);
}

fn is_xmas(
    word: &Vec<char>,
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    data: &Vec<Vec<char>>,
) -> usize {
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if word_in_direction(word, x, y, dx, dy, width, height, data) {
                count += 1;
            }
        }
    }

    return count;
}

fn is_x_mas(x: isize, y: isize, data: &Vec<Vec<char>>) -> bool {
    let x = x as usize;
    let y = y as usize;
    if data[y][x] != 'A' {
        return false;
    }
    let (tl, tr, bl, br) = (
        data[y - 1][x - 1],
        data[y - 1][x + 1],
        data[y + 1][x - 1],
        data[y + 1][x + 1],
    );
    // Check Left Top to Bottom Right and its reverse AND
    // Check Right Top to Bottom Left and its reverse
    return ((tl == 'S' || tl == 'M') && (br == 'S' || br == 'M') && tl != br)
        && ((tr == 'S' || tr == 'M') && (bl == 'S' || bl == 'M') && tr != bl);
}

fn word_in_direction(
    word: &Vec<char>,
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
    while i < word.len() {
        let pos_x = x + (dx * (i as isize));
        let pos_y = y + (dy * (i as isize));
        if pos_x < 0 || pos_y < 0 || pos_x >= width || pos_y >= height {
            return false;
        }
        let char_at_pos = data[pos_y as usize][pos_x as usize];
        if char_at_pos != word[i as usize] {
            return false;
        }
        i += 1;
    }
    return true;
}
