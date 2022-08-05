use heraclitus_compiler::prelude::*;
use crate::{utils::metadata::ParserMetadata, modules::{Type, Typed}};
use super::expr::Expr;

#[derive(Debug)]
pub struct Parenthesis {
    value: Box<Expr>,
    kind: Type
}

impl Typed for Parenthesis {
    fn get_type(&self) -> Type {
        self.kind.clone()
    }
}

impl SyntaxModule<ParserMetadata> for Parenthesis {
    syntax_name!("Parenthesis");

    fn new() -> Self {
        Parenthesis {
            value: Box::new(Expr::new()),
            kind: Type::Void
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        token(meta, "(")?;
        syntax(meta, &mut *self.value)?;
        self.kind = self.value.get_type();
        token(meta, ")")?;
        Ok(())
    }
}