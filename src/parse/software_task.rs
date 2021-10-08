use syn::{parse, ForeignItemFn, ItemFn, Stmt};

use crate::modality_probe::parse::modality_probe_args;
use crate::{
    ast::{SoftwareTask, SoftwareTaskArgs},
    parse::util,
};

impl SoftwareTask {
    pub(crate) fn parse(args: SoftwareTaskArgs, mut item: ItemFn) -> parse::Result<Self> {
        let valid_signature =
            util::check_fn_signature(&item) && util::type_is_unit(&item.sig.output);

        let span = item.sig.ident.span();

        let name = item.sig.ident.to_string();

        let probe = item
            .attrs
            .iter()
            .position(|attr| util::attr_eq(attr, "modality_probe"))
            .map(|pos| modality_probe_args(item.attrs.remove(pos).tokens))
            .transpose()?;

        if valid_signature {
            if let Some((context, Ok(inputs))) = util::parse_inputs(item.sig.inputs, &name) {
                let (cfgs, attrs) = util::extract_cfgs(item.attrs);

                return Ok(SoftwareTask {
                    args,
                    attrs,
                    cfgs,
                    context,
                    inputs,
                    stmts: item.block.stmts,
                    is_extern: false,
                    probe,
                });
            }
        }

        Err(parse::Error::new(
            span,
            &format!(
                "this task handler must have type signature `fn({}::Context, ..)`",
                name
            ),
        ))
    }
}

impl SoftwareTask {
    pub(crate) fn parse_foreign(
        args: SoftwareTaskArgs,
        mut item: ForeignItemFn,
    ) -> parse::Result<Self> {
        let valid_signature =
            util::check_foreign_fn_signature(&item) && util::type_is_unit(&item.sig.output);

        let span = item.sig.ident.span();

        let name = item.sig.ident.to_string();

        let probe = item
            .attrs
            .iter()
            .position(|attr| util::attr_eq(attr, "modality_probe"))
            .map(|pos| modality_probe_args(item.attrs.remove(pos).tokens))
            .transpose()?;

        if valid_signature {
            if let Some((context, Ok(inputs))) = util::parse_inputs(item.sig.inputs, &name) {
                let (cfgs, attrs) = util::extract_cfgs(item.attrs);

                return Ok(SoftwareTask {
                    args,
                    attrs,
                    cfgs,
                    context,
                    inputs,
                    stmts: Vec::<Stmt>::new(),
                    is_extern: true,
                    probe,
                });
            }
        }

        Err(parse::Error::new(
            span,
            &format!(
                "this task handler must have type signature `fn({}::Context, ..)`",
                name
            ),
        ))
    }
}
