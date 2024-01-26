//! The constants that are used in the library and that can be used in a binary crate.

/// JPEG quality default value
pub const QUALITY: i32 = 75;

/// A minimum file size for which to perform file size reduction.
/// - DEFAULT = 0
/// - S = 100 kB
/// - M = 500 kB
/// - L = 1 MB
#[derive(Debug)]
pub enum Size {
    DEFAULT = 0,
    S = 102400,
    M = 512000,
    L = 1048576,
}
