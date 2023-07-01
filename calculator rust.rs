enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(operator: Operator, num1: f64, num2: f64) -> Option<f64> {
    match operator {
        Operator::Add => Some(num1 + num2),
        Operator::Subtract => Some(num1 - num2),
        Operator::Multiply => Some(num1 * num2),
        Operator::Divide => {
            if num2 != 0.0 {
                Some(num1 / num2)
            } else {
                None
            }
        }
    }
}

fn main() {
    println!("Simple Calculator");

    let num1: f64 = 10.0;
    let num2: f64 = 5.0;

    let add_result = calculate(Operator::Add, num1, num2);
    let sub_result = calculate(Operator::Subtract, num1, num2);
    let mul_result = calculate(Operator::Multiply, num1, num2);
    let div_result = calculate(Operator::Divide, num1, num2);

    match add_result {
        Some(result) => println!("Addition Result: {}", result),
        None => println!("Addition: Invalid operation (division by zero)"),
    }

    match sub_result {
        Some(result) => println!("Subtraction Result: {}", result),
        None => println!("Subtraction: Invalid operation (division by zero)"),
    }

    match mul_result {
        Some(result) => println!("Multiplication Result: {}", result),
        None => println!("Multiplication: Invalid operation (division by zero)"),
    }

    match div_result {
        Some(result) => println!("Division Result: {}", result),
        None => println!("Division: Invalid operation (division by zero)"),
    }
}
