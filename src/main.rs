mod server;
mod handler;
mod model;

use std::io::Error;
use server::Server;

#[async_std::main]
async fn main() -> Result<(), Error> {
    let server = Server::new();
    server.run().await
}
