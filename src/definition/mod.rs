//! Definitions.

/// OSRS definitions.
#[cfg(not(feature = "rs3"))]
pub mod osrs;
/// RS3 definitions.
#[cfg(any(feature = "rs3", doc))]
pub mod rs3;