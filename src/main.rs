mod app;
mod handler;
mod model;
mod service;

mod server;

use std::io::Error;
use app::account::DefaultAccountManager;
use server::Server;
use service::account::DefaultAccountService;

#[async_std::main]
async fn main() -> Result<(), Error> {
    let server = Server::new();

    let default_account_service = DefaultAccountService{};

    let default_account_manager = DefaultAccountManager::new(Box::new(default_account_service));

    server.run().await
}
