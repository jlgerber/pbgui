use crate::utility::qs;
pub use num_enum::{IntoPrimitive, UnsafeFromPrimitive};
use qt_core::QString;
pub use qt_thread_conductor::traits::{FromQString, ToQString};
use qt_widgets::cpp_core::{CppBox, Ref};
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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, UnsafeFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ChangeType {
    Distribution,
    Withs,
    PkgCoord,
    Unknown,
}

impl ToQString for ChangeType {
    fn to_qstring(&self) -> CppBox<QString> {
        match self {
            Self::Distribution => qs("Distribution"),
            Self::Withs => qs("Withs"),
            Self::PkgCoord => qs("PkgCoord"),
            Self::Unknown => qs("Unknown"),
        }
    }
}

impl FromQString for ChangeType {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().to_lowercase().as_str() {
            "distribution" => ChangeType::Distribution,
            "withs" => ChangeType::Withs,
            "PkgCoord" => ChangeType::PkgCoord,
            _ => ChangeType::Unknown,
        }
    }
}
