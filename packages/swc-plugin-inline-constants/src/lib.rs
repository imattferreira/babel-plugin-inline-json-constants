/**
 * TODO
 * - add possibility to customize constants dir entrypoint
 * - TEST THIS SHIT!
 */
/**
 * usage:
 * const timeout = constantify("TIMES.SECONDS.TEN");
 */
mod utils;

use swc_core::ecma::visit::*;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_ecma_ast::{CallExpr, Callee, Expr, Program};
use swc_ecma_visit::Fold;
use utils::array::{last_index, split};
use utils::brain;
use utils::expr::to_call_expr;
use utils::fs::{file_exists, is_constantify, to_constants_file_path};
use utils::json::{get_constants_json, get_json_key};

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

                        let mut cache = brain::instance();
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
