use std::collections::{hash_map, HashMap};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    definition::rs3::{FetchDefinition, ItemDefinition},
    Cache,
};

/// Loads all item definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ItemLoader(HashMap<u32, ItemDefinition>);

impl_rs3_loader!(ItemLoader, ItemDefinition, index_id: 19);

#[cfg(all(test, feature = "rs3"))]
mod items {
    use super::ItemLoader;

    fn item_loader() -> crate::Result<ItemLoader> {
        ItemLoader::new(&crate::Cache::new("./data/rs3_cache")?)
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
    fn master_mining_cape() -> crate::Result<()> {
        let item_loader = item_loader()?;
        let item = item_loader.load(31285).unwrap();

        assert_eq!(item.name, "Mining master cape");
        assert!(!item.stackable);
        assert!(item.members_only);
        assert_eq!(item.cost, 120_000);
        assert_eq!(item.interface_options[1], "Wear");

        Ok(())
    }

    #[test]
    fn luminite_stone_spirit() -> crate::Result<()> {
        let item_loader = item_loader()?;
        let item = item_loader.load(44806).unwrap();

        assert_eq!(item.name, "Luminite stone spirit");
        assert!(item.stackable);
        assert!(!item.members_only);
        assert_eq!(item.cost, 840);

        Ok(())
    }

    #[test]
    fn light_animica() -> crate::Result<()> {
        let item_loader = item_loader()?;
        let item = item_loader.load(44830).unwrap();

        assert_eq!(item.name, "Light animica");
        assert!(!item.stackable);
        assert!(item.members_only);
        assert_eq!(1734, item.cost);

        Ok(())
    }

    #[test]
    fn elder_rune_pickaxe_5() -> crate::Result<()> {
        let item_loader = item_loader()?;
        let item = item_loader.load(45652).unwrap();

        assert_eq!(item.name, "Elder rune pickaxe + 5");
        assert!(!item.stackable);
        assert!(item.members_only);
        assert_eq!(item.cost, 1_066_668);
        assert_eq!(item.interface_options[1], "Wield");
        
        Ok(())
    }

    #[test]
    fn wergali_incense_sticks() -> crate::Result<()> {
        let item_loader = item_loader()?;
        let item = item_loader.load(47707).unwrap();

        assert_eq!(item.name, "Wergali incense sticks");
        assert!(item.stackable);
        assert!(item.members_only);
        assert_eq!(item.cost, 208);
        assert_eq!(item.interface_options[0], "Light");

        Ok(())
    }

    #[test]
    fn non_existent() -> crate::Result<()> {
        let item_loader = item_loader()?;

        let item = item_loader.load(65_535);
        assert!(item.is_none());

        Ok(())
    }
}
