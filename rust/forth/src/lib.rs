use std::collections::HashMap;
use std::rc::Rc;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    symbols: HashMap<String, Operation>,
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

enum Token {
    Number(Value),
    Word(String),
    BeginDefinition,
    EndDefinition,
}

type Code = Vec<Operation>;

#[derive(Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Duplicate,
    Drop,
    Swap,
    Over,
    Push(Value),
    Subroutine(Rc<Code>),
}

macro_rules! hashmap {
    () => { crate::HashMap::new() };
    ($($key:literal => $value:expr),+ $(,)?) => {
        {
            let mut hm = crate::HashMap::new();
            $(hm.insert(String::from($key), $value);)*
            hm
        }
    };
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            symbols: hashmap!(
                "+"    => Operation::Add,
                "-"    => Operation::Subtract,
                "*"    => Operation::Multiply,
                "/"    => Operation::Divide,
                "dup"  => Operation::Duplicate,
                "drop" => Operation::Drop,
                "swap" => Operation::Swap,
                "over" => Operation::Over
            ),
            stack: Vec::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        match self.parse(input) {
            Ok(code) => self.exec(&code),
            Err(error) => Err(error),
        }
    }

    fn parse(&mut self, input: &str) -> std::result::Result<Code, Error> {
        let mut code: Vec<Operation> = Vec::new();

        let mut tokens = tokenizer(input);

        while let Some(token) = tokens.next() {
            match token {
                Token::Number(value) => {
                    code.push(Operation::Push(value));
                }
                Token::Word(word) => match self.symbols.get(&word) {
                    Some(symbol) => {
                        code.push(symbol.clone());
                    }
                    None => {
                        return Err(Error::UnknownWord);
                    }
                },
                Token::BeginDefinition => match self.parse_definition(&mut tokens) {
                    Ok(()) => {}
                    Err(error) => {
                        return Err(error);
                    }
                },
                Token::EndDefinition => {
                    return Err(Error::InvalidWord);
                }
            }
        }
        Ok(code)
    }

    fn parse_definition(&mut self, input: &mut TokenIterator) -> Result {
        let mut code: Vec<Operation> = Vec::new();

        let ident = match input.consume_word() {
            Some(ident) => ident,
            None => return Err(Error::InvalidWord),
        };

        while let Some(token) = input.next() {
            match token {
                Token::Number(value) => code.push(Operation::Push(value)),
                Token::Word(word) => match self.symbols.get(&word) {
                    Some(symbol) => {
                        code.push(symbol.clone());
                    }
                    None => {
                        return Err(Error::UnknownWord);
                    }
                },
                Token::EndDefinition => {
                    self.symbols
                        .insert(ident, Operation::Subroutine(Rc::new(code)));
                    return Ok(());
                }
                _ => {
                    return Err(Error::InvalidWord);
                }
            }
        }

        Err(Error::InvalidWord)
    }

    fn exec(&mut self, code: &Code) -> Result {
        for op in code.iter() {
            let op_result = match op {
                Operation::Add => self.arithmetic_binary_op(|arg1, arg2| Ok(arg1 + arg2)),
                Operation::Subtract => self.arithmetic_binary_op(|arg1, arg2| Ok(arg1 - arg2)),
                Operation::Multiply => self.arithmetic_binary_op(|arg1, arg2| Ok(arg1 * arg2)),
                Operation::Divide => self.arithmetic_binary_op(|arg1, arg2| match arg2 {
                    0 => Err(Error::DivisionByZero),
                    _ => Ok(arg1 / arg2),
                }),
                Operation::Duplicate => match self.stack.iter().rev().nth(0) {
                    Some(value) => {
                        self.stack.push(*value);
                        Ok(())
                    }
                    None => Err(Error::StackUnderflow),
                },
                Operation::Drop => match self.stack.pop() {
                    Some(_) => Ok(()),
                    None => Err(Error::StackUnderflow),
                },
                Operation::Swap => {
                    let stack_size = self.stack.len();
                    if stack_size > 1 {
                        self.stack.swap(stack_size - 2, stack_size - 1);
                        Ok(())
                    } else {
                        Err(Error::StackUnderflow)
                    }
                }
                Operation::Over => match self.stack.iter().rev().nth(1) {
                    Some(value) => {
                        self.stack.push(*value);
                        Ok(())
                    }
                    None => Err(Error::StackUnderflow),
                },
                Operation::Push(value) => {
                    self.stack.push(*value);
                    Ok(())
                }
                Operation::Subroutine(code) => self.exec(code),
            };

            if op_result.is_err() {
                return op_result;
            }
        }

        Ok(())
    }

    fn arithmetic_binary_op(
        &mut self,
        op: fn(Value, Value) -> std::result::Result<Value, Error>,
    ) -> Result {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(arg2), Some(arg1)) => match op(arg1, arg2) {
                Ok(value) => {
                    self.stack.push(value);
                    Ok(())
                }
                Err(error) => Err(error),
            },
            _ => Err(Error::StackUnderflow),
        }
    }
}

fn skip_whitespace(input: &str) -> &str {
    for (index, char) in input.chars().enumerate() {
        if !char.is_ascii_whitespace() {
            return &input[index..];
        }
    }
    &input[input.len()..]
}

fn scan_until_whitespace(input: &str) -> (&str, &str) {
    for (index, char) in input.chars().enumerate() {
        if char.is_ascii_whitespace() {
            return (&input[0..index], &input[(index + 1)..]);
        }
    }
    (&input[..], &input[input.len()..])
}

fn tokenize(input: &str) -> (Token, &str) {
    let input = skip_whitespace(input);
    let (word, next_input) = scan_until_whitespace(input);

    if let Ok(value) = Value::from_str_radix(word, 10) {
        return (Token::Number(value), next_input);
    }

    if word.len() == 1 {
        let first_char = word.chars().nth(0);

        match first_char {
            Some(':') => {
                return (Token::BeginDefinition, next_input);
            }
            Some(';') => {
                return (Token::EndDefinition, next_input);
            }
            Some(_) => {}
            None => {}
        }
    }

    (Token::Word(word.to_ascii_lowercase()), next_input)
}

struct TokenIterator<'a> {
    input: &'a str,
}

impl<'a> TokenIterator<'a> {
    fn consume_word(&mut self) -> Option<String> {
        if let Some(token) = self.next() {
            if let Token::Word(word) = token {
                Some(word)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.len() == 0 {
            return None;
        }

        let (token, next_input) = tokenize(self.input);

        self.input = next_input;

        Some(token)
    }
}

fn tokenizer<'a>(input: &'a str) -> TokenIterator<'a> {
    TokenIterator { input }
}
