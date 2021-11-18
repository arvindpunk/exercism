#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    let mut valid = true;

    for input in inputs.iter() {
        let result = match input {
            CalculatorInput::Value(x) => {
                stack.push(x.to_owned());
                let d: Result<i32, i32> = Result::Ok(x.to_owned());
                d
            },
            x => {
                let a = stack.pop();
                let b = stack.pop();
                let result = match x {
                    CalculatorInput::Add => perform_operation(b, a, i32::checked_add),
                    CalculatorInput::Subtract => perform_operation(b, a, i32::checked_sub),
                    CalculatorInput::Multiply => perform_operation(b, a, i32::checked_mul),
                    CalculatorInput::Divide => perform_operation(b, a, i32::checked_div),
                    _ => None
                };
                if result.is_some() {
                    stack.push(result.unwrap());
                    Result::Ok(0)
                } else {
                    valid = false;
                    Result::Err(-1)
                }
            }
        };
        if result.is_err() {
            break;
        }
    }
    if valid && stack.len() == 1 {
        match stack.first() {
            Some(&x) => Some(x),
            _ => None
        }
    } else {
        None
    }
}

pub fn perform_operation(a: Option<i32>, b: Option<i32>, 
        f: fn(i32, i32) -> Option<i32>) -> Option<i32> {
    if a.is_some() && b.is_some() {
        let result = f(a.unwrap(), b.unwrap());
        result
    } else {
        None
    }
}