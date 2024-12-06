use std::collections::{HashMap, HashSet};

fn get_input() -> String {
    String::from(include_str!("./input.txt"))
}

pub fn day_five_part_one() {
    let input = get_input();
    let mut pages_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut updates: Vec<&str> = Vec::new();
    
    parse_input(&input, &mut pages_map, &mut updates);
    
    let mut total = 0;
    
    'update: for s in updates {
        let pages = s
            .split(",")
            .collect::<Vec<&str>>();
        for i in 1..pages.len() {
            if !pages_map.get(&pages[i]).unwrap().contains(&pages[i-1]) {
                continue 'update;
            }
        }
        total += pages[(pages.len()-1)/2].parse::<i32>().unwrap();
        
    }
    println!("{}", total);
}

pub fn day_five_part_two() {
    let input = get_input();
    let mut pages_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut updates: Vec<&str> = Vec::new();

    parse_input(&input, &mut pages_map, &mut updates);

    let mut total = 0;

    for s in updates {
        let mut pages = s
            .split(",")
            .collect::<Vec<&str>>();
        let mut had_error = false;

        let mut i = 1;
        while i < pages.len() {
            if !pages_map.get(&pages[i]).unwrap().contains(&pages[i - 1]) {
                pages.swap(i, i - 1);
                had_error = true;
                i = 1;
                continue;
            }
            i += 1;
        }
        
        if had_error {
            total += pages[(pages.len()-1)/2].parse::<i32>().unwrap();
        }
    }
    println!("{}", total);
}

fn parse_input<'a>(input: &'a str, pages_map: &mut HashMap<&'a str, HashSet<&'a str>>, updates: &mut Vec<&'a str>) {
    let mut map_parsed = false;
    for line in input.lines() {
        if line.trim().is_empty() {
            map_parsed = true;
            continue;
        }
        if !map_parsed {
            populate_pages_map(pages_map, line);
        } else {
            updates.push(line);
        }
    }
}

fn populate_pages_map<'a>(pages_map: &mut HashMap<&'a str, HashSet<&'a str>>, line: &'a str) {
    let pages = line.split("|").collect::<Vec<&str>>();
    let parent = pages[1];
    let child = pages[0];
    if pages_map.contains_key(parent) {
        pages_map.get_mut(parent).unwrap().insert(child);
    } else {
        pages_map.insert(parent, HashSet::from([child]));
    }
}