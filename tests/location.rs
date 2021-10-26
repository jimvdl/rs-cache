mod common;

mod osrs {
    use super::common;

    use rscache::loader::osrs::LocationLoader;
    
    #[test]
    fn load_locations() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;

        let keys: [u32; 4] = [3030157619, 2364842415, 3297319647, 1973582566];

        let mut location_loader = LocationLoader::new(&cache);
        let location_def = location_loader.load(12850, &keys)?;

        assert_eq!(location_def.region_x, 50);
        assert_eq!(location_def.region_y, 50);
        assert_eq!(location_def.region_base_coords(), (3200, 3200));
        assert_eq!(location_def.data.len(), 4730);

        Ok(())
    }
}
