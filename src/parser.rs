use crate::ast::{
    Assign, Binary, Call, Expr, ExprStmt, FnStmt, IfStmt, LetStmt, Literal, Logical, PrintStmt,
    Stmt, Unary, WhileStmt,
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

        if let Tk::While = self.peek().kind {
            //consumes the 'while' token
            self.next_token();
            return self.while_statement();
        }

        if let Tk::For = self.peek().kind {
            //consumes the 'for' token
            self.next_token();
            return self.for_statement();
        }

        if let Tk::Fn = self.peek().kind {
            //consues the 'fn' token
            self.next_token();
            return self.fn_statement();
        }

        self.expr_statement()
    }

    fn fn_statement(&mut self) -> Result<Stmt, InterpErr> {
        let ident = self.expect(Tk::Identifier, "Expected identifier")?;
        self.expect(Tk::LeftParen, "Expected '(' after function identifier");

        let mut args = Vec::new();
        while !matches!(self.peek().kind, Tk::RightParen) {
            if args.len() > 255 {
                return Err(InterpErr::RuntimeError {
                    line: self.peek().line,
                    msg: "Cant have more than 255 parameters".to_string(),
                });
            }
            args.push(self.next_token().clone());
            if Tk::Semicolon == self.peek().kind {
                self.next_token();
            }
        }
        self.expect(Tk::RightParen, "Expected ')' after paremeters");
        self.expect(Tk::LeftBrace, "Expected '{' before function body");
        let body = self.block()?;
        Ok(Stmt::FnStmt(FnStmt::new(ident, args, body)))
    }

    fn for_statement(&mut self) -> Result<Stmt, InterpErr> {
        let init;
        if let Tk::Semicolon = self.peek().kind {
            init = None;
        } else if let Tk::Let = self.peek().kind {
            //consumes the 'let' token
            self.next_token();
            init = Some(self.let_declaration()?);
        } else {
            init = Some(self.expr_statement()?);
        }

        //Condition
        let mut condition = None;
        if !matches!(self.peek().kind, Tk::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.expect(Tk::Semicolon, "Expected ';' after loop condition")?;

        //Increment
        let mut increment = None;
        if !matches!(self.peek().kind, Tk::Semicolon) {
            increment = Some(self.expression()?);
        }

        //body
        let mut body = self.statement()?;

        // Desugaring into a while loop. THIS IS FUCKING MAGIC!!!
        if let Some(inc) = increment {
            body = Stmt::Block(Vec::from([body, Stmt::ExprStmt(ExprStmt::new(inc))]))
        }

        if let None = condition {
            condition = Some(Expr::Lit(Literal::Bool(true)))
        }

        body = Stmt::WhileStmt(WhileStmt::new(condition.unwrap(), Box::new(body)));

        if let Some(i) = init {
            body = Stmt::Block(Vec::from([i, body]))
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt, InterpErr> {
        let condition = self.expression()?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::WhileStmt(WhileStmt::new(condition, body)))
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
        let expr = self.or()?;

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

    fn or(&mut self) -> Result<Expr, InterpErr> {
        let mut left = self.and()?;

        while let Tk::Or = self.peek().kind {
            let operator = self.next_token().clone();
            let right = self.and()?;
            left = Expr::Logical(Logical::new(Box::new(left), operator, Box::new(right)))
        }

        Ok(left)
    }

    fn and(&mut self) -> Result<Expr, InterpErr> {
        let mut left = self.equality()?;

        while let Tk::And = self.peek().kind {
            let operator = self.next_token().clone();
            let right = self.equality()?;
            left = Expr::Logical(Logical::new(Box::new(left), operator, Box::new(right)))
        }

        Ok(left)
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

        self.call()
    }

    fn call(&mut self) -> Result<Expr, InterpErr> {
        let mut expr = self.primary()?;

        loop {
            if let Tk::LeftParen = self.peek().kind {
                //consumes the '(' token
                self.next_token();
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, e: Expr) -> Result<Expr, InterpErr> {
        let mut args = Vec::new();
        if !matches!(self.peek().kind, Tk::RightParen) {
            loop {
                if args.len() > 255 {
                    return Err(InterpErr::RuntimeError {
                        line: self.peek().line,
                        msg: "functions only accept a maximum of 255 arguments".to_string(),
                    });
                }
                args.push(self.expression()?);
                if let Tk::Comma = self.peek().kind {
                    self.next_token();
                } else {
                    break;
                }
            }
        }

        let paren = self.expect(Tk::RightParen, "Expect ')' after arguments")?;
        Ok(Expr::Call(Call::new(Box::new(e), paren, args)))
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

    fn expect(&mut self, kind: TokenKind, msg: &str) -> Result<Token, InterpErr> {
        if kind == self.peek().kind {
            return Ok(self.next_token().clone());
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
