extern crate rcc;
use rcc::strtol;
use std::env;

#[derive(Default, Debug, Clone)]
struct Token {
    val: Option<i32>, // Number literal
    op: Option<char>
}

#[derive(Default,Clone)]
struct Node {
    lhs: Option<Box<Node>>, //左辺
    rhs: Option<Box<Node>>, //右辺
    val: Option<i32>,
    operator: Option<char>,
}

impl Node {
    // 左辺と右辺を受け取る2項演算子の関数を定義する
    fn new(op: char, lhs: Box<Node>, rhs: Box<Node>,) -> Self {
        Self {
            lhs: Some(lhs),
            rhs: Some(rhs),
            operator: Some(op),
            ..Default::default()
        }
    }

    // 数値を受け取れる関数を定義する
    fn new_code_num(val: i32) -> Self {
        Self {
            val: Some(val),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    fn expr(tokens: Vec<Token>) -> (Self, Vec<Token>) {
        let (mut node, tokens) = Self::mul(tokens);
        for token in &tokens {
            match token.op {
                Some('+') => {
                    let (rhs, _tokens) = Self::mul(tokens[1..].to_vec());
                    node = Self::new('+', Box::new(node), Box::new(rhs));
                }
                Some('-') => {
                    let (rhs, _tokens) = Self::mul(tokens[1..].to_vec());
                    node = Self::new('-',Box::new(node), Box::new(rhs));
                }
                _ => (),
            }
        }
        return (node, tokens);
    }

    fn mul(tokens: Vec<Token>) -> (Self, Vec<Token>) {
        let (mut node, tokens) = Self::primary(tokens);
        for token in &tokens {
            match token.op {
                Some('*') => {
                    let (rhs, _tokens) = Self::primary(tokens[1..].to_vec());
                    node = Self::new('*', Box::new(node), Box::new(rhs));
                }
                Some('/') => {
                    let (rhs, _tokens) = Self::primary( tokens[1..].to_vec());
                    node = Self::new('/', Box::new(node), Box::new(rhs));
                }
                _ => (),
            }
        }
        return (node, tokens);
    }

    fn primary(tokens: Vec<Token>) -> (Self, Vec<Token>) {
        if tokens[0].op == Some('(') {
            let close_index = tokens
                .iter()
                .position(|token| token.op == Some(')'))
                .unwrap();
            return Self::expr(tokens[1..(close_index - 1)].to_vec());
        } else {
            return (Self::new_code_num(tokens[0].val.unwrap()), tokens[1..].to_vec());
        };
    }
}

// トークナイザー実装する
fn tokenize(mut p: String) -> Vec<Token> {
    // Tokenized input is stored to this vec.
    let mut tokens: Vec<Token> = vec![];

    while let Some(c) = p.chars().nth(0) {
        // 空白を読み飛ばす
        if c.is_whitespace() {
            p = p.split_off(1); // p++
            continue;
        }

        // + or -　or * or /
        if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')'{
            let token = Token {
                op: Some(c),
                ..Default::default()
            };
            p = p.split_off(1);
            tokens.push(token);
            continue;
        }

        // Number
        if c.is_ascii_digit() {
            let (n,  remaining) = strtol(&p);
            p = remaining;
            let token = Token {
                val: n,
                op: None,
                ..Default::default()
            };
            tokens.push(token);
            continue;
        }

        eprint!("トークナイズできません: {}\n", p);
        panic!("");
    }
    return tokens;
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprint!("Usage: rcc <code>\n");
        return;
    }
    let tokens = tokenize(args.nth(1).unwrap());

    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");

    for (index, token) in (0_usize..).zip(tokens.iter()) {
        let index_number = 0;
        if index == index_number {
            println!("  mov rax, {}", token.val.unwrap());
            continue;
        }
        if let Some(val) = token.val {
            match tokens[index - 1].op {
                Some('+') => {
                    println!("  add rax, {}", val);
                }
                Some('-') => {
                    println!("  sub rax, {}", val);
                }
                Some(_) | None => {
                    println!("数ではありません");
                }
            }
        }
        if token.val == None && token.op == None {
            println!("  ret");
        }
    }
}
