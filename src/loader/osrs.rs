use std::collections::{
    hash_map::{self, Entry},
    HashMap,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    definition::osrs::{
        Definition, FetchDefinition, ItemDefinition, LocationDefinition, MapDefinition,
        NpcDefinition, ObjectDefinition,
    },
    Cache,
};

/// Loads all item definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ItemLoader(HashMap<u16, ItemDefinition>);

impl_osrs_loader!(ItemLoader, ItemDefinition, index_id: 2, archive_id: 10);

/// Loads all npc definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NpcLoader(HashMap<u16, NpcDefinition>);

impl_osrs_loader!(NpcLoader, NpcDefinition, index_id: 2, archive_id: 9);

/// Loads all object definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ObjectLoader(HashMap<u16, ObjectDefinition>);

impl_osrs_loader!(ObjectLoader, ObjectDefinition, index_id: 2, archive_id: 6);

/// Loads maps definitions lazily from the current cache.
#[derive(Debug)]
pub struct MapLoader<'cache> {
    cache: &'cache Cache,
    maps: HashMap<u16, MapDefinition>,
}

impl<'cache> MapLoader<'cache> {
    /// Make a new `MapLoader`.
    ///
    /// This takes a `Cache` by references with a `'cache` lifetime.
    /// All the map definitions are loaded lazily where the `&'cache Cache` is used
    /// to cache them internally on load.
    pub fn new(cache: &'cache Cache) -> Self {
        Self {
            cache,
            maps: HashMap::new(),
        }
    }

    pub fn load(&mut self, id: u16) -> crate::Result<&MapDefinition> {
        if let Entry::Vacant(entry) = self.maps.entry(id) {
            let x = id >> 8;
            let y = id & 0xFF;

            let map_archive = self.cache.archive_by_name(5, format!("m{}_{}", x, y))?;
            let buffer = self.cache.read_archive(map_archive)?.decode()?;

            entry.insert(MapDefinition::new(id, &buffer)?);
        }

        Ok(&self.maps[&id])
    }
}

/// Loads location definitions lazily from the current cache.
#[derive(Debug)]
pub struct LocationLoader<'cache> {
    cache: &'cache Cache,
    locations: HashMap<u16, LocationDefinition>,
}

impl<'cache> LocationLoader<'cache> {
    /// Make a new `LocationLoader`.
    ///
    /// This takes a `Cache` by references with a `'cache` lifetime.
    /// All the location definitions are loaded lazily where the `&'cache Cache` is used
    /// to cache them internally on load.
    pub fn new(cache: &'cache Cache) -> Self {
        Self {
            cache,
            locations: HashMap::new(),
        }
    }

    /// Loads the location data for a particular region.
    ///
    /// Also takes a `keys: [u32; 4]` because the location archive is encrypted
    /// with XTEA. The buffer is automatically decoded with the given keys.
    pub fn load(&mut self, id: u16, keys: &[u32; 4]) -> crate::Result<&LocationDefinition> {
        if let Entry::Vacant(entry) = self.locations.entry(id) {
            let x = id >> 8;
            let y = id & 0xFF;

            let loc_archive = self.cache.archive_by_name(5, format!("l{}_{}", x, y))?;
            let buffer = self
                .cache
                .read_archive(loc_archive)?
                .with_xtea_keys(*keys)
                .decode()?;

            entry.insert(LocationDefinition::new(id, &buffer)?);
        }

        Ok(&self.locations[&id])
    }
}

#[cfg(test)]
mod items {
    use super::ItemLoader;
    use crate::test_util;

    fn item_loader() -> crate::Result<ItemLoader> {
        ItemLoader::new(&test_util::osrs_cache()?)
    }

    #[test]
    fn blue_partyhat() -> crate::Result<()> {
        let item_loader = item_loader()?;
        let item = item_loader.load(1042).unwrap();

        assert_eq!(item.name, "Blue partyhat");
        assert!(!item.stackable);
        assert!(!item.members_only);

        Ok(())
    }

    #[test]
    fn magic_logs() -> crate::Result<()> {
        let item_loader = item_loader()?;
        let item = item_loader.load(1513).unwrap();

        assert_eq!(item.name, "Magic logs");
        assert!(!item.stackable);
        assert!(item.members_only);

        Ok(())
    }

