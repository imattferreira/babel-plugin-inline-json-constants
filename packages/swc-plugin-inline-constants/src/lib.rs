/**
 * TODO
 * - add possibility to customize constants dir entrypoint
 * - TEST THIS SHIT!
 */
/**
 * usage:
 * const timeout = constantify("TIMES.SECONDS.TEN");
 */
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use swc_core::atoms::Atom;
use swc_core::ecma::ast::{Ident, KeyValueProp, Null, ObjectLit, Prop, PropName, PropOrSpread};
use swc_core::ecma::visit::*;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_ecma_ast::{
    ArrayLit, Bool, CallExpr, Callee, Expr, ExprOrSpread, Lit, Number, Program, Str,
};
use swc_ecma_utils::swc_common::{Span, DUMMY_SP};
use swc_ecma_visit::Fold;

lazy_static! {
    static ref CACHE: Mutex<HashMap<String, serde_json::Value>> = Mutex::new(HashMap::new());
}

fn last_index<T>(vector: &Vec<T>) -> usize {
    vector.len() - 1
}

fn is_constantify(expr: &str) -> bool {
    expr.eq("constantify")
}

fn split(path: &Atom) -> Vec<&str> {
    path.split(".").collect::<Vec<&str>>()
}

fn to_constants_file_path(file: &str) -> String {
    "./constants/".to_owned() + file
}

fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

fn get_constants_json(file_path: &str) -> serde_json::Value {
    let content = fs::read_to_string(file_path).unwrap();

    serde_json::from_str(&content).expect("file content should be a json")
}

fn get_json_key<'a>(json: &'a serde_json::Value, key: &'a str) -> &'a serde_json::Value {
    &json[key]
}

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

fn to_call_expr(value: &serde_json::Value, span: Span) -> CallExpr {
    CallExpr {
        callee: Callee::Expr(to_expr(value, DUMMY_SP)),
        args: vec![],
        span,
        type_args: None,
    }
}

pub struct TransformFold;

impl Fold for TransformFold {
    fn fold_call_expr(&mut self, call_expr: CallExpr) -> CallExpr {
        let dumped = call_expr.clone();

        if let Callee::Expr(callee) = call_expr.callee {
            match callee.ident() {
                Some(item) => {
                    if !is_constantify(&item.sym) {
                        return dumped;
                    }

                    let first_arg = call_expr.args.first().expect("missing constant path!");

                    if let Expr::Ident(arg) = &*first_arg.expr {
                        let constant_path = split(&arg.sym);

                        if constant_path.is_empty() {
                            println!("invalid constant path");
                            return dumped;
                        }

                        let mut cache = CACHE.lock().unwrap();
                        let file = constant_path.first().unwrap();
                        let is_file_cached = cache.contains_key(*file);

                        if !is_file_cached {
                            let file_path = to_constants_file_path(file);

                            if !file_exists(&file_path) {
                                println!("file of constants not exists");
                                return dumped;
                            }

                            cache.insert(file.to_string(), get_constants_json(&file_path));
                        }

                        let mut value = cache.get(*file).unwrap();

                        for (index, key) in constant_path.iter().enumerate() {
                            // index 0 is the file
                            if index == 0 {
                                continue;
                            }

                            value = get_json_key(&value, &key);

                            if value.is_null() && index < last_index(&constant_path) {
                                println!("constant is [null]");
                                break;
                            }
                        }

                        return to_call_expr(value, call_expr.span);
                    }
                }
                None => {}
            };
        }

        return dumped;
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut TransformFold)
}

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
// test!(
//     Default::default(),
//     |_| as_folder(TransformVisitor),
//     simple_transform_global_var,
//     // Input codes
//     r#"const msg = constant.str("APP.MESSAGE");"#,
//     // Output codes after transformed with plugin
//     r#"const msg = false;"#
// );
