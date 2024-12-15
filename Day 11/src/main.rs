#![allow(dead_code)]
use std::fs;

fn main() {
    let mut blink_count: u8 = 25;
    let input = match read_input() {
        Ok(_input) => {_input},
        Err(err) => {
            eprint!("{err}");
            err.to_string()
        },
    };

    let mut stones: Vec<u64> = input.split_whitespace().map(|s|s.to_string().parse().expect("Womp womp")).collect();
    loop {
        stones = blink(stones);
        println!(">>{blink_count}<<");
        blink_count -= 1;
        if blink_count == 0{
            println!("There are {:?} stones", stones.len());
            break;
        }
    }
}

fn read_input() -> Result<String, std::io::Error>{
    let contents= fs::read_to_string("./src/input.txt")
        .expect("Error reading input file");
    Ok(contents)
}

fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();
    for stone in stones{
        if stone == 0 {
            output.push(rule_one(stone));
            // println!("{:?}", output.last())
        }
        else if stone.to_string().len() & 1 == 0 {
            output.extend(rule_two(stone).into_iter());
            // println!("{:?}, {:?}", output.last(), output[output.len()-2])
        }
        else {
            output.push(rule_three(stone));
            // println!("{:?}", output.last())
        }
    }

    output
}

fn rule_one(stone: u64) -> u64{ stone + 1 }

fn rule_two(stone: u64) -> Vec<u64> {
    let mut digits = Vec::new();
    let mut n = stone;

    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();

    let mid = digits.len() / 2;
    let lhs = digits[..mid].iter().fold(0, |acc, &d| acc * 10 + d);
    let rhs = digits[mid..].iter().fold(0, |acc, &d| acc * 10 + d);

    let result = vec![lhs, rhs];
    result
}

fn rule_three(stone: u64) -> u64{ stone.wrapping_mul(2024)}

