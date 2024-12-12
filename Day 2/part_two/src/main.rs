#![allow(dead_code)]
use std::fs;
fn main() {
    let input = read_input();
    let rows = get_rows(input);
    parse_rows(rows);
}

fn read_input() -> Result<String, std::io::Error>{
    let contents= fs::read_to_string("../src/input.txt")
        .expect("Error reading input file");
    Ok(contents)
}

fn get_rows(input: Result<String, std::io::Error>) -> Vec<Vec<i32>>{
    let mut rows: Vec<Vec<i32>> = Vec::new();
    let list = match input{
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error parsing lines: {err}");
            String::new()
        },
    };

    for line in list.lines(){
        let mut inner_row: Vec<i32> = Vec::new();
        let row = line.split(" ");

        for n in row{
            let num = n.parse::<i32>();
            match num{
                Ok(n) => inner_row.push(n),
                Err(err) => eprintln!("Error: {err}")
            }
        }

        rows.push(inner_row);
    }
    println!("{:?}", rows.len());
    rows
}

fn parse_rows(rows: Vec<Vec<i32>>){
    let mut valid_rows: u16 = 0;
    'outer: for mut row in rows{
        let mut len = row.len();
        let diff_dir = get_diff((row[0], row[1]));
        let increasing: bool = diff_dir.1;
        let mut dampened: bool = false;

        println!("{:?}", row.len()-1);
        for n in 0..len-1{
            let calc = get_diff((row[n], row[n+1]));
            let diff = calc.0;
            let mut is_valid: bool = false;
            if diff.abs() > 3 || diff == 0 || calc.1 != increasing{
                if !dampened{
                    dampened = true;
                    is_valid = retry_permutations(row);
                    if is_valid{
                        valid_rows += 1;
                    }
                }
                continue 'outer;
            }
        }
        valid_rows += 1;
        println!("Found {:?} valid rows", valid_rows);
    }
}

fn retry_permutations(row: Vec<i32>) -> bool{
    let diff_dir = get_diff((row[0], row[1]));
    let increasing: bool = diff_dir.1;
    let is_valid: bool = false;
    println!("Retry: {:?}", row.len()-1);
    for n in 0..row.len()-1{
        let diff = diff_dir.0;
        if diff.abs() > 3 || diff == 0 || diff_dir.1 != increasing{
            continue;
        }
    }
    println!("{is_valid}");
    is_valid
}

fn get_diff(ints: (i32, i32)) -> (i32, bool){
    let mut diff: i32;
    let mut increasing: bool = false;

    diff = ints.0 - ints.1;
    if diff == 0{
        increasing = false;
        return (diff, increasing)
    }

    if ints.0 < ints.1{
        increasing = true;
        diff *= -1;
    };
    (diff, increasing)
}

