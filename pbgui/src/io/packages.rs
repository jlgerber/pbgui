//! Write out packages.xml
//!

//use quick_xml::{de::from_str, de::DeError, se::to_string};
//use serde::{Deserialize, Serialize};
use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;

//#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[xml_element("show")]
pub struct Show {
    #[sxs_type_attr]
    name: String,
    #[sxs_type_element(rename = "packages")]
    packages: Packages,
    #[sxs_type_element(rename = "roles")]
    roles: Roles,
}

impl Show {
    pub fn new<I: Into<String>>(show: I) -> Self {
        Self {
            name: show.into(),
            packages: Packages::new(),
            roles: Roles::new(),
        }
    }

    pub fn add_package(&mut self, package: Package) {
        self.packages.push(package)
    }

    pub fn add_role(&mut self, role: Role) {
        self.roles.push(role)
    }
}

#[xml_element("package")]
pub struct Packages {
    #[sxs_type_multi_element(rename = "package")]
    package: Vec<Package>,
}

impl Packages {
    pub fn new() -> Self {
        Self {
            package: Vec::new(),
        }
    }

    pub fn push(&mut self, package: Package) {
        self.package.push(package)
    }
}

//#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[xml_element("package")]
pub struct Package {
    #[sxs_type_attr]
    name: String,
    #[sxs_type_attr]
    version: String,
    #[sxs_type_multi_element(rename = "withs")]
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
//#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[xml_element("with")]
pub struct With {
    #[sxs_type_attr]
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

#[xml_element("role")]
pub struct Roles {
    #[sxs_type_multi_element(rename = "role")]
    role: Vec<Role>,
}
impl Roles {
    pub fn new() -> Roles {
        Roles { role: Vec::new() }
    }

    pub fn push(&mut self, role: Role) {
        self.role.push(role)
    }
}
//#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[xml_element("role")]
pub struct Role {
    #[sxs_type_attr]
    name: String,
    #[sxs_type_element(rename = "packages")]
    packages: Packages,
}

impl Role {
    pub fn new<I: Into<String>>(name: I) -> Self {
        Self {
            name: name.into(),
            packages: Packages::new(),
        }
    }
    pub fn add_package(&mut self, package: Package) {
        self.packages.push(package);
    }

    /// add a package to the packages managed by the role
    pub fn add_package_owned(mut self, package: Package) -> Self {
        self.packages.push(package);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_serialize_show_no_role() {
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
        let xml = XMLElement::from(show);
        assert_eq!(xml.to_string_pretty("\n", "  ").as_str(), 
        "<show name=\"FACILITY\">\n  <packages name=\"maya\" version=\"2018.5.1\">\n     <withs package=\"xerces\"/>\n    <withs package=\"mayapipeline\"/>\n  </packages>\n  <packages name=\"houdini\" version=\"17.5.432\">\n    <withs package=\"houd_pipeline\"/>\n    <withs package=\"houd_camera\"/>\n  </packages>\n</show>"
        );
    }
    #[test]
    fn can_serialize_show_no_withs() {
        let mut show = Show::new("FACILITY");
        show.add_package(Package::new("maya", "2018.5.1"));
        show.add_package(Package::new("houdini", "17.5.432"));
        let xml = XMLElement::from(show);
        assert_eq!(xml.to_string_pretty("\n", "  ").as_str(), 
        "<show name=\"FACILITY\">\n  <packages name=\"maya\" version=\"2018.5.1\"/>\n  <packages name=\"houdini\" version=\"17.5.432\"/>\n</show>"
        );
    }

    #[test]
    fn can_serialize_show_with_roles() {
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

        show.add_role(
            Role::new("model")
                .add_package_owned(
                    Package::new("maya", "2020.1.0")
                        .add_with_owned(With::new("xerces"))
                        .add_with_owned(With::new("mayapipeline"))
                        .add_with_owned(With::new("modelpipeline")),
                )
                .add_package_owned(Package::new("zbrush", "14"))
                .add_package_owned(
                    Package::new("atomic", "1.2.3")
                        .add_with_owned(With::new("vray"))
                        .add_with_owned(With::new("vray_for_maya")),
                ),
        );

        let xml = XMLElement::from(show);
        assert_eq!(
            xml.to_string_pretty("\n", "  ").as_str(), 
            "<show name=\"FACILITY\">\n  <packages>\n    <package name=\"maya\" version=\"2018.5.1\">\n      <withs package=\"xerces\"/>\n      <withs package=\"mayapipeline\"/>\n    </package>\n    <package name=\"houdini\" version=\"17.5.432\">\n      <withs package=\"houd_pipeline\"/>\n      <withs package=\"houd_camera\"/>\n    </package>\n  </packages>\n  <roles>\n    <role name=\"model\">\n      <packages>\n        <package name=\"maya\" version=\"2020.1.0\">\n          <withs package=\"xerces\"/>\n          <withs package=\"mayapipeline\"/>\n          <withs package=\"modelpipeline\"/>\n        </package>\n        <package name=\"zbrush\" version=\"14\"/>\n        <package name=\"atomic\" version=\"1.2.3\">\n          <withs package=\"vray\"/>\n          <withs package=\"vray_for_maya\"/>\n        </package>\n      </packages>\n    </role>\n  </roles>\n</show>"
        );
    }
}
