use rscache::{
    checksum::{RsaChecksum, RsaKeys},
    Cache,
};

pub const EXPONENT: &[u8] = b"5206580307236375668350588432916871591810765290737810323990754121164270399789630501436083337726278206128394461017374810549461689174118305784406140446740993";
pub const MODULUS: &[u8] = b"6950273013450460376345707589939362735767433035117300645755821424559380572176824658371246045200577956729474374073582306250298535718024104420271215590565201";

struct IncomingUpdatePacket {
    pub index_id: u8,
    pub archive_id: u32,
    // 0 is low priority, 1 is high priority
    pub priority: u8,
}

// This example illustrates the rs3 update protocol.
// You can use this to handle client requests for cache data.
fn main() -> Result<(), rscache::Error> {
    let cache = Cache::new("./data/rs3_cache")?;
    let packet = IncomingUpdatePacket {
        index_id: 255,
        archive_id: 10,
        priority: 0,
    };

    let buffer = match packet {
        IncomingUpdatePacket {
            index_id: 255,
            archive_id: 255,
            .. // the priority is not really relevant for this example
        } => RsaChecksum::with_keys(&cache, RsaKeys::new(EXPONENT, MODULUS))?.encode()?,
        IncomingUpdatePacket {
            index_id,
            archive_id,
            ..
        } => cache.read(index_id, archive_id).map(|mut buffer| {
            if index_id != 255 {
                let len = buffer.len();
                buffer.truncate(len - 2);
            }
            buffer
        })?,
    };

    for data_block in buffer.chunks(102395) {
        let mut data = allocate_buffer(packet.index_id, packet.archive_id, data_block.len());

        encode_index_id(&mut data, packet.index_id);
        encode_archive_id(&mut data, packet.archive_id, packet.priority);
        if packet.index_id == 255 && packet.archive_id == 255 {
            encode_length(&mut data, buffer.len() as u32);
            encode_remaining(&mut data[10..], &buffer);
        } else {
            encode_remaining(&mut data[5..], &buffer);
        }
        // write data to the client
        // stream.write_all(&data)?;

        println!("{:?}", data);
    }
    Ok(())
}

fn allocate_buffer(index_id: u8, archive_id: u32, len: usize) -> Vec<u8> {
    if index_id == 255 && archive_id == 255 {
        vec![0; len + 10]
    } else {
        vec![0; len + 5]
    }
}

fn encode_index_id(buffer: &mut [u8], index_id: u8) {
    buffer[0] = index_id;
}

fn encode_archive_id(buffer: &mut [u8], archive_id: u32, priority: u8) {
    // packet_id 1 means it is a priority packet, 0 means no priority.
    let archive_id = if priority == 0 {
        archive_id | !0x7fffffff
    } else {
        archive_id
    };

    buffer[1..=4].copy_from_slice(&archive_id.to_be_bytes());
}

fn encode_length(buffer: &mut [u8], length: u32) {
    buffer[6..=9].copy_from_slice(&length.to_be_bytes());
}

fn encode_remaining(buffer: &mut [u8], buf: &[u8]) {
    buffer.copy_from_slice(buf);
}
