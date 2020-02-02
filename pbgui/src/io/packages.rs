//! Generate packages.xml
//!

//use quick_xml::{de::from_str, de::DeError, se::to_string};
//use serde::{Deserialize, Serialize};
use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;
use std::mem;

/// Determine if an XML element is closed (eg <foo />)
pub trait IsClosed {
    fn is_closed(&self) -> bool;
}

impl IsClosed for XMLElement {
    fn is_closed(&self) -> bool {
        self.contents.is_none() && self.text.is_none()
    }
}

/// Top level element
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

/// Rust representation of the xml element with the `package` tag. Unfortunately,
/// the tag would more accurately be considered a distribution than a package.
/// However, we stick with Package to be consistent with the output xml file.
#[xml_element("package")]
pub struct Package {
    #[sxs_type_attr]
    name: String,
    #[sxs_type_attr]
    version: String,
    #[sxs_type_attr]
    os: Option<String>,
    #[sxs_type_attr]
    site: Option<String>,
    #[sxs_type_multi_element(rename = "withs")]
    withs: Vec<With>,
}

impl Package {
    /// New up a Package isntance. (Note that we use the terminology of the packages.xml target file.
    /// However, the Package struct should probably be named Distribution)
    pub fn new<I: Into<String>>(name: I, version: I, os: Option<I>, site: Option<I>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            withs: Vec::new(),
            os: os.map(|x| x.into()),
            site: site.map(|x| x.into()),
        }
    }

    /// New up a Package instance given the name and version of a distribution
    ///
    /// # Arguments
    ///
    /// * `name` - The package name
    /// * `version` - The package version
    ///
    /// # Returns
    /// * Package instance
    pub fn from_name_and_version<I: Into<String>>(name: I, version: I) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            withs: Vec::new(),
            os: None,
            site: None,
        }
    }

    /// Construct a Package from a distribution, returning an Option
    ///
    /// # Arguments
    ///
    /// * `dist` - The distribution, eg foo-1.2.3
    ///
    /// # Returns
    /// * Some(package) if successful
    /// * None otherwise
    pub fn from_dist(dist: &str) -> Option<Self> {
        if let &[name, version] = &*dist.split("-").collect::<Vec<_>>() {
            Some(Self {
                name: name.into(),
                version: version.into(),
                withs: Vec::new(),
                os: None,
                site: None,
            })
        } else {
            None
        }
    }

    /// Add a `with`ß to the list of withs
    ///
    /// # Arguments
    ///
    /// * `with` - An Option wrapped with package name
    pub fn add_with(&mut self, with: With) {
        self.withs.push(with);
    }

    /// Add a with package, returning Self. Used as part of a builder pattern.
    ///
    /// # Arguments
    ///
    /// * `with` - An Option wrapped with package name
    ///
    /// # Returns
    /// * Self
    pub fn add_with_owned(mut self, with: With) -> Self {
        self.withs.push(with);
        self
    }

    /// Set the os field.
    ///
    /// # Arguments
    ///
    /// * `os` - An Option wrapped platform name
    pub fn set_os<I: Into<String>>(&mut self, os: Option<I>) {
        self.os = os.map(|x| x.into());
    }
    /// Set the os field and return Self. Used as part of a builder pattern
    ///
    /// # Arguments
    ///
    /// * `os` - An Option wrapped platform name
    ///
    /// # Returns
    /// * Self
    pub fn set_os_owned<I: Into<String>>(mut self, os: Option<I>) -> Self {
        self.os = os.map(|x| x.into());
        self
    }

    /// Set the site.
    ///
    /// # Arguments
    ///
    /// * `site` - An Option wrapped site name
    ///
    /// Returns
    /// None
    pub fn set_site<I: Into<String>>(&mut self, site: Option<I>) {
        self.site = site.map(|x| x.into());
    }

    /// Set the site and return Self. Used as part of a builder pattern
    ///
    /// # Arguments
    ///
    /// * `site` - An Option wrapped site name
    ///
    /// # Returns
    ///
    /// * Self
    pub fn set_site_owned<I: Into<String>>(mut self, site: Option<I>) -> Self {
        self.site = site.map(|x| x.into());
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

/// A set of role
#[xml_element("role")]
pub struct Roles {
    #[sxs_type_multi_element(rename = "role")]
    role: Vec<Role>,
}
impl Roles {
    pub fn new() -> Roles {
        Roles { role: Vec::new() }
    }
    /// Add a role into the list of child roles
    ///
    /// # Arguments
    ///
    /// * `role` - An instance of type Role
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

    /// add a package to the packages managed by the role, returning Self. Used
    /// as part of a builder pattern.
    ///
    /// # Parameters
    ///
    /// * `package` - An instance of Package
    pub fn add_package_owned(mut self, package: Package) -> Self {
        self.packages.push(package);
        self
    }
}

/// Converter which generates an XMLElement Tree from an entry which impls  Into<XMLElement>
///
/// # Example
///
/// ```rust
/// use simple_xml_serialize::XMLElement;
///
/// fn main() {
///      let mut show = Show::new("FACILITY");
///
///      show.add_package(
///      Package::new("maya", "2018.5.1", None, None)
///         .set_os_owned(Some("cent7_64"))
///         .add_with_owned(With::new("xerces"))
///         .add_with_owned(With::new("mayapipeline"))
///      );
///
///     let xml = ToXml::new().to_xml(show);
/// }
pub struct ToXml {
    pub prune_closed: bool,
}

impl ToXml {
    pub fn new() -> Self {
        Self { prune_closed: true }
    }
    /// Convert entry into xml element provided it implw Into<XMLElemet>
    pub fn to_xml(&self, entry: impl Into<XMLElement>) -> XMLElement {
        let mut xml = entry.into();
        if self.prune_closed {
            Self::prune_closed_contents(&mut xml);
        }
        xml
    }

    /// Given an element, generate a pretty string rep
    pub fn to_pretty_string(elem: &XMLElement) -> String {
        elem.to_string_pretty("\n", "  ")
    }

    /// Given an element, generate a string representation
    pub fn to_string(elem: &XMLElement) -> String {
        elem.to_string()
    }

    // given a node, prune its closed contents
    fn prune_closed_contents(elem: &mut XMLElement) {
        if elem.contents.is_some() {
            let contents = elem.contents.take().unwrap();
            let contents = contents
                .into_iter()
                .filter(|x| !x.is_closed())
                .collect::<Vec<_>>();
            let mut contents = if contents.len() > 0 {
                Some(contents)
            } else {
                None
            };
            mem::swap(&mut elem.contents, &mut contents);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_serialize_show_no_role() {
        let mut show = Show::new("FACILITY");
        show.add_package(
            Package::new("maya", "2018.5.1", None, None)
                .set_os_owned(Some("cent7_64"))
                .add_with_owned(With::new("xerces"))
                .add_with_owned(With::new("mayapipeline")),
        );
        show.add_package(
            Package::new("houdini", "17.5.432", None, None)
                .add_with_owned(With::new("houd_pipeline"))
                .add_with_owned(With::new("houd_camera")),
        );
        let converter = ToXml::new();
        let xml = converter.to_xml(show);
        assert_eq!(xml.to_string_pretty("\n", "  ").as_str(),
        "<show name=\"FACILITY\">\n  <packages>\n    <package name=\"maya\" version=\"2018.5.1\" os=\"cent7_64\">\n      <withs package=\"xerces\"/>\n      <withs package=\"mayapipeline\"/>\n    </package>\n    <package name=\"houdini\" version=\"17.5.432\">\n      <withs package=\"houd_pipeline\"/>\n      <withs package=\"houd_camera\"/>\n    </package>\n  </packages>\n</show>"
        );
    }
    #[test]
    fn can_serialize_show_no_withs() {
        let mut show = Show::new("FACILITY");
        show.add_package(Package::new("maya", "2018.5.1", None, None));
        show.add_package(Package::new("houdini", "17.5.432", None, None));
        let converter = ToXml::new();
        let xml = converter.to_xml(show);
        assert_eq!(xml.to_string_pretty("\n", "  ").as_str(), 
        "<show name=\"FACILITY\">\n  <packages name=\"maya\" version=\"2018.5.1\"/>\n  <packages name=\"houdini\" version=\"17.5.432\"/>\n</show>"
        );
    }

    #[test]
    fn can_serialize_show_with_roles() {
        let mut show = Show::new("FACILITY");
        show.add_package(
            Package::new("maya", "2018.5.1", None, None)
                .add_with_owned(With::new("xerces"))
                .add_with_owned(With::new("mayapipeline")),
        );
        show.add_package(
            Package::new("houdini", "17.5.432", None, None)
                .add_with_owned(With::new("houd_pipeline"))
                .add_with_owned(With::new("houd_camera")),
        );

        show.add_role(
            Role::new("model")
                .add_package_owned(
                    Package::new("maya", "2020.1.0", None, None)
                        .add_with_owned(With::new("xerces"))
                        .add_with_owned(With::new("mayapipeline"))
                        .add_with_owned(With::new("modelpipeline")),
                )
                .add_package_owned(Package::new("zbrush", "14", None, None))
                .add_package_owned(
                    Package::new("atomic", "1.2.3", None, None)
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
