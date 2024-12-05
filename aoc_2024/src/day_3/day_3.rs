use regex::Regex;

fn get_input() -> String {
    String::from(include_str!("./input.txt"))
}

pub fn day_three_part_one() {
    let input = get_input();
    let init_re = Regex::new("mul\\(\\d{1,3},\\d{1,3}\\)").unwrap();
    let numb_re = Regex::new("\\d{1,3}").unwrap();
    let total = init_re
        .find_iter(&input)
        .map(|m| m.as_str())
        .map(|s| numb_re.find_iter(s).map(|m|m.as_str()).collect::<Vec<&str>>())
        .map(|v| v.iter().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>())
        .map(|v| v.iter().product::<u32>())
        .sum::<u32>();
    println!("{}", total);
}

pub fn day_three_part_two() {
    let input = get_input();
    let init_re = Regex::new("(^|do[^n]).*?($|don't)").unwrap();
    let mul_re = Regex::new("mul\\(\\d{1,3},\\d{1,3}\\)").unwrap();
    let numb_re = Regex::new("\\d{1,3}").unwrap();

    let filtered_muls = init_re
        .find_iter(&input)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>()
        .join("|");

    let total = mul_re
        .find_iter(&filtered_muls)
        .map(|m| m.as_str())
        .map(|s| numb_re.find_iter(s).map(|m|m.as_str()).collect::<Vec<&str>>())
        .map(|v| v.iter().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>())
        .map(|v| v.iter().product::<u32>())
        .sum::<u32>();
    println!("{}", total);
}