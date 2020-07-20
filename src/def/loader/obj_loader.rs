use std::collections::HashMap;

use super::super::ObjectDefinition;
use crate::{ Cache, Loader };
use crate::def::loader;

/// Caches all the object definitions that were loaded.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ObjectLoader {
    pub objects: HashMap<u16, ObjectDefinition>
}

impl Loader<ObjectDefinition> for ObjectLoader {
    fn new(cache: &Cache) -> crate::Result<Self> {
        let objects = loader::parse_defs(cache, 6)?;

        Ok(Self { objects })
    }

    fn load(&self, id: u16) -> Option<&ObjectDefinition> {
        self.objects.get(&id)
    }
}