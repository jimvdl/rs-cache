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
        Cache::new("./data/osrs_cache")
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

pub mod rs3 {
    use rscache::{ Cache, store::FileStore };

    use rscache::ldr::rs3::ItemLoader;

    pub const EXPONENT: &'static [u8] = b"5206580307236375668350588432916871591810765290737810323990754121164270399789630501436083337726278206128394461017374810549461689174118305784406140446740993";
    pub const MODULUS: &'static [u8] = b"6950273013450460376345707589939362735767433035117300645755821424559380572176824658371246045200577956729474374073582306250298535718024104420271215590565201";

    pub fn setup() -> rscache::Result<Cache<FileStore>> {
        Cache::new("./data/rs3_cache")
    }

    pub fn load_items(cache: &Cache<FileStore>) -> rscache::Result<ItemLoader> {
        ItemLoader::new(&cache)
    }
}