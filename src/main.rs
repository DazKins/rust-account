mod app;
mod handler;
mod model;
mod service;

mod server;

use std::{io::Error, sync::Arc};
use app::account_manager::default::DefaultAccountManager;
use server::Server;
use service::account::default::DefaultAccountService;

#[async_std::main]
async fn main() -> Result<(), Error> {
    let default_account_service = Arc::new(DefaultAccountService);

    let default_account_manager = Arc::new(
        DefaultAccountManager::new(
            default_account_service.clone(),
            default_account_service.clone()
        )
    );

    let a = default_account_manager.clone();
    let b = default_account_manager.clone(); // Need to work out how to do this better...

    let server = Server::new(
        move |req| handler::account::get(req, a.clone()),
        move |req| handler::account::put(req, b.clone())
    );

    server.run().await
}
