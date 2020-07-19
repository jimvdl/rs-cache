#![allow(dead_code)]

use rscache::{ Cache, CacheError };

use rscache::{
    ItemLoader,
    NpcLoader,
    ObjectLoader,
};

use sha1::Sha1;

pub fn setup() -> Result<Cache, CacheError> {
    Cache::new("./data/cache")
}

pub fn load_items(cache: &Cache) -> Result<ItemLoader, CacheError> {
    ItemLoader::new(&cache)
}

pub fn load_npcs(cache: &Cache) -> Result<NpcLoader, CacheError> {
    NpcLoader::new(&cache)
}

pub fn load_objects(cache: &Cache) -> Result<ObjectLoader, CacheError> {
    ObjectLoader::new(&cache)
}

pub fn hash(buffer: &[u8]) -> String {
    let mut m = Sha1::new();

    m.update(&buffer);
    m.digest().to_string()
}