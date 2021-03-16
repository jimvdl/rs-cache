use rscache::{ Cache, store::FileStore };

pub const EXPONENT: &'static [u8] = b"5206580307236375668350588432916871591810765290737810323990754121164270399789630501436083337726278206128394461017374810549461689174118305784406140446740993";
pub const MODULUS: &'static [u8] = b"6950273013450460376345707589939362735767433035117300645755821424559380572176824658371246045200577956729474374073582306250298535718024104420271215590565201";

// This example illustrates the rs3 update protocol.
// You can use this to handle client requests for cache data.
fn main() -> rscache::Result<()> {
    let cache: Cache<FileStore> = Cache::new("./data/rs3_cache")?;

    let index_id = 255;
    let archive_id = 10;
    let packet_id = 0;

    let buf = if index_id == 255 && archive_id == 255 {
        cache.create_checksum()?.encode_rs3(EXPONENT, MODULUS)?
    } else {
        let mut buf = cache.read(index_id, archive_id)?;

        if index_id != 255 {
            buf.truncate(buf.len() - 2);
            buf
        } else {
            buf
        }
    };

    // TODO: improve readability
    for data_block in buf.chunks(102395) {
        let mut data = if index_id == 255 && archive_id == 255 {
            vec![0; data_block.len() + 10]
        } else {
            vec![0; data_block.len() + 5]
        };

        data[0] = index_id;
        data[1..=4].copy_from_slice(&(if packet_id == 0 { archive_id | !0x7fffffff } else { archive_id }).to_be_bytes());
        
        if index_id == 255 && archive_id == 255 {
            data[6..=9].copy_from_slice(&(buf.len() as u32).to_be_bytes());
            data[10..].copy_from_slice(&buf);
        } else {
            data[5..].copy_from_slice(data_block);
        }
        
        // write data to the client
        // stream.write_all(&data)?;

        println!("{:?}", data);
    }
 
	Ok(())
}