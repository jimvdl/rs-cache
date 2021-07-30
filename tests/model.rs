mod common;

mod osrs {
    use super::common;
    
    #[test]
    fn load_model_something() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let model_loader = common::osrs::load_models(&cache)?;
        
        // let model = model_loader.load(1042).unwrap();
        for model in &model_loader.mdls {
            
        }

        panic!();
        
        Ok(())
    }
}

mod rs3 {
}