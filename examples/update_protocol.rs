use rscache::Cache;

fn main() -> rscache::Result<()> {
    let cache = Cache::new("./data/cache")?;

    let index_id = 2; // Config index.
    let archive_id = 10; // random archive.

    let mut buffer = if index_id == 255 && archive_id == 255 {
		cache.create_checksum()?.encode()?
	} else {
		let mut buf = cache.read(index_id, archive_id)?.to_vec();
		if index_id != 255 {
			buf.truncate(buf.len() - 2);
			buf
		} else {
			buf
		}
	};

	let compression = buffer[0];
	let length = u32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]]);

	buffer.drain(..5);

	let mut data = vec![0; buffer.len() + 8];
	data[0] = index_id;
	data[1..3].copy_from_slice(&archive_id.to_be_bytes());
	data[3] = compression;
	data[4..8].copy_from_slice(&length.to_be_bytes());
	data[8..].copy_from_slice(&buffer);

	let chunks = data.len() / 512;
	for index_id in (0..data.len() + chunks).step_by(512) {
		if index_id == 0 || data.len() == 512 {
			continue;
		}

		data.insert(index_id, 255);
	}

    // write data to the client
    // stream.write_all(&data)?;
    
	Ok(())
}