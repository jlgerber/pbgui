pub use packybara::packrat::PackratDb;
use packybara::packrat::{Client, NoTls};
pub struct ClientProxy {}

impl ClientProxy {
    pub fn connect() -> Result<Client, Box<dyn std::error::Error>> {
        let client = Client::connect(
            "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
            NoTls,
        )?;
        Ok(client)
    }
}
