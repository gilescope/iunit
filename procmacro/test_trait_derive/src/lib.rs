#![crate_type = "dylib"]
#![feature(plugin_registrar, rustc_private)]

#[macro_use] extern crate syntax;
extern crate rustc_plugin;

use syntax::abi::Abi;
use syntax::tokenstream::{TokenStream};
use syntax::ast;
use syntax::util::ThinVec;
use syntax::ext::quote::rt::Span;
use syntax::ext::hygiene::SyntaxContext;
use syntax::ast::{Ident, Item, PathParameters, PathSegment, ParenthesizedParameterData, ExprKind,Expr, ItemKind, StmtKind, Stmt, TyKind, MethodSig,TraitItemKind,
                  Block, Unsafety, Constness, FunctionRetTy, FnDecl,BlockCheckMode,NodeId };
use syntax::ext::quote::rt::BytePos;
use syntax::codemap;
use syntax::codemap::Spanned;
use syntax::ext::base::{ExtCtxt, MultiModifier, Annotatable};
use syntax::ext::build::AstBuilder;
use syntax::ptr::P;
use syntax::symbol::Symbol;

use rustc_plugin::Registry;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(Symbol::intern("trait_tests"),
                                  MultiModifier(Box::new(expand_meta_trait_test)));
}

fn expand_meta_trait_test(cx: &mut ExtCtxt,
                          span: codemap::Span,
                          _: &ast::MetaItem,
                          annot_item: Annotatable) -> Annotatable {
    let item_kind;
    let mut item = annot_item.expect_item();
    {
        match item.node {
            ItemKind::Trait(a, b, ref c, ref d,  ref trait_items) => {
                let mut test_names = vec![];
                for meth in trait_items {
                    test_names.push(meth.ident.name);
                }



//                let y = (
//                    MethodSig {
//                        abi: Abi::Rust,
//                        constness: Spanned {
//                            node: Constness::NotConst,
//                            span
//                        },
//                        decl: P(FnDecl {
//                            inputs: vec![],
//                            output: FunctionRetTy::Default(
//                                span
//                            ),
//                            variadic: false
//                        }),
//
//                        unsafety: Unsafety::Normal
//                    },
//                    Some(cx.block(span, vec![])));

             //   let fn_call = quote_item!{ Self::test_all(); };
                let fn_call = Stmt {
                    id: ast::DUMMY_NODE_ID,
                    node: StmtKind::Semi(P(Expr{
                        id: ast::DUMMY_NODE_ID,
                        node: ExprKind::Call(P(Expr {
                            span,
                            attrs:ThinVec::new(),
                            id:ast::DUMMY_NODE_ID,
                                             node:
                            ExprKind::Path(None, ::syntax::ast::Path {
                                span,
                                segments: vec![PathSegment { span, parameters: None, identifier: Ident::from_str("test_all") }, ]
                            }
                            )
                        }), vec![]),
                        span,
                        attrs: ThinVec::new()
                    })),
                    span,
                };

//                Some(QPath), PAth(None, )){
//                identifier: Ident::from_str("test_all"),
//                span,
//                parameters: Some(P(PathParameters::Parenthesized(ParenthesizedParameterData{ span, inputs:vec![], output:None})))

                let body = cx.block(span, vec![
                    fn_call
                ]);
                //let my_fn = cx.item_fn(span, Ident::from_str("test_all"), vec![], cx.ty(span, TyKind::Tup(vec![])), body);
//                let new_trait = item.clone();
//                new_trait.trait_items.push(my_fn);

//                let TraitItemKind(a1,b1,c1,d1,e1) = trait_items[0].node.clone();
                let func = ::syntax::ast::TraitItemKind::Method(MethodSig {
                        abi: Abi::Rust,
                        constness: Spanned {
                            node: Constness::NotConst,
                            span
                        },
                        decl: P(FnDecl {
                            inputs: vec![],
                            output: FunctionRetTy::Default(
                                span
                            ),
                            variadic: false
                        }),

                        unsafety: Unsafety::Normal
                    }, Some(body) );
                //let func :String = trait_items[0].node.clone();

                let prop = ast::TraitItem {
                    attrs: Vec::new(),
                    ident: Ident::from_str("test_all"),
                    tokens: Some(TokenStream::empty()),
                   id:ast::DUMMY_NODE_ID,
                   span,
                   generics: trait_items[0].generics.clone(),
                    node: func
                   // generics:
                    //..trait_items[0].clone()
                };

                let mut items = trait_items.clone();
                items.push(prop);
                item_kind = ItemKind::Trait(a, b, c.clone(), d.clone(), items);
                return Annotatable::Item(P(Item{ node: item_kind, ..(*item).clone() }))
//                return Annotatable::Item(P(ast::Item { ..(*my_fn).clone()}));

//
//                let first = &trait_items[0];
//
//                trait_items.push(prop);

//              cx.span_err(
//                  span, &format!("match {:#?} {:#?}", test_names[0], prop));
            }
//        ItemKind::Fn(ref decl, unsafety, _constness, abi, _, _) => {
//            let prop_ident = cx.expr_ident(span, item.ident);
//            let prop_ty = cx.ty(span, TyKind::BareFn(P(ast::BareFnTy {
//                unsafety: unsafety,
//                abi: abi,
//                generic_params: vec![],
//                decl: decl.clone().map(|mut decl| {
//                    for arg in decl.inputs.iter_mut() {
//                        arg.pat = arg.pat.clone().map(|mut pat| {
//                            pat.node = PatKind::Wild;
//                            pat
//                        });
//                    }
//                    decl
//                }),
//            })));
//            let inner_ident = cx.expr_cast(span, prop_ident, prop_ty);
//            return wrap_item(cx, span, &*item, inner_ident);
//        },
//        ItemKind::Static(..) => {
//            let inner_ident = cx.expr_ident(span, item.ident);
//            return wrap_item(cx, span, &*item, inner_ident);
//        },
            _ => {
                cx.span_err(
                    span, "#[trait_tests] only supported on traits");
            }
        }
    }
    Annotatable::Item(P(Item{  ..(*item).clone() }))
}


