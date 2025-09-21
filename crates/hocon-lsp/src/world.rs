use ahash::HashMap;
use async_lsp::{ClientSocket, router::Router};
use derive_more::{Deref, DerefMut};
use indexmap::IndexMap;
use lsp_types::Url;

#[derive(Debug)]
pub(crate) struct World {
    pub(crate) client: ClientSocket,
}

impl World {
    pub(crate) fn new_router(client: ClientSocket) -> Router<Self> {
        let world = World { client };
        Router::from_language_server(world)
    }
}

#[derive(Debug, Deref, DerefMut)]
pub(crate) struct Workspaces(IndexMap<Url, Workspace>);

#[derive(Debug)]
pub(crate) struct Workspace {
    pub(crate) root: Url,
    pub(crate) documents: HashMap<Url, Document>,
}

#[derive(Debug)]
pub(crate) struct Document {}
