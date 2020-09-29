// use std::collections::HashMap;
//
// use generational_arena::Index;
//
// use crate::ast::Expression;
// use crate::types::Type;
// use crate::types::Type::Number;
//
// #[derive(Debug)]
// pub struct TypeInference {
//     types: HashMap<String, Type>, // TODO: change `String` -> `&'a str` for performance
// }
//
// impl TypeInference {
//     pub fn new() -> Self {
//         let mut types = HashMap::new();
//         for i in 0..4 {
//             let size = 2_usize.pow(i);
//             types.insert(format!("u{}", size * 8), Number { size, signed: true });
//             types.insert(format!("s{}", size * 8), Number { size, signed: false });
//         }
//         Self { types }
//     }
//
//     pub fn infer_ast_of_file(&self, ast: &Index) -> Result<(), String> {
//         match ast {
//             Expression::Body(expressions) => {
//                 let mut functions = HashMap::new();
//                 for expression in expressions {
//                     if let Expression::Declaration(declaration, typ, value) = expression {
//                         if let Expression::FunctionHeader(name, vars) = declaration {
//                             match value {
//                                 Expression::Body(body) => {
//                                     functions.insert(name, (typ, vars, body));
//                                 }
//                                 _ => {
//                                     return Err(format!(
//                                         "Function {} does not contain a body",
//                                         name
//                                     ));
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 for (function_name, (_return_type, _function_params, function_expressions)) in functions {
//                     println!("function: {}", function_name);
//                     for expression in function_expressions {
//                         if let Expression::Declaration(declaration, ref mut typ, _value) = expression {
//                             if let Type::Undetermined { name } = typ {
//                                 if let Some(inferred_type) = self.types.get(name) {
//                                     *typ = (*inferred_type).clone();
//                                     if let Expression::Ident(var_name) = declaration {
//                                         println!("type of {} is {}", var_name, inferred_type);
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 Ok(())
//             }
//             _ => Err("Expression given is not a body".to_string()),
//         }
//     }
// }
