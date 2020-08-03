use std::collections::HashMap;

use crate::def::osrs::ObjectDefinition;
use crate::{ Cache, Store, Loader, util };

/// Caches all the object definitions that were loaded.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ObjectLoader {
    pub objects: HashMap<u16, ObjectDefinition>
}

impl ObjectLoader {
    pub fn new<S: Store>(cache: &Cache<S>) -> crate::Result<Self> {
        Loader::new(cache)
    }

    pub fn load(&self, id: u16) -> Option<&ObjectDefinition> {
        Loader::load(self, id)
    }
}

impl Loader<ObjectDefinition> for ObjectLoader {
    fn new<S: Store>(cache: &Cache<S>) -> crate::Result<Self> {
        let objects = util::parse_defs(cache, 6)?;

        Ok(Self { objects })
    }

    fn load(&self, id: u16) -> Option<&ObjectDefinition> {
        self.objects.get(&id)
    }
}