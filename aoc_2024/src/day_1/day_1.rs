struct Lists {
    list_a: Vec<i32>,
    list_b: Vec<i32>,
}

fn get_input() -> String {
    String::from(include_str!("./input.txt"))
}

fn get_lists(input: &str) -> Lists {
    let mut lists = Lists {
        list_a: Vec::new(),
        list_b: Vec::new(),
    };

    input
        .split("\n")
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .for_each(|x| {
            lists.list_a.push(x[0].parse::<i32>().unwrap());
            lists.list_b.push(x[1].parse::<i32>().unwrap());
        });
    lists
}

pub fn day_one_part_one() {
    let input = get_input();
    let mut lists = get_lists(&input);

    lists.list_a.sort();
    lists.list_b.sort();
    let mut total : i32 = 0;
    for i in 0..lists.list_a.len() {
        total += (lists.list_a[i] - lists.list_b[i]).abs();
    }
    println!("{}", total);
}

pub fn day_one_part_two() {
    let input = get_input();
    let lists = get_lists(&input);

    let mut overall_val : i32 = 0;
    for x in lists.list_a {
        let mut total : i32 = 0;
        for y in 0..lists.list_b.len() {
            if x == lists.list_b[y] {
                total += 1;
            }
        }
        overall_val += x * total;
    }
    println!("{}", overall_val);
}