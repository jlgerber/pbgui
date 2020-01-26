pub use packybara::packrat::{Client, NoTls};
use std::fmt;

/// ConnectParams provide connection parameters for the ClientProxy via
/// ClientProxy::new.
#[derive(PartialEq, Eq, Debug)]
pub struct ConnectParams<'a> {
    host: &'a str,
    user: &'a str,
    password: &'a str,
    dbname: &'a str,
    port: u64,
}

impl<'a> ConnectParams<'a> {
    /// New up a ConnectParams instance.
    ///
    /// # Arguments
    /// * `host` - The name or address of the host
    /// * `user` - The user name
    /// * `password` - The user's password
    /// * `dbname` - The database name
    /// * `port` - The port on which the database is listening
    pub fn new(
        host: &'a str,
        user: &'a str,
        password: &'a str,
        dbname: &'a str,
        port: u64,
    ) -> Self {
        Self {
            host: host,
            user: user,
            password: password,
            dbname: dbname,
            port,
        }
    }
}

impl<'a> std::fmt::Display for ConnectParams<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "host={} user={} dbname={} password={} port={}",
            self.host, self.user, self.dbname, self.password, self.port
        )
    }
}

impl<'a> Default for ConnectParams<'a> {
    fn default() -> ConnectParams<'a> {
        ConnectParams::new("127.0.0.1", "postgres", "example", "packrat", 5432)
    }
}

pub struct ClientProxy {}

impl ClientProxy {
    /// Connect to the database, returning a Client instance if successful.
    ///
    /// # Arguments
    /// * `params` - An instance of ConenctParams
    ///
    /// # Returns
    /// * Ok(Client) if successful
    /// * Err(error) otherwise
    pub fn connect(params: ConnectParams) -> Result<Client, Box<dyn std::error::Error>> {
        let connect_str = params.to_string();
        let client = Client::connect(connect_str.as_str(), NoTls)?;
        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_default_construct() {
        let default = ConnectParams::default();
        assert_eq!(
            "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
            default.to_string().as_str()
        );
    }

    #[test]
    fn can_customize_connect_params() {
        let cp = ConnectParams {
            host: "Fred",
            ..Default::default()
        };
        assert_eq!(
            "host=Fred user=postgres dbname=packrat password=example port=5432",
            cp.to_string().as_str()
        )
    }
}
