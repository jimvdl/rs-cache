use std::{ io::Read, fs::File };
use rscache::{ Cache, Store, arc::Archive };

fn main() -> rscache::Result<()> {
    // Uses default Cache struct with custom store.
    let cache: Cache<CustomStore> = Cache::new("./data/cache")?;
    let buffer = cache.read(255, 18)?;

    println!("buffer: {:?}", buffer);

    Ok(())
}

// This struct will function as your data storage.
// Your custom internal read should be able to read from
// this inner buffer.
pub struct CustomStore {
    data: Vec<u8>
}

impl Store for CustomStore {
    fn new(mut main_file: File) -> rscache::Result<Self> {
        let mut buffer = Vec::new();
        main_file.read_to_end(&mut buffer)?;
        
        Ok(Self { data: buffer })
    }

    fn read(&self, archive: &Archive) -> rscache::Result<Vec<u8>> {
        // Implement your own read here.

        println!("{:?}", archive);

        // For the sake of this example: only return the first
        // 25 bytes of the internal buffer.
		Ok(self.data[..25].to_vec())
    }
}