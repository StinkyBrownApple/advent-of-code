fn get_input() -> String {
    String::from(include_str!("./input.txt"))
}

pub fn day_four_part_one() {
    let input = get_input();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut total = 0;
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if lines[i].as_bytes()[j] as char == 'X' {
                for k in -1..=1i32 {
                    for l in -1..=1i32 {
                        if k == 0 && l == 0 {
                            continue;
                        }
                        if search_direction(&lines, i, j, k, l, 'M') {
                            total += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", total);
}

pub fn day_four_part_two() {
    let input = get_input();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut total = 0;
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if lines[i].as_bytes()[j] as char == 'A' {
                if search_corners(&lines, i, j) {
                    total += 1;
                }
            }
        }
    }
    println!("{}", total);
}

fn search_corners(lines: &[&str], start_x: usize, start_y: usize) -> bool {
    if start_y == 0 || start_x == 0 {return false}
    let top_y = start_y - 1usize;
    let bottom_y = start_y + 1usize;
    let left_x = start_x - 1usize;
    let right_x = start_x + 1usize;
    
    if !check_corners(lines, left_x, top_y, right_x, bottom_y) {
        return false;
    }
    
    check_corners(lines, right_x, top_y, left_x, bottom_y)
}

fn check_corners(lines: &[&str], corn_x: usize, corn_y: usize, opp_corn_x: usize, opp_corn_y: usize) -> bool {
    (get_char_at_coords(lines, corn_x, corn_y) == 'S' && get_char_at_coords(lines, opp_corn_x, opp_corn_y) == 'M') || 
        (get_char_at_coords(lines, corn_x, corn_y) == 'M' && get_char_at_coords(lines, opp_corn_x, opp_corn_y) == 'S')
}

fn get_char_at_coords(lines: &[&str], x: usize, y: usize) -> char {
    *lines
        .get(x)
        .unwrap_or(&".").as_bytes()
        .get(y)
        .unwrap_or(&('.' as u8)) as char
}

fn search_direction(lines: &[&str], start_x: usize, start_y: usize, x_dir: i32, y_dir: i32, target_char: char) -> bool {
    let new_x = get_new_coord(start_x, x_dir);
    let new_y = get_new_coord(start_y, y_dir);
    
    if new_x == usize::MAX || new_y == usize::MAX {return false}
    
    if get_char_at_coords(lines, new_x, new_y) == target_char {
        return if target_char == 'M' {
            search_direction(lines, new_x, new_y, x_dir, y_dir, 'A')
        } else if target_char == 'A' {
            search_direction(lines, new_x, new_y, x_dir, y_dir, 'S')
        } else if target_char == 'S' {
            true
        } else {
            false
        }
    }
    false
}

fn get_new_coord(start: usize, dir: i32) -> usize {
    if dir.is_negative() {
        if start as i32 - dir.abs() < 0 {return usize::MAX}
        start - dir.abs() as usize
    } else {
        start + dir as usize
    }
}