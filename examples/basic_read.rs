use rscache::Cache;
use rscache::CacheError;

fn main() -> Result<(), CacheError> {
    let cache = Cache::new("./data/cache")?;

    let index_id = 2; // Config index.
    let archive_id = 10; // random archive.
    let buffer = cache.read(index_id, archive_id)?;

    for data_block in buffer.iter() {
        println!("data_block: {:?}, length: {}", &data_block[..25], data_block.len());
    }

    Ok(())
}