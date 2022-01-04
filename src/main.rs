extern crate rcc;
use rcc::strtol;
use std::env;

#[derive(Default, Debug, Clone)]
struct Token {
    val: Option<i32>, // Number
    op: Option<char> // character
}

#[derive(Default, Clone, Debug)]
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

#[allow(dead_code)]
fn gen(node: &Node) {
    if let Some(val) = &node.val {
        print!("  push {}", val);
    }

    if let Some(rhs) = &node.rhs {
        gen(&rhs);
    }

    if let Some(lhs) = &node.lhs {
        gen(&lhs);
    }
    println!("  pop rdi");
    println!("  pop rax");

    match &node.operator {
        Some('+') => {
            print!("  add rax, rdi");
        }
        Some('-') => {
            print!("  sub rax, rdi");
        }
        Some('*') => {
            print!(" imul rax, rdi");
        }
        Some('/') => {
            print!("  cqo");
            print!("  idiv rdi");
        }
        _ => {
            print!("  push rax");
        }
    }
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprint!("引数の個数が正しくありません");
        return;
    }

    // トークナイズしてパースする
    let user_input = args.nth(1);
    let tokens = tokenize(user_input.unwrap());
    let expr = Node::expr(tokens);
    println!("{:#?}", expr);

    // アセンブリの前半部分を出力
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    print!("  pop rax");
    print!("  ret");
}
