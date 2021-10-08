use proc_macro2::TokenStream as TokenStream2;
use syn::{
    parenthesized,
    parse::{self, ParseStream, Parser},
    Ident, LitInt, Token,
};

use crate::modality_probe::ast::ModalityProbeArgs;

pub fn modality_probe_args(tokens: TokenStream2) -> parse::Result<ModalityProbeArgs> {
    (|input: ParseStream<'_>| -> parse::Result<ModalityProbeArgs> {
        let mut name = None;
        let mut size = None;
        let mut local_name = None;

        let content;
        parenthesized!(content in input);
        loop {
            if content.is_empty() {
                break;
            }

            // #ident = ..
            let ident: Ident = content.parse()?;
            let _: Token![=] = content.parse()?;

            let ident_s = ident.to_string();
            match &*ident_s {
                "name" => {
                    if name.is_some() {
                        return Err(parse::Error::new(
                            ident.span(),
                            "argument appears more than once",
                        ));
                    }

                    // #ident
                    let ident = content.parse()?;

                    name = Some(ident);
                }

                "size" => {
                    if size.is_some() {
                        return Err(parse::Error::new(
                            ident.span(),
                            "argument appears more than once",
                        ));
                    }

                    // #lit
                    let lit: LitInt = content.parse()?;

                    if !lit.suffix().is_empty() {
                        return Err(parse::Error::new(
                            lit.span(),
                            "this literal must be unsuffixed",
                        ));
                    }

                    let value = lit.base10_parse::<u32>().ok();
                    if value.is_none() || value == Some(0) {
                        return Err(parse::Error::new(
                            lit.span(),
                            "this literal must be in the range 1...u32::MAX",
                        ));
                    }

                    size = Some(value.unwrap());
                }

                "local_name" => {
                    if local_name.is_some() {
                        return Err(parse::Error::new(
                            ident.span(),
                            "argument appears more than once",
                        ));
                    }

                    // #ident
                    let ident = content.parse()?;

                    local_name = Some(ident);
                }

                _ => {
                    return Err(parse::Error::new(ident.span(), "unexpected argument"));
                }
            }

            if content.is_empty() {
                break;
            }

            // ,
            let _: Token![,] = content.parse()?;
        }

        let name = if let Some(n) = name {
            n
        } else {
            return Err(parse::Error::new(content.span(), "`name = ...` is missing"));
        };

        let size = if let Some(s) = size {
            s
        } else {
            return Err(parse::Error::new(content.span(), "`size = ...` is missing"));
        };

        Ok(ModalityProbeArgs::new(name, size, local_name))
    })
    .parse2(tokens)
}
