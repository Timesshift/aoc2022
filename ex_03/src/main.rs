use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("ex_01 : {}",ex_01("src/inputs/input.txt"));
    println!("ex_02 : {}",ex_02("src/inputs/input.txt"));
}

fn ex_01(path: &str) -> i32 {
    let results = read_input_01(path);
    let mut result = 0i32;
    for item in results {
        if item.is_uppercase() {
            result += map_value(item.to_ascii_lowercase()) + 26;
        } else {
            result += map_value(item);
        }
    }

    result
}

fn ex_02(path: &str) -> i32 {
    let results = read_input_02(path);
    let mut result = 0i32;
    for item in results {
        if item.is_uppercase() {
            result += map_value(item.to_ascii_lowercase()) + 26;
        } else {
            result += map_value(item);
        }
    }

    result
}

fn read_input_01(path: &str) -> Vec<char> {
    let Ok(mut file) = File::open(path) else {
        panic!()
    };

    let mut data = String::new();
    let mut result: Vec<char> = Vec::new();
    let Ok(_) = file.read_to_string(&mut data) else {
        panic!()
    };

    for line in data.lines(){
        let mut cp1_charcters = HashSet::new();
        let mut cp2_charcters = HashSet::new();
        let compartments = line.split_at(line.len() / 2);

        for chars in compartments.0.chars() {
            cp1_charcters.insert(chars);
        }

        for chars in compartments.1.chars() {
           cp2_charcters.insert(chars);
        }

        let intersection = cp1_charcters
            .intersection(&cp2_charcters);

            for item in intersection {
                result.push(*item);
            }
    }

   result
}

fn read_input_02(path: &str) -> Vec<char> {
    let Ok(mut file) = File::open(path) else {
        panic!()
    };

    let mut data = String::new();
    let mut result: Vec<char> = Vec::new();
    let mut s: Vec<HashSet<char>> = Vec::new();
    let mut count = 0;

    s.push(HashSet::new());
    s.push(HashSet::new());
    s.push(HashSet::new());

    let Ok(_) = file.read_to_string(&mut data) else {
        panic!()
    };

    for line in data.lines() {
        for characters in line.chars() {
            s[count].insert(characters);
        }
        count +=1;

            if count == 3 {
                let intersection = s[0].intersection(&s[1]);
                
                for item in intersection {
                    if s[2].contains(item) {
                        result.push(*item);
                        break;
                    }
                    else {
                        continue
                    }
                }
                s[0].clear();
                s[1].clear();
                s[2].clear();
                count = 0;
             }
    }

result
}

fn map_value(input: char) -> i32 {
    match input {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        _ => panic!()
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        assert_eq!(157,ex_01("src/inputs/example.txt"))
    }

    #[test]
    fn example_02() {
        assert_eq!(70,ex_02("src/inputs/example.txt"))
    }

    #[test]
    fn input_01() {
        assert_eq!(8153,ex_01("src/inputs/input.txt"))
    }

    #[test]
    fn input_02() {
        assert_eq!(2342,ex_02("src/inputs/input.txt"))
    }
}