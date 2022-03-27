mod test_util;

#[cfg(test)]
mod osrs {
    use super::test_util;

    mod items {
        use super::test_util;
        use rscache::loader::osrs::ItemLoader;

        fn item_loader() -> ItemLoader {
            ItemLoader::new(&test_util::osrs_cache()).unwrap()
        }

        #[test]
        fn blue_partyhat() {
            let item_loader = item_loader();
            let item = item_loader.load(1042).unwrap();

            assert_eq!(item.name, "Blue partyhat");
            assert!(!item.stackable);
            assert!(!item.members_only);
        }

        #[test]
        fn magic_logs() {
            let item_loader = item_loader();
            let item = item_loader.load(1513).unwrap();

            assert_eq!(item.name, "Magic logs");
            assert!(!item.stackable);
            assert!(item.members_only);
        }

        #[test]
        fn noted() {
            let item_loader = item_loader();
            let item = item_loader.load(1512).unwrap();

            assert!(item.stackable);
            assert!(!item.members_only);
        }

        #[test]
        fn non_existent() {
            let item_loader = item_loader();
            assert!(item_loader.load(65_535).is_none());
        }
    }

    mod npcs {
        use super::test_util;
        use rscache::loader::osrs::NpcLoader;

        fn npc_loader() -> NpcLoader {
            NpcLoader::new(&test_util::osrs_cache()).unwrap()
        }

        #[test]
        fn woodsman_tutor() {
            let npc_loader = npc_loader();
            let npc = npc_loader.load(3226).unwrap();

            assert_eq!(npc.name, "Woodsman tutor");
            assert!(npc.interactable);
        }

        #[test]
        fn last_valid_npc() {
            let npc_loader = npc_loader();
            let npc = npc_loader.load(8691).unwrap();

            assert_eq!(npc.name, "Mosol Rei");
            assert!(npc.interactable);
        }

        #[test]
        fn non_existent() {
            let npc_loader = npc_loader();
            assert!(npc_loader.load(65_535).is_none());
        }
    }

    mod objects {
        use super::test_util;
        use rscache::loader::osrs::ObjectLoader;

        fn obj_loader() -> ObjectLoader {
            ObjectLoader::new(&test_util::osrs_cache()).unwrap()
        }

        #[test]
        fn law_rift() {
            let obj_loader = obj_loader();
            let obj = obj_loader.load(25034).unwrap();

            assert_eq!(obj.name, "Law rift");
            assert_eq!(obj.animation_id, 2178);
            assert!(obj.solid);
            assert!(!obj.obstruct_ground);
        }

        #[test]
        fn furnace() {
            let obj_loader = obj_loader();
            let obj = obj_loader.load(2030).unwrap();

            assert_eq!(obj.name, "Furnace");
            assert!(obj.solid);
            assert!(!obj.obstruct_ground);
        }

        #[test]
        fn bank_table() {
            let obj_loader = obj_loader();
            let obj = obj_loader.load(590).unwrap();

            assert_eq!(obj.name, "Bank table");
            assert_eq!(obj.supports_items, Some(1));
            assert!(obj.solid);
            assert!(!obj.obstruct_ground);
        }

        #[test]
        fn dungeon_door() {
            let obj_loader = obj_loader();
            let obj = obj_loader.load(1725).unwrap();

            assert_eq!(obj.name, "Dungeon door");
            assert_eq!(obj.wall_or_door, Some(1));
            assert_eq!(obj.supports_items, Some(0));
            assert!(obj.solid);
            assert!(!obj.obstruct_ground);
        }
    }

    mod locations {
        use super::test_util;
        use rscache::loader::osrs::LocationLoader;

        #[test]
        fn lumbridge() {
            let cache = test_util::osrs_cache();

            let keys: [u32; 4] = [3030157619, 2364842415, 3297319647, 1973582566];

            let mut location_loader = LocationLoader::new(&cache);
            let location_def = location_loader.load(12850, &keys).unwrap();

            assert_eq!(location_def.region_x, 50);
            assert_eq!(location_def.region_y, 50);
            assert_eq!(location_def.region_base_coords(), (3200, 3200));
            assert_eq!(location_def.data.len(), 4730);
        }
    }

    mod maps {
        use super::test_util;
        use rscache::loader::osrs::MapLoader;

        #[test]
        fn lumbridge() {
            let cache = test_util::osrs_cache();

            let mut map_loader = MapLoader::new(&cache);
            let map_def = map_loader.load(12850).unwrap();

            assert_eq!(map_def.region_x, 50);
            assert_eq!(map_def.region_y, 50);
            assert_eq!(map_def.region_base_coords(), (3200, 3200));
        }
    }
}

#[cfg(all(test, feature = "rs3"))]
mod rs3 {
    use super::test_util;

    mod items {
        use super::test_util;
        use rscache::loader::rs3::ItemLoader;
        
        fn item_loader() -> ItemLoader {
            ItemLoader::new(&test_util::rs3_cache()).unwrap()
        }
        #[test]
        fn blue_partyhat() {
            let item_loader = item_loader();
            let item = item_loader.load(1042).unwrap();
            assert_eq!(item.name, "Blue partyhat");
            assert!(!item.stackable);
            assert!(!item.members_only);
        }
        #[test]
        fn master_mining_cape() {
            let item_loader = item_loader();
            let item = item_loader.load(31285).unwrap();
            assert_eq!(item.name, "Mining master cape");
            assert!(!item.stackable);
            assert!(item.members_only);
            assert_eq!(item.cost, 120_000);
            assert_eq!(item.interface_options[1], "Wear");
        }
        #[test]
        fn luminite_stone_spirit() {
            let item_loader = item_loader();
            let item = item_loader.load(44806).unwrap();
            assert_eq!(item.name, "Luminite stone spirit");
            assert!(item.stackable);
            assert!(!item.members_only);
            assert_eq!(item.cost, 840);
        }
        #[test]
        fn light_animica() {
            let item_loader = item_loader();
            let item = item_loader.load(44830).unwrap();
            assert_eq!(item.name, "Light animica");
            assert!(!item.stackable);
            assert!(item.members_only);
            assert_eq!(1734, item.cost);
        }
        #[test]
        fn elder_rune_pickaxe_5() {
            let item_loader = item_loader();
            let item = item_loader.load(45652).unwrap();
            assert_eq!(item.name, "Elder rune pickaxe + 5");
            assert!(!item.stackable);
            assert!(item.members_only);
            assert_eq!(item.cost, 1_066_668);
            assert_eq!(item.interface_options[1], "Wield");
        }
        #[test]
        fn wergali_incense_sticks() {
            let item_loader = item_loader();
            let item = item_loader.load(47707).unwrap();
            assert_eq!(item.name, "Wergali incense sticks");
            assert!(item.stackable);
            assert!(item.members_only);
            assert_eq!(item.cost, 208);
            assert_eq!(item.interface_options[0], "Light");
        }
        #[test]
        fn non_existent() {
            let item_loader = item_loader();
            let item = item_loader.load(65_535);
            assert!(item.is_none());
        }
    }
}
