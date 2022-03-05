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
    let default_account_service = Arc::new(DefaultAccountService);
    let default_account_manager = Arc::new(DefaultAccountManager::new(default_account_service.clone()));

    let server = Server::new(move |req| {
        handler::account::get(req, default_account_manager.clone())
    });

    server.run().await
}
