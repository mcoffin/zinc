#![feature(rustc_private, plugin_registrar, quote)]

extern crate rustc;
extern crate syntax;

use rustc::plugin::Registry;
use syntax::ast::MetaItem;
use syntax::codemap::{Span, DUMMY_SP};
use syntax::ext::base::{Annotatable, ExtCtxt, MultiDecorator};
use syntax::ext::build::AstBuilder;

macro_rules! and_return {
    ($a:stmt) => (
        {
            $a;
            return;
        }
    )
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(syntax::parse::token::intern("zinc_main"),
                                  MultiDecorator(Box::new(decorator_zinc_main)));
}

pub fn decorator_zinc_main(cx: &mut ExtCtxt,
                           sp: Span,
                           _: &MetaItem,
                           item: &Annotatable,
                           push: &mut FnMut(Annotatable)) {
    let main = match item {
        &Annotatable::Item(ref main_item) => (*main_item).ident,
        _ => and_return!(cx.span_err(sp, "zinc_main must be an item")),
    };

    let call_main = cx.expr_call_ident(DUMMY_SP, main, Vec::new());
    let start = quote_item!(cx,
        #[start]
        fn start(_: isize, _: *const *const u8) -> isize {
            $call_main;
            0
        }
    ).unwrap();
    push(Annotatable::Item(start));
}
