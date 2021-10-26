//! Loaders for definitions.

/// OSRS loaders.
#[cfg(not(feature = "rs3"))]
pub mod osrs;
/// RS3 loaders.
#[cfg(any(feature = "rs3", doc))]
pub mod rs3;
