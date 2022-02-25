use std::io::Error;

use crate::handler;

pub struct Server {
    tide_server: tide::Server<()>
}

impl Server{
    pub fn new() -> Self {
        let mut server = tide::new();
        server.at("/accounts/:accountId")
            .get(|req: tide::Request<()>| handler::account::get(req));

        return Server {
            tide_server: server
        }
    }

    pub async fn run(self) -> Result<(), Error> {
        self.tide_server.listen("0.0.0.0:8080").await
    }
}
