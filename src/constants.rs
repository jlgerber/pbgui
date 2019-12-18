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

pub const COL_PC_VPINID: i32 = 0;
pub const COL_PC_DISTID: i32 = 1;
pub const COL_PC_PKGCOORDID: i32 = 2;
pub const COL_PC_DISPLAY: i32 = 3;

pub const PC_HEADERS: &[(i32, &'static str, bool)] = &[
    (COL_PC_VPINID, "vpinid", true),
    (COL_PC_DISTID, "distid", true),
    (COL_PC_PKGCOORDID, "pkgcoordid", true),
    (COL_PC_DISPLAY, "display", false),
];

//
pub const COL_REV_TXID: i32 = 0;
pub const COL_REV_AUTHOR: i32 = 1;
pub const COL_REV_DATETIME: i32 = 2;
pub const COL_REV_COMMENT: i32 = 3;

pub const REV_HEADERS: &[(i32, &'static str, bool)] = &[
    (COL_REV_TXID, "tx_id", false),
    (COL_REV_AUTHOR, "author", false),
    (COL_REV_DATETIME, "datetime", false),
    (COL_REV_COMMENT, "comment", false),
];
