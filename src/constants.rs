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
