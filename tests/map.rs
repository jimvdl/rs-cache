mod common;

mod osrs {
    use super::common;

    use rscache::{ Cache, def::osrs::{ Definition, MapDefinition }, codec };
    #[inline]
    pub fn load_map_def(cache: &Cache, region_id: u16) -> rscache::Result<Option<MapDefinition>> {
        let x = region_id >> 8;
        let y = region_id & 0xFF;

        if let Ok(map_archive) = cache.archive_by_name(5, format!("m{}_{}", x, y)) {
            let buffer = cache.read_archive(&map_archive)?;
            let buffer = codec::decode(&buffer)?;
            
            return Ok(Some(MapDefinition::new(region_id, &buffer)?))
        }

        Ok(None)
    }
    
    #[test]
    fn load_lumbridge_map_data() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;

        let lumbridge_region_id = 12850;

        // let mut map_loader = rscache::ldr::osrs::MapLoader::new();
        // let map_def = map_loader.load(&cache, lumbridge_region_id)?;
        // let region_base_x = ((lumbridge_region_id as u32 >> 8) & 0xFF) << 6;
        // let region_base_y = (lumbridge_region_id as u32 & 0xFF) << 6;
        
        for i in 0..30000 {
            let map_def = load_map_def(&cache, i)
                .expect(&format!("Failed to load map definition for region: {}", i));
            
            if let Some(map_def) = map_def {
                println!("{}", map_def.blocked_tiles().len());
            }
        }

        // panic!();
        
        // let mut blocked_tiles = Vec::new();
        
        // for z in 0..4 {
        //     for x in 0..64 {
        //         for y in 0..64 {
        //             let setting = map_def.map_data(x, y, z).settings;
                    
        //             if setting & 1 == 1 {
        //                 blocked_tiles.push((region_base_x as usize + x, region_base_y as usize + y, z));
        //             }
        //         }
        //     }
        // }
        
        // if let Some(def) = map_def {
        //     assert_eq!(def.blocked_tiles().len(), 533);
        // }
        // panic!("{:?}", map_def.data.len());
        // panic!("{} {}", lumbridge_region_id as u32 >> 8, lumbridge_region_id as u32 & 0xFF);
        
        Ok(())
    }
}