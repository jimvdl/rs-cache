mod common;

mod osrs {
    use super::common;
    
    #[test]
    fn load_item_blue_partyhat() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let item_loader = common::osrs::load_items(&cache)?;
        
        let item = item_loader.load(1042).unwrap();
        
        assert_eq!(item.name, "Blue partyhat");
        assert!(!item.stackable);
        assert!(!item.members_only);
        
        Ok(())
    }
    
    #[test]
    fn load_item_magic_logs() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let item_loader = common::osrs::load_items(&cache)?;
        
        let item = item_loader.load(1513).unwrap();
        
        assert_eq!(item.name, "Magic logs");
        assert!(!item.stackable);
        assert!(item.members_only);
        
        Ok(())
    }
    
    #[test]
    fn load_item_logs_noted() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let item_loader = common::osrs::load_items(&cache)?;
        
        let item = item_loader.load(1512).unwrap();
        
        assert!(item.stackable);
        assert!(!item.members_only);
        
        Ok(())
    }
    
    #[test]
    fn incorrect_item_id() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let item_loader = common::osrs::load_items(&cache)?;
        
        let item = item_loader.load(65_535);
        
        assert!(item.is_none());
        
        Ok(())
    }
}

mod rs3 {
    use super::common;
    
    #[test]
    fn load_item_blue_partyhat() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        let item_loader = common::rs3::load_items(&cache)?;
        
        let item = item_loader.load(1042).unwrap();
        
        assert_eq!(item.name, "Blue partyhat");
        assert!(!item.stackable);
        assert!(!item.members_only);
        
        Ok(())
    }

    #[test]
    fn load_item_master_mining_cape() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        let item_loader = common::rs3::load_items(&cache)?;

        let item = item_loader.load(31285).unwrap();

        assert_eq!(item.name, "Mining master cape");
        assert!(!item.stackable);
        assert!(item.members_only);
        assert_eq!(item.cost, 120_000);
        assert_eq!(item.interface_options[1], "Wear");
        
        Ok(())
    }

    #[test]
    fn load_item_luminite_stone_spirit() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        let item_loader = common::rs3::load_items(&cache)?;

        let item = item_loader.load(44806).unwrap();

        assert_eq!(item.name, "Luminite stone spirit");
        assert!(item.stackable);
        assert!(!item.members_only);
        assert_eq!(item.cost, 840);
        
        Ok(())
    }

    #[test]
    fn load_item_light_animica() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        let item_loader = common::rs3::load_items(&cache)?;

        let item = item_loader.load(44830).unwrap();

        assert_eq!(item.name, "Light animica");
        assert!(!item.stackable);
        assert!(item.members_only);
        assert_eq!(1734, item.cost);
        
        Ok(())
    }

    #[test]
    fn load_item_elder_rune_pickaxe_5() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        let item_loader = common::rs3::load_items(&cache)?;

        let item = item_loader.load(45652).unwrap();

        assert_eq!(item.name, "Elder rune pickaxe + 5");
        assert!(!item.stackable);
        assert!(item.members_only);
        assert_eq!(item.cost, 1_066_668);
        assert_eq!(item.interface_options[1], "Wield");
        
        Ok(())
    }

    #[test]
    fn load_item_wergali_incense_sticks() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        let item_loader = common::rs3::load_items(&cache)?;

        let item = item_loader.load(47707).unwrap();

        assert_eq!(item.name, "Wergali incense sticks");
        assert!(item.stackable);
        assert!(item.members_only);
        assert_eq!(item.cost, 208);
        assert_eq!(item.interface_options[0], "Light");
        
        Ok(())
    }

    #[test]
    fn incorrect_item_id() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        let item_loader = common::rs3::load_items(&cache)?;
        
        let item = item_loader.load(65_535);
        
        assert!(item.is_none());
        
        Ok(())
    }
}