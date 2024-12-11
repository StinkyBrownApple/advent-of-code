use std::time::Instant;
use rayon::prelude::*;

fn get_input() -> String {
    String::from(include_str!("./input.txt"))
}

pub fn day_eleven_part_one() {
    day_eleven(25); // Finished 25 iters in 31.9875ms. Answer: 218079
}

pub fn day_eleven_part_two() {
    day_eleven(40); // Finished 40 iters in 3.653700459s. Answer: 115133671
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
        stones = stones
            .par_chunks_mut(1000)
            .map(|chunk| {
                let mut new_stones: Vec<usize> = vec![];
                chunk.iter_mut().for_each(|j| {
                    if *j == 0 {
                        *j = 1;
                        return;
                    }
                let len = j.to_string().len() as u32;
                if len % 2 == 0 {
                    let shift = 10usize.pow(len / 2);
                    let split_a = *j / shift;
                    let split_b = *j % shift;
                    *j = split_a;
                    new_stones.push(split_b);
                    return;
                }
                *j = *j * 2024;
                });
                let mut concat_stones = chunk.iter().map(|x| *x).collect::<Vec<usize>>();
                concat_stones.append(&mut new_stones);
                return concat_stones;
            })
            .flatten()
            .collect::<Vec<usize>>();
        println!("Loop {} took {:?}. Stone count: {:?}", i, loop_time.elapsed(), stones.len());
    }
    println!("Finished {} iters in {:?}. Answer: {}", iters, full_time.elapsed(), stones.len());
}