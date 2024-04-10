#[path = "../lox/lib.rs"]
mod lox;

use lox::expr::*;
use lox::token::{Object, Token, TokenType};

struct AstPrinter {}

impl AstPrinter {
    fn parenthesize(&mut self, name: String, exprs: Vec<&Expr>) -> String {
        let mut parenthized = format!("({name}");
        for expr in exprs {
            parenthized.push(' ');
            parenthized.push_str(&expr.accept(self));
        }
        parenthized.push(')');

        return parenthized;
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_literal_expr(&mut self, expr: &Literal) -> String {
        match &expr.value {
            Object::Str(val) => val.to_string(),
            Object::Num(val) => val.to_string(),
            _ => "nil".to_string(),
        }
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> String {
        return self.parenthesize(expr.operator.lexeme.to_string(), vec![&expr.right]);
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> String {
        return self.parenthesize(
            expr.operator.lexeme.to_string(),
            vec![&expr.left, &expr.right],
        );
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> String {
        return self.parenthesize("group".to_string(), vec![&expr.expression]);
    }
}

impl AstPrinter {
    fn print(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(lit) => self.visit_literal_expr(&lit),
            Expr::Unary(unary) => self.visit_unary_expr(&unary),
            Expr::Binary(binary) => self.visit_binary_expr(&binary),
            Expr::Grouping(grouping) => self.visit_grouping_expr(&grouping),
        }
    }
}

fn main() {
    let expression = Expr::Binary(Binary::new(
        Box::new(Expr::Unary(Unary::new(
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            Box::new(Expr::Literal(Literal::new(Object::Num(123.0)))),
        ))),
        Token::new(TokenType::Star, "*".to_string(), None, 1),
        Box::new(Expr::Grouping(Grouping::new(Box::new(Expr::Literal(
            Literal::new(Object::Num(45.67)),
        ))))),
    ));

    let mut printer = AstPrinter {};
    println!("{}", printer.print(&expression));
}
