use crate::{
    ast::{
        Assign, Binary, Call, Expr, ExprStmt, IfStmt, LetStmt, Literal, Logical, PrintStmt, Stmt,
        Unary, WhileStmt,
    },
    env::Environment,
    error::InterpErr,
    error::InterpErr as Ie,
    native::Clock,
    obj::LoxObject,
    token::TokenKind as Tk,
};
use format as fmt;
use std::{cell::RefCell, rc::Rc};

pub struct Interpreter {
    //represents the current environment being used by the interpreter
    env: Rc<RefCell<Environment>>,

    //represents the global environment
    globals: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Rc::new(RefCell::new(Environment::new(None)));
        RefCell::borrow_mut(&mut globals).define("clock", LoxObject::Callable(Box::new(Clock {})));
        Self {
            env: Rc::clone(&globals),
            globals: Rc::clone(&globals),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), InterpErr> {
        Ok(for stmt in stmts {
            self.execute(&stmt)?
        })
    }

    fn execute(&mut self, s: &Stmt) -> Result<(), InterpErr> {
        match s {
            Stmt::ExprStmt(expr_stmt) => self.expr_stmt_exec(expr_stmt),
            Stmt::PrintStmt(print_stmt) => self.print_stmt_exec(print_stmt),
            Stmt::LetStmt(let_stmt) => self.let_stmt_exec(let_stmt),
            Stmt::Block(block) => self.block_stmt_exec(block.iter().collect()),
            Stmt::IfStmt(if_stmt) => self.if_stmt_exec(if_stmt),
            Stmt::WhileStmt(while_stmt) => self.while_stmt_exec(while_stmt),
        }
    }

    fn while_stmt_exec(&mut self, w: &WhileStmt) -> Result<(), InterpErr> {
        while truthy(&self.evaluate(&w.condition)?) {
            self.execute(&w.body)?
        }

        Ok(())
    }

    fn if_stmt_exec(&mut self, c: &IfStmt) -> Result<(), InterpErr> {
        let condition = truthy(&self.evaluate(&c.condition)?);

        if condition {
            self.execute(&c.if_branch)
        } else {
            match &c.else_branch {
                Some(branch) => self.execute(&branch),
                None => Ok(()),
            }
        }
    }

    //sets the new env as the current one, executes
    //all statements and then sets the env as the previos one again
    fn block_stmt_exec(&mut self, stmts: Vec<&Stmt>) -> Result<(), InterpErr> {
        let previous = Rc::clone(&self.env);
        self.env = Rc::new(RefCell::new(Environment::new(Some(self.env.clone()))));

        let result = stmts.into_iter().try_for_each(|stat| self.execute(stat));

        self.env = previous;
        result
    }

    fn let_stmt_exec(&mut self, l: &LetStmt) -> Result<(), InterpErr> {
        match &l.initializer {
            Some(init) => {
                let value = self.evaluate(&init)?;
                RefCell::borrow_mut(&self.env).define(&l.ident.lexeme, value);
            }
            None => {
                RefCell::borrow_mut(&self.env).define(&l.ident.lexeme, LoxObject::Null);
            }
        }

        Ok(())
    }

    fn print_stmt_exec(&mut self, ps: &PrintStmt) -> Result<(), InterpErr> {
        let value = self.evaluate(&ps.expr)?;
        println!("{value}");
        Ok(())
    }

    fn expr_stmt_exec(&mut self, es: &ExprStmt) -> Result<(), InterpErr> {
        self.evaluate(&es.expr)?;
        Ok(())
    }

    fn evaluate(&mut self, e: &Expr) -> Result<LoxObject, InterpErr> {
        match e {
            Expr::Assign(assign) => self.assign_eval(assign),
            Expr::Unary(unary) => self.unary_eval(unary),
            Expr::Binary(binary) => self.binary_eval(binary),
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Var(v) => RefCell::borrow_mut(&self.env).get(v),
            Expr::Lit(literal) => self.literal_eval(literal),
            Expr::Logical(logical) => self.logical_eval(logical),
            Expr::Call(call) => self.call_eval(call),
        }
    }

    fn literal_eval(&self, l: &Literal) -> Result<LoxObject, InterpErr> {
        // basically just converts from literal to a lox object
        match l {
            Literal::Str(s) => Ok(LoxObject::Str(s.clone())),
            Literal::Number(n) => Ok(LoxObject::Number(*n)),
            Literal::Bool(b) => Ok(LoxObject::Bool(*b)),
            Literal::Null => Ok(LoxObject::Null),
        }
    }

