pub mod osrs;

use crate::{ Cache, Store };
use super::Definition;

pub trait Loader<T: Definition>: Sized {
    fn new<S: Store>(cache: &Cache<S>) -> crate::Result<Self>;
    fn load(&self, id: u16) -> Option<&T>;
}