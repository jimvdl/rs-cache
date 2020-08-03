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

impl_loader!(ItemLoader, ItemDefinition, items, 10);
impl_loader!(NpcLoader, NpcDefinition, npcs, 9);
impl_loader!(ObjectLoader, ObjectDefinition, objs, 6);