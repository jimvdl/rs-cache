mod common;

mod osrs {
    use super::common;

    #[test]
    fn load_player_backpack() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;
        let inventory_loader = common::osrs::load_inventories(&cache)?;

        let inventory = inventory_loader.load(93).unwrap();

        assert_eq!(28, inventory.capacity.unwrap());

        Ok(())
    }
}