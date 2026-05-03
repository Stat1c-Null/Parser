#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    If,
    Elif,
    Else,
    Identifier(String),
    Number(String),
    StringLit(String),
    Assign,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Comma,
    Colon,
    Newline,
    Indent,
    Dedent,
    Eof,
}

impl Token {
    pub fn name(&self) -> String {
        match self {
            Token::If => "IF".into(),
            Token::Elif => "ELIF".into(),
            Token::Else => "ELSE".into(),
            Token::Identifier(s) => format!("IDENTIFIER({})", s),
            Token::Number(s) => format!("NUMBER({})", s),
            Token::StringLit(s) => format!("STRING(\"{}\")", s),
            Token::Assign => "ASSIGN".into(),
            Token::Eq => "EQ".into(),
            Token::NotEq => "NOTEQ".into(),
            Token::Lt => "LT".into(),
            Token::Gt => "GT".into(),
            Token::LtEq => "LTEQ".into(),
            Token::GtEq => "GTEQ".into(),
            Token::Plus => "PLUS".into(),
            Token::Minus => "MINUS".into(),
            Token::Star => "STAR".into(),
            Token::Slash => "SLASH".into(),
            Token::LParen => "LPAREN".into(),
            Token::RParen => "RPAREN".into(),
            Token::Comma => "COMMA".into(),
            Token::Colon => "COLON".into(),
            Token::Newline => "NEWLINE".into(),
            Token::Indent => "INDENT".into(),
            Token::Dedent => "DEDENT".into(),
            Token::Eof => "EOF".into(),
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut indent_stack: Vec<usize> = vec![0];

    for (line_no, raw_line) in input.split('\n').enumerate() {
        let line_no = line_no + 1;
        let trimmed = raw_line.trim_start();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let indent = raw_line.len() - trimmed.len();
        let current = *indent_stack.last().unwrap();

        if indent > current {
            indent_stack.push(indent);
            tokens.push(Token::Indent);
        } else if indent < current {
            while indent < *indent_stack.last().unwrap() {
                indent_stack.pop();
                tokens.push(Token::Dedent);
            }
            if indent != *indent_stack.last().unwrap() {
                return Err(format!(
                    "Syntax Error: Inconsistent indentation on line {}",
                    line_no
                ));
            }
        }

        tokenize_line(trimmed, line_no, &mut tokens)?;
        tokens.push(Token::Newline);
    }

    while indent_stack.len() > 1 {
        indent_stack.pop();
        tokens.push(Token::Dedent);
    }
    tokens.push(Token::Eof);

    Ok(tokens)
}

fn tokenize_line(line: &str, line_no: usize, tokens: &mut Vec<Token>) -> Result<(), String> {
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            ' ' | '\t' => i += 1,
            '#' => break,
            ':' => { tokens.push(Token::Colon); i += 1; }
            '(' => { tokens.push(Token::LParen); i += 1; }
            ')' => { tokens.push(Token::RParen); i += 1; }
            ',' => { tokens.push(Token::Comma); i += 1; }
            '+' => { tokens.push(Token::Plus); i += 1; }
            '-' => { tokens.push(Token::Minus); i += 1; }
            '*' => { tokens.push(Token::Star); i += 1; }
            '/' => { tokens.push(Token::Slash); i += 1; }
            '<' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(Token::LtEq);
                    i += 2;
                } else {
                    tokens.push(Token::Lt);
                    i += 1;
                }
            }
            '>' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(Token::GtEq);
                    i += 2;
                } else {
                    tokens.push(Token::Gt);
                    i += 1;
                }
            }
            '=' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(Token::Eq);
                    i += 2;
                } else {
                    tokens.push(Token::Assign);
                    i += 1;
                }
            }
            '!' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(Token::NotEq);
                    i += 2;
                } else {
                    return Err(format!(
                        "Syntax Error: Unexpected character '!' on line {}",
                        line_no
                    ));
                }
            }
            '"' | '\'' => {
                let quote = c;
                i += 1;
                let start = i;
                while i < chars.len() && chars[i] != quote {
                    i += 1;
                }
                if i >= chars.len() {
                    return Err(format!(
                        "Syntax Error: Unterminated string literal on line {}",
                        line_no
                    ));
                }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::StringLit(s));
                i += 1;
            }
            ch if ch.is_ascii_digit() => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let n: String = chars[start..i].iter().collect();
                tokens.push(Token::Number(n));
            }
            ch if ch.is_alphabetic() || ch == '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                let tok = match s.as_str() {
                    "if" => Token::If,
                    "elif" => Token::Elif,
                    "else" => Token::Else,
                    _ => Token::Identifier(s),
                };
                tokens.push(tok);
            }
            _ => {
                return Err(format!(
                    "Syntax Error: Unexpected character '{}' on line {}",
                    c, line_no
                ));
            }
        }
    }
    Ok(())
}
