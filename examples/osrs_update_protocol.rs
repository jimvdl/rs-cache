use rscache::Cache;

struct IncomingUpdatePacket {
    pub index_id: u8,
    pub archive_id: u32,
}

// This example illustrates the osrs update protocol.
// You can use this to handle client requests for cache data.
fn main() -> rscache::Result<()> {
    let cache = Cache::new("./data/osrs_cache")?;
    let packet = IncomingUpdatePacket{ index_id: 255, archive_id: 10 };

    let mut buffer = if packet.index_id == 255 && packet.archive_id == 255 {
        cache.create_checksum()?.encode_osrs()?
    } else {
        let buf = cache.read(packet.index_id, packet.archive_id)?;
        format_buffer(buf, packet.index_id)
    };

    let compression = buffer[0];
    let length = parse_length(&buffer);

    buffer.drain(..5);

    let mut data = vec![0; buffer.len() + 8];
    data[0] = packet.index_id;
    data[1..3].copy_from_slice(&(packet.archive_id as u16).to_be_bytes());
    data[3] = compression;
    data[4..8].copy_from_slice(&length.to_be_bytes());
    data[8..].copy_from_slice(&buffer);

    // Adds separators (255) to tell the client to keep reading bytes.
    let chunks = data.len() / 512;
    for index_id in (0..data.len() + chunks).step_by(512) {
        if index_id == 0 || data.len() == 512 {
            continue;
        }

        data.insert(index_id, 255);
    }

    // write data to the client
    // stream.write_all(&data)?;

    println!("{:?}", data);
    
    Ok(())
}

fn format_buffer(mut buffer: Vec<u8>, index_id: u8) -> Vec<u8> {
    if index_id != 255 {
        buffer.truncate(buffer.len() - 2);
        buffer
    } else {
        buffer
    }
}

fn parse_length(buffer: &[u8]) -> u32 {
    u32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]])
}