#![allow(dead_code)]

use sha1::Sha1;

pub fn hash(buffer: &[u8]) -> String {
    let mut m = Sha1::new();

    m.update(&buffer);
    m.digest().to_string()
}

pub mod osrs {
    use rscache::{ Cache, store::MemoryStore };
    
    use rscache::ldr::osrs::{
        ItemLoader,
        NpcLoader,
        ObjectLoader
    };

    pub fn setup() -> rscache::Result<Cache<MemoryStore>> {
        Cache::new("./data/cache")
    }

    pub fn load_items(cache: &Cache<MemoryStore>) -> rscache::Result<ItemLoader> {
        ItemLoader::new(&cache)
    }

    pub fn load_npcs(cache: &Cache<MemoryStore>) -> rscache::Result<NpcLoader> {
        NpcLoader::new(&cache)
    }
    
    pub fn load_objects(cache: &Cache<MemoryStore>) -> rscache::Result<ObjectLoader> {
        ObjectLoader::new(&cache)
    }
}