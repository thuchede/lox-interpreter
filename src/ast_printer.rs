use std::io::BufWriter;
use std::ops::Add;
use crate::expression::*;

#[derive(Clone, Copy)]
pub struct AstPrinter {

}

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {

        }
    }

    fn parenthesize(self, name: String, exprs: Vec<&Expr>) -> String {
        let mut builder = String::new();
        builder.push_str(format!("({name}").as_str());
        for x in exprs {
            builder.push_str(" ");
            let res = match x {
                Expr::Binary(e) => {
                    e.accept(self)
                }
                Expr::Grouping(e) => {
                    e.accept(self)
                }
                Expr::Literal(e) => {
                    e.accept(self)
                }
                Expr::Unary(e) => {
                    e.accept(self)
                }
            };
            builder.push_str(res.as_str());
        }
        builder.push_str(")");
        return builder;
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, element: &Binary) -> String {
        self.parenthesize(element.operator.lexeme.clone(), vec![element.left.as_ref(), element.right.as_ref()])
    }

    fn visit_grouping(&self, element: &Grouping) -> String {
        self.parenthesize("Group".to_string(), vec![element.expression.as_ref()])
    }

    fn visit_literal(&self, element: &Literal) -> String {
        element.value.clone().unwrap_or_else(|| "nil".to_string())
    }

    fn visit_unary(&self, element: &Unary) -> String {
        self.parenthesize(element.operator.lexeme.clone(), vec![element.right.as_ref()])
    }
}