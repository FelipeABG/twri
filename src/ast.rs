use define_macro::define;

use crate::token::Token;

define! {
    enum stmt ->  exprStmt(ExprStmt)
                | printStmt(PrintStmt);

    struct exprStmt -> expr(Expr);
    struct printStmt -> expr(Expr);
    enum expr ->  unary(Unary)
                | binary(Binary)
                | grouping(Box<Expr>)
                | lit(Literal);

    struct unary -> operator(Token), right(Box<Expr>);
    struct binary -> left(Box<Expr>), operator(Token), right(Box<Expr>);
    enum literal -> str(String) | number(f64) | bool(bool) | null;
}
