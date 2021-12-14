extern crate rcc;
use rcc::strtol;
use std::env;
use std::process::exit;

enum TokenType {
    Num,
}

#[derive(Default, Debug)]
struct Token {
    ty: i32,
    val: i32,
    input: String,
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
            p.split_off(1);
            tokens.push(token);
            continue;
        }

        // Number
        if c.is_ascii_digit() {
            let (n, mut remaining) = strtol(&p);
            p = remaining;
            let token = Token {
                ty: TokenType::Num as i32,
                input: org.clone(),
                val: n.unwrap() as i32,
            };
            tokens.push(token);
            continue;
        }

        eprint!("トークナイズできません: {}\n", p);
        exit(1);
    }
    return tokens;
}

fn fail(tokens: &Vec<Token>, i: usize) {
    eprint!("unexpected character: {:?}\n", tokens[i]);
    exit(1);
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprint!("Usage: rcc <code>\n");
        return;
    }

    let p = args.nth(1).unwrap();

    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");

    let (n, mut p) = strtol(&p);
    print!("  mov rax, {}\n", n.unwrap());

    while let Some(c) = p.chars().nth(0) {
        let s = p.split_off(1);
        if c == '+' {
            let (n, remaining) = strtol(&s);
            p = remaining;
            println!("  add rax, %ld{}\n", n.unwrap());
            continue;
        }

        if c == '-' {
            let (n, remaining) = strtol(&s);
            p = remaining;
            print!("  sub rax, {}\n", n.unwrap());
            continue;
        }

        eprint!("予期しない文字です:'' {}\n", p);
        return;
    }

    print!(" ret\n");
    return;
}
