use std::fs::File;
use std::io::Read;

fn main() {
    println!("ex01 result: {}",ex_01("src/inputs/input.txt"));
    println!("ex02 result: {}",ex_02("src/inputs/input.txt"));
}


fn read_input(path: &str, index:usize) -> i32{
    let Ok(mut file) = File::open(path) else {
        panic!()
    };

    let mut data = String::new();
    let mut result =vec![0,0];
    let Ok(_) = file.read_to_string(&mut data) else {
        panic!()
    };

    for line in data.lines() {
       let (p_one,p_two) = line.split_once(',').unwrap();

        let a_idx: Vec<&str> = p_one.split('-').collect();
        let a_start = a_idx[0].parse::<u32>().unwrap();
        let a_end = a_idx[1].parse::<u32>().unwrap();
    
        let b_idx: Vec<&str> = p_two.split('-').collect();
        let b_start = b_idx[0].parse::<u32>().unwrap();
        let b_end = b_idx[1].parse::<u32>().unwrap();
        
        if (a_start <= b_start && a_end >= b_end) | (a_start >= b_start && a_end <= b_end) { result[0] += 1; }
    
        let a: Vec<u32> = (a_start..=a_end).collect();
        let b: Vec<u32> = (b_start..=b_end).collect();
    
        if a.iter().any(|i| b.contains(i)){ result[1] += 1; }
    }

result[index]
}

fn ex_01(path: &str) -> i32 {
    read_input(path,0)
}

fn ex_02(path: &str) -> i32 {
    read_input(path,1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        assert_eq!(2,ex_01("src/inputs/example.txt"))
    }

    #[test]
    fn input_01() {
        assert_eq!(450,ex_01("src/inputs/input.txt"))
    }

    #[test]
    fn input_02() {
        assert_eq!(837,ex_02("src/inputs/input.txt"))
    }
}
