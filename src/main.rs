extern crate rcc;
use rcc::strtol;
use std::env;

#[derive(Debug)]
enum TokenType {
    Num,
}

// Token type
#[derive(Default,Debug,Clone)]
struct Token {
    ty: i32, // Token type
    val: i32, // Number literal
    input: String, // Token string (for error reporting)
    operator: Option<char>
}

fn tokenize(mut p: String) -> Vec<Token> {
    // Tokenized input is stored to this vec.
    let mut tokens: Vec<Token> = vec![];

    let org = p.clone();
    while let Some(c) = p.chars().nth(0) {
        // Skip whitespce
        if c.is_whitespace() {
            p = p.split_off(1); // p++
            continue;
        }

        // + or -
        if c == '+' || c == '-' {
            let token = Token {
                ty: c as i32,
                input: org.clone(),
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
                ty: TokenType::Num as i32,
                input: org.clone(),
                val: n.unwrap() as i32,
                operator: None
            };
            tokens.push(token);
            continue;
        }

        eprint!("トークナイズできません: {}\n", p);
        panic!("");
    }
    return tokens;
}

fn fail(tokens: &Vec<Token>, i: usize) {
    eprint!("数ではありません: {:?}\n", tokens[i]);
    panic!("");
}

#[derive(Debug)]
#[allow(unused)]
enum NodeType {
    Num,
}

#[derive(Default,Clone)]

struct Node {
    ty: i32, // Token type
    lhs: Option<Box<Node>>, //左辺
    rhs: Option<Box<Node>>, //右辺
    val: Option<i32>, // Number literal
    operator: Option<char>,
}
impl Node {
    // 左辺と右辺を受け取る2項演算子の関数を定義する
    fn new(op: char, lhs: Box<Node>, rhs: Box<Node>,) -> Self {
        Self {
            ty: NodeType::Num as i32,
            lhs: Some(lhs),
            rhs: Some(rhs),
            operator: Some(op),
            ..Default::default()
        }
    }

    // 数値を受け取れる関数を定義する
    fn new_code_num(val: i32) -> Self {
        Self {
            ty: NodeType::Num as i32,
            val: Some(val),
            ..Default::default()
        }
    }
    #[allow(dead_code)]
    fn expr(tokens: Vec<Token>) -> (Self, Vec<Token>) {
        let (mut node, tokens) = Self::mul(tokens);
        for token in &tokens {
            match token.operator {
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
            match token.operator {
                Some('*') => {
                    let (rhs, _tokens) = Self::primary(tokens[1..].to_vec());
                    node = Self::new('*', Box::new(node), Box::new(rhs));
                }
                Some('/') => {
                    let (rhs, _tokens) = Self::primary(tokens[1..].to_vec());
                    node = Self::new('/', Box::new(node), Box::new(rhs));
                }
                _ => (),
            }
        }
        return (node, tokens);
    }

    fn primary(tokens: Vec<Token>) -> (Self, Vec<Token>) {
        if tokens[0].operator == Some('(') {
            let close_index = tokens
                .iter()
                .position(|token| token.operator == Some(')'))
                .unwrap();
            return Self::expr(tokens[1..(close_index - 1)].to_vec());
        } else {
            return (Self::new_code_num(tokens[0].ty), tokens[1..].to_vec());
        };
    }
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

    if tokens[0].ty != TokenType::Num as i32 {
        fail(&tokens, 0);
    }
    print!("  mov rax, {}\n", tokens[0].val);

    let mut i = 1;
    while i != tokens.len() {
        if tokens[i].ty == '+' as i32 {
            i += 1;
            if tokens[i].ty != TokenType::Num as i32 {
                fail(&tokens, i);
            }
            print!("  add rax, {}\n", tokens[i].val);
            i += 1;
            continue;
        }

        if tokens[i].ty == '-' as i32 {
            i += 1;
            if tokens[i].ty != TokenType::Num as i32 {
                fail(&tokens, i);
            }
            print!("  sub rax, {}\n", tokens[i].val);
            i += 1;
            continue;
        }

        fail(&tokens, i);
    }

    print!("  ret\n");
}
