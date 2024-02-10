use crate::ast::{Expr, LoxVal};

pub fn visit_literal_expr(expr: Expr) -> LoxVal {
    match expr {
        Expr::Literal(expr_val) => expr_val,
        _ => {
            panic!("asd");
        }
    }
}

fn visit_grouping_expr(expr: Expr) {
    evaluate(expr);
}

fn evaluate(expr: Expr) {
    if let Expr::Grouping(group) = expr {
        group;
    }
}
