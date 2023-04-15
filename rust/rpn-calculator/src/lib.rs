#[derive(Debug, Clone)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();
    for arg in inputs {
        let result = match arg {
            CalculatorInput::Value(value) => {
                stack.push(*value);
                Ok(())
            }
            CalculatorInput::Add => op(&mut stack, |a, b| a + b),
            CalculatorInput::Subtract => op(&mut stack, |a, b| a - b),
            CalculatorInput::Multiply => op(&mut stack, |a, b| a * b),
            CalculatorInput::Divide => op(&mut stack, |a, b| a / b),
        };
        if result.is_err() {
            return None;
        }
    }

    if stack.len() == 1 {
        Some(stack[0])
    } else {
        None
    }
}

fn op(stack: &mut Vec<i32>, operation: fn(i32, i32) -> i32) -> Result<(), ()> {
    if let Some(operand2) = stack.pop() {
        if let Some(operand1) = stack.pop() {
            let result = operation(operand1, operand2);
            stack.push(result);
            return Ok(());
        }
    }

    Err(())
}
