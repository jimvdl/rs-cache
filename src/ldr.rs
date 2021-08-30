//! Loading definitions for certain games.

/// OSRS specific loaders.
pub mod osrs;
/// RS3 specific loaders.
pub mod rs3;

use crate::{ CacheCore, Definition };

/// The core of each Loader tasked with loading certain definitions.
pub trait Loader: Sized {
    type Definition: Definition;

    fn new<C: CacheCore>(cache: &C) -> crate::Result<Self>;
    fn load(&self, id: u32) -> Option<&Self::Definition>;
}