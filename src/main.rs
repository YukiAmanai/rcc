use std::env;
mod token;
mod node;
use token::Token;
use node::Node;

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
    node::gen(&expr);
    print!("  pop rax\n");
    print!("  ret\n");
}
