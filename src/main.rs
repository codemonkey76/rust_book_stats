use std::io;
use std::collections::HashMap;

fn main() {

    let mut numbers = Vec::new();
    // Take a list of numbers from the user
    // stop when they enter a non number
    loop {
        println!("Enter a number: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Expected a number");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        numbers.push(input);
    }
    

    // Calculate the mode, median and mean of the numbers
    // Print the results
    let modes = mode(&numbers);
    print!("Modes: ");
    for (i, mode) in modes.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        print!("{}", mode);
    }
    println!();
    println!("Median: {}", median(&numbers));
    println!("Mean: {}", mean(&numbers));

}

fn mean(numbers: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    sum as f32 / numbers.len() as f32
}

fn mode(vec: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for num in vec {
        *map.entry(num).or_insert(0) += 1;
    }
    let mut max_val = 0;
    let mut modes = Vec::new();
    for (key, val) in map {
        if val > max_val {
            max_val = val;
            modes.clear();
            modes.push(*key);
        } else if val == max_val {
            modes.push(*key);
        }
    }
    modes
}

fn median(numbers: &Vec<i32>) -> i32 {
    let mut numbers = numbers.clone();
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}
