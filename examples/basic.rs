use anyhow::Result;
use proc_macro2::TokenStream;
use qsp::Expr;

type Entries = Vec<Entry>;
type Entry = ((String, String), String);

fn main() -> Result<()> {
    let input_string = r#"
    (User
        (post (
            ("device" ( "wakeup" "sleep" ))

            ("host" (
                "status"
                ("devices" ("list" "scan"))
            ))
        ))
        (get ("lala"))
    )
    "#;

    let token_stream: TokenStream = input_string.parse().unwrap();
    let ast = qsp::parse(token_stream).unwrap();
    for ((method, user), route) in eval_permission(&ast)? {
        println!("{method} - {user}: {route}");
    }

    Ok(())
}

fn eval_route_tree(
    rt_expr: &Expr,
    current_prefix: String,
    pname: &str,
    mname: &str,
) -> Result<Entries> {
    if let Ok(strlit) = rt_expr.as_str_lit() {
        Ok(vec![(
            (mname.into(), pname.into()),
            format!("{current_prefix}/{}", strlit.contained_string()),
        )])
    } else {
        let (new_prefix_elem, children) = rt_expr.pair_split()?;
        Ok(children.try_flat_map(|rt| {
            eval_route_tree(
                rt,
                format!(
                    "{current_prefix}/{}",
                    new_prefix_elem.as_str_lit()?.contained_string()
                ),
                pname,
                mname,
            )
        })?)
    }
}

fn eval_method(method_expr: &Expr, pname: &str) -> Result<Entries> {
    let (method, route_trees) = method_expr.pair_split()?;
    let mname = method.as_identifier()?.to_string();
    Ok(route_trees.try_flat_map(|rt| eval_route_tree(rt, "".into(), pname, &mname))?)
}

fn eval_permission(permission_expr: &Expr) -> Result<Entries> {
    let (permission, methods) = permission_expr.head_tail_split()?;
    let pname = permission.as_identifier()?.to_string();
    Ok(methods.try_flat_map(|m| eval_method(m, &pname))?)
}
