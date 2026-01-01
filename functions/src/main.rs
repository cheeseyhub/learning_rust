use std::io;
fn celcius_to_farenheint(x: i32) -> i32 {
    (x * 9 / 5) + 32
}
fn get_integer() -> i32 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read the line ");
    input_text
        .trim()
        .parse::<i32>()
        .expect("Invalid Input:not a valid Integer")
}

fn nth_fib(n: i32) -> i32 {
    if n <= 2 {
        return n - 1;
    }
    return nth_fib(n - 1) + nth_fib(n - 2);
}

fn main() {
    //Make a program that converts Fahrenheit and Celcius;
    println!("Enter temperature in celcius : ");

    let temperate_in_celcius = get_integer();

    println!(
        "The temperature in farenheint is {}",
        celcius_to_farenheint(temperate_in_celcius)
    );

    println!("The {}th fibonacci is {}", 8, nth_fib(8));
}
