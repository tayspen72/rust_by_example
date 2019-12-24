mod ch01_hello_world;
mod ch02_primatives;
mod ch03_custom_types;
mod ch04_variable_bindings;
mod ch05_types;
mod ch06_conversion;
mod ch07_expressions;
mod ch08_flow_control;

use std::io;

fn print_summary(){
    println!("Chapter Summary:");
    println!("\tChapter 1: Basic Hello World");
    println!("\tChapter 2: Primatives");
    println!("\tChapter 3: Custom Types");
    println!("\tChapter 4: Variable Bindings");
    println!("\tChapter 5: Types");
    println!("\tChapter 6: Conversion");
    println!("\tChapter 7: Expressions");
    println!("\tChapter 8: Flow Control");
    println!("");
}

fn main(){
    println!("Welcom to Rust By Example!");
    print_summary();

    println!("Enter a chapter to review:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim() {
        "1" => ch01_hello_world::hello_world(),
        "2" => ch02_primatives::primatives(),
        "3" => ch03_custom_types::custom_types(),
        "4" => ch04_variable_bindings::variable_bindings(),
        "5" => ch05_types::types(),
        "6" => ch06_conversion::conversion(),
        "7" => ch07_expressions::expressions(),
        "8" => ch08_flow_control::flow_control(),
        _ => println!("Invalid response {}. Quitting...", input)
    }
}