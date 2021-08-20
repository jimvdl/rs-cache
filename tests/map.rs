mod common;

mod osrs {
    use super::common;
    
    #[test]
    fn load_lumbridge_map_data() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;

        let lumbridge_region_id = 12850;
        let region_base_x = ((lumbridge_region_id as u32 >> 8) & 0xFF) << 6;
        let region_base_y = (lumbridge_region_id as u32 & 0xFF) << 6;

        let map_def = rscache::util::osrs::load_map_def(&cache, lumbridge_region_id)
            .expect(&format!("Failed to load map definition for region: {}", lumbridge_region_id))
            .expect(&format!("Map data for region {} not found.", lumbridge_region_id));

        let mut blocked_tiles = Vec::new();
        
        for z in 0..4 {
            for x in 0..64 {
                for y in 0..64 {
                    let setting = map_def.map_data(x, y, z).settings;

                    if setting & 1 == 1 {
                        blocked_tiles.push((region_base_x as usize + x, region_base_y as usize + y, z));
                    }
                }
            }
        }

        assert_eq!(blocked_tiles.len(), 533);
        
        Ok(())
    }
}