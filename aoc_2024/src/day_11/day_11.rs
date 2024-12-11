use std::time::Instant;

fn get_input() -> String {
    String::from(include_str!("./input.txt"))
}

pub fn day_eleven_part_one() {
    day_eleven(25); // Finished 25 iters in 119.865708ms. Answer: 218079
}

pub fn day_eleven_part_two() {
    day_eleven(75);
}

pub fn day_eleven(iters: usize) {
    let mut stones = get_input()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let full_time = Instant::now();
    for i in 0..iters {
        let loop_time = Instant::now();
        let mut new_stones: Vec<usize> = Vec::new();
        for j in 0..stones.len() {
            if stones[j] == 0 {
                new_stones.push(1);
                continue;
            }
            let stone_string = stones[j].to_string();
            if stone_string.len() % 2 == 0 {
                let stone_chars = stone_string.chars();
                let split_a = stone_chars.clone().take(stone_string.len() / 2).collect::<String>().parse::<usize>().unwrap();
                let split_b = stone_chars.skip(stone_string.len() / 2).collect::<String>().parse::<usize>().unwrap();
                new_stones.push(split_a);
                new_stones.push(split_b);
                continue;
            }
            new_stones.push(stones[j] * 2024);
        }
        stones = new_stones;
        println!("Loop {} took {:?}", i, loop_time.elapsed());
    }
    println!("Finished {} iters in {:?}. Answer: {}", iters, full_time.elapsed(), stones.len());
}