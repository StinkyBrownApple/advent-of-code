fn get_input() -> String {
    String::from(include_str!("./input.txt"))
}

pub fn day_nine_part_one() {
    let input = get_input()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();
    
    let mut uncompressed = uncompress(&input);
    defrag(&mut uncompressed);
    let checksum = get_checksum(&uncompressed);
    println!("{:?}", checksum);
}

pub fn day_nine_part_two() {
    let input = get_input()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    let mut uncompressed = uncompress(&input);
    defrag_part_2(&mut uncompressed);
    let checksum = get_checksum_part_two(&uncompressed);
    println!("{:?}", checksum);
}

fn uncompress(compressed: &Vec<u8>) -> Vec<usize> {
    let mut uncompressed: Vec<usize> = Vec::new();
    let mut empty_space = false;
    for i in 0..compressed.len() {
        for _ in 0..compressed[i] {
            uncompressed.push(if !empty_space {i/2} else {usize::MAX});
        }
        empty_space = !empty_space;
    }
    uncompressed
}

fn defrag(uncompressed: &mut Vec<usize>) {
    for i in (0..=uncompressed.len() - 1).rev() {
        if uncompressed[i] == usize::MAX {
            continue;
        }
        for j in 0..i {
            if uncompressed[j] == usize::MAX {
                uncompressed[j] = uncompressed[i];
                uncompressed[i] = usize::MAX;
                break;
            }
        }
    }
}

fn get_checksum(defragged: &Vec<usize>) -> usize {
    let mut total: usize = 0;
    for i in 0..defragged.len() {
        if defragged[i] == usize::MAX {
            break;
        }
        total += i * defragged[i];
    }
    total
}

fn defrag_part_2(uncompressed: &mut Vec<usize>) {
    let mut i: usize = uncompressed.len() - 1;
    loop {
        if uncompressed[i] == usize::MAX {
            i -= 1;
            continue;
        }
        let mut block_size = 0;
        loop {
            if (block_size <= i) && (uncompressed[i-block_size] == uncompressed[i]) {
                block_size += 1;
            } else {
                break;
            }
        }
        'replace: for j in 0..i {
            for k in 0..block_size {
                if uncompressed[j + k] != usize::MAX {
                    continue 'replace;
                }
            }
            for k in 0..block_size {
                uncompressed[j + k] = uncompressed[i - k];
                uncompressed[i - k] = usize::MAX;
            }
            break 'replace;
        }
        if block_size > i {
            break;
        }
        i -= block_size;
    }
}

//Same as part one but doesn't exit early at an empty slot, instead just continues
fn get_checksum_part_two(defragged: &Vec<usize>) -> usize {
    let mut total: usize = 0;
    for i in 0..defragged.len() {
        if defragged[i] == usize::MAX {
            continue;
        }
        total += i * defragged[i];
    }
    total
}