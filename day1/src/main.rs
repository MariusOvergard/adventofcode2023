use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {

    let number_converter = HashMap::from([
        (String::from("one"), String::from("1")),
        (String::from("two"), String::from("2")),
        (String::from("three"), String::from("3")),
        (String::from("four"), String::from("4")),
        (String::from("five"), String::from("5")),
        (String::from("six"), String::from("6")),
        (String::from("seven"), String::from("7")),
        (String::from("eight"), String::from("8")),
        (String::from("nine"), String::from("9")),
    ]);

    let file_path = "input.txt";

    let pattern = "one|three|four|five|six|nine|seven|two|eight";

    let re: Regex  = Regex::new(format!("{}{}", r"[0-9]|", pattern).as_str()).unwrap();
    let reversed: Regex  = Regex::new(format!("{}{}", r"[0-9]|", pattern.chars().rev().collect::<String>()).as_str()).unwrap();

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let result: Vec<_> = contents.lines().collect();

    let mut sum = 0;

    for line in result.iter() {
        let digits:String  = re.find_iter(line)
        .filter_map(|d| {
            let v = d.as_str();
            if number_converter.contains_key(v) {
                let val = &number_converter[v];
                val.parse::<String>().ok()
            } else{
                v.parse::<String>().ok()
            }
        })
        .collect();

        let re_digits:String  = reversed.find_iter(&line.chars().rev().collect::<String>())
        .filter_map(|d| {
            let v = d.as_str().chars().rev().collect::<String>();
            if number_converter.contains_key(&v) {
                let val = &number_converter[&v];
                val.parse::<String>().ok()
            } else{
                v.parse::<String>().ok()
            }
        })
        .collect();

        let first = digits.chars().rev().last().unwrap();
        let last = re_digits.chars().rev().last().unwrap();
        let number = format!("{}{}", first, last);

        sum += number.parse::<i32>().unwrap();
    }

    println!("{}", sum);
}
