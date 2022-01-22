extern crate rcc;
use rcc::strtol;
use std::env;
use std::process::exit;

#[derive(Default, Debug, Clone)]
struct Token {
    val: Option<i64>,    // Number
    op: Option<String>,  // character
    len: Option<String>, // length
}

#[derive(Default, Clone, Debug)]
struct Node {
    lhs: Option<Box<Node>>, //左辺
    rhs: Option<Box<Node>>, //右辺
    val: Option<i64>,
    operator: Option<String>,
}

impl Node {
    // 左辺と右辺を受け取る2項演算子の関数を定義する
    fn new(op: String, lhs: Node, rhs: Node) -> Self {
        Self {
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
            operator: Some(op),
            ..Default::default()
        }
    }

    // 数値を受け取れる関数を定義する
    fn new_code_num(val: i64) -> Self {
        Self {
            val: Some(val),
            ..Default::default()
        }
    }

    fn expr(tokens: &mut Vec<Token>) -> Self {
        let node = Self::equality(tokens);
        return node;
    }

    fn equality(tokens: &mut Vec<Token>) -> Self {
        let mut node = Self::relational(tokens);
        while tokens.len() != 0 {
            let token = &tokens[0];
            match &token.op {
                Some(op) => match op.as_ref() {
                    "==" => {
                        let rhs = Self::relational(tokens);
                        node = Self::new("+".to_string(), node, rhs);
                    }
                    "!=" => {
                        let rhs = Self::relational(tokens);
                        node = Self::new("-".to_string(), node, rhs);
                    }
                    _ => {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }
        }
        return node;
    }

    fn relational(tokens: &mut Vec<Token>) -> Self {
        let mut node = Self::add(tokens);
        while tokens.len() != 0 {
            let token = &tokens[0];
            match &token.op {
                Some(op) => match op.as_ref() {
                    "<" => {
                        let rhs = Self::add(tokens);
                        node = Self::new("+".to_string(), node, rhs);
                    }
                    "<=" => {
                        let rhs = Self::add(tokens);
                        node = Self::new("+".to_string(), node, rhs);
                    }
                    ">" => {
                        let rhs = Self::add(tokens);
                        node = Self::new("+".to_string(), node, rhs);
                    }
                    ">=" => {
                        let rhs = Self::add(tokens);
                        node = Self::new("+".to_string(), node, rhs);
                    }
                    _ => {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }
        }
        return node;
    }

    fn add(tokens: &mut Vec<Token>) -> Self {
        let mut node = Self::mul(tokens);
        while tokens.len() != 0 {
            let token = &tokens[0];
            match &token.op {
                Some(op) => match op.as_ref() {
                    "+" => {
                        let rhs = Self::mul(tokens);
                        node = Self::new("+".to_string(), node, rhs);
                    }
                    "-" => {
                        let rhs = Self::mul(tokens);
                        node = Self::new("+".to_string(), node, rhs);
                    }
                    _ => {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }
        }
        return node;
    }

    fn mul(tokens: &mut Vec<Token>) -> Self {
        let mut node = Self::unary(tokens);
        while tokens.len() != 0 {
            let token = &tokens[0];
            match &token.op {
                Some(op) => match op.as_ref() {
                    "*" => {
                        tokens.remove(0);
                        let rhs = Node::unary(tokens);
                        node = Node::new("*".to_string(), node, rhs);
                    }
                    "/" => {
                        tokens.remove(0);
                        let rhs = Node::unary(tokens);
                        node = Node::new("/".to_string(), node, rhs);
                    }
                    _ => {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }
        }
        return node;
    }

    fn unary(tokens: &mut Vec<Token>) -> Self {
        let token = &tokens[0];
        match &token.op {
            Some(op) => match op.as_ref() {
                "+" => {
                    tokens.remove(0);
                    return Self::primary(tokens);
                }
                "-" => {
                    tokens.remove(0);
                    return Self::new("-".to_string(),Self::new_code_num(0),Self::primary(tokens),);
                }
                _ => {
                    return Self::primary(tokens);
                }
            },
            _ => {
                return Self::primary(tokens);
            }
        }
    }

    fn primary(tokens: &mut Vec<Token>) -> Self {
        match &tokens[0].op {
            Some(op) => match op.as_ref() {
                "(" => {
                    let close_index = tokens
                        .iter()
                        .position(|token| token.op == Some(")".to_string()))
                        .unwrap();
                    let mut exp = tokens[1..close_index].to_vec();
                    tokens.drain(0..(close_index + 1));
                    return Node::expr(&mut exp);
                }
                _ => {
                    let num = tokens[0].val.unwrap();
                    tokens.remove(0);
                    return Node::new_code_num(num);
                }
            },
            _ => {
                let num = tokens[0].val.unwrap();
                tokens.remove(0);
                return Node::new_code_num(num);
            }
        }
    }
}

impl Token {
    // トークナイザー実装する
    fn tokenize(mut p: String) -> Vec<Token> {
        // Tokenized input is stored to this vec.
        let mut tokens: Vec<Token> = vec![];
        let current_token = String::from("");

        while let Some(c) = p.chars().nth(0) {
            // 空白を読み飛ばす
            if c.is_whitespace() {
                p = p.split_off(1);
                continue;
            }

            if c == '=' && current_token.len() > 0 {
                let token = Token {
                    op: Some(c.to_string()),
                    len: Some(current_token.clone()),
                    ..Default::default()
                };
                tokens.push(token);
                continue;
            }

            // + or -　or * or / or ( or )
            if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' {
                let token = Token {
                    op: Some(c.to_string()),
                    ..Default::default()
                };
                p = p.split_off(1);
                tokens.push(token);
                continue;
            }

            // Number
            if c.is_ascii_digit() {
                let (n, remaining) = strtol(&p);
                p = remaining;
                let token = Token {
                    val: n,
                    op: None,
                    ..Default::default()
                };
                tokens.push(token);
                continue;
            }

            eprint!("トークナイズできません: {}", c);
            exit(1);
        }

        tokens.push(Token {
            ..Default::default()
        });

        return tokens;
    }
}

fn gen(node: &Node) {
    if let Some(val) = node.val {
        print!("  push {}", val);
    }

    if let Some(rhs) = &node.rhs {
        gen(&rhs);
    }

    if let Some(lhs) = &node.lhs {
        gen(&lhs);
    }
    print!("  pop rdi\n");
    print!("  pop rax\n");

    match &node.operator {
        Some(op) => match op.as_ref() {
            "+" => {
                print!("  add rax, rdi\n");
            }
            "-" => {
                print!("  sub rax, rdi\n");
            }
            "*" => {
                print!("  imul rax, rdi\n");
            }
            "/" => {
                print!("  cqo\n");
                print!("  idiv rdi\n");
            }
            _ => {}
        },
        _ => {}
    }
    print!("  push rax\n");
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprint!("引数の個数が正しくありません");
        return;
    }

    // トークナイズしてパースする
    let user_input = args.nth(1);
    let mut tokens = Token::tokenize(user_input.unwrap());
    let expr = Node::expr(&mut tokens);

    // アセンブリの前半部分を出力
    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");
    gen(&expr);
    // println!("{:#?}", expr);

    print!("  pop rax\n");
    print!("  ret\n");
}
