use crate::token::Token;

#[derive(Default, Clone, Debug)]
pub struct Node {
    pub lhs: Option<Box<Node>>, //左辺
    pub rhs: Option<Box<Node>>, //右辺
    pub val: Option<i64>,
    pub operator: Option<String>,
}

impl Node {
    // 左辺と右辺を受け取る2項演算子の関数を定義する
    pub fn new(op: String, lhs: Node, rhs: Node) -> Self {
        Self {
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
            operator: Some(op),
            ..Default::default()
        }
    }

    // 数値を受け取れる関数を定義する
    pub fn new_code_num(val: i64) -> Self {
        Self {
            val: Some(val),
            ..Default::default()
        }
    }

    pub fn expr(tokens: &mut Vec<Token>) -> Self {
        let node = Self::equality(tokens);
        return node;
    }

    pub fn equality(tokens: &mut Vec<Token>) -> Self {
        let mut node = Self::relational(tokens);
        while tokens.len() != 0 {
            let token = &tokens[0];
            match &token.op {
                Some(op) => match op.as_ref() {
                    "==" => {
                        tokens.remove(0);
                        let rhs = Self::relational(tokens);
                        node = Self::new("==".to_string(), node, rhs);
                    }
                    "!=" => {
                        tokens.remove(0);
                        let rhs = Self::relational(tokens);
                        node = Self::new("!=".to_string(), node, rhs);
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

    pub fn relational(tokens: &mut Vec<Token>) -> Self {
        let mut node = Self::add(tokens);
        while tokens.len() != 0 {
            let token = &tokens[0];
            match &token.op {
                Some(op) => match op.as_ref() {
                    "<" => {
                        tokens.remove(0);
                        let rhs = Self::add(tokens);
                        node = Self::new("<".to_string(), node, rhs);
                    }
                    "<=" => {
                        tokens.remove(0);
                        let rhs = Self::add(tokens);
                        node = Self::new("<=".to_string(), node, rhs);
                    }
                    ">" => {
                        tokens.remove(0);
                        let rhs = Self::add(tokens);
                        node = Self::new("<".to_string(), rhs, node);
                    }
                    ">=" => {
                        tokens.remove(0);
                        let rhs = Self::add(tokens);
                        node = Self::new("<=".to_string(), rhs, node);
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

    pub fn add(tokens: &mut Vec<Token>) -> Self {
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

    pub fn mul(tokens: &mut Vec<Token>) -> Self {
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

    pub fn unary(tokens: &mut Vec<Token>) -> Self {
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

    pub fn primary(tokens: &mut Vec<Token>) -> Self {
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
