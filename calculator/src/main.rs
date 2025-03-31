use std::io::{self, Write};

#[derive(Debug)]
enum EvalResult {
    Valid(i32),
    Invalid,
    Quit,
}

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Token {
    Number(i32),
    Op(Operation),
}

fn precedence(op: &Operation) -> u8 {
    match op {
        Operation::Add => 0,
        Operation::Subtract => 0,
        Operation::Multiply => 1,
        Operation::Divide => 1,
    }
}

fn eval(input: &str) -> EvalResult {
    if input == "quit" {
        return EvalResult::Quit;
    }

    let mut token_queue: Vec<Token> = Vec::new();
    let mut op_stack: Vec<Operation> = Vec::new();

    // shunting-yard algorithm
    for c in input.chars() {
        if c.is_whitespace() {
            continue;
        }

        let token = match c {
            '+' => Token::Op(Operation::Add),
            '-' => Token::Op(Operation::Subtract),
            '*' => Token::Op(Operation::Multiply),
            '/' => Token::Op(Operation::Divide),
            '0'..='9' => Token::Number(c.to_digit(10).unwrap() as i32),
            _ => return EvalResult::Invalid,
        };

        match token {
            Token::Number(_) => token_queue.push(token),
            Token::Op(op) => {
                while let Some(last_op) = op_stack.pop() {
                    if precedence(&op) <= precedence(&last_op) {
                        token_queue.push(Token::Op(last_op));
                    } else {
                        op_stack.push(last_op);
                        break;
                    }
                }
                op_stack.push(op);
            }
        }
    }

    while let Some(op) = op_stack.pop() {
        token_queue.push(Token::Op(op));
    }

    println!("tokens: {:?}", token_queue);

    let mut num_stack: Vec<i32> = Vec::new();

    for token in token_queue {
        match token {
            Token::Number(num) => num_stack.push(num),
            Token::Op(op) => {
                let a = num_stack.pop().unwrap();
                let b = num_stack.pop().unwrap();
                match op {
                    Operation::Add => num_stack.push(b + a),
                    Operation::Subtract => num_stack.push(b - a),
                    Operation::Multiply => num_stack.push(b * a),
                    Operation::Divide => num_stack.push(b / a),
                };
            }
        }
    }

    if let Some(result) = num_stack.pop() {
        return EvalResult::Valid(result);
    }

    EvalResult::Invalid
}

fn main() {
    println!("Simple calculator. Enter mathematical expression (single digits and +-*/) or quit.");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush!");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input = input.trim();

        match eval(input) {
            EvalResult::Valid(result) => println!("= {result}"),
            EvalResult::Invalid => println!("Invalid input!"),
            EvalResult::Quit => break,
        }
    }
}
