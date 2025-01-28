use crate::ast::{
    Assign, Binary, Expr, ExprStmt, IfStmt, LetStmt, Literal, PrintStmt, Stmt, Unary,
};
use crate::error::InterpErr;
use crate::error::InterpErr as Ie;
use crate::token::Token;
use crate::token::TokenKind as Tk;
use crate::token::TokenKind;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    // A program is a list of statements. So parsing the list of tokens
    // is generating a list os statements
    pub fn parse(&mut self) -> Result<Vec<Stmt>, InterpErr> {
        let mut statements = Vec::new();

        while !self.finished() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    // Declarations are statements that declare names (variables, functions, classes)
    fn declaration(&mut self) -> Result<Stmt, InterpErr> {
        if let Tk::Let = self.peek().kind {
            //Consumes the 'let' keyword
            self.next_token();
            return self.let_declaration();
        }

        self.statement()
    }

    fn let_declaration(&mut self) -> Result<Stmt, InterpErr> {
        let ident = self.next_token().clone();
        match ident.kind {
            Tk::Identifier => {
                let mut init = None;
                if let Tk::Equal = self.peek().kind {
                    // Consumes the '=' token
                    self.next_token();
                    init = Some(self.expression()?);
                }
                self.expect(Tk::Semicolon, "Expect ';' after declaration")?;
                Ok(Stmt::LetStmt(LetStmt::new(ident, init)))
            }
            _ => Err(InterpErr::RuntimeError {
                line: ident.line,
                msg: "Expected identifier".to_string(),
            }),
        }
    }

    fn statement(&mut self) -> Result<Stmt, InterpErr> {
        if let Tk::Print = self.peek().kind {
            //Consumes the 'print' token
            self.next_token();
            return self.print_statement();
        }

        if let Tk::LeftBrace = self.peek().kind {
            //consumes the '{' token;
            self.next_token();
            return Ok(Stmt::Block(self.block()?));
        }

        if let Tk::If = self.peek().kind {
            //consumes the 'if' token
            self.next_token();
            return self.if_statement();
        }

        self.expr_statement()
    }

    fn if_statement(&mut self) -> Result<Stmt, InterpErr> {
        let condition = self.expression()?;
        let if_branch = Box::new(self.statement()?);

        let mut else_branch = None;
        if let Tk::Else = self.peek().kind {
            //consumes the 'else' token
            self.next_token();
            else_branch = Some(Box::new(self.statement()?))
        }

        Ok(Stmt::IfStmt(IfStmt::new(condition, if_branch, else_branch)))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, InterpErr> {
        let mut stmts = Vec::new();

        while !matches!(self.peek().kind, Tk::RightBrace) && !self.finished() {
            stmts.push(self.declaration()?);
        }

        self.expect(Tk::RightBrace, "Expected '}' at end of block")?;
        Ok(stmts)
    }

    fn print_statement(&mut self) -> Result<Stmt, InterpErr> {
        let expr = self.expression()?;
        self.expect(Tk::Semicolon, "Expect ';' after value")?;
        Ok(Stmt::PrintStmt(PrintStmt::new(expr)))
    }

    fn expr_statement(&mut self) -> Result<Stmt, InterpErr> {
        let expr = self.expression()?;
        self.expect(Tk::Semicolon, "Expect ';' after expression")?;
        Ok(Stmt::ExprStmt(ExprStmt::new(expr)))
    }

    fn expression(&mut self) -> Result<Expr, InterpErr> {
        self.assign()
    }

    fn assign(&mut self) -> Result<Expr, InterpErr> {
        let expr = self.equality()?;

        if let Tk::Equal = self.peek().kind {
            //consumens the '=' token
            let equals = self.next_token().clone();
            let value = self.assign()?;

            if let Expr::Var(v) = expr {
                let ident = v;
                return Ok(Expr::Assign(Assign::new(ident, Box::new(value))));
            }

            return Err(Ie::RuntimeError {
                line: equals.line,
                msg: "Invalid assignment target.".to_string(),
            });
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, InterpErr> {
        let mut expr = self.comparison()?;

        while let Tk::BangEqual | Tk::EqualEqual = self.peek().kind {
            let left = Box::new(expr.clone());
            let operator = self.next_token().clone();
            let right = Box::new(self.comparison()?.clone());
            expr = Expr::Binary(Binary::new(left, operator, right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, InterpErr> {
        let mut expr = self.term()?;

        while let Tk::Greater | Tk::GreaterEqual | Tk::Less | Tk::LessEqual = self.peek().kind {
            let left = Box::new(expr.clone());
            let operator = self.next_token().clone();
            let right = Box::new(self.term()?.clone());
            expr = Expr::Binary(Binary::new(left, operator, right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, InterpErr> {
        let mut expr = self.factor()?;

        while let Tk::Minus | Tk::Plus = self.peek().kind {
            let left = Box::new(expr.clone());
            let operator = self.next_token().clone();
            let right = Box::new(self.factor()?.clone());
            expr = Expr::Binary(Binary::new(left, operator, right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, InterpErr> {
        let mut expr = self.unary()?;

        while let Tk::Slash | Tk::Star = self.peek().kind {
            let left = Box::new(expr.clone());
            let operator = self.next_token().clone();
            let right = Box::new(self.unary()?.clone());
            expr = Expr::Binary(Binary::new(left, operator, right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, InterpErr> {
        while let Tk::Bang | Tk::Minus = self.peek().kind {
            let operator = self.next_token().clone();
            let right = Box::new(self.unary()?.clone());
            return Ok(Expr::Unary(Unary::new(operator, right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, InterpErr> {
        match self.peek().clone().kind {
            TokenKind::False => {
                self.next_token();
                Ok(Expr::Lit(Literal::Bool(false)))
            }
            TokenKind::True => {
                self.next_token();
                Ok(Expr::Lit(Literal::Bool(true)))
            }
            TokenKind::Null => {
                self.next_token();
                Ok(Expr::Lit(Literal::Null))
            }
            TokenKind::Number(n) => {
                self.next_token();
                Ok(Expr::Lit(Literal::Number(n)))
            }
            TokenKind::String(s) => {
                self.next_token();
                Ok(Expr::Lit(Literal::Str(s)))
            }
            TokenKind::LeftParen => {
                self.next_token();
                let expr = Box::new(self.expression()?);
                self.expect(Tk::RightParen, "Expected ')' after expression")?;
                Ok(Expr::Grouping(expr))
            }
            TokenKind::Identifier => Ok(Expr::Var(self.next_token().clone())),
            _ => Err(Ie::SyntaxError {
                line: self.peek().line,
                msg: "Expected Expression".to_string(),
                place: self.peek().lexeme.clone(),
            }),
        }
    }

    fn expect(&mut self, kind: TokenKind, msg: &str) -> Result<(), InterpErr> {
        if kind == self.peek().kind {
            self.next_token();
            return Ok(());
        }

        Err(Ie::SyntaxError {
            line: self.peek().line,
            msg: msg.to_string(),
            place: self.peek().lexeme.clone(),
        })
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn finished(&self) -> bool {
        self.tokens[self.current].kind == Tk::Eof
    }

    fn next_token(&mut self) -> &Token {
        if !self.finished() {
            self.current += 1
        };
        self.previous()
    }
}