fn wrap_item(cx: &mut ExtCtxt,
             span: codemap::Span,
             item: &ast::Item,
             inner_ident: P<ast::Expr>) -> Annotatable {
    // Copy original function without attributes
    let prop = P(ast::Item {attrs: Vec::new(), ..item.clone()});
    // ::quickcheck::quickcheck
    let check_ident = Ident::from_str("quickcheck");
    let check_path = vec!(check_ident, check_ident);
    // Wrap original function in new outer function,
    // calling ::quickcheck::quickcheck()
    let fn_decl = Stmt {
        id: ast::DUMMY_NODE_ID,
        node: StmtKind::Item(prop),
        span: span,
    };
    let check_call = Stmt {
        id: ast::DUMMY_NODE_ID,
        node: StmtKind::Expr(cx.expr_call_global(span, check_path, vec![inner_ident])),
        span: span,
    };
    let body = cx.block(span, vec![fn_decl, check_call]);
    let test = cx.item_fn(span, item.ident, vec![], cx.ty(span, TyKind::Tup(vec![])), body);

    // Copy attributes from original function
    let mut attrs = item.attrs.clone();
    // Add #[test] attribute
    attrs.push(cx.attribute(
        span, cx.meta_word(span, Symbol::intern("test"))));
    // Attach the attributes to the outer function
    Annotatable::Item(P(ast::Item {attrs: attrs, ..(*test).clone()}))
}

//
//
//use std::panic;
//
//use errors::FatalError;
//use proc_macro::{TokenStream, __internal};
//use syntax::ast::{self, ItemKind, Attribute, Mac};
//use syntax::attr::{mark_used, mark_known};
//use syntax::codemap::Span;
//use syntax::ext::base::*;
//use syntax::visit::Visitor;
//
//struct MarkAttrs<'a>(&'a [ast::Name]);
//
////impl<'a> Visitor<'a> for MarkAttrs<'a> {
////    fn visit_attribute(&mut self, attr: &Attribute) {
////        if let Some(name) = attr.name() {
////            if self.0.contains(&name) {
////                mark_used(attr);
////                mark_known(attr);
////            }
////        }
////    }
////
////    fn visit_mac(&mut self, _mac: &Mac) {}
////}
//
//pub struct ProcMacroDerive2 {
//    inner: fn(TokenStream) -> TokenStream,
//    attrs: Vec<ast::Name>,
//}
//
//impl ProcMacroDerive2 {
//    pub fn new(inner: fn(TokenStream) -> TokenStream, attrs: Vec<ast::Name>) -> ProcMacroDerive {
//        ProcMacroDerive { inner: inner, attrs: attrs }
//    }
//}
//
//impl MultiItemModifier for ProcMacroDerive2 {
//    fn expand(&self,
//              ecx: &mut ExtCtxt,
//              span: Span,
//              _meta_item: &ast::MetaItem,
//              item: Annotatable)
//              -> Vec<Annotatable> {
//        let item = match item {
//            Annotatable::Item(item) => item,
//            Annotatable::ImplItem(_) |
//            Annotatable::TraitItem(_) => {
//                ecx.span_err(span, "proc-macro derives may only be \
//                                    applied to struct/enum items");
//                return Vec::new()
//            }
//        };
////        match item.node {
////            ItemKind::Struct(..) |
////            ItemKind::Enum(..) => {},
////            _ => {
////                ecx.span_err(span, "proc-macro derives may only be \
////                                    applied to struct/enum items");
////                return Vec::new()
////            }
////        }
//
//        // Mark attributes as known, and used.
//        MarkAttrs(&self.attrs).visit_item(&item);
//
//        let input = __internal::new_token_stream(ecx.resolver.eliminate_crate_var(item.clone()));
//        let res = __internal::set_sess(ecx, || {
//            let inner = self.inner;
//            panic::catch_unwind(panic::AssertUnwindSafe(|| inner(input)))
//        });
//
//        let stream = match res {
//            Ok(stream) => stream,
//            Err(e) => {
//                let msg = "proc-macro derive panicked";
//                let mut err = ecx.struct_span_fatal(span, msg);
//                if let Some(s) = e.downcast_ref::<String>() {
//                    err.help(&format!("message: {}", s));
//                }
//                if let Some(s) = e.downcast_ref::<&'static str>() {
//                    err.help(&format!("message: {}", s));
//                }
//
//                err.emit();
//                panic!(FatalError);
//            }
//        };
//
//        __internal::set_sess(ecx, || {
//            match __internal::token_stream_parse_items(stream) {
//                Ok(new_items) => new_items.into_iter().map(Annotatable::Item).collect(),
//                Err(_) => {
//                    // FIXME: handle this better
//                    let msg = "proc-macro derive produced unparseable tokens";
//                    ecx.struct_span_fatal(span, msg).emit();
//                    panic!(FatalError);
//                }
//            }
//        })
//    }
//}
//
//
//


//
//
//fn impl_call_trait_tests(ast: &syn::DeriveInput) -> quote::Tokens {
//    let name = &ast.ident;
//    quote! {
//        #[test] fn HashSet_tests1() { HashSet::<#name>::test_all(); }
//    }
//}
//
