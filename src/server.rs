use std::io::Error;

pub struct Server {
    tide_server: tide::Server<()>
}

impl Server{
    pub fn new(
        get_account_handler: impl tide::Endpoint<()>,
        put_account_handler: impl tide::Endpoint<()>
    ) -> Self {
        let mut server = tide::new();
        server.at("/accounts/:accountId")
            .get(get_account_handler);

        server.at("/accounts/:accountId")
            .put(put_account_handler);

        return Server {
            tide_server: server
        }
    }

    pub async fn run(self) -> Result<(), Error> {
        self.tide_server.listen("0.0.0.0:8080").await
    }
}
