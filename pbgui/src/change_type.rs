//! Provides the Change enum which models proposed versionpin changes, as
//! well as the ChangeType enum, which provides a companion enum that simplifies
//! Change to an enum of descriminants
use crate::utility::qs;
pub use num_enum::{IntoPrimitive, UnsafeFromPrimitive};
use packybara::types::IdType;
use qt_core::QString;
pub use qt_thread_conductor::traits::{FromQString, ToQString};
use qt_widgets::cpp_core::{CppBox, Ref};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use strum_macros::{AsRefStr, EnumDiscriminants, EnumString, IntoStaticStr};
/// The Change that the user has requested.
///
/// ChangeType is derived from Change. ChangeType is a companion enum that
/// provides the descriminant names without their values.
///
/// to_qstring and from_qstring are impl'ed for ChangeType
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumDiscriminants, Clone)]
#[strum_discriminants(name(ChangeType))]
#[strum_discriminants(derive(EnumString, IntoStaticStr, AsRefStr, PartialOrd, Ord))]
pub enum Change {
    ChangeDistribution {
        vpin_id: IdType,
        new_dist_id: IdType,
    },
    AddDistribution {
        distribution: String,
        level: String,
        role: String,
        platform: String,
        site: String,
    },
    ChangePkgCoord {
        vpin_id: IdType,
        version: String,
        level: String,
        role: String,
        platform: String,
        site: String,
    },
    ChangeWiths {
        vpin_id: IdType,
        withs: Vec<String>,
    },
    Unknown,
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl Change {
    /// Retrieve an identifing unique id for the Change instance
    ///
    /// # Argugments
    ///
    /// * None
    ///
    /// # Returns
    ///
    /// * u64 guaranteed to be unique to the change variant inputs
    pub fn id(&self) -> u64 {
        match self {
            Change::ChangeDistribution { vpin_id, .. } => *vpin_id as u64,
            Change::AddDistribution {
                distribution,
                level,
                role,
                platform,
                site,
            } => {
                let val = format!("{}-{}-{}-{}-{}", distribution, level, role, platform, site);
                calculate_hash(&val)
            }
            Change::ChangePkgCoord { vpin_id, .. } => *vpin_id as u64,
            Change::ChangeWiths { vpin_id, .. } => *vpin_id as u64,
            Change::Unknown => panic!("unable to retrieve id for unknown type"),
        }
    }

    /// Determine whether a change is of a particular change type
    ///
    /// # Arguments
    ///
    /// * `ctype` - a ChangeType variant
    ///
    /// # Returns
    ///
    /// * bool, indicating whether the Change instance is of the provided ChangeType instance
    pub fn is_a(&self, ctype: ChangeType) -> bool {
        match self {
            Change::ChangeDistribution { .. } => ctype == ChangeType::ChangeDistribution,
            Change::AddDistribution { .. } => ctype == ChangeType::AddDistribution,
            Change::ChangePkgCoord { .. } => ctype == ChangeType::ChangePkgCoord,
            Change::ChangeWiths { .. } => ctype == ChangeType::ChangeWiths,
            Change::Unknown => ctype == ChangeType::Unknown,
        }
    }
}

impl ToQString for ChangeType {
    fn to_qstring(&self) -> CppBox<QString> {
        let s: &'static str = self.into();
        qs(&s)
    }
}

impl FromQString for ChangeType {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match ChangeType::from_str(qs.to_std_string().as_str()) {
            Ok(t) => t,
            Err(_) => ChangeType::Unknown,
        }
    }
}
