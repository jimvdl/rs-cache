mod common;

mod osrs {
    use super::common;

    #[test]
    fn read_from_ref_table() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        
        let archive = cache.read(255, 10)?;
        
        let hash = common::hash(&archive);
        assert_eq!("64fb9fcf381a547bb7beafbc3b7ba4fd847f21ef", &hash);
        assert_eq!(77, archive.len());
        
        Ok(())
    }
    
    #[test]
    fn read_from_0_16() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        
        let archive = cache.read(0, 16)?;
        
        let hash = common::hash(&archive);
        assert_eq!("ad53ed574c3539400c822a9fc4c028b3e5e50e33", &hash);
        assert_eq!(1543, archive.len());
        
        Ok(())
    }
    
    #[test]
    fn read_from_0_191() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        
        let archive = cache.read(0, 191)?;
        
        let hash = common::hash(&archive);
        assert_eq!("cd459f6ccfbd81c1e3bfadf899624f2519e207a9", &hash);
        assert_eq!(2055, archive.len());
        
        Ok(())
    }
    
    #[test]
    fn read_from_2_10() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        
        let archive = cache.read(2, 10)?;
        
        let hash = common::hash(&archive);
        assert_eq!("c6ee1518e9a39a42ecaf946c6c84a942cb3102f4", &hash);
        assert_eq!(260_537, archive.len());
        
        Ok(())
    }
    
    #[test]
    fn read_from_7_24918() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        
        let archive = cache.read(7, 24918)?;
        
        let hash = common::hash(&archive);
        assert_eq!("fe91e9e9170a5a05ed2684c1db1169aa7ef4906e", &hash);
        assert_eq!(803, archive.len());
        
        Ok(())
    }

    #[test]
    fn read_from_2_25000_fails() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        
        assert!(cache.read(2, 25_000).is_err());
        
        Ok(())
    }
}

mod rs3 {
    use super::common;

    #[test]
    fn read_from_0_25() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        
        let archive = cache.read(0, 25)?;
        
        let hash = common::hash(&archive);
        assert_eq!("81e455fc58fe5ac98fee4df5b78600bbf43e83f7", &hash);
        assert_eq!(1576, archive.len());
        
        Ok(())
    }

    #[test]
    fn read_from_7_0() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        
        let archive = cache.read(7, 0)?;

        let hash = common::hash(&archive);
        assert_eq!("b33919c6e4677abc6ec1c0bdd9557f820a163559", &hash);
        assert_eq!(529, archive.len());
        
        Ok(())
    }

    #[test]
    fn read_from_2_25000_fails() -> rscache::Result<()> {
        let cache = common::rs3::setup()?;
        
        assert!(cache.read(2, 25_000).is_err());
        
        Ok(())
    }
}