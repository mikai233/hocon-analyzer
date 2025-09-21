use crate::world::World;
use async_lsp::{LanguageServer, ResponseError};
use futures::future::BoxFuture;
use lsp_types::request::Request;
use std::ops::ControlFlow;


type ResponseFuture<R, E> = BoxFuture<'static, Result<<R as Request>::Result, E>>;

impl LanguageServer for World {
    type Error = ResponseError;
    type NotifyResult = ControlFlow<async_lsp::Result<()>>;

    fn initialize(
        &mut self,
        params: <lsp_types::request::Initialize as Request>::Params,
    ) -> ResponseFuture<lsp_types::request::Initialize, Self::Error> {
        unimplemented!()
    }

    fn hover(
        &mut self,
        params: <lsp_types::lsp_request!("textDocument/hover") as Request>::Params,
    ) -> ResponseFuture<lsp_types::lsp_request!("textDocument/hover"), Self::Error> {
        unimplemented!()
    }
}
