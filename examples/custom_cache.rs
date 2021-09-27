use std::path::Path;

use rscache::{ 
    CacheCore, CacheRead, 
    idx::Indices,
    error::ReadError,
};

fn main() -> rscache::Result<()> {
    let cache = CustomCache::new("./data/osrs_cache")?;
    let buffer = cache.read(255, 2)?;

    println!("buffer: {:?}", buffer);
    
    Ok(())
}

// CustomCache is essentially a Cache but its just to give you an
// idea of how to implement your own cache.
struct CustomCache {
    data: Vec<u8>,
    indices: Indices,
    // You can also include other variables if needed.
}

impl CacheCore for CustomCache {
    fn new<P: AsRef<Path>>(path: P) -> rscache::Result<Self> {
        let path = path.as_ref();

        // load your own data here.
        let data = vec![0; 250];
        let indices = Indices::new(path)?;

        Ok(Self { data, indices })
    }
}

impl CacheRead for CustomCache {
    fn read(&self, index_id: u8, archive_id: u32) -> rscache::Result<Vec<u8>> {
        let index = self.indices.get(&index_id)
            .ok_or(ReadError::IndexNotFound(index_id))?;

        let _archive = index.archives.get(&archive_id)
            .ok_or(ReadError::ArchiveNotFound(index_id, archive_id))?;

        // Read the bytes with your own custom way of reading.
        let buffer = self.data[..25].to_vec();

        Ok(buffer)
    }
}