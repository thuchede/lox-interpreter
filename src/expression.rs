use std::any::Any;
use crate::token::Token;
enum Expr {
	Binary(Binary),
	Grouping(Grouping),
	Literal(Literal),
	Unary(Unary),
}

struct Binary {
	left: Box<Expr>,
	operator: Token,
	right: Box<Expr>,
}

impl Binary {
	fn new(left: Box<Expr>, operator: Token, right: Box<Expr>, ) -> Self {
		Binary {
			left,
			operator,
			right,
		}
	}
}

struct Grouping {
	expression: Box<Expr>,
}

impl Grouping {
	fn new(expression: Box<Expr>, ) -> Self {
		Grouping {
			expression,
		}
	}
}

struct Literal {
	value: Box<dyn Any>,
}

impl Literal {
	fn new(value: Box<dyn Any>, ) -> Self {
		Literal {
			value,
		}
	}
}

struct Unary {
	operator: Token,
	right: Box<Expr>,
}

impl Unary {
	fn new(operator: Token, right: Box<Expr>, ) -> Self {
		Unary {
			operator,
			right,
		}
	}
}

