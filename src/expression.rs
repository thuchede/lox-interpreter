use std::any::Any;
use crate::token::Token;
enum Expr {
	Binary(Binary),
	Grouping(Grouping),
	Literal(Literal),
	Unary(Unary),
}

trait Visitor<R> {
	fn visit_binary(self, element: Binary) -> R ;
	fn visit_grouping(self, element: Grouping) -> R ;
	fn visit_literal(self, element: Literal) -> R ;
	fn visit_unary(self, element: Unary) -> R ;
}

trait VisitedElement {
	fn accept<S: Visitor<R>, R>(self, visitor: S) -> R;
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

impl VisitedElement for Binary {
	fn accept<V: Visitor<R>, R>(self, visitor: V) -> R {
		visitor.visit_binary(self)
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

impl VisitedElement for Grouping {
	fn accept<V: Visitor<R>, R>(self, visitor: V) -> R {
		visitor.visit_grouping(self)
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

impl VisitedElement for Literal {
	fn accept<V: Visitor<R>, R>(self, visitor: V) -> R {
		visitor.visit_literal(self)
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

impl VisitedElement for Unary {
	fn accept<V: Visitor<R>, R>(self, visitor: V) -> R {
		visitor.visit_unary(self)
	}
}

