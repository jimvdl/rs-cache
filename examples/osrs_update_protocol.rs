use nom::number::complete::{be_u32, be_u8};
use rscache::{checksum::Checksum, Cache};

struct IncomingUpdatePacket {
    pub index_id: u8,
    pub archive_id: u32,
}

const HEADER_LEN: usize = 8;
const DATA_LEN: usize = 512;

// This example illustrates the osrs update protocol.
// You can use this to handle client requests for cache data.
fn main() -> Result<(), rscache::Error> {
    let cache = Cache::new("./data/osrs_cache")?;

    // The client would send a packet that would look something like this:
    let packet = IncomingUpdatePacket {
        index_id: 255,
        archive_id: 10,
    };

    let buffer = match packet {
        IncomingUpdatePacket {
            index_id: 255,
            archive_id: 255,
        } => Checksum::new(&cache)?.encode()?,
        IncomingUpdatePacket {
            index_id,
            archive_id,
        } => cache.read(index_id, archive_id as u32).map(|mut buffer| {
            if index_id != 255 {
                buffer.truncate(buffer.len() - 2);
            }
            buffer
        })?,
    };

    let (buffer, compression) = be_u8(buffer.as_slice())?;
    let (buffer, length) = be_u32(buffer)?;

    let mut archive_data = Vec::with_capacity(buffer.len() + HEADER_LEN);
    archive_data.push(packet.index_id);
    archive_data.extend(&(packet.archive_id as u16).to_be_bytes());
    archive_data.push(compression);
    archive_data.extend(&length.to_be_bytes());
    archive_data.extend(buffer);

    let chunks = archive_data.len() / DATA_LEN;
    for index in (0..archive_data.len() + chunks).step_by(DATA_LEN) {
        if index == 0 || archive_data.len() == DATA_LEN {
            continue;
        }

        archive_data.insert(index, 255);
    }

    // Write data to the client
    // stream.write_all(&data)?;

    println!("{:?}", archive_data);
    assert_eq!(archive_data.len(), 81);

    Ok(())
}
