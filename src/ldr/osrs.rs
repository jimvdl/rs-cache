use std::collections::HashMap;

use crate::{ Store, Loader, util, Cache };

use crate::def::osrs::{
    ItemDefinition,
    NpcDefinition,
    ObjectDefinition
};

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemLoader {
    pub items: HashMap<u16, ItemDefinition>
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct NpcLoader {
    pub npcs: HashMap<u16, NpcDefinition>
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ObjectLoader {
    pub objs: HashMap<u16, ObjectDefinition>
}

impl_loader!(ItemLoader, ItemDefinition, items, archive_id: 10);
impl_loader!(NpcLoader, NpcDefinition, npcs, archive_id: 9);
impl_loader!(ObjectLoader, ObjectDefinition, objs, archive_id: 6);