fn get_input() -> String {
    String::from(include_str!("./input.txt"))
}

fn is_safe(vec: &Vec<i32>) -> bool {
    vec.is_sorted_by(|a, b| ((b - a) < 4) && ((b - a) > 0)) ||
        vec.is_sorted_by(|a, b| ((a - b) < 4) && ((a - b) > 0))
}

pub fn day_two_part_one() {
    let input = get_input();
    let safe_rows  = input
        .lines()
        .map(
            |line| line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        )
        .filter(|vec| is_safe(vec))
        .count();
    println!("{}", safe_rows);
}

pub fn day_two_part_two() {
    let input = get_input();
    let safe_rows  = input
        .lines()
        .map(
            |line| line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        )
        .filter(|vec| {
            if is_safe(vec) {return true}
            for i in 0..vec.len() {
                let mut vec_clone = vec.clone();
                vec_clone.remove(i);
                if is_safe(&vec_clone) {return true}
            }
            false
        })
        .count();
    println!("{}", safe_rows);
}