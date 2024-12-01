#![allow(dead_code)]
use std::fs;

fn main() {
    let input = read_input();
    let full_list = match input{
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("{err}");
            String::new()
        },
    };
    let list_col = split_input(full_list);
    calculate_total_dist(list_col)
}

fn read_input() -> Result<String, std::io::Error>{
    let contents= fs::read_to_string("./src/input.txt")
        .expect("Error reading input file");
    Ok(contents)
}

fn split_input(input: String) -> (Vec<i32>, Vec<i32>){
    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();
    for line in input.lines(){
        let mut values = line.split("   ")
            .map(|num| num.parse::<i32>());

        if let (Some(Ok(val1)), Some(Ok(val2))) = (values.next(), values.next()) {
            list_one.push(val1);
            list_two.push(val2);
        }
    }
    list_one.sort();
    list_two.sort();
    for n in 0..=list_one.len()-1{
        println!("{:?}, {:?}", list_one[n], list_two[n])
    }
    (list_one, list_two)
}

fn calculate_total_dist(list_col: (Vec<i32>, Vec<i32 >)){
    let one = list_col.0;
    let two = list_col.1;
    let idx = one.len() - 1;
    let mut sum: i128 = 0;

    for n in 0..=idx{
        let value = one[n] - two[n];
        sum += std::convert::Into::<i128>::into(one[n].abs_diff(two[n]))
    }
    println!("{sum}")
}
