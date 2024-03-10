use crate::math;
use std::io;

fn main() {
    let mut num1: String = String::new();
    let mut num2 = String::new();
    let mut op = String::new();

    println!("Enter first number: ");
    io::stdin().read_line(&mut num1).expect("Invalid input");
    println!("Enter operator (+, -, *, /): ");
    io::stdin().read_line(&mut op).expect("Invalid input");
    println!("Enter second number: ");
    io::stdin().read_line(&mut num2).expect("Invalid input");

    let first_num: f64 = num1.trim().parse().expect("Invalid number");
    let second_num: f64 = num2.trim().parse().expect("Invalid number");
    let result = match op.trim() {
        "+" => math::add(first_num, second_num),
        "-" => math::subtract(first_num, second_num),
        "*" => math::multiply(first_num, second_num),
        "/" => math::divide(first_num, second_num),
        _ => panic!("Unsupported operator!"),
    };

    println!("Result: {}", result);
}


