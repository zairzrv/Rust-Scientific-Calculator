use std::io::{self, Write};
//Scientific Calculator
//By: Zair Ruiz-Velazquez

//accepts prompts and returns user input as a String "-> String"
fn read_line(prompt: &str) -> String{
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

//Enums for operations
#[derive(Debug)]
enum Operation{
    Add,
    Subtract,
    Multiply,
    Divide,
    Percent,
    Power,
    Log,
    Sqrt,
    Fact,
}

//Matches with operation that require two doubles (f64) 1 + 1
impl Operation{
    fn needs_two_numbers(&self) -> bool{
        matches!(self, Operation::Add | Operation::Subtract | Operation::Multiply
        | Operation::Divide | Operation::Percent | Operation::Power)
    }
}

//Determines symbol off operation type Add => "+"
fn symbol (op: &Operation) -> &str{
    match op{
        Operation::Add => "+",
        Operation::Subtract => "-",
        Operation::Multiply => "*",
        Operation::Divide => "/",
        Operation::Percent => "%",
        Operation::Power => "^",
        Operation::Log => "log",
        Operation::Sqrt => "sqrt",
        Operation::Fact => "!",
    }
}

//prompts user for f64s and operation type "Operation: {+}"?
//also tells main to quit or continue loop, Enter key or 'quit'
//calls read_number(), parse_op(), read_line(), calculate(), symbol()
fn run_once() -> Result<bool, String>{
    let first = read_line("First number: ");
    if first.to_lowercase() == "quit"{
        return Ok(false);
    }
    let a: f64 = first.parse().map_err(|_| format!("'{first}' is not a number"))?;
    if !a.is_finite(){      
        return Err("Please enter a real number".to_string());
    }
    let op = parse_op(&read_line("Operation: "))?;
    //condition for operations that utilize two f64s
    let b = if op.needs_two_numbers() {
        read_number("Second number: ")?
    }
    else{
        0.0
    };
    let result = calculate(&op, a, b)?;
    if op.needs_two_numbers(){
        println!("{a} {} {b} = {result}", symbol(&op));
    }
    else{
        if let Operation::Fact = op{
            println!("{a}{} = {result}", symbol(&op));
        }
        else{
            println!("{} {a} = {result}", symbol(&op));
        };
    };
    Ok(true)
}

//converts +, -, /... to Operation enums
fn parse_op(text: &str) -> Result<Operation, String>{
    match text{
        "+" => Ok(Operation::Add),
        "-" => Ok(Operation::Subtract),
        "*" => Ok(Operation::Multiply),
        "/" => Ok(Operation::Divide),
        "%" => Ok(Operation::Percent),
        "^" => Ok(Operation::Power),
        "log" => Ok(Operation::Log),
        "sqrt" => Ok(Operation::Sqrt),
        "!" => Ok(Operation::Fact),
        other => Err(format!("Unknown operation: '{other}'")),
    }
}

//takes a str and converts it to an f64 if it is a valid str
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

//Puts user inputs into actual functions which do the calculations
fn calculate(op: &Operation, a: f64, b: f64) -> Result<f64, String>{
    match op{
        Operation::Add => Ok(add(a, b)),
        Operation::Subtract => Ok(subtract(a, b)),
        Operation::Multiply => Ok(multiply(a, b)),
        Operation::Divide => divide(a, b),
        Operation::Percent => Ok(percent(a, b)),
        Operation::Power => Ok(power(a, b)),
        Operation::Log => log(a),
        Operation::Sqrt => square_root(a),
        Operation::Fact => factorial(a),
    }
}

//add function
fn add (a: f64, b: f64) -> f64{
    a + b
}

//subtract function
fn subtract (a: f64, b: f64) -> f64{
    a - b
}

//multiply function
fn multiply (a: f64, b: f64) -> f64{
    a * b
}

//divide function, with error checking
fn divide (a: f64, b: f64) -> Result<f64, String>{
    if b == 0.0{
        Err("Cannot divide by zero".to_string())
    }
    else{
        Ok(a/b)
    }
}

//percent function
fn percent (a: f64, b: f64) -> f64{
    (a * b) / 100.0
}

//power function
fn power(a: f64, b: f64) -> f64{
    a.powf(b)
}

//logarithm function
fn log (a: f64) -> Result<f64, String>{
    if a < 1.0{
        return Err("Cannot take the logarithm of a negative number or zero".to_string())
    }
    Ok(a.log10())
}

//square root function
fn square_root(a: f64) -> Result<f64, String>{
    if a < 0.0{
        return Err("Cannot take the square root of a negative number".to_string());
    }
    Ok(a.sqrt())
}

//factorial function, with error checking
fn factorial(n: f64) -> Result<f64, String>{
    if n < 0.0 || n.fract() != 0.0 {
        return Err("Factorial needs a non-negative whole number".to_string());
    }
    if n > 170.0{
        return Err("Too large: 171! Is bigger than an f64 can hold".to_string());   
    }
    let mut result = 1.0;
    for i in 1..=(n as u64){
        result *= i as f64;
    }
    Ok(result)
}

//banner at beginning of program
fn banner(){
    println!("===Scientific Calculator=== (type 'quit' to exit).")
}

//main 
fn main() {
    banner();
    //loops through run once function
    //user presses input = continue (true)
    //user types 'quit' = break, (false)
    //Err msg returned from run_once
    loop{
        match run_once() {
            Ok(true) => {}
            Ok(false) => break,
            Err(msg) => println!("Error: {msg}"),
        }
    }
    println!("Goodbye");
}