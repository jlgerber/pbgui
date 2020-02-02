//! Write out packages.xml
//!

use quick_xml::{de::from_str, de::DeError, se::to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Show {
    name: String,
    packages: Vec<Package>,
    roles: Vec<Role>,
}

impl Show {
    pub fn new<I: Into<String>>(show: I) -> Self {
        Self {
            name: show.into(),
            packages: Vec::new(),
            roles: Vec::new(),
        }
    }

    pub fn add_package(&mut self, package: Package) {
        self.packages.push(package)
    }

    pub fn add_role(&mut self, role: Role) {
        self.roles.push(role)
    }
}

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// pub struct Packages {
//     packages: Vec<Package>,
//     roles: Vec<Roles>,
// }

// impl  Packages {
//     pub fn new() -> Self {
//         Self {
//             packages: Vec::new()
//         }
//     }
//     pub fn from_packages(packges: Vec<Package>) ->
// }

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Package {
    name: String,
    version: String,
    withs: Vec<With>,
}
impl Package {
    /// New up a package
    pub fn new<I: Into<String>>(name: I, version: I) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            withs: Vec::new(),
        }
    }

    /// Add a with to the list of withs
    pub fn add_with(&mut self, with: With) {
        self.withs.push(with);
    }

    pub fn add_with_owned(mut self, with: With) -> Self {
        self.withs.push(with);
        self
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct With {
    package: String,
}

impl With {
    /// New up a With
    pub fn new<I: Into<String>>(package: I) -> Self {
        Self {
            package: package.into(),
        }
    }
}

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// pub struct Roles {
//     roles: Vec<Role>,
// }

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Role {
    name: String,
    packages: Vec<Package>,
}

impl Role {
    pub fn new<I: Into<String>>(name: I) -> Self {
        Self {
            name: name.into(),
            packages: Vec::new(),
        }
    }
    /// add a package to the packages managed by the role
    pub fn add_package(&mut self, package: Package) {
        self.packages.push(package)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_serialize_package() {
        let mut show = Show::new("FACILITY");
        show.add_package(
            Package::new("maya", "2018.5.1")
                .add_with_owned(With::new("xerces"))
                .add_with_owned(With::new("mayapipeline")),
        );
        show.add_package(
            Package::new("houdini", "17.5.432")
                .add_with_owned(With::new("houd_pipeline"))
                .add_with_owned(With::new("houd_camera")),
        );
        let xml = to_string(&show).unwrap();
        assert_eq!(xml.as_str(), "");
    }
}
