extern crate rcc;
use rcc::strtol;
use std::env;

enum TokenType {
    Num, // Number literal
}

// Token type
#[derive(Default, Debug)]
struct Token {
    ty: i32, // Token type
    val: i32, // Number literal
    input: String, // Token string (for error reporting)
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
