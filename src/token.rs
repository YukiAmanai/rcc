#[derive(Default, Debug, Clone)]
pub struct Token {
    pub val: Option<i64>,    // Number
    pub op: Option<String>,  // character
}

impl Token {
    fn operator(op: String) -> Self {
        Self {
            op: Some(op),
            ..Default::default()
        }
    }
    fn number(val: i64) -> Self {
        Self {
            val: Some(val),
            ..Default::default()
        }
    }

    // パーサー実装
    pub fn perser(input: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut input = input;

        loop {
            if input.is_empty() {
                break;
            }
            consume_whitespace(&mut input);

            if let Some(token) = consume_number(&mut input) {
                tokens.push(token);
                continue;
            }
            if let Some(token) = consume_operator(&mut input) {
                tokens.push(token);
                continue;
            }
        }
            return tokens;
    }
}

// 最初の空白除去
fn consume_whitespace(input: &mut String) {
    loop {
        match input.chars().next() {
            Some(c) if c.is_whitespace() => {
                input.remove(0);
            }
            _ => {
                break;
            }
        }
    }
}

// 数字のconsume関数実装
fn consume_number(input: &mut String) ->Option<Token> {
    let mut digits = "".to_string();
    loop {
        match input.chars().next() {
            Some(c) if c.is_ascii_digit() => {
                digits += &c.to_string();
                input.remove(0);
            }
            _ => {
                break;
            }
        }
    }
    if digits.is_empty() {
        None
    } else {
        Some(Token::number(digits.parse::<i64>().unwrap()))
    }
}

// オペランドのconsume関数実装
fn consume_operator(input: &mut String) -> Option<Token> {
    if input.starts_with("==") ||
        input.starts_with("!=") ||
        input.starts_with("<=") ||
        input.starts_with(">=") {
            let token = Some(Token::operator(input[..2].to_string()));
            input.drain(0..2);
            return token;
        }
    match input.chars().next() {
        Some(c) if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' || c == '>' || c == '<' => {
            input.remove(0);
            Some(Token::operator(c.to_string()))
        }
        _ => None,
    }
}
