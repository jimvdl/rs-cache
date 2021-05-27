mod common;

mod osrs {
    use super::common;
    
    #[test]
    fn load_blue_partyhat() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let item_loader = common::osrs::load_items(&cache)?;
        
        let item = item_loader.load(1042).unwrap();
        
        assert_eq!("Blue partyhat", item.name);
        assert!(!item.stackable);
        assert!(!item.members_only);
        
        Ok(())
    }
    
    #[test]
    fn load_magic_logs() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let item_loader = common::osrs::load_items(&cache)?;
        
        let item = item_loader.load(1513).unwrap();
        
        assert_eq!("Magic logs", item.name);
        assert!(!item.stackable);
        assert!(item.members_only);
        
        Ok(())
    }
    
    #[test]
    fn load_logs_noted() -> rscache::Result<()> {
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
    fn load_blue_partyhat() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        let item_loader = common::rs3::load_items(&cache)?;
        
        let item = item_loader.load(1042).unwrap();
        
        assert_eq!("Blue partyhat", item.name);
        assert!(!item.stackable);
        assert!(!item.members_only);
        
        Ok(())
    }

    #[test]
    fn load_master_mining_cape() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        let item_loader = common::rs3::load_items(&cache)?;

        let item = item_loader.load(31285).unwrap();

        assert_eq!("Mining master cape", item.name);
        assert!(!item.stackable);
        assert!(item.members_only);
        assert_eq!(120_000, item.cost);
        assert_eq!("Wear".to_owned(), item.interface_options[1]);
        
        Ok(())
    }

    #[test]
    fn load_luminite_stone_spirit() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        let item_loader = common::rs3::load_items(&cache)?;

        let item = item_loader.load(44806).unwrap();

        assert_eq!("Luminite stone spirit", item.name);
        assert!(item.stackable);
        assert!(!item.members_only);
        assert_eq!(840, item.cost);
        
        Ok(())
    }

    #[test]
    fn load_light_animica() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        let item_loader = common::rs3::load_items(&cache)?;

        let item = item_loader.load(44830).unwrap();

        assert_eq!("Light animica", item.name);
        assert!(!item.stackable);
        assert!(item.members_only);
        assert_eq!(1734, item.cost);
        
        Ok(())
    }

    #[test]
    fn load_elder_rune_pickaxe_5() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        let item_loader = common::rs3::load_items(&cache)?;

        let item = item_loader.load(45652).unwrap();

        assert_eq!("Elder rune pickaxe + 5", item.name);
        assert!(!item.stackable);
        assert!(item.members_only);
        assert_eq!(1_066_668, item.cost);
        assert_eq!("Wield".to_owned(), item.interface_options[1]);
        
        Ok(())
    }

    #[test]
    fn load_wergali_incense_sticks() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        let item_loader = common::rs3::load_items(&cache)?;

        let item = item_loader.load(47707).unwrap();

        assert_eq!("Wergali incense sticks", item.name);
        assert!(item.stackable);
        assert!(item.members_only);
        assert_eq!(208, item.cost);
        assert_eq!("Light".to_owned(), item.interface_options[0]);
        
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