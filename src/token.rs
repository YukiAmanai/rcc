use rcc::strtol;
use std::process::exit;

#[derive(Default, Debug, Clone)]
pub struct Token {
    pub val: Option<i64>,    // Number
    pub op: Option<String>,  // character
    pub len: Option<String>, // length
}

impl Token {
    // トークナイザー実装する
    pub fn tokenize(mut p: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut current_token = String::from("");

        while let Some(c) = p.chars().nth(0) {
            // 空白を読み飛ばす
            if c.is_whitespace() {
                p = p.split_off(1);
                continue;
            }

            if (current_token == ">" || current_token == "<") && c != '=' {
                let token = Token {
                    op: Some(c.to_string()),
                    len: Some(current_token.clone()),
                    ..Default::default()
                };
                current_token = String::from("");
                tokens.push(token);
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

            if c == '=' || c == '!' || c == '<' || c == '>' {
                let token = Token {
                    op: Some(c.to_string()),
                    len: Some(current_token.clone()),
                    ..Default::default()
                };
                p = p.split_off(1);
                tokens.push(token);
                continue;
            }

            // + or -　or * or / or ( or )
            if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' {
                let token = Token {
                    op: Some(c.to_string()),
                    len: Some(current_token.clone()),
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
                    ..Default::default()
                };
                tokens.push(token);
                continue;
            }

            eprint!("トークナイズできません: {}", c);
            exit(1);
        }

        // tokens.push(Token {
        //     ..Default::default()
        // });

        return tokens;
    }
}
