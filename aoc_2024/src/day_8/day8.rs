use std::collections::{HashSet};

fn get_input() -> String {
    String::from(include_str!("./input.txt"))
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    
    pub fn vec_to_other(&self, other: Vector) -> Vector {
        Self {x: other.x - self.x, y: other.y - self.y}
    }
    
    pub fn scale(&self, factor: isize) -> Vector {
        Self {x: self.x * factor, y: self.y * factor, }
    }
    
    pub fn is_inbounds(&self, map_scale: &Vector) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < map_scale.x && self.y < map_scale.y
    }
    
    pub fn apply_vector(&self, other: &Vector) -> Vector {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

pub fn day_eight_part_one() {
    let input = get_input();
    let map = get_map(&*input);

    do_map_loop(&map, part_one_get_antinodes);
}

pub fn day_eight_part_two() {
    let input = get_input();
    let map = get_map(&*input);
    
    do_map_loop(&map, part_two_get_antinodes);
}

fn part_one_get_antinodes(antinodes: &mut HashSet<Vector>, ant_x: isize, ant_y: isize, curr_x: isize, curr_y: isize, map_scale: &Vector) {
    let current_ant = Vector::new(ant_x, ant_y);
    let found_ant = Vector::new(curr_x, curr_y);
    let antinode = current_ant.apply_vector(&current_ant.vec_to_other(found_ant).scale(2));
    if antinode.is_inbounds(&map_scale) {
        antinodes.insert(antinode);
    }
}

fn part_two_get_antinodes(antinodes: &mut HashSet<Vector>, ant_x: isize, ant_y: isize, curr_x: isize, curr_y: isize, map_scale: &Vector) {
    let current_ant = Vector::new(ant_x, ant_y);
    let found_ant = Vector::new(curr_x, curr_y);
    let dist_vec = &current_ant.vec_to_other(found_ant);

    let mut antinode = current_ant.apply_vector(dist_vec);
    while antinode.is_inbounds(&map_scale) {
        antinodes.insert(antinode.clone());
        antinode = antinode.apply_vector(dist_vec);
    }
}

fn do_map_loop(map: &Vec<Vec<char>>, antinode_fn: fn(&mut HashSet<Vector>, isize, isize, isize, isize, &Vector)) {
    let mut antinodes: HashSet<Vector> = HashSet::new();
    let map_scale = Vector::new(map[0].len() as isize, map.len() as isize);

    for current_ant_y in 0..map.len() {
        for current_ant_x in 0..map[current_ant_y].len() {
            if map[current_ant_y][current_ant_x] == '.' {
                continue;
            }

            for y in 0..map.len() {
                for x in 0..map[y].len() {
                    if y == current_ant_y || x == current_ant_x {
                        continue;
                    }
                    if map[y][x] == map[current_ant_y][current_ant_x] {
                        antinode_fn(&mut antinodes, current_ant_x as isize, current_ant_y as isize, x as isize, y as isize, &map_scale);
                    }
                }
            }
        }
    }
    println!("{}", antinodes.len());
}

fn get_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}