    fn call_eval(&mut self, c: &Call) -> Result<LoxObject, InterpErr> {
        let callee = self.evaluate(&c.callee)?;
        let mut args = Vec::new();

        for arg in &c.args {
            args.push(self.evaluate(arg)?);
        }

        if let LoxObject::Callable(callable) = callee {
            if args.len() != callable.arity() {
                return rt_error(
                    c.paren.line,
                    &fmt!(
                        "Expected {} arguments, but {} where provided",
                        callable.arity(),
                        args.len()
                    ),
                );
            }

            return callable.call(self, args);
        }

        return rt_error(c.paren.line, "Can only call functions and classes");
    }

    fn logical_eval(&mut self, l: &Logical) -> Result<LoxObject, InterpErr> {
        let left = self.evaluate(&l.left)?;

        if let Tk::Or = l.operator.kind {
            if truthy(&left) {
                return Ok(left);
            }
        } else {
            if !truthy(&left) {
                return Ok(left);
            }
        }

        self.evaluate(&l.right)
    }

    fn assign_eval(&mut self, a: &Assign) -> Result<LoxObject, InterpErr> {
        let value = self.evaluate(&a.value)?;
        RefCell::borrow_mut(&self.env).assign(a.ident.clone(), value.clone())?;
        Ok(value)
    }

    fn binary_eval(&mut self, b: &Binary) -> Result<LoxObject, InterpErr> {
        let left = self.evaluate(&b.left)?;
        let right = self.evaluate(&b.right)?;

        match b.operator.kind {
            Tk::Minus => {
                if let (LoxObject::Number(l), LoxObject::Number(r)) = (left, right) {
                    return Ok(LoxObject::Number(l - r));
                }

                rt_error(b.operator.line, "Operands must be number")
            }
            Tk::Slash => {
                if let (LoxObject::Number(l), LoxObject::Number(r)) = (left, right) {
                    return Ok(LoxObject::Number(l / r));
                }

                rt_error(b.operator.line, "Operands must be number")
            }
            Tk::Star => {
                if let (LoxObject::Number(l), LoxObject::Number(r)) = (left, right) {
                    return Ok(LoxObject::Number(l * r));
                }

                rt_error(b.operator.line, "Operands must be number")
            }
            Tk::Plus => match (left, right) {
                (LoxObject::Number(l), LoxObject::Number(r)) => Ok(LoxObject::Number(l + r)),
                (LoxObject::Str(l), LoxObject::Str(r)) => Ok(LoxObject::Str(l + &r)),
                _ => rt_error(b.operator.line, "Operand must be 'string' or 'number'"),
            },
            Tk::Greater => {
                if let (LoxObject::Number(l), LoxObject::Number(r)) = (left, right) {
                    return Ok(LoxObject::Bool(l > r));
                }

                rt_error(b.operator.line, "Operands must be number")
            }
            Tk::GreaterEqual => {
                if let (LoxObject::Number(l), LoxObject::Number(r)) = (left, right) {
                    return Ok(LoxObject::Bool(l >= r));
                }

                rt_error(b.operator.line, "Operands must be number")
            }
            Tk::Less => {
                if let (LoxObject::Number(l), LoxObject::Number(r)) = (left, right) {
                    return Ok(LoxObject::Bool(l < r));
                }

                rt_error(b.operator.line, "Operands must be number")
            }
            Tk::LessEqual => {
                if let (LoxObject::Number(l), LoxObject::Number(r)) = (left, right) {
                    return Ok(LoxObject::Bool(l <= r));
                }

                rt_error(b.operator.line, "Operands must be number")
            }
            Tk::BangEqual => Ok(LoxObject::Bool(left != right)),
            Tk::EqualEqual => Ok(LoxObject::Bool(left == right)),
            _ => rt_error(b.operator.line, "Invalid operator"),
        }
    }

    fn unary_eval(&mut self, u: &Unary) -> Result<LoxObject, InterpErr> {
        let right = self.evaluate(&u.right)?;

        match u.operator.kind {
            Tk::Bang => Ok(LoxObject::Bool(!truthy(&right))),
            Tk::Minus => {
                if let LoxObject::Number(n) = right {
                    return Ok(LoxObject::Number(-n));
                }

                rt_error(u.operator.line, "Operand must be a number")
            }
            _ => rt_error(u.operator.line, "Ivalid operator"),
        }
    }
}

fn rt_error(line: usize, msg: &str) -> Result<LoxObject, InterpErr> {
    Err(Ie::RuntimeError {
        line,
        msg: msg.to_string(),
    })
}

fn truthy(v: &LoxObject) -> bool {
    match v {
        LoxObject::Bool(b) => *b,
        LoxObject::Null => false,
        _ => true,
    }
}
