use rscache::{ Cache, store::FileStore };

pub const EXPONENT: &'static [u8] = b"5206580307236375668350588432916871591810765290737810323990754121164270399789630501436083337726278206128394461017374810549461689174118305784406140446740993";
pub const MODULUS: &'static [u8] = b"6950273013450460376345707589939362735767433035117300645755821424559380572176824658371246045200577956729474374073582306250298535718024104420271215590565201";

struct IncomingUpdatePacket {
    pub index_id: u8,
    pub archive_id: u32,
    pub packet_id: u8
}

// This example illustrates the rs3 update protocol.
// You can use this to handle client requests for cache data.
fn main() -> rscache::Result<()> {
    let cache: Cache<FileStore> = Cache::new("./data/rs3_cache")?;
    let packet = IncomingUpdatePacket{ index_id: 255, archive_id: 10, packet_id: 0 };

    let buf = if packet.index_id == 255 && packet.archive_id == 255 {
        cache.create_checksum()?.encode_rs3(EXPONENT, MODULUS)?
    } else {
        let buf = cache.read(packet.index_id, packet.archive_id)?;
        format_buffer(buf, packet.index_id)
    };

    for data_block in buf.chunks(102395) {
        let mut data = allocate_buffer(packet.index_id, packet.archive_id, data_block.len());

        encode_index_id(&mut data, packet.index_id);
        encode_archive_id(&mut data, packet.archive_id, packet.packet_id);
        if packet.index_id == 255 && packet.archive_id == 255 {
            encode_length(&mut data, buf.len() as u32);
            encode_remaining(&mut data[10..], &buf);
        } else {
            encode_remaining(&mut data[5..], &buf);
        }
        
        // write data to the client
        // stream.write_all(&data)?;

        println!("{:?}", data);
    }
 
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

fn encode_archive_id(buffer: &mut [u8], archive_id: u32, packet_id: u8) {
    // packet_id 1 means it is a priority packet, 0 means no priority.
    let archive_id = if packet_id == 0 { 
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