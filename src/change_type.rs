use crate::utility::qs;
pub use num_enum::{IntoPrimitive, UnsafeFromPrimitive};
use packybara::types::IdType;
use qt_core::QString;
pub use qt_thread_conductor::traits::{FromQString, ToQString};
use qt_widgets::cpp_core::{CppBox, Ref};
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
        package: String,
        version: String,
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
