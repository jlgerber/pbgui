//! This module deals with column ids, names, and initial visibility.
//!

/// COL is for the versionpin table
pub const COL_ID: i32 = 0;
pub const COL_DISTRIBUTION: i32 = 1;
pub const COL_LEVEL: i32 = 2;
pub const COL_ROLE: i32 = 3;
pub const COL_PLATFORM: i32 = 4;
pub const COL_SITE: i32 = 5;
pub const COL_WITHS: i32 = 6;
pub const COL_DISTRIBUTION_ID: i32 = 7;
pub const COL_PKGCOORD_ID: i32 = 8;
pub const HEADERS: &[(i32, &'static str, bool)] = &[
    (COL_ID, "Id", false),
    (COL_DISTRIBUTION, "Distribution", false),
    (COL_LEVEL, "Level", false),
    (COL_ROLE, "Role", false),
    (COL_PLATFORM, "Platform", false),
    (COL_SITE, "Site", false),
    (COL_WITHS, "WIths", false),
    (COL_DISTRIBUTION_ID, "Dist Id", true),
    (COL_PKGCOORD_ID, "PkdCoord Id", true),
];

/// COL_PC -> Pin COnfiguraiton Table
pub const COL_PC_CHANGETYPE: i32 = 0;
pub const COL_PC_CONTEXT: i32 = 1;
pub const COL_PC_OLD_VALUE: i32 = 2;
pub const COL_PC_BECOMES: i32 = 3;
pub const COL_PC_NEW_VALUE: i32 = 4;

pub const PC_HEADERS: &[(i32, &'static str, bool)] = &[
    (COL_PC_CHANGETYPE, "ChangeType", false),
    (COL_PC_CONTEXT, "context", false),
    (COL_PC_OLD_VALUE, "old_value", false),
    // only to display -> arrow
    (COL_PC_BECOMES, "becomes", false),
    (COL_PC_NEW_VALUE, "new_value", false),
];
// COL_REV Revision Table
pub const COL_REV_TXID: i32 = 0;
pub const COL_REV_AUTHOR: i32 = 1;
pub const COL_REV_DATETIME: i32 = 2;
pub const COL_REV_COMMENT: i32 = 3;

pub const REV_HEADERS: &[(i32, &'static str, bool)] = &[
    (COL_REV_TXID, "Tx ID", false),
    (COL_REV_AUTHOR, "Author", false),
    (COL_REV_DATETIME, "Datetime", false),
    (COL_REV_COMMENT, "Comment", false),
];

/// COL_CHNG Revision Change table
pub const COL_CHNG_ID: i32 = 0;
pub const COL_CHNG_TXID: i32 = 1;
pub const COL_CHNG_ACTION: i32 = 2;
pub const COL_CHNG_LEVEL: i32 = 3;
pub const COL_CHNG_ROLE: i32 = 4;
pub const COL_CHNG_PLATFORM: i32 = 5;
pub const COL_CHNG_SITE: i32 = 6;
pub const COL_CHNG_PKG: i32 = 7;
pub const COL_CHNG_OLD: i32 = 8;
pub const COL_CHNG_NEW: i32 = 9;

pub const CHNG_HEADERS: &[(i32, &'static str, bool)] = &[
    (COL_CHNG_ID, "Id", true),
    (COL_CHNG_TXID, "TX Id", true),
    (COL_CHNG_ACTION, "Action", false),
    (COL_CHNG_LEVEL, "Level", false),
    (COL_CHNG_ROLE, "Role", false),
    (COL_CHNG_PLATFORM, "Platform", false),
    (COL_CHNG_SITE, "Site", false),
    (COL_CHNG_PKG, "Package", false),
    (COL_CHNG_OLD, "Old", false),
    (COL_CHNG_NEW, "New", false),
];
