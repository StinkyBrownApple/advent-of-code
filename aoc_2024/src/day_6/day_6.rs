use std::collections::HashSet;

fn get_input() -> String {
    String::from(include_str!("./input.txt"))
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line|
            line
                .as_bytes()
                .iter()
                .map(|b| *b as char)
                .collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>()
}

pub fn day_six_part_one() {
    let input = get_input();
    
    let mut grid = get_grid(&input);
    
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut dir_x: i32 = 0;
    let mut dir_y: i32 = -1;
    
    get_starting_coords(&grid, &mut pos_x, &mut pos_y);
    
    loop {
        if is_out_of_bounds(&grid, pos_x, pos_y, dir_x, dir_y) {
            grid[pos_y][pos_x] = 'X';
            break;
        } else if grid[get_new_coord(pos_y, dir_y)][get_new_coord(pos_x, dir_x)] == '#' {
            get_new_dir(&mut dir_x, &mut dir_y);
            continue;
        } else {
            grid[pos_y][pos_x] = 'X';
            pos_x = get_new_coord(pos_x, dir_x);
            pos_y = get_new_coord(pos_y, dir_y);
        }
    }

    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'X' {
                total += 1;
            }
        }
    }
    println!("{}", total);
}

pub fn day_six_part_two() {
    let input = get_input();

    let grid = get_grid(&input);

    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut dir_x;
    let mut dir_y;

    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '.' {
                let mut new_grid = grid.clone();
                new_grid[y][x] = '#';
                get_starting_coords(&grid, &mut pos_x, &mut pos_y);
                dir_x = 0;
                dir_y = -1;
                let mut hit_walls: HashSet<String> = HashSet::new();
                loop {
                    if is_out_of_bounds(&new_grid, pos_x, pos_y, dir_x, dir_y) {
                        break;
                    } else if new_grid[get_new_coord(pos_y, dir_y)][get_new_coord(pos_x, dir_x)] == '#' {
                        let mut hit_wall = String::new();
                        hit_wall.push_str(&get_new_coord(pos_y, dir_y).to_string());
                        hit_wall.push_str(&get_new_coord(pos_x, dir_x).to_string());
                        hit_wall.push_str(&dir_y.to_string());
                        hit_wall.push_str(&dir_x.to_string());
                        if hit_walls.contains(&hit_wall) {
                            total += 1;
                            break;
                        }
                        hit_walls.insert(hit_wall);
                        get_new_dir(&mut dir_x, &mut dir_y);
                    } else {
                        new_grid[pos_y][pos_x] = 'X';
                        pos_x = get_new_coord(pos_x, dir_x);
                        pos_y = get_new_coord(pos_y, dir_y);
                    }
                }
            }
        }
    }
    
    println!("{}", total);
}

fn get_starting_coords(grid: &Vec<Vec<char>>, pos_x: &mut usize, pos_y: &mut usize) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' {
                *pos_x = j;
                *pos_y = i;
                return;
            }
        }
    }
}

fn is_out_of_bounds(grid: &Vec<Vec<char>>, pos_x: usize, pos_y: usize, dir_x: i32, dir_y: i32) -> bool {
    get_new_coord(pos_y, dir_y) == usize::MAX || get_new_coord(pos_x, dir_x) == usize::MAX || get_new_coord(pos_y, dir_y) == grid.len() || get_new_coord(pos_x, dir_x) == grid[0].len()
}

fn get_new_dir(dir_x: &mut i32, dir_y: &mut i32) {
    if *dir_y == -1 {
        *dir_y = 0;
        *dir_x = 1;
    } else if *dir_y == 1 {
        *dir_y = 0;
        *dir_x = -1;
    } else if *dir_x == -1 {
        *dir_y = -1;
        *dir_x = 0;
    } else {
        *dir_y = 1;
        *dir_x = 0;
    }
}

fn get_new_coord(start: usize, dir: i32) -> usize {
    if dir.is_negative() {
        if start as i32 - dir.abs() < 0 {return usize::MAX}
        start - dir.abs() as usize
    } else {
        start + dir as usize
    }
}