pub use preferences::{traits::*, DDContext, DDPathProvider, DDPreferenceFinder, PreferenceName};
use serde::Deserialize;

/* Example document
---
database:
    host:
    user:
    password:
    dbname:
    port:
*/

#[derive(Debug, PartialEq, Deserialize)]
pub struct PbguiPrefs {
    pub database: PbguiDbPrefs,
}
pub use crate::messaging::client_proxy::ConnectParams;

#[derive(Debug, PartialEq, Deserialize)]
pub struct PbguiDbPrefs {
    pub host: String,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub port: u64,
}

impl std::default::Default for PbguiDbPrefs {
    fn default() -> Self {
        let cp = ConnectParams::default();
        Self {
            host: cp.host.to_string(),
            user: cp.user.to_string(),
            password: cp.password.to_string(),
            dbname: cp.dbname.to_string(),
            port: cp.port,
        }
    }
}
// This gives us two functions -- load and load_file
impl Preference for PbguiPrefs {
    type PreferenceStruct = PbguiPrefs;
    type PreferenceFinder = DDPreferenceFinder<DDPathProvider>;
}

impl PbguiPrefs {
    /// Construct a ConectParams instance from a config. Note that the
    /// lifetime of the ConnectParams is intrinsicly tied to that of
    /// the prefs, as COnnectParams is non-owning.
    pub fn as_connectparams(&self) -> ConnectParams {
        ConnectParams::new(
            self.database.host.as_str(),
            self.database.user.as_str(),
            self.database.password.as_str(),
            self.database.dbname.as_str(),
            self.database.port,
        )
    }
}
