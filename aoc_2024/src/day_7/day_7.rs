use std::str::FromStr;
use crate::day_7::day_7::OpCount::{THREE, TWO};

fn get_input() -> String {
    String::from(include_str!("./input.txt"))
}

pub fn day_seven_part_one() {
    let input = get_input();
    do_day_seven(input, TWO);
}

pub fn day_seven_part_two() {
    let input = get_input();
    do_day_seven(input, THREE);
}

fn do_day_seven(input: String, op_count: OpCount) {
    let total: i64 = input
        .lines()
        .map(|line| Calc::new(line, op_count.clone()))
        .filter(|calc| calc.can_be_valid())
        .map(|calc| calc.total)
        .sum();
    println!("{}", total);
}

struct Calc {
    total: i64,
    numbers: Vec<i32>,
    op_count: OpCount,
}

#[derive(PartialEq, Eq, Clone)]
enum OpCount {
    TWO,
    THREE
}

impl Calc {
    pub fn new(input_str: &str, op_count: OpCount) -> Self {
        let split = input_str.split(":").collect::<Vec<&str>>();
        let total = split[0].trim().parse::<i64>().unwrap();
        let numbers = split[1]
            .trim()
            .split_whitespace()
            .map(i32::from_str)
            .map(Result::unwrap)
            .collect::<Vec<i32>>();
        Self { total, numbers, op_count }
    }

    fn can_be_valid(&self) -> bool {
        self.build_operators()
            .iter()
            .map(|vec| self.do_calc(&vec))
            .any(|output| output == self.total)
    }
    
    fn build_operators(&self) -> Vec<Vec<char>> {
        if self.op_count == TWO {
            self.build_operators_base_2()
        } else {
            self.build_operators_base_3()
        }
    }

    fn build_operators_base_2(&self) -> Vec<Vec<char>> {
        let mut operators: Vec<Vec<char>> = vec![];
        let op_combos = 2i32.pow((self.numbers.len() - 1) as u32);
        for i in 0..op_combos {
            let str = format!("{:032b}", i);
            let mut char_vec = str
                .chars()
                .rev()
                .take(self.numbers.len() - 1)
                .map(|c| return if c == '0' { '*' } else { '+' })
                .collect::<Vec<char>>();
            char_vec.reverse();
            operators.push(char_vec);
        }
        operators
    }

    fn build_operators_base_3(&self) -> Vec<Vec<char>> {
        let mut operators: Vec<Vec<char>> = vec![];
        let op_combos = 3i32.pow((self.numbers.len() - 1) as u32);
        for i in 0..op_combos {
            let char_vec = self.to_base_three(i, (self.numbers.len() - 1) as u32)
                .chars()
                .map(|c| return if c == '0' { '*' } else if c == '1' { '+' } else { '|' })
                .collect::<Vec<char>>();
            operators.push(char_vec);
        }
        operators
    }
    
    fn to_base_three(&self, number: i32, padding: u32) -> String {
        let mut total = number;
        let mut base_3_str = String::new();
        while total >= 1 {
            let rem = total % 3;
            total /= 3;
            base_3_str.push_str(&rem.to_string());
        }
        while base_3_str.len() < padding as usize {
            base_3_str.push('0');
        }
        base_3_str.chars().rev().collect::<String>()
    }

    fn do_calc(&self, operators: &[char]) -> i64 {
        let mut i = 0;
        let mut total = self.numbers[i] as i64;
        for op in operators {
            i += 1;
            if *op == '*' {
                total *= self.numbers[i] as i64;
            } else if *op == '+' {
                total += self.numbers[i] as i64;
            } else if *op == '|' {
                let curr_num_str = self.numbers[i].to_string();
                let mut curr_tot_str = total.to_string();
                curr_tot_str.push_str(&curr_num_str);
                total = curr_tot_str.parse::<i64>().unwrap();
            }
        }
        total
    }
}