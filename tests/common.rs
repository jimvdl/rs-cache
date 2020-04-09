use rscache::{ Cache, CacheError };
use sha1::Sha1;

pub fn setup() -> Result<Cache, CacheError> {
    Cache::new("./data/cache")
}

pub fn hash(buffer: &[u8]) -> String {
    let mut m = Sha1::new();

    m.update(&buffer);
    m.digest().to_string()
}