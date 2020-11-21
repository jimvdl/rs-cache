mod common;

mod osrs {
    use super::common;
    use rscache::def::osrs::MapDefinition;
    
    #[test]
    fn load_lumbridge_map_data() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;

        let lumbridge_region_id = 12850u32;

        let x = lumbridge_region_id >> 8;
        let y = lumbridge_region_id & 0xFF;
        let region_base_x = ((lumbridge_region_id >> 8) & 0xFF) << 6;
        let region_base_y = (lumbridge_region_id & 0xFF) << 6;

        let map_archive = cache.archive_by_name(5, &format!("m{}_{}", x, y))?;
        let buffer = cache.read_archive(&map_archive)?;
        let buffer = rscache::codec::decode(&buffer)?;

        let map_def = MapDefinition::new(x, y, &buffer)?;
        
        for z in 0..4 {
            for x in 0..64 {
                for y in 0..64 {
                    let setting = map_def.map_data(x, y, z).settings;

                    if setting & 1 == 1 {
                        println!("blocked tile: {} {} {}", region_base_x as usize + x, region_base_y as usize + y, z);
                    }
                }
            }
        }

        assert!(false);
        
        Ok(())
    }
}