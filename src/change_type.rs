use crate::utility::qs;
pub use num_enum::{IntoPrimitive, UnsafeFromPrimitive};
use packybara::types::IdType;
use qt_core::QString;
pub use qt_thread_conductor::traits::{FromQString, ToQString};
use qt_widgets::cpp_core::{CppBox, Ref};
use std::str::FromStr;
use strum_macros::{AsRefStr, EnumDiscriminants, EnumString, IntoStaticStr};
/// The ChangeType that the user has requested.
///
/// This enum derives IntoPrimitive and UnsafeFromPrimitive, which afford
/// the ability to convert from and to i32.
///
/// # Example
/// ```ignore
/// let number: i32 = ChangeType::Distribution.into();
/// unsafe{ let changetype = ChangeType::from_unchecked(1);}
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumDiscriminants)]
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
        // match self {
        //     Self::ChangeDistribution => qs("ChangeDistribution"),
        //     Self::AddDistribution => qs("AddDistribution"),
        //     Self::ChangeWiths => qs("ChangeWiths"),
        //     Self::ChangePkgCoord => qs("ChangePkgCoord"),
        //     Self::Unknown => qs("Unknown"),
        // }
    }
}

impl FromQString for ChangeType {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match ChangeType::from_str(qs.to_std_string().as_str()) {
            Ok(t) => t,
            Err(_) => ChangeType::Unknown,
        }
        // match qs.to_std_string().as_str() {
        //     "ChangeDistribution" => ChangeType::ChangeDistribution,
        //     "AddDistribution" => ChangeType::AddDistribution,
        //     "ChangeWiths" => ChangeType::ChangeWiths,
        //     "ChangePkgCoord" => ChangeType::ChangePkgCoord,
        //     _ => ChangeType::Unknown,
        // }
    }
}
