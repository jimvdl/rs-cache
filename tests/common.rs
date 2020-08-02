#![allow(dead_code)]

use sha1::Sha1;

pub mod osrs {
    use rscache::OsrsCache;

    pub fn setup() -> rscache::Result<OsrsCache> {
        OsrsCache::new("./data/cache")
    }
}

pub mod rs3 {
    use rscache::Rs3Cache;

    pub fn setup() -> rscache::Result<Rs3Cache> {
        Rs3Cache::new("./data/cache")
    }
}

// pub fn load_items(cache: &Cache) -> rscache::Result<ItemLoader> {
//     ItemLoader::new(&cache)
// }

// pub fn load_npcs(cache: &Cache) -> rscache::Result<NpcLoader> {
//     NpcLoader::new(&cache)
// }

// pub fn load_objects(cache: &Cache) -> rscache::Result<ObjectLoader> {
//     ObjectLoader::new(&cache)
// }

pub fn hash(buffer: &[u8]) -> String {
    let mut m = Sha1::new();

    m.update(&buffer);
    m.digest().to_string()
}