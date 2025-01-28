use define_macro::define;

use crate::token::Token;

define! {
    enum stmt ->  exprStmt(ExprStmt)
                | printStmt(PrintStmt)
                | letStmt(LetStmt)
                | block(Vec<Stmt>)
                | ifStmt(IfStmt);

    struct ifStmt -> condition(Expr), if_branch(Box<Stmt>), else_branch(Option<Box<Stmt>>);
    struct exprStmt -> expr(Expr);
    struct printStmt -> expr(Expr);
    struct letStmt -> ident(Token), initializer(Option<Expr>);

    enum expr ->  assign(Assign)
                | unary(Unary)
                | binary(Binary)
                | grouping(Box<Expr>)
                | lit(Literal)
                | logical(Logical)
                | var(Token);

    struct assign -> ident(Token), value(Box<Expr>);
    struct unary -> operator(Token), right(Box<Expr>);
    struct binary -> left(Box<Expr>), operator(Token), right(Box<Expr>);
    struct logical -> left(Box<Expr>), operator(Token), right(Box<Expr>);
    enum literal -> str(String) | number(f64) | bool(bool) | null;
}
