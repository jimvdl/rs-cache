mod common;

#[cfg(feature = "osrs")]
mod osrs {
    use super::common;
    
    #[test]
    fn load_law_rift() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let obj_loader = common::osrs::load_objects(&cache)?;
        
        let obj = obj_loader.load(25034).unwrap();
        
        assert_eq!(obj.name, "Law rift");
        assert_eq!(obj.animation_id, 2178);
        assert!(obj.solid);
        assert!(!obj.obstruct_ground);
        
        Ok(())
    }
    
    #[test]
    fn load_furnace() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let obj_loader = common::osrs::load_objects(&cache)?;
        
        let obj = obj_loader.load(2030).unwrap();
        
        assert_eq!(obj.name, "Furnace");
        assert!(obj.solid);
        assert!(!obj.obstruct_ground);
        
        Ok(())
    }
    
    #[test]
    fn load_bank_table() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let obj_loader = common::osrs::load_objects(&cache)?;
        
        let obj = obj_loader.load(590).unwrap();
        
        assert_eq!(obj.name, "Bank table");
        assert_eq!(obj.supports_items, Some(1));
        assert!(obj.solid);
        assert!(!obj.obstruct_ground);
        
        Ok(())
    }
    
    #[test]
    fn load_dungeon_door() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let obj_loader = common::osrs::load_objects(&cache)?;
        
        let obj = obj_loader.load(1725).unwrap();
        
        assert_eq!(obj.name, "Dungeon door");
        assert_eq!(obj.wall_or_door, Some(1));
        assert_eq!(obj.supports_items, Some(0));
        assert!(obj.solid);
        assert!(!obj.obstruct_ground);
        
        Ok(())
    }
}