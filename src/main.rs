//Scientific Calculator
//By: Zair Ruiz-Velazquez

//Operation enums
use std::io::{self, Write};

fn read_line(prompt: &str) -> String{
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

#[derive(Debug)]
enum Operation{
    Add,
    Subtract,
    Multiply,
    Percent,
    Divide,
}

fn symbol (op: &Operation) -> &str{
    match op{
        Operation::Add => "+",
        Operation::Subtract => "-",
        Operation::Multiply => "*",
        Operation::Divide => "/",
        Operation::Percent => "%",
    }
}

fn run_once() -> Result<(), String>{
    let a = read_number("First number: ")?;
    let op = parse_op(&read_line("Operation: "))?;
    let b = read_number("Second number: ")?;
    let result = calculate(&op, a, b)?;
    println!("{a} {} {b} = {result}", symbol(&op));
    Ok(())
}

fn parse_op(text: &str) -> Result<Operation, String>{
    match text{
        "+" => Ok(Operation::Add),
        "-" => Ok(Operation::Subtract),
        "*" => Ok(Operation::Multiply),
        "/" => Ok(Operation::Divide),
        "%" => Ok(Operation::Percent),
        other => Err(format!("Unkown operation: '{other}'")),
    }
}

fn read_number(prompt: &str) -> Result<f64, String>{
    let input = read_line(prompt);
    let n: f64 = input.parse().map_err(|_| format!("'{input}' is not a number"))?;
    if !n.is_finite(){
        return Err("Please enter a real number".to_string());
    }
    else{
    Ok(n)
    }
}

fn calculate(op: &Operation, a: f64, b: f64) -> Result<f64, String>{
    match op{
        Operation::Add => Ok(add(a, b)),
        Operation::Subtract => Ok(subtract(a, b)),
        Operation::Multiply => Ok(multiply(a, b)),
        Operation::Divide => divide(a, b),
        Operation::Percent => Ok(percent(a, b)),
    }
}

fn add (a: f64, b: f64) -> f64{
    a + b
}

fn subtract (a: f64, b: f64) -> f64{
    a - b
}

fn multiply (a: f64, b: f64) -> f64{
    a * b
}

fn divide (a: f64, b: f64) -> Result<f64, String>{
    if b == 0.0{
        Err("Cannot divide by zero".to_string())
    }
    else{
        Ok(a/b)
    }
}

fn percent (a: f64, b: f64) -> f64{
    (a * b) / 100.0
}

fn banner(){
    println!("===Scientific Calculator=== (type 'quit' to exit).")
}

fn main() {
    banner();
    loop{
        if let Err(msg) = run_once(){
            println!("Error: {msg}");
        }
        let answer = read_line("Continue? (Enter = yes, 'quit' = exit): ");
        if answer.to_lowercase() == "quit"{
            break;
        }
    }
    println!("Goodbye");
}
