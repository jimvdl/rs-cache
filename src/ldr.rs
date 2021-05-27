//! Loading definitions for certain games.

/// OSRS specific loaders.
pub mod osrs;
/// RS3 specific loaders.
pub mod rs3;

use crate::{ Cache, Store, Definition };

/// The core of each Loader tasked with loading certain definitions.
pub trait Loader<T: Definition>: Sized {
    fn new<S: Store>(cache: &Cache<S>) -> crate::Result<Self>;
    fn load(&self, id: u16) -> Option<&T>;
}