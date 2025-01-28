use define_macro::define;

use crate::token::Token;

define! {
    enum stmt ->  exprStmt(ExprStmt)
                | printStmt(PrintStmt)
                | letStmt(LetStmt);

    struct exprStmt -> expr(Expr);
    struct printStmt -> expr(Expr);
    struct letStmt -> ident(Token), initializer(Option<Expr>);

    enum expr ->  assign(Assign)
                | unary(Unary)
                | binary(Binary)
                | grouping(Box<Expr>)
                | lit(Literal)
                | var(Token);

    struct assign -> ident(Token), value(Box<Expr>);
    struct unary -> operator(Token), right(Box<Expr>);
    struct binary -> left(Box<Expr>), operator(Token), right(Box<Expr>);
    enum literal -> str(String) | number(f64) | bool(bool) | null;
}
