use super::lexer::{tokenize, Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(input: String) -> Result<Self, String> {
        let tokens = tokenize(&input)?;
        Ok(Parser { tokens, pos: 0 })
    }

    pub fn parse(&mut self) -> Result<(), String> {
        self.parse_program()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        if !matches!(t, Token::Eof) {
            self.pos += 1;
        }
        t
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Syntax Error: Expected \"{}\", but found \"{}\"",
                expected.name(),
                self.peek().name()
            ))
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Token::Newline) {
            self.advance();
        }
    }

    fn parse_program(&mut self) -> Result<(), String> {
        self.skip_newlines();
        while !matches!(self.peek(), Token::Eof) {
            self.parse_statement()?;
            self.skip_newlines();
        }
        Ok(())
    }

    fn parse_statement(&mut self) -> Result<(), String> {
        match self.peek() {
            Token::If => self.parse_if_stmt(),
            Token::Elif => Err("Syntax Error: 'elif' without matching 'if'".to_string()),
            Token::Else => Err("Syntax Error: 'else' without matching 'if'".to_string()),
            _ => self.parse_simple_stmt(),
        }
    }

    // if_stmt:
    //   | 'if' named_expression ':' block elif_stmt
    //   | 'if' named_expression ':' block [else_block]
    fn parse_if_stmt(&mut self) -> Result<(), String> {
        self.expect(&Token::If)?;
        self.parse_named_expression()?;
        self.expect(&Token::Colon)?;
        self.parse_block()?;
        match self.peek() {
            Token::Elif => self.parse_elif_stmt(),
            Token::Else => self.parse_else_block(),
            _ => Ok(()),
        }
    }

    // elif_stmt:
    //   | 'elif' named_expression ':' block elif_stmt
    //   | 'elif' named_expression ':' block [else_block]
    fn parse_elif_stmt(&mut self) -> Result<(), String> {
        self.expect(&Token::Elif)?;
        self.parse_named_expression()?;
        self.expect(&Token::Colon)?;
        self.parse_block()?;
        match self.peek() {
            Token::Elif => self.parse_elif_stmt(),
            Token::Else => self.parse_else_block(),
            _ => Ok(()),
        }
    }

    // else_block: 'else' ':' block
    fn parse_else_block(&mut self) -> Result<(), String> {
        self.expect(&Token::Else)?;
        self.expect(&Token::Colon)?;
        self.parse_block()
    }

    // block: NEWLINE INDENT statement+ DEDENT
    fn parse_block(&mut self) -> Result<(), String> {
        self.expect(&Token::Newline)?;
        self.skip_newlines();
        self.expect(&Token::Indent)?;

        self.parse_statement()?;
        loop {
            self.skip_newlines();
            match self.peek() {
                Token::Dedent => {
                    self.advance();
                    break;
                }
                Token::Eof => {
                    return Err(
                        "Syntax Error: Expected \"DEDENT\", but found \"EOF\"".to_string()
                    );
                }
                _ => {
                    self.parse_statement()?;
                }
            }
        }
        Ok(())
    }

    fn parse_simple_stmt(&mut self) -> Result<(), String> {
        match self.peek().clone() {
            Token::Identifier(_) => {
                self.advance();
                match self.peek() {
                    Token::Assign => {
                        self.advance();
                        self.parse_named_expression()?;
                    }
                    Token::LParen => {
                        self.parse_call_args()?;
                    }
                    _ => { /* bare identifier expression */ }
                }
                self.expect_stmt_end()
            }
            Token::Number(_) | Token::StringLit(_) => {
                self.parse_named_expression()?;
                self.expect_stmt_end()
            }
            other => Err(format!(
                "Syntax Error: Unexpected token \"{}\"",
                other.name()
            )),
        }
    }

    fn expect_stmt_end(&mut self) -> Result<(), String> {
        match self.peek() {
            Token::Newline => {
                self.advance();
                Ok(())
            }
            Token::Eof | Token::Dedent => Ok(()),
            other => Err(format!(
                "Syntax Error: Expected \"NEWLINE\", but found \"{}\"",
                other.name()
            )),
        }
    }

    fn parse_call_args(&mut self) -> Result<(), String> {
        self.expect(&Token::LParen)?;
        if !matches!(self.peek(), Token::RParen) {
            self.parse_named_expression()?;
            while matches!(self.peek(), Token::Comma) {
                self.advance();
                self.parse_named_expression()?;
            }
        }
        self.expect(&Token::RParen)
    }

    // named_expression here = a comparison or arithmetic expression
    fn parse_named_expression(&mut self) -> Result<(), String> {
        self.parse_additive()?;
        match self.peek() {
            Token::Lt | Token::Gt | Token::LtEq | Token::GtEq | Token::Eq | Token::NotEq => {
                self.advance();
                self.parse_additive()?;
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_additive(&mut self) -> Result<(), String> {
        self.parse_multiplicative()?;
        while matches!(self.peek(), Token::Plus | Token::Minus) {
            self.advance();
            self.parse_multiplicative()?;
        }
        Ok(())
    }

    fn parse_multiplicative(&mut self) -> Result<(), String> {
        self.parse_term()?;
        while matches!(self.peek(), Token::Star | Token::Slash) {
            self.advance();
            self.parse_term()?;
        }
        Ok(())
    }

    fn parse_term(&mut self) -> Result<(), String> {
        match self.peek().clone() {
            Token::Identifier(_) => {
                self.advance();
                if matches!(self.peek(), Token::LParen) {
                    self.parse_call_args()?;
                }
                Ok(())
            }
            Token::Number(_) | Token::StringLit(_) => {
                self.advance();
                Ok(())
            }
            Token::Minus => {
                self.advance();
                self.parse_term()
            }
            Token::LParen => {
                self.advance();
                self.parse_named_expression()?;
                self.expect(&Token::RParen)
            }
            other => Err(format!(
                "Syntax Error: Expected expression, but found \"{}\"",
                other.name()
            )),
        }
    }
}
