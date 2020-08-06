#![allow(dead_code)]

use sha1::Sha1;

pub fn hash(buffer: &[u8]) -> String {
    let mut m = Sha1::new();

    m.update(&buffer);
    m.digest().to_string()
}

pub mod osrs {
    use rscache::OsrsCache;
    
    use rscache::ldr::osrs::{
        ItemLoader,
        NpcLoader,
        ObjectLoader
    };

    pub fn setup() -> rscache::Result<OsrsCache> {
        OsrsCache::new("./data/cache")
    }

    pub fn load_items(cache: &OsrsCache) -> rscache::Result<ItemLoader> {
        ItemLoader::new(&cache)
    }

    pub fn load_npcs(cache: &OsrsCache) -> rscache::Result<NpcLoader> {
        NpcLoader::new(&cache)
    }
    
    pub fn load_objects(cache: &OsrsCache) -> rscache::Result<ObjectLoader> {
        ObjectLoader::new(&cache)
    }
}