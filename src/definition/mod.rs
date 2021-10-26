//! Definitions.

/// OSRS definitions.
#[cfg(not(feature = "rs3"))]
pub mod osrs;
/// RS3 definitions.
#[cfg(feature = "rs3")]
pub mod rs3;