use std::path::PathBuf;
use trussed::{
    backend::BackendId,
    serde_extensions::*,
    virt::{self, Filesystem, Ram, StoreProvider},
    Error, Platform,
};

use crate::{BackendContext, HmacSha256P256Extension};

#[derive(Default)]
pub struct Dispatcher {
    backend: super::Backend,
}

pub enum BackendIds {
    HmacSha256P256,
}

pub enum ExtensionIds {
    HmacSha256P256,
}

impl ExtensionId<HmacSha256P256Extension> for Dispatcher {
    type Id = ExtensionIds;
    const ID: ExtensionIds = ExtensionIds::HmacSha256P256;
}
impl From<ExtensionIds> for u8 {
    fn from(value: ExtensionIds) -> Self {
        match value {
            ExtensionIds::HmacSha256P256 => 0,
        }
    }
}

impl TryFrom<u8> for ExtensionIds {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::HmacSha256P256),
            _ => Err(Error::FunctionNotSupported),
        }
    }
}

impl ExtensionDispatch for Dispatcher {
    type BackendId = BackendIds;
    type Context = BackendContext;
    type ExtensionId = ExtensionIds;
    fn core_request<P: Platform>(
        &mut self,
        _backend: &Self::BackendId,
        _ctx: &mut trussed::types::Context<Self::Context>,
        _request: &trussed::api::Request,
        _resources: &mut trussed::service::ServiceResources<P>,
    ) -> Result<trussed::Reply, Error> {
        Err(Error::RequestNotAvailable)
    }

    fn extension_request<P: Platform>(
        &mut self,
        _backend: &Self::BackendId,
        extension: &Self::ExtensionId,
        ctx: &mut trussed::types::Context<Self::Context>,
        request: &trussed::api::request::SerdeExtension,
        resources: &mut trussed::service::ServiceResources<P>,
    ) -> Result<trussed::api::reply::SerdeExtension, Error> {
        let _ = &extension;
        let _ = &ctx;
        let _ = &request;
        let _ = &resources;
        // Dereference to avoid compile issue when all features are disabled requiring a default branch
        // See https://github.com/rust-lang/rust/issues/78123#
        match *extension {
            ExtensionIds::HmacSha256P256 => {
                ExtensionImpl::<HmacSha256P256Extension>::extension_request_serialized(
                    &mut self.backend,
                    &mut ctx.core,
                    &mut ctx.backends,
                    request,
                    resources,
                )
            }
        }
    }
}

pub type Client<S, D = Dispatcher> = virt::Client<S, D>;
pub type MultiClient<S, D = Dispatcher> = virt::MultiClient<S, D>;

pub fn with_client<S, R, F>(store: S, client_id: &str, f: F) -> R
where
    F: FnOnce(Client<S>) -> R,
    S: StoreProvider,
{
    virt::with_platform(store, |platform| {
        platform.run_client_with_backends(
            client_id,
            Dispatcher::default(),
            &[
                BackendId::Custom(BackendIds::HmacSha256P256),
                BackendId::Core,
            ],
            f,
        )
    })
}

pub fn with_fs_client<P, R, F>(internal: P, client_id: &str, f: F) -> R
where
    F: FnOnce(Client<Filesystem>) -> R,
    P: Into<PathBuf>,
{
    with_client(Filesystem::new(internal), client_id, f)
}

pub fn with_ram_client<R, F>(client_id: &str, f: F) -> R
where
    F: FnOnce(Client<Ram>) -> R,
{
    with_client(Ram::default(), client_id, f)
}
