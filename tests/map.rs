mod common;

#[cfg(not(feature = "rs3"))]
mod osrs {
    use super::common;
    use rscache::loader::osrs::MapLoader;
    #[test]
    fn load_lumbridge_map_data() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;

        let mut map_loader = MapLoader::new(&cache);
        let map_def = map_loader.load(12850).unwrap();

        assert_eq!(map_def.region_x, 50);
        assert_eq!(map_def.region_y, 50);
        assert_eq!(map_def.region_base_coords(), (3200, 3200));

        Ok(())
    }
}
