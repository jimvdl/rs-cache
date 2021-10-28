//! Defines RuneScape data structures.

/// OSRS definitions.
#[cfg(feature = "osrs")]
pub mod osrs;
/// RS3 definitions.
#[cfg(any(feature = "rs3", doc))]
pub mod rs3;
