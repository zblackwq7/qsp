// fn main() {
//     let input_string = r#"
//     (User
//         (post (
//             ("device" ( "wakeup" "sleep" ))

//             ("host" (
//                 "status" ("devices" ("list" "scan"))
//             ))
//         ))
//         (get ("lala"))
//     )
//     "#;

//     for entry in eval_permission(&Expr::parse(input_string)?)? {
//         println!("{entry:?}");
//     }
// }

// fn eval_route_tree(
//     rt_expr: &Expr,
//     current_prefix: String,
//     pname: &str,
//     mname: &str,
// ) -> Result<Entries> {
//     if let Ok(strlit) = rt_expr.as_strlit() {
//         Seq::once((
//             (mname, pname),
//             format!("{current_prefix}/{}", strlit.as_str()),
//         ))
//     } else {
//         (new_prefix_elem, children) = rt_expr.head_tail_split()?;
//         children.try_flat_map(|rt| {
//             eval_route_tree(
//                 rt,
//                 format!("{current_prefix}/{new_prefix_elem}"),
//                 pname,
//                 mname,
//             )
//         })
//     }
// }

// fn eval_method(method_expr: &Expr, pname: &str) -> Result<Entries> {
//     let (method, route_trees) = method_expr.head_tail_split()?;
//     let mname = method.as_symbol()?.as_str();
//     route_trees.try_flat_map(|rt| eval_route_tree(rt, "".into(), pname, mname))?
// }

// fn eval_permission(permission_expr: &Expr) -> Result<Entries> {
//     let (permission, methods) = permission_expr.head_tail_split()?;
//     let pname = permission.as_symbol()?.as_str();
//     methods.try_flat_map(|m| eval_method(m, pname))
// }
//
fn main() {}
