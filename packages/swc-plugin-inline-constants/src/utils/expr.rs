use swc_core::atoms::Atom;
use swc_core::ecma::ast::{Ident, KeyValueProp, Null, ObjectLit, Prop, PropName, PropOrSpread};
use swc_core::ecma::visit::*;
use swc_ecma_ast::{ArrayLit, Bool, CallExpr, Callee, Expr, ExprOrSpread, Lit, Number, Str};
use swc_ecma_utils::swc_common::{Span, DUMMY_SP};

// NumericLiteral
fn to_number_expr(value: &serde_json::Value, span: Span) -> Box<Expr> {
    Box::new(Expr::Lit(Lit::Num(Number {
        span,
        value: value.as_f64().unwrap().into(),
        raw: Some(value.as_str().unwrap().into()),
    })))
}

// StringLiteral
fn to_str_expr(value: &serde_json::Value, span: Span) -> Box<Expr> {
    Box::new(Expr::Lit(Lit::Str(Str {
        raw: Some(Atom::new(value.as_str().unwrap())),
        span,
        value: Atom::new(value.as_str().unwrap()),
    })))
}

// BooleanLiteral
fn to_bool_expr(value: &serde_json::Value, span: Span) -> Box<Expr> {
    Box::new(Expr::Lit(Lit::Bool(Bool {
        span,
        value: value.as_bool().unwrap(),
    })))
}

// NullLiteral
fn to_null_expr(span: Span) -> Box<Expr> {
    Box::new(Expr::Lit(Lit::Null(Null { span })))
}

// ArrayExpression
fn to_array_expr(value: &serde_json::Value, span: Span) -> Box<Expr> {
    let elems = value
        .as_array()
        .unwrap()
        .iter()
        .map(|i| to_expr_or_spread(i))
        .collect::<Vec<Option<ExprOrSpread>>>();

    Box::new(Expr::Array(ArrayLit { span, elems }))
}

// ObjectLiteral
fn to_obj_expr(value: &serde_json::Value, span: Span) -> Box<Expr> {
    let props = value
        .as_object()
        .unwrap()
        .iter()
        .map(|(key, value)| {
            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(Ident::new(Atom::new(key.to_string()), DUMMY_SP)),
                value: to_expr(value, DUMMY_SP),
            })))
        })
        .collect();

    Box::new(Expr::Object(ObjectLit { span, props }))
}

fn to_expr(value: &serde_json::Value, span: Span) -> Box<Expr> {
    if value.is_boolean() {
        return to_bool_expr(value, span);
    }

    if value.is_number() {
        return to_number_expr(value, span);
    }

    if value.is_array() {
        return to_array_expr(value, span);
    }

    if value.is_null() {
        return to_null_expr(span);
    }

    if value.is_object() {
        return to_obj_expr(value, span);
    }

    to_str_expr(value, span)
}

fn to_expr_or_spread(value: &serde_json::Value) -> Option<ExprOrSpread> {
    Some(ExprOrSpread {
        expr: to_expr(value, DUMMY_SP),
        spread: None,
    })
}

pub fn to_call_expr(value: &serde_json::Value, span: Span) -> CallExpr {
    CallExpr {
        callee: Callee::Expr(to_expr(value, DUMMY_SP)),
        args: vec![],
        span,
        type_args: None,
    }
}
