use rscache::Cache;

fn main() -> rscache::Result<()> {
    let cache = Cache::new("./data/cache")?;

    let index_id = 2; // Config index.
    let archive_id = 10; // random archive.
    let buffer = cache.read(index_id, archive_id)?;
    
    println!("owned: {:?}", &buffer[..25]);

    Ok(())
}