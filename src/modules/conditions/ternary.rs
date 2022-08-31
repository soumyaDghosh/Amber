use heraclitus_compiler::prelude::*;
use crate::modules::Typed;
use crate::modules::expression::binop::{parse_left_expr, expression_arms_of_same_type};
use crate::modules::expression::expr::Expr;
use crate::translate::module::TranslateModule;
use crate::utils::metadata::{ParserMetadata, TranslateMetadata};

#[derive(Debug)]
pub struct Ternary {
    cond: Box<Expr>,
    true_expr: Box<Expr>,
    false_expr: Box<Expr>
}

impl Typed for Ternary {
    fn get_type(&self) -> crate::modules::Type {
        self.true_expr.get_type()
    }
}

impl SyntaxModule<ParserMetadata> for Ternary {
    syntax_name!("Ternary Expression");

    fn new() -> Self {
        Ternary {
            cond: Box::new(Expr::new()),
            true_expr: Box::new(Expr::new()),
            false_expr: Box::new(Expr::new())
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        parse_left_expr(meta, &mut self.cond, "then")?;
        let tok = meta.get_current_token();
        token(meta, "then")?;
        parse_left_expr(meta, &mut self.true_expr, "else")?;
        token(meta, "else")?;
        syntax(meta, &mut *self.false_expr)?;
        // Return an error if the arms are not of the same type
        let error = "Ternary operation can only be used on arguments of the same type";
        expression_arms_of_same_type(meta, &self.true_expr, &self.false_expr, tok, error);
        Ok(())
    }
}

impl TranslateModule for Ternary {
    fn translate(&self, meta: &mut TranslateMetadata) -> String {
        let cond = self.cond.translate(meta);
        let true_expr = self.true_expr.translate(meta);
        let false_expr = self.false_expr.translate(meta);
        format!("$(if [ {} != 0 ]; then echo {}; else echo {}; fi)", cond, true_expr, false_expr)
    }
}