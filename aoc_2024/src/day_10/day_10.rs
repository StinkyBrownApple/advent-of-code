fn get_input() -> String {
    String::from(include_str!("./input.txt"))
}

pub fn day_ten_part_one() {
    day_ten(true);
}

pub fn day_ten_part_two() {
    day_ten(false);
}

fn day_ten(day_one: bool) {
    let mut map = get_input()
        .lines()
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let mut total = 0usize;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                total += follow_path(&mut map, x, y, &mut 0, day_one);
                if day_one {
                    reset_map(&mut map);
                }
            }
        }
    }
    println!("{}", total);
}

fn reset_map(map: &mut [Vec<u8>]) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == u8::MAX {
                map[y][x] = 9;
            }
        }
    }
}

fn follow_path(map: &mut [Vec<u8>], x: usize, y: usize, good_paths: &mut usize, unique_paths: bool) -> usize {
    let iter = [[-1, 0], [1, 0], [0, -1], [0, 1]];
    for it in iter {
        let i = it[1];
        let j = it[0];
        if (x == 0 && j == -1) || (y == 0 && i == -1) || (x == map[0].len() - 1 && j == 1) || (y == map.len() - 1 && i == 1) {
            continue;
        }
        let new_x = ((x as isize) + j) as usize;
        let new_y = ((y as isize) + i) as usize;
        if map[y][x] == 8 && map[new_y][new_x] == 9 {
            if unique_paths {
                map[new_y][new_x] = u8::MAX;
            }
            *good_paths += 1;
        } else if map[new_y][new_x] == map[y][x] + 1 {
            *good_paths = follow_path(map, new_x, new_y, good_paths, unique_paths);
        }
    }
    *good_paths
}