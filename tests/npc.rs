mod common;

mod osrs {
    use super::common;
    
    #[test]
    fn load_woodsman_tutor() -> common::Result<()> {
        let cache = common::osrs::setup()?;
        let npc_loader = common::osrs::load_npcs(&cache)?;
        
        let npc = npc_loader.load(3226).unwrap();
        
        assert_eq!(npc.name, "Woodsman tutor");
        assert!(npc.interactable);
        
        Ok(())
    }
    
    #[test]
    fn load_last_npc() -> common::Result<()> {
        let cache = common::osrs::setup()?;
        let npc_loader = common::osrs::load_npcs(&cache)?;
        
        let npc = npc_loader.load(8691).unwrap();
        
        assert_eq!(npc.name, "Mosol Rei");
        assert!(npc.interactable);
        
        Ok(())
    }
    
    #[test]
    fn incorrect_npc_id() -> common::Result<()> {
        let cache = common::osrs::setup()?;
        let npc_loader = common::osrs::load_npcs(&cache)?;
        
        let npc = npc_loader.load(65_535);
        
        assert!(npc.is_none());
        
        Ok(())
    }
}