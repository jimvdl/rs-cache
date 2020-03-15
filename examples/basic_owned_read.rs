use rscache::Cache;
use rscache::CacheError;

// LinkedList extensions (includes to_vec())
use rscache::LinkedListExt;

fn main() -> Result<(), CacheError> {
    let cache = Cache::new("./data/cache")?;

    let index_id = 2; // Config index.
    let archive_id = 10; // random archive.
    let buffer = cache.read(index_id, archive_id)?;

    // Turns LinkedList<&[u8]> into Vec<u8>
    let owned = buffer.to_vec();
    
    println!("owned: {:?}", &owned[..25]);

    Ok(())
}