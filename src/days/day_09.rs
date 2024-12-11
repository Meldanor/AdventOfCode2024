#[derive(Clone, Copy, Debug)]
struct FileSector {
    start: usize,
    end: usize,
    id: Option<usize>,
}

impl FileSector {
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

pub fn run(input: &Vec<String>) {
    let file_sectors = parse_input(input);
    // print_file_sectors(&file_sectors);
    let compressed_sectors = compress_files(&file_sectors);
    // print_file_sectors(&compressed_sectors);
    //println!("{:?}", file_sectors);
}

fn print_file_sectors(file_sectors: &Vec<FileSector>) {
    for sector in file_sectors.iter() {
        let length = sector.end - sector.start;
        let char_to_print = match sector.id {
            Some(id) => id.to_string(),
            None => ".".to_string(),
        };
        for _i in 0..length {
            print!("{}", char_to_print);
        }
    }
    println!();
}

// 2333133121414131402
// 00...111...2...333.44.5555.6666.777.888899
// 00...111...2...333.44.5555.6666.777.888899

/*
    0..111....22222
    02.111....2222.
    022111....222..
    0221112...22...
    02211122..2....
    022111222......
*/

fn compress_files(original: &Vec<FileSector>) -> Vec<FileSector> {
    let mut copy = original.clone();
    let mut free_sector_index = 1;
    let mut file_to_move_index = copy.len() - 1;
    loop {
        //print_file_sectors(&copy);
        let mut free_sector = &mut copy[free_sector_index].unwrap();
        let mut file_to_move = &mut copy[file_to_move_index];
        let file_length = file_to_move.len();
        println!(
            "{} >= {} = {}",
            free_sector.len(),
            file_to_move.len(),
            free_sector.len() >= file_length
        );
        // Move file completly and keep free_sector
        if free_sector.len() >= file_length {
            file_to_move.start = free_sector.start;
            file_to_move.end = file_to_move.start + file_length;
            free_sector.start += file_length;
            if free_sector.len() == 0 {
                copy.swap_remove(free_sector_index);
                free_sector_index += 2;
            }
            file_to_move_index += 2;
        }
        // Move file partly
        else {
            let free_sector_length = free_sector.len();
            // Convert the free sector to the file
            free_sector.id = file_to_move.id;
            // Shrink the file to move

            println!("end_before={}", file_to_move.end);
            file_to_move.end = file_to_move.end - free_sector_length;
            println!("end_after={}", file_to_move.end);
            println!("new_length={}", file_to_move.len());
            free_sector_index += 2;
        }
        if file_to_move_index >= copy.len() || free_sector_index >= copy.len() {
            break;
        }
    }
    return copy;
}
fn parse_input(input: &Vec<String>) -> Vec<FileSector> {
    let mut result: Vec<FileSector> = Vec::new();
    let line: Vec<char> = input[0].chars().collect();
    let mut start = 0;
    let mut file_id = 0;
    let mut i = 0;
    while i < line.len() {
        let file_length = line[i].to_digit(10).unwrap() as usize;
        result.push(FileSector {
            start,
            end: start + file_length,
            id: Some(file_id),
        });
        start = start + file_length;
        file_id += 1;
        if i + 1 >= line.len() {
            break;
        }
        let empty_sector = line[i + 1].to_digit(10).unwrap();
        result.push(FileSector {
            start,
            end: start + empty_sector as usize,
            id: None,
        });
        start = start + file_length + 1;
        i += 2;
    }
    return result;
}
