#[derive(Debug)]
enum Operation{
    Add,
    Subtract,
    Multiply,
    Percent,
}

fn symbol (op: &Operation) -> &str{
    match op{
        Operation::Add => "+",
        Operation::Subtract => "-",
        Operation::Multiply => "*",
        Operation::Percent => "%",
    }
}

fn calculate(op: &Operation, a: f64, b: f64) -> f64{
    match op{
        Operation::Add => add(a, b),
        Operation::Subtract => subtract(a, b),
        Operation::Multiply => multiply(a, b),
        Operation::Percent => percent(a, b),
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

fn percent (a: f64, b: f64) -> f64{
    (a * b) / 100.0
}

fn main() {
    let op = Operation::Multiply;
    let result = calculate(&op, 5.0, 3.0);
    println!("5 {} 3 = {result}", symbol(&op) );
    }
