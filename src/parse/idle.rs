use proc_macro2::TokenStream as TokenStream2;
use syn::{parse, ItemFn};

use crate::modality_probe::parse::modality_probe_args;
use crate::{
    ast::{Idle, IdleArgs},
    parse::util,
};

impl IdleArgs {
    pub(crate) fn parse(tokens: TokenStream2) -> parse::Result<Self> {
        crate::parse::idle_args(tokens)
    }
}

impl Idle {
    pub(crate) fn parse(args: IdleArgs, mut item: ItemFn) -> parse::Result<Self> {
        let valid_signature = util::check_fn_signature(&item)
            && item.sig.inputs.len() == 1
            && util::type_is_bottom(&item.sig.output);

        let name = item.sig.ident.to_string();

        let probe = item
            .attrs
            .iter()
            .position(|attr| util::attr_eq(attr, "modality_probe"))
            .map(|pos| modality_probe_args(item.attrs.remove(pos).tokens))
            .transpose()?;

        if valid_signature {
            if let Some((context, Ok(rest))) = util::parse_inputs(item.sig.inputs, &name) {
                if rest.is_empty() {
                    return Ok(Idle {
                        args,
                        attrs: item.attrs,
                        context,
                        name: item.sig.ident,
                        stmts: item.block.stmts,
                        probe,
                    });
                }
            }
        }

        Err(parse::Error::new(
            item.sig.ident.span(),
            &format!(
                "this `#[idle]` function must have signature `fn({}::Context) -> !`",
                name
            ),
        ))
    }
}
