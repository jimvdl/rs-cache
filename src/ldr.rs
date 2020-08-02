mod osrs;

use crate::{ Cache, Store };
use super::Definition;

pub trait Loader<T: Definition, S: Store>: Sized {
    fn new(cache: &Cache<S>) -> crate::Result<Self>;
    fn load(&self, id: u16) -> Option<&T>;
}