mod app;
mod handler;
mod model;
mod service;

mod server;

use std::{io::Error, sync::Arc};
use app::account::DefaultAccountManager;
use server::Server;
use service::account::DefaultAccountService;

#[async_std::main]
async fn main() -> Result<(), Error> {
    let default_account_service = DefaultAccountService;
    let default_account_manager = DefaultAccountManager::new(Box::new(default_account_service));
    let r = Arc::new(default_account_manager);

    let server = Server::new(move |req| {
        handler::account::get(req, r.clone())
    });

    server.run().await
}
