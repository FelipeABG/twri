use std::fmt::Display;

use crate::{
    ast::{
        Assign, Binary, Expr, ExprStmt, IfStmt, LetStmt, Literal, Logical, PrintStmt, Stmt, Unary,
    },
    env::Environment,
    error::InterpErr,
    error::InterpErr as Ie,
    token::TokenKind as Tk,
};

// Literals are a bit of syntax that produces a value. They exist in the source code.
// Values dont exist in the source code, they are computed in the interprerter.
type Value = Literal;

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Literal::Str(s) => format!("{s}"),
            Literal::Number(n) => format!("{n}"),
            Literal::Null => format!("null"),
            Literal::Bool(b) => format!("{b}"),
        };
        write!(f, "{msg}")
    }
}

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(None),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), InterpErr> {
        Ok(for stmt in stmts {
            self.execute(stmt)?
        })
    }

    fn execute(&mut self, s: Stmt) -> Result<(), InterpErr> {
        match s {
            Stmt::ExprStmt(expr_stmt) => self.expr_stmt_exec(expr_stmt),
            Stmt::PrintStmt(print_stmt) => self.print_stmt_exec(print_stmt),
            Stmt::LetStmt(let_stmt) => self.let_stmt_exec(let_stmt),
            Stmt::Block(block) => self.block_stmt_exec(block),
            Stmt::IfStmt(if_stmt) => self.if_stmt_exec(if_stmt),
        }
    }

    fn if_stmt_exec(&mut self, c: IfStmt) -> Result<(), InterpErr> {
        let condition = truthy(&self.evaluate(c.condition)?);

        if condition {
            self.execute(*c.if_branch)
        } else {
            match c.else_branch {
                Some(branch) => self.execute(*branch),
                None => Ok(()),
            }
        }
    }

    //sets the new env as the current one, executes
    //all statements and then sets the env as the previos one again
    fn block_stmt_exec(&mut self, stmts: Vec<Stmt>) -> Result<(), InterpErr> {
        let previous = self.env.clone();
        let new = Environment::new(Some(Box::new(self.env.clone())));
        self.env = new;

        for stmt in stmts {
            self.execute(stmt)?;
        }

        self.env = previous;
        Ok(())
    }

    fn let_stmt_exec(&mut self, l: LetStmt) -> Result<(), InterpErr> {
        match l.initializer {
            Some(init) => {
                let value = self.evaluate(init)?;
                self.env.define(l.ident.lexeme, value);
            }
            None => {
                self.env.define(l.ident.lexeme, Value::Null);
            }
        }

        Ok(())
    }

    fn print_stmt_exec(&mut self, ps: PrintStmt) -> Result<(), InterpErr> {
        let value = self.evaluate(ps.expr)?;
        println!("{value}");
        Ok(())
    }

    fn expr_stmt_exec(&mut self, es: ExprStmt) -> Result<(), InterpErr> {
        self.evaluate(es.expr)?;
        Ok(())
    }

    fn evaluate(&mut self, e: Expr) -> Result<Value, InterpErr> {
        match e {
            Expr::Assign(assign) => self.assign_eval(assign),
            Expr::Unary(unary) => self.unary_eval(unary),
            Expr::Binary(binary) => self.binary_eval(binary),
            Expr::Grouping(expr) => self.evaluate(*expr),
            Expr::Var(v) => self.env.get(v),
            Expr::Lit(literal) => Ok(literal),
            Expr::Logical(logical) => self.logical_eval(logical),
        }
    }

    fn logical_eval(&mut self, l: Logical) -> Result<Value, InterpErr> {
        let left = self.evaluate(*l.left)?;

        if let Tk::Or = l.operator.kind {
            if truthy(&left) {
                return Ok(left);
            }
        } else {
            if !truthy(&left) {
                return Ok(left);
            }
        }

        self.evaluate(*l.right)
    }

    fn assign_eval(&mut self, a: Assign) -> Result<Value, InterpErr> {
        let value = self.evaluate(*a.value)?;
        self.env.assign(a.ident, value.clone())?;
        Ok(value)
    }

    fn binary_eval(&mut self, b: Binary) -> Result<Value, InterpErr> {
        let left = self.evaluate(*b.left)?;
        let right = self.evaluate(*b.right)?;

        match b.operator.kind {
            Tk::Minus => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    return Ok(Value::Number(l - r));
                }

                rt_error(b.operator.line, "Operands must be number")
            }
            Tk::Slash => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    return Ok(Value::Number(l / r));
                }

                rt_error(b.operator.line, "Operands must be number")
            }
            Tk::Star => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    return Ok(Value::Number(l * r));
                }

                rt_error(b.operator.line, "Operands must be number")
            }
            Tk::Plus => {
                if let (Literal::Number(l), Literal::Number(r)) = (left.clone(), right.clone()) {
                    return Ok(Value::Number(l + r));
                }

                if let (Literal::Str(l), Literal::Str(r)) = (left, right) {
                    return Ok(Value::Str(l + &r));
                }

                rt_error(b.operator.line, "Operand must be 'string' or 'number'")
            }
            Tk::Greater => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    return Ok(Value::Bool(l > r));
                }

                rt_error(b.operator.line, "Operands must be number")
            }
            Tk::GreaterEqual => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    return Ok(Value::Bool(l >= r));
                }

                rt_error(b.operator.line, "Operands must be number")
            }
            Tk::Less => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    return Ok(Value::Bool(l < r));
                }

                rt_error(b.operator.line, "Operands must be number")
            }
            Tk::LessEqual => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    return Ok(Value::Bool(l <= r));
                }

                rt_error(b.operator.line, "Operands must be number")
            }
            Tk::BangEqual => Ok(Value::Bool(left != right)),
            Tk::EqualEqual => Ok(Value::Bool(left == right)),
            _ => rt_error(b.operator.line, "Invalid operator"),
        }
    }

    fn unary_eval(&mut self, u: Unary) -> Result<Value, InterpErr> {
        let right = self.evaluate(*u.right)?;

        match u.operator.kind {
            Tk::Bang => Ok(Value::Bool(!truthy(&right))),
            Tk::Minus => {
                if let Literal::Number(n) = right {
                    return Ok(Value::Number(-n));
                }

                rt_error(u.operator.line, "Operand must be a number")
            }
            _ => rt_error(u.operator.line, "Ivalid operator"),
        }
    }
}

fn rt_error(line: usize, msg: &str) -> Result<Value, InterpErr> {
    Err(Ie::RuntimeError {
        line,
        msg: msg.to_string(),
    })
}

fn truthy(v: &Literal) -> bool {
    match v {
        Literal::Bool(b) => *b,
        Literal::Null => false,
        _ => true,
    }
}
