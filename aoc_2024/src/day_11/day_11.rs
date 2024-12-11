use std::time::Instant;

fn get_input() -> String {
    String::from(include_str!("./input.txt"))
}

pub fn day_eleven_part_one() {
    day_eleven(25); // Finished 25 iters in 44.306333ms. Answer: 218079
}

pub fn day_eleven_part_two() {
    day_eleven(40); // Finished 40 iters in 20.252521125s. Answer: 115133671
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
        for j in 0..stones.len() {
            if stones[j] == 0 {
                stones[j] = 1;
                continue;
            }
            let len = stones[j].to_string().len() as u32;
            if len % 2 == 0 {
                let shift = 10usize.pow(len / 2);
                let split_a = stones[j] / shift;
                let split_b = stones[j] % shift;
                stones[j] = split_a;
                stones.push(split_b);
                continue;
            }
            stones[j] = stones[j] * 2024;
        }
        println!("Loop {} took {:?}", i, loop_time.elapsed());
    }
    println!("Finished {} iters in {:?}. Answer: {}", iters, full_time.elapsed(), stones.len());
}