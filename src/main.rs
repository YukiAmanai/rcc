use std::env;
mod node;
mod token;
use node::Node;
use token::Token;

// コード生成の関数
pub fn gen(node: &Node) {
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
            "==" => {
                print!("  cmp rax, rdi\n");
                print!("  sete al\n");
                print!("  movzb rax, al\n");
            }
            "<" => {
                print!("  cmp rax, rdi\n");
                print!("  setl al\n");
                print!("  movzb rax, al\n");
            }
            "<=" => {
                print!("  cmp rax, rdi\n");
                print!("  setle al\n");
                print!("  movzb rax, al\n");
            }
            "!=" => {
                print!("  cmp rax, rdi\n");
                print!("  setne al\n");
                print!("  movzb rax, al\n");
            }
            _ => {}
        },
        _ => {}
    }
    print!("  push rax\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprint!("引数の個数が正しくありません");
        return;
    }

    // トークナイズしてパースする
    let user_input = &args[1];
    let mut tokens = Token::perser(user_input.to_string());
    let expr = Node::expr(&mut tokens);

    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");
    gen(&expr);
    print!("  pop rax\n");
    print!("  ret\n");
}
