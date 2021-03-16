mod common;

#[cfg(feature = "osrs")]
mod osrs {
    use super::common;
    
    #[test]
    fn load_law_rift() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let obj_loader = common::osrs::load_objects(&cache)?;
        
        let obj = obj_loader.load(25034).unwrap();
        
        assert_eq!("Law rift", obj.name);
        assert_eq!(2178, obj.animation_id);
        assert!(obj.solid);
        assert!(!obj.obstruct_ground);
        
        Ok(())
    }
    
    #[test]
    fn load_furnace() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let obj_loader = common::osrs::load_objects(&cache)?;
        
        let obj = obj_loader.load(2030).unwrap();
        
        assert_eq!("Furnace", obj.name);
        assert!(obj.solid);
        assert!(!obj.obstruct_ground);
        
        Ok(())
    }
    
    #[test]
    fn load_bank_table() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let obj_loader = common::osrs::load_objects(&cache)?;
        
        let obj = obj_loader.load(590).unwrap();
        
        assert_eq!("Bank table", obj.name);
        assert_eq!(Some(1), obj.supports_items);
        assert!(obj.solid);
        assert!(!obj.obstruct_ground);
        
        Ok(())
    }
    
    #[test]
    fn load_dungeon_door() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let obj_loader = common::osrs::load_objects(&cache)?;
        
        let obj = obj_loader.load(1725).unwrap();
        
        assert_eq!("Dungeon door", obj.name);
        assert_eq!(Some(1), obj.wall_or_door);
        assert_eq!(Some(0), obj.supports_items);
        assert!(obj.solid);
        assert!(!obj.obstruct_ground);
        
        Ok(())
    }
}