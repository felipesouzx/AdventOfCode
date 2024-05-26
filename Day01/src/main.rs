use std::fs;
use std::collections::HashMap;


fn reverse_string(target: &String) -> String {
    let mut result: String = String::new();
    for ch in target.chars().rev() {
        result.push(ch);
    }
    return result;
}


fn main() {
    let mut spelled_numbers: HashMap<&str, char> = HashMap::new();
    spelled_numbers.insert("one", '1');
    spelled_numbers.insert("two", '2');
    spelled_numbers.insert("three", '3');
    spelled_numbers.insert("four", '4');
    spelled_numbers.insert("five", '5');
    spelled_numbers.insert("six", '6');
    spelled_numbers.insert("seven", '7');
    spelled_numbers.insert("eight", '8');
    spelled_numbers.insert("nine", '9');


    let file_path = "src/input.txt";
    let file_content = fs::read_to_string(file_path)
        .expect("Should've been a file.");


    let mut result: i32 = 0;

    for line in file_content.lines() {
        let mut line_result: String = String::new();

        // First Number
        let mut str_progress: String = String::new();
        for ch in line.chars() {
            if ch.is_numeric() {
                line_result.push(ch);
                break;
            }
            str_progress.push(ch);
            for (spelled, number) in spelled_numbers.iter() {
                if str_progress.contains(spelled) {
                    line_result.push(number.clone());
                    break;
                }
            }
            if line_result.is_empty() {
                continue;
            }
            break;
        }

        // Second number
        str_progress.clear();
        for ch in line.chars().rev() {
            if ch.is_numeric() {
                line_result.push(ch);
                break;
            }
            str_progress.push(ch);
            for (spelled, number) in spelled_numbers.iter() {
                let reverse_spelled = reverse_string(&spelled.to_string());
                if str_progress.contains(&reverse_spelled) {
                    line_result.push(number.clone());
                    break;
                }
            }
            if line_result.len() < 2 {
                continue;
            }
            break;
        }

        result += str::parse::<i32>(&line_result).unwrap();
    }


    println!("Your result is: {}", result);
}