    #[test]
    fn noted() -> crate::Result<()> {
        let item_loader = item_loader()?;
        let item = item_loader.load(1512).unwrap();

        assert!(item.stackable);
        assert!(!item.members_only);

        Ok(())
    }

    #[test]
    fn non_existent() -> crate::Result<()> {
        let item_loader = item_loader()?;

        assert!(item_loader.load(65_535).is_none());

        Ok(())
    }
}

#[cfg(test)]
mod npcs {
    use super::NpcLoader;
    use crate::test_util;

    fn npc_loader() -> crate::Result<NpcLoader> {
        NpcLoader::new(&test_util::osrs_cache()?)
    }

    #[test]
    fn woodsman_tutor() -> crate::Result<()> {
        let npc_loader = npc_loader()?;
        let npc = npc_loader.load(3226).unwrap();
        
        assert_eq!(npc.name, "Woodsman tutor");
        assert!(npc.interactable);
        
        Ok(())
    }
    
    #[test]
    fn last_valid_npc() -> crate::Result<()> {
        let npc_loader = npc_loader()?;
        let npc = npc_loader.load(8691).unwrap();
        
        assert_eq!(npc.name, "Mosol Rei");
        assert!(npc.interactable);
        
        Ok(())
    }
    
    #[test]
    fn non_existent() -> crate::Result<()> {
        let npc_loader = npc_loader()?;
        
        assert!(npc_loader.load(65_535).is_none());
        
        Ok(())
    }
}

#[cfg(test)]
mod objects {
    use super::ObjectLoader;
    use crate::test_util;

    fn obj_loader() -> crate::Result<ObjectLoader> {
        ObjectLoader::new(&test_util::osrs_cache()?)
    }

    #[test]
    fn law_rift() -> crate::Result<()> {
        let obj_loader = obj_loader()?;
        let obj = obj_loader.load(25034).unwrap();
        
        assert_eq!(obj.name, "Law rift");
        assert_eq!(obj.animation_id, 2178);
        assert!(obj.solid);
        assert!(!obj.obstruct_ground);
        
        Ok(())
    }
    
    #[test]
    fn furnace() -> crate::Result<()> {
        let obj_loader = obj_loader()?;
        let obj = obj_loader.load(2030).unwrap();
        
        assert_eq!(obj.name, "Furnace");
        assert!(obj.solid);
        assert!(!obj.obstruct_ground);
        
        Ok(())
    }
    
    #[test]
    fn bank_table() -> crate::Result<()> {
        let obj_loader = obj_loader()?;
        let obj = obj_loader.load(590).unwrap();
        
        assert_eq!(obj.name, "Bank table");
        assert_eq!(obj.supports_items, Some(1));
        assert!(obj.solid);
        assert!(!obj.obstruct_ground);
        
        Ok(())
    }
    
    #[test]
    fn dungeon_door() -> crate::Result<()> {
        let obj_loader = obj_loader()?;
        let obj = obj_loader.load(1725).unwrap();
        
        assert_eq!(obj.name, "Dungeon door");
        assert_eq!(obj.wall_or_door, Some(1));
        assert_eq!(obj.supports_items, Some(0));
        assert!(obj.solid);
        assert!(!obj.obstruct_ground);
        
        Ok(())
    }
}

#[cfg(test)]
mod locations {
    use super::LocationLoader;
    use crate::test_util;

    #[test]
    fn lumbridge() -> crate::Result<()> {
        let cache = test_util::osrs_cache()?;
        
        let keys: [u32; 4] = [3030157619, 2364842415, 3297319647, 1973582566];
        
        let mut location_loader = LocationLoader::new(&cache);
        let location_def = location_loader.load(12850, &keys)?;
        
        assert_eq!(location_def.region_x, 50);
        assert_eq!(location_def.region_y, 50);
        assert_eq!(location_def.region_base_coords(), (3200, 3200));
        assert_eq!(location_def.data.len(), 4730);
        
        Ok(())
    }
}

#[cfg(test)]
mod maps {
    use super::MapLoader;
    use crate::test_util;

    #[test]
    fn lumbridge() -> crate::Result<()> {
        let cache = test_util::osrs_cache()?;

        let mut map_loader = MapLoader::new(&cache);
        let map_def = map_loader.load(12850).unwrap();

        assert_eq!(map_def.region_x, 50);
        assert_eq!(map_def.region_y, 50);
        assert_eq!(map_def.region_base_coords(), (3200, 3200));

        Ok(())
    }
}
    