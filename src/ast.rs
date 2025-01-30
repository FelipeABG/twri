use define_macro::define;

use crate::token::Token;

define! {
    enum stmt ->  exprStmt(ExprStmt)
                | printStmt(PrintStmt)
                | letStmt(LetStmt)
                | block(Vec<Stmt>)
                | ifStmt(IfStmt)
                | whileStmt(WhileStmt)
                | fnSttmt(FnStmt);

    struct FnStmt -> ident(Token), args(Vec<Token>), body(Box<Stmt>);
    struct forStmt -> range(Expr), body(Box<Stmt>);
    struct whileStmt -> condition(Expr), body(Box<Stmt>);
    struct ifStmt -> condition(Expr), if_branch(Box<Stmt>), else_branch(Option<Box<Stmt>>);
    struct exprStmt -> expr(Expr);
    struct printStmt -> expr(Expr);
    struct letStmt -> ident(Token), initializer(Option<Expr>);

    enum expr ->  assign(Assign)
                | unary(Unary)
                | binary(Binary)
                | call(Call)
                | grouping(Box<Expr>)
                | lit(Literal)
                | logical(Logical)
                | var(Token);

    struct call -> callee(Box<Expr>), paren(Token), args(Vec<Expr>);
    struct assign -> ident(Token), value(Box<Expr>);
    struct unary -> operator(Token), right(Box<Expr>);
    struct binary -> left(Box<Expr>), operator(Token), right(Box<Expr>);
    struct logical -> left(Box<Expr>), operator(Token), right(Box<Expr>);
    enum literal -> str(String) | number(f64) | bool(bool) | null;
}
