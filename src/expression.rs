use std::any::Any;
use crate::token::Token;
pub enum Expr {
	Binary(Binary),
	Grouping(Grouping),
	Literal(Literal),
	Unary(Unary),
}

pub trait Visitor<R> {
	fn visit_binary(&self, element: &Binary) -> R ;
	fn visit_grouping(&self, element: &Grouping) -> R ;
	fn visit_literal(&self, element: &Literal) -> R ;
	fn visit_unary(&self, element: &Unary) -> R ;
}

pub trait VisitedElement {
	fn accept<S: Visitor<R>, R>(&self, visitor: S) -> R;
}

pub struct Binary {
	pub left: Box<Expr>,
	pub operator: Token,
	pub right: Box<Expr>,
}

impl Binary {
	pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>, ) -> Self {
		Binary {
			left,
			operator,
			right,
		}
	}
}

impl VisitedElement for Binary {
	fn accept<V: Visitor<R>, R>(&self, visitor: V) -> R {
		visitor.visit_binary(&self)
	}
}

pub struct Grouping {
	pub expression: Box<Expr>,
}

impl Grouping {
	pub fn new(expression: Box<Expr>, ) -> Self {
		Grouping {
			expression,
		}
	}
}

impl VisitedElement for Grouping {
	fn accept<V: Visitor<R>, R>(&self, visitor: V) -> R {
		visitor.visit_grouping(&self)
	}
}

pub struct Literal {
	pub value: Option<String>,
}

impl Literal {
	pub fn new(value: Option<String>, ) -> Self {
		Literal {
			value,
		}
	}
}

impl VisitedElement for Literal {
	fn accept<V: Visitor<R>, R>(&self, visitor: V) -> R {
		visitor.visit_literal(&self)
	}
}

pub struct Unary {
	pub operator: Token,
	pub right: Box<Expr>,
}

impl Unary {
	pub fn new(operator: Token, right: Box<Expr>, ) -> Self {
		Unary {
			operator,
			right,
		}
	}
}

impl VisitedElement for Unary {
	fn accept<V: Visitor<R>, R>(&self, visitor: V) -> R {
		visitor.visit_unary(&self)
	}
}

impl VisitedElement for Expr {
	fn accept<S: Visitor<R>, R>(&self, visitor: S) -> R {
		match self {
			 Expr::Binary(b) => {b.accept(visitor)},
			 Expr::Grouping(b) => {b.accept(visitor)},
			 Expr::Literal(b) => {b.accept(visitor)},
			 Expr::Unary(b) => {b.accept(visitor)},
		}
	}
}
