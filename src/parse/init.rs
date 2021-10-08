use proc_macro2::TokenStream as TokenStream2;

use syn::{parse, ItemFn};

use crate::modality_probe::parse::modality_probe_args;
use crate::{
    ast::{Init, InitArgs},
    parse::util,
};

impl InitArgs {
    pub(crate) fn parse(tokens: TokenStream2) -> parse::Result<Self> {
        crate::parse::init_args(tokens)
    }
}

impl Init {
    pub(crate) fn parse(args: InitArgs, mut item: ItemFn) -> parse::Result<Self> {
        let valid_signature = util::check_fn_signature(&item) && item.sig.inputs.len() == 1;

        let span = item.sig.ident.span();

        let name = item.sig.ident.to_string();

        let probe = item
            .attrs
            .iter()
            .position(|attr| util::attr_eq(attr, "modality_probe"))
            .map(|pos| modality_probe_args(item.attrs.remove(pos).tokens))
            .transpose()?;

        if valid_signature {
            if let Ok((user_shared_struct, user_local_struct)) =
                util::type_is_init_return(&item.sig.output, &name)
            {
                if let Some((context, Ok(rest))) = util::parse_inputs(item.sig.inputs, &name) {
                    if rest.is_empty() {
                        return Ok(Init {
                            args,
                            attrs: item.attrs,
                            context,
                            name: item.sig.ident,
                            stmts: item.block.stmts,
                            user_shared_struct,
                            user_local_struct,
                            probe,
                        });
                    }
                }
            }
        }

        Err(parse::Error::new(
            span,
            &format!(
                "the `#[init]` function must have signature `fn({}::Context) -> (Shared resources struct, Local resources struct, {0}::Monotonics)`",
                name
            ),
        ))
    }
}
