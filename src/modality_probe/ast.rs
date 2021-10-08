use crate::ast::{Local, LocalResource};
use proc_macro2::Span;
use quote::quote;
use syn::{Expr, Ident, LitStr, Type};

#[derive(Debug)]
#[non_exhaustive]
pub struct ModalityProbeArgs {
    /// Probe name
    pub name: Ident,
    /// Description
    pub description: Option<LitStr>,
    /// Storage buffer size in bytes
    pub size: u32,
    /// Name of the local variable, defaults to 'probe'
    pub local_name: Ident,
    /// Name of the probe-backing storage buffer
    pub storage_name: Ident,
    /// Probe-backing storage resource
    pub storage_resource: Local,
    /// Probe resource
    pub probe_resource: LocalResource,
}

impl ModalityProbeArgs {
    pub fn new(name: Ident, size: u32, local_name: Option<Ident>) -> Self {
        let gensize = size as usize;
        ModalityProbeArgs {
            name,
            description: None,
            size,
            local_name: local_name.unwrap_or_else(|| Ident::new("probe", Span::call_site())),
            storage_name: Ident::new("probe_storage", Span::call_site()),
            storage_resource: Local {
                attrs: Default::default(),
                cfgs: Default::default(),
                ty: Box::new(Type::Verbatim(quote!(
                    [core::mem::MaybeUninit<u8>; #gensize]
                ))),
                expr: Box::new(Expr::Verbatim(quote!(
                    [core::mem::MaybeUninit::new(0); #gensize]
                ))),
            },
            probe_resource: LocalResource {
                attrs: Default::default(),
                cfgs: Default::default(),
                ty: Box::new(Type::Verbatim(quote!(
                    modality_probe_sys::ModalityProbe<'static>
                ))),
            },
        }
    }

    pub fn storage_declared_local_resource(&self) -> (&Ident, &Local) {
        (&self.storage_name, &self.storage_resource)
    }

    pub fn probe_local_resource(&self) -> (&Ident, &LocalResource) {
        (&self.local_name, &self.probe_resource)
    }

    pub fn resource_idents(&self) -> (&Ident, &Ident) {
        (&self.storage_name, &self.local_name)
    }
}
