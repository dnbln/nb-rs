use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input,
    token::{Brace, Break},
    Block, Expr, ExprBlock, ExprBreak, ItemFn, Stmt,
};

#[proc_macro_attribute]
pub fn blocking(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let result = blocking_impl(input);
    let r = quote! {#result};

    r.into()
}

fn df_expr() -> Expr {
    Expr::Break(ExprBreak {
        attrs: vec![],
        break_token: Break {
            span: Span::call_site(),
        },
        expr: None,
        label: None,
    })
}

fn blocking_stmts(stmts: &mut [Stmt]) {
    for stmt in stmts.iter_mut() {
        match stmt {
            Stmt::Local(local_var) => {
                if let Some((_, exp)) = &mut local_var.init {
                    blocking_expr(exp);
                }
            }
            Stmt::Expr(e) => {
                blocking_expr(e);
            }
            Stmt::Semi(e, _) => {
                blocking_expr(e);
            }
            _ => {}
        }
    }
}

fn blocking_block(block: &mut Block) {
    blocking_stmts(&mut block.stmts);
}

fn blocking_expr(e: &mut Expr) {
    match e {
        Expr::Array(a) => {
            for el in a.elems.iter_mut() {
                blocking_expr(el);
            }
        }
        Expr::Assign(a) => {
            blocking_expr(&mut a.left);
            blocking_expr(&mut a.right);
        }
        Expr::AssignOp(a) => {
            blocking_expr(&mut a.left);
            blocking_expr(&mut a.right);
        }
        Expr::Async(a) => {
            *e = Expr::Block(ExprBlock {
                block: std::mem::replace(
                    &mut a.block,
                    Block {
                        brace_token: Brace {
                            span: Span::call_site(),
                        },
                        stmts: vec![],
                    },
                ),
                attrs: vec![],
                label: None,
            });

            blocking_expr(e);
        }
        Expr::Await(a) => {
            *e = std::mem::replace(&mut *a.base, df_expr());
            blocking_expr(e);
        }
        Expr::Binary(b) => {
            blocking_expr(&mut b.left);
            blocking_expr(&mut b.right);
        }
        Expr::Block(b) => {
            blocking_block(&mut b.block);
        }
        Expr::Box(b) => {
            blocking_expr(&mut b.expr);
        }
        Expr::Break(b) => {
            if let Some(brexpr) = &mut b.expr {
                blocking_expr(brexpr);
            }
        }
        Expr::Call(c) => {
            blocking_expr(&mut c.func);

            for arg in c.args.iter_mut() {
                blocking_expr(arg);
            }
        }
        Expr::Cast(c) => {
            blocking_expr(&mut c.expr);
        }
        Expr::Field(f) => {
            blocking_expr(&mut f.base);
        }
        Expr::ForLoop(e) => {
            blocking_expr(&mut e.expr);

            blocking_block(&mut e.body);
        }
        Expr::Group(g) => {
            blocking_expr(&mut g.expr);
        }
        Expr::If(cond) => {
            blocking_expr(&mut cond.cond);

            blocking_block(&mut cond.then_branch);

            if let Some((_, else_branch)) = &mut cond.else_branch {
                blocking_expr(else_branch);
            }
        }
        Expr::Index(ind) => {
            blocking_expr(&mut ind.expr);

            blocking_expr(&mut ind.index);
        }
        Expr::Let(l) => {
            blocking_expr(&mut l.expr);
        }
        Expr::Loop(l) => {
            blocking_block(&mut l.body);
        }
        Expr::Match(m) => {
            blocking_expr(&mut m.expr);

            for arm in m.arms.iter_mut() {
                blocking_expr(&mut arm.body);
            }
        }
        Expr::MethodCall(mc) => {
            blocking_expr(&mut mc.receiver);

            for arg in mc.args.iter_mut() {
                blocking_expr(arg);
            }
        }
        Expr::Paren(p) => {
            blocking_expr(&mut p.expr);
        }
        Expr::Range(r) => {
            if let Some(from) = &mut r.from {
                blocking_expr(from);
            }
            if let Some(to) = &mut r.to {
                blocking_expr(to);
            }
        }
        Expr::Reference(r) => {
            blocking_expr(&mut r.expr);
        }
        Expr::Repeat(r) => {
            blocking_expr(&mut r.expr);
        }
        Expr::Return(r) => {
            if let Some(rval) = &mut r.expr {
                blocking_expr(rval);
            }
        }
        Expr::Try(t) => {
            blocking_expr(&mut t.expr);
        }
        Expr::TryBlock(tb) => {
            blocking_block(&mut tb.block);
        }
        Expr::Tuple(tp) => {
            for tv in tp.elems.iter_mut() {
                blocking_expr(tv);
            }
        }
        Expr::Unary(un) => {
            blocking_expr(&mut un.expr);
        }
        Expr::Unsafe(uns) => {
            blocking_block(&mut uns.block);
        }
        Expr::While(w) => {
            blocking_expr(&mut w.cond);

            blocking_block(&mut w.body);
        }
        Expr::Yield(y) => {
            if let Some(e) = &mut y.expr {
                blocking_expr(e);
            }
        }
        _ => {}
    }
}

fn blocking_impl(mut input: ItemFn) -> ItemFn {
    if input.sig.asyncness.is_none() {
        return input;
    }

    input.sig.asyncness = None;

    blocking_stmts(&mut input.block.stmts);

    input
}
