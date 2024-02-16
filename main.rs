/*
Jon Bennett
CISS 301
Lab3 02/10/2024
*/

//Discription:
//This program runs a Fibonacci Squence, Factorial, and digit of pi in a user friendly manner
//on user input of a positive integer.  



use std::io;

// Section of code to do factorial
fn factorial(num: u128) -> u128 {
    match num {
        0 | 1 => 1,
        _ => factorial(num - 1) * num,
    }
}

// Section of code to calculate the Fibonacci sequence
fn fibonacci(n: u128, memo: &mut Vec<u128>) -> u128 {
    if n <= 0 {
        return n;
    } else if n == 1 {
        return 1;
    }
    if n as usize >= memo.len() {
        memo.resize(n as usize + 1, 0);
    }
    if memo[n as usize] == 0 {
        memo[n as usize] = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
    }
    memo[n as usize]
}

// Section of code that does the equation of (3/2)^(n-1)
fn calculate_power(n: u128) -> f64 {
    let base: f32 = 1.5;
    let powerof = n - 1;
    let result = base.powf(powerof as f32);
    result.into()
}

// Calculating digits of pi
fn pi_digit(n: u64) -> u64 {
    let pi_string = format!("{:.1$}", std::f64::consts::PI, n as usize + 1);
    let pi_char = pi_string.chars().nth(n as usize + 1).unwrap();
    pi_char.to_digit(10).unwrap() as u64
}

//Main portion of the program that takes a positive integer from the user and allows them to run three set of information
//at the sametime. That being the Fibonacci Sequence, The Factorial, and the digit of pi. While providing the limit information 
//of the system to provide expected incounters to come within the program. 
fn main() {
    println!("Fibonacci generator\n");
    println!("The limits of each section in order as they are read from left to right are:\n 12, 15, 35\nYou will not be able to go pass 35 :) enjoy");
    println!("Type 'quit' to end the program");
    let mut memo: Vec<u128> = vec![0, 1];
    loop {
        let mut input = String::new();
        println!("\nEnter a positive integer:");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "quit" {
            break;
        }

        let n: u128 = match input.trim().parse() {
            Ok(num) if num == 0 =>{
                println!("Zero for all lets try to do something bigger then zero :)");
                continue;
            }
            Ok(num) if num >=35 =>{
                println!("You have reached the limit for all 3 sections, lets stay below 35 :)");
                continue;
            }
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid integer.");
                continue;
            }
        };

        for i in n..(n + 1) {
            let fib_number = fibonacci(i, &mut memo);
            let digits = pi_digit(n.try_into().unwrap());
            println!("| If n = {} then (3/2)^(n-1) = {} | Factorial is: {} |The {}th digit of pi is {}",fib_number,calculate_power(fib_number),factorial(i),n,digits);
        }
    }
}
