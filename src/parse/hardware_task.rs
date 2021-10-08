use syn::{parse, ForeignItemFn, ItemFn, Stmt};

use crate::modality_probe::parse::modality_probe_args;
use crate::{
    ast::{HardwareTask, HardwareTaskArgs},
    parse::util,
};

impl HardwareTask {
    pub(crate) fn parse(args: HardwareTaskArgs, mut item: ItemFn) -> parse::Result<Self> {
        let span = item.sig.ident.span();
        let valid_signature = util::check_fn_signature(&item)
            && item.sig.inputs.len() == 1
            && util::type_is_unit(&item.sig.output);

        let name = item.sig.ident.to_string();

        if name == "init" || name == "idle" {
            return Err(parse::Error::new(
                span,
                "tasks cannot be named `init` or `idle`",
            ));
        }

        let probe = item
            .attrs
            .iter()
            .position(|attr| util::attr_eq(attr, "modality_probe"))
            .map(|pos| modality_probe_args(item.attrs.remove(pos).tokens))
            .transpose()?;

        if valid_signature {
            if let Some((context, Ok(rest))) = util::parse_inputs(item.sig.inputs, &name) {
                if rest.is_empty() {
                    let (cfgs, attrs) = util::extract_cfgs(item.attrs);

                    return Ok(HardwareTask {
                        args,
                        cfgs,
                        attrs,
                        context,
                        stmts: item.block.stmts,
                        is_extern: false,
                        probe,
                    });
                }
            }
        }

        Err(parse::Error::new(
            span,
            &format!(
                "this task handler must have type signature `fn({}::Context)`",
                name
            ),
        ))
    }
}

impl HardwareTask {
    pub(crate) fn parse_foreign(
        args: HardwareTaskArgs,
        mut item: ForeignItemFn,
    ) -> parse::Result<Self> {
        let span = item.sig.ident.span();
        let valid_signature = util::check_foreign_fn_signature(&item)
            && item.sig.inputs.len() == 1
            && util::type_is_unit(&item.sig.output);

        let name = item.sig.ident.to_string();

        if name == "init" || name == "idle" {
            return Err(parse::Error::new(
                span,
                "tasks cannot be named `init` or `idle`",
            ));
        }

        let probe = item
            .attrs
            .iter()
            .position(|attr| util::attr_eq(attr, "modality_probe"))
            .map(|pos| modality_probe_args(item.attrs.remove(pos).tokens))
            .transpose()?;

        if valid_signature {
            if let Some((context, Ok(rest))) = util::parse_inputs(item.sig.inputs, &name) {
                if rest.is_empty() {
                    let (cfgs, attrs) = util::extract_cfgs(item.attrs);

                    return Ok(HardwareTask {
                        args,
                        cfgs,
                        attrs,
                        context,
                        stmts: Vec::<Stmt>::new(),
                        is_extern: true,
                        probe,
                    });
                }
            }
        }

        Err(parse::Error::new(
            span,
            &format!(
                "this task handler must have type signature `fn({}::Context)`",
                name
            ),
        ))
    }
}
