use rscache::Cache;
use sha1::Sha1;

pub fn osrs_cache() -> Cache {
    Cache::new("./data/osrs_cache").unwrap()
}

#[cfg(all(test, feature = "rs3"))]
pub fn rs3_cache() -> Cache {
    Cache::new("./data/rs3_cache").unwrap()
}

#[allow(dead_code)]
pub fn hash(buffer: &[u8]) -> String {
    let mut m = Sha1::new();

    m.update(buffer);
    m.digest().to_string()
}
