mod common;

#[cfg(feature = "osrs")]
mod osrs {
    use super::common;
    
    #[test]
    fn load_woodsman_tutor() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let npc_loader = common::osrs::load_npcs(&cache)?;
        
        let npc = npc_loader.load(3226).unwrap();
        
        assert_eq!("Woodsman tutor", npc.name);
        assert!(npc.interactable);
        
        Ok(())
    }
    
    #[test]
    fn load_last_npc() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let npc_loader = common::osrs::load_npcs(&cache)?;
        
        let npc = npc_loader.load(8691).unwrap();
        
        assert_eq!("Mosol Rei", npc.name);
        assert!(npc.interactable);
        
        Ok(())
    }
    
    #[test]
    fn incorrect_npc_id() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let npc_loader = common::osrs::load_npcs(&cache)?;
        
        let npc = npc_loader.load(65_535);
        
        assert!(npc.is_none());
        
        Ok(())
    }
}