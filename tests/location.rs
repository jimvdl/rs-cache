mod common;

mod osrs {
    use std::convert::TryInto;
    use super::common;

    use rscache::{ Cache, def::osrs::{ Definition, LocationDefinition }, codec };
    #[inline]
    pub fn load_loc_def(cache: &Cache, region_id: u16, keys: [u32; 4]) -> rscache::Result<Option<LocationDefinition>> {
        let x = region_id >> 8;
        let y = region_id & 0xFF;

        if let Ok(map_archive) = cache.archive_by_name(5, format!("l{}_{}", x, y)) {
            let buffer = cache.read_archive(&map_archive)?;
            let buffer = codec::decode_with_keys(&buffer, &keys)?;

            println!("{:?}", &buffer[..25]);
            
            return Ok(Some(LocationDefinition::new(region_id, &buffer)?))
        }

        Ok(None)
    }
    
    #[test]
    fn load_locations() -> rscache::Result<()> {
        let cache = common::osrs::setup()?;

        let one = -1264809677;
        let two = -1930124881;
        let three = -997647649;
        let keys: [u32; 4] = [one as u32, two as u32, three as u32, 1973582566];
        let loc_def = load_loc_def(&cache, 12850, keys)?;

        panic!("{:?}", loc_def);
        
        Ok(())
    }
}