use std::fs::File;
use std::io::Read;

fn main() {
    println!("ouput ex_01: {}", ex_01("src/inputs/input.txt"));
    println!("ouput ex_02: {}", ex_02("src/inputs/input.txt"));
}

fn read_input(path: &str) -> Vec<(String, String)> {
    let Ok(mut file) = File::open(path) else {
        panic!()
    };

    let mut data = String::new();
    let mut input:Vec<(String,String)> = Vec::new();

    let Ok(_) = file.read_to_string(&mut data) else {
        panic!()
    };

    for line in data.lines() {
        let Ok(item) = line.parse::<String>() else {
            panic!()
        };

        let mut split = item.split(' ');
        
        let enemy_move = split.next().unwrap().to_string();
        let my_move = split.next().unwrap().to_string();

        input.push((enemy_move,my_move));
    }

    return input;
}


fn ex_01(path: &str) -> i32 {
    let mut result = 0i32;

    for p_res in read_input(path) {
        result += handle_01((&p_res.0.as_str(),&p_res.1.as_str()));
    };

    result
}

fn ex_02(path: &str) -> i32 {
    let mut result = 0i32;

    for p_res in read_input(path) {
        result += handle_02((&p_res.0.as_str(),&p_res.1.as_str()));
    };

    result
}

fn handle_01(input: (&str,&str)) -> i32 {
    match input {
        ("A","X") => 4, 
        ("A","Y") => 8, 
        ("A","Z") => 3,  
        ("B","X") => 1, 
        ("B","Y") => 5, 
        ("B","Z") => 9, 
        ("C","X") => 7, 
        ("C","Y") => 2, 
        ("C","Z") => 6, 
        _ => panic!()
    }
}

fn handle_02(input: (&str,&str)) -> i32 {
    match input {
        ("A","X") => 3,
        ("A","Y") => 4, 
        ("A","Z") => 8, 
        ("B","X") => 1, 
        ("B","Y") => 5, 
        ("B","Z") => 9,
        ("C","X") => 2, 
        ("C","Y") => 6, 
        ("C","Z") => 7,
        _ => panic!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(ex_01("src/inputs/example.txt"),15);
    }

    #[test]
    fn ex_01_test(){
        assert_eq!(ex_01("src/inputs/input.txt"),9177);
    }

    #[test]
    fn ex_02_test(){
        assert_eq!(ex_02("src/inputs/input.txt"),12111);
    }

}

