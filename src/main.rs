use std::env;

fn main() {
    let mut args = env::args();
    if args.len() != 2  {
        eprint!("引数の個数が正しくありません\n");
        return;
    }
    println!(".intel_syntax noprefix\n");
    println!(".globl main\n");
    println!("main:\n");
    println!("  mov rax, %d{}\n",args.nth(1).unwrap());
    println!("  ret\n");
    return;
}
