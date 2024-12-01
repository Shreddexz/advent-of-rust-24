#![allow(dead_code)]
use std::{collections::HashMap, fs};

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
    let sim_dict = get_number_similarity(list_col);
    let similarity_score = calculate_similarity_score(sim_dict);
    println!("{similarity_score}")
}

fn read_input() -> Result<String, std::io::Error>{
    let contents= fs::read_to_string("../src/input.txt")
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
    (list_one, list_two)
}

fn get_number_similarity(list_col: (Vec<i32>, Vec<i32 >)) -> HashMap<i32, i32>{
    let one = list_col.0;
    let two = list_col.1;
    let idx = one.len() - 1;
    let mut dict: HashMap<i32, i32> = HashMap::new();

    (0..=idx).for_each(|n| {
        let mut occ = 0;
        for num in two.clone(){
            if num == one[n]{
                occ += 1;
            }
        }
        if occ > 0{
            dict.insert(one[n], occ);
        }
    });
    for ele in &dict {
        println!("{:?}, {:?}", ele.0, ele.1)
    }
    dict
}

fn calculate_similarity_score(dict: HashMap<i32, i32>) -> i32{
    let mut similarity: i32  = 0;
    for num in &dict {
        let sim = num.0 * num.1;
        similarity += sim;
    }
    similarity
}
