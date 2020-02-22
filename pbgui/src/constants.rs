//! Provides constants which model column ids, names, and initial visibility.

/// versionpin table's ID column index
pub const COL_ID: i32 = 0;
/// versionpin table's distribution column index
pub const COL_DISTRIBUTION: i32 = 1;
/// versionpin table's level column index
pub const COL_LEVEL: i32 = 2;
/// versionpin table's role column index
pub const COL_ROLE: i32 = 3;
/// versionpin table's platform column index
pub const COL_PLATFORM: i32 = 4;
/// versionpin table's site column index
pub const COL_SITE: i32 = 5;
/// versionpin table's withs column index
pub const COL_WITHS: i32 = 6;
/// versionpin table's distribution_id (ie the distribution's id in the db) column index
pub const COL_DISTRIBUTION_ID: i32 = 7;
/// versionpin table's pkgcoord_id column index
pub const COL_PKGCOORD_ID: i32 = 8;
/// versionpin headers as an array of tuples where the tuple
/// consists of (column index, name, visibility boolean)
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

/// versionpin change table's changetype column index
pub const COL_PC_CHANGETYPE: i32 = 0;
/// versionpin change table's context column index
pub const COL_PC_CONTEXT: i32 = 1;
/// versionpin change table's old value column index
pub const COL_PC_OLD_VALUE: i32 = 2;
/// versionpin change table's becomes column index
pub const COL_PC_BECOMES: i32 = 3;
/// versionpin change table's new value column index
pub const COL_PC_NEW_VALUE: i32 = 4;
/// versionpin changes table headers as an array of tuples where the tuple
/// consists of (column index, name, visibility boolean)
pub const PC_HEADERS: &[(i32, &'static str, bool)] = &[
    (COL_PC_CHANGETYPE, "ChangeType", false),
    (COL_PC_CONTEXT, "context", false),
    (COL_PC_OLD_VALUE, "old_value", false),
    // only to display -> arrow
    (COL_PC_BECOMES, "becomes", false),
    (COL_PC_NEW_VALUE, "new_value", false),
];
/// change revision table's transaction id column index
pub const COL_REV_TXID: i32 = 0;
/// change revision table's author column index
pub const COL_REV_AUTHOR: i32 = 1;
/// change revision table's datetime column index
pub const COL_REV_DATETIME: i32 = 2;
/// change revision table's comment column index
pub const COL_REV_COMMENT: i32 = 3;
/// change revision table headers as an array of tuples where the tuple
/// consists of (column index, name, visibility boolean)
pub const REV_HEADERS: &[(i32, &'static str, bool)] = &[
    (COL_REV_TXID, "Tx ID", false),
    (COL_REV_AUTHOR, "Author", false),
    (COL_REV_DATETIME, "Datetime", false),
    (COL_REV_COMMENT, "Comment", false),
];

/// change table's id column index
pub const COL_CHNG_ID: i32 = 0;
/// change table's transaction id column index
pub const COL_CHNG_TXID: i32 = 1;
/// change table's action column index
pub const COL_CHNG_ACTION: i32 = 2;
/// change table's level column index
pub const COL_CHNG_LEVEL: i32 = 3;
/// change table's role column index
pub const COL_CHNG_ROLE: i32 = 4;
/// change table's platform column index
pub const COL_CHNG_PLATFORM: i32 = 5;
/// change table's site column index
pub const COL_CHNG_SITE: i32 = 6;
/// change table's pkg column index
pub const COL_CHNG_PKG: i32 = 7;
/// change table's old column index
pub const COL_CHNG_OLD: i32 = 8;
/// change table's new column index
pub const COL_CHNG_NEW: i32 = 9;
/// change table headers as an array of tuples where the tuple
/// consists of (column index, name, visibility boolean)
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
