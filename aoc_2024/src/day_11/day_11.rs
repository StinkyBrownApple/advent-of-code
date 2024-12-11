use std::collections::HashMap;
use std::time::Instant;

fn get_input() -> String {
    String::from(include_str!("./input.txt"))
}

pub fn day_eleven_part_one() {
    day_eleven(25); // Finished 25 iters in 5.6995ms. Answer: 218079
}

pub fn day_eleven_part_two() {
    day_eleven(75); // Finished 75 iters in 145.72825ms. Answer: 259755538429618
}

fn day_eleven(iters: usize) {
    let stones = get_input()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    
    let mut total: usize = 0;
    let start_time = Instant::now();
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    for stone in stones {
        let loop_time = Instant::now();
        total += recurse(stone, 0, iters, &mut cache);
        println!("Stone {} took {:?}. Current stone count: {:?}", stone, loop_time.elapsed(), total);

    }
    println!("Finished {} iters in {:?}. Answer: {}", iters, start_time.elapsed(), total);
}

fn recurse(num: usize, iteration: usize, target_iteration: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if cache.contains_key(&(num, iteration)) {
        return *cache.get(&(num, iteration)).unwrap();
    }
    if iteration == target_iteration {
        return 1;
    }
    if num == 0 {
        let cache_val = recurse(1, iteration + 1, target_iteration, cache);
        cache.insert((num, iteration), cache_val);
        return cache_val;
    }
    let len = num.to_string().len() as u32;
    if len % 2 == 0 {
        let shift = 10usize.pow(len / 2);
        let split_a = num / shift;
        let split_b = num % shift;
        let cache_a = recurse(split_a, iteration + 1, target_iteration, cache);
        let cache_b = recurse(split_b, iteration + 1, target_iteration, cache);
        cache.insert((num, iteration), cache_a + cache_b);
        return cache_a + cache_b;
    }
    recurse(num * 2024, iteration + 1, target_iteration, cache)
}