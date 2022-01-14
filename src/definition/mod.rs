//! Defines RuneScape data structures.

/// OSRS definitions.
pub mod osrs;
/// RS3 definitions.
#[cfg(feature = "rs3")]
#[cfg_attr(docsrs, doc(cfg(feature = "rs3")))]
pub mod rs3;
