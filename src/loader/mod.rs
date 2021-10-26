//! Loaders for definitions.

/// OSRS loaders.
#[cfg(not(feature = "rs3"))]
pub mod osrs;
#[cfg(feature = "rs3")]
/// RS3 loaders.
pub mod rs3;