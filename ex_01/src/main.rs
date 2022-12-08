use std::fs::File;
use std::io::Read;

fn main() {
    println!("ex_01 result: {}",ex_01("src/inputs/input.txt"));
    println!("ex_02 result: {}",ex_02("src/inputs/input.txt"));
}

fn read_input(path: &str) -> Vec<i32> {
    let Ok(mut file) = File::open(path) else {
        panic!()
    };

    let mut data = String::new();
    let mut input:Vec<i32> = Vec::new();

    let Ok(_) = file.read_to_string(&mut data) else {
        panic!()
    };

    for line in data.lines() {
        let Ok(value) = line.parse::<i32>() else {
            input.push(-1);
            continue;
        };
        input.push(value);
    }

    return input;
}


fn ex_01(path: &str) -> i32{
    let result = read_input(path);
    let mut curr_max = 0i32;
    let mut temp = 0i32;

    for value in result {
        if value == -1 {
            if temp > curr_max {
                curr_max = temp;
            }

            temp = 0;
        }else {
            temp += value;
        }
    }

    curr_max
}

fn ex_02(path: &str) -> i32{
    let result = read_input(path);
    let mut output:Vec<i32> = vec![];
    let mut temp = 0i32;

    for value in result {
        if value == -1 {
            output.push(temp);
            temp = 0;
        }
        else {
            temp += value;
        }
    }

    output.sort();
    output.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(24_000,ex_01("src/inputs/example.txt"))
    }

    #[test]
    fn input_01() {
        assert_eq!(70_764,ex_01("src/inputs/input.txt"))
    }

    #[test]
    fn input_02() {
        assert_eq!(203_905,ex_02("src/inputs/input.txt"))
    }
}