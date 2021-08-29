use std::{ io::{ Write, Read, }, fs::File };

use crate::{
    arc::ArchiveRef,
    sec::Sector,
    error::ParseError,
    sec::{
        SECTOR_SIZE,
        SECTOR_DATA_SIZE,
        SECTOR_HEADER_SIZE
    },
};

use super::{ Store, ReadIntoWriter };

/// Cache inner reading from memory.
#[derive(Debug)]
pub struct MemoryStore {
    data: Vec<u8>
}

impl MemoryStore {
    #[inline]
    fn read_internal<W: Write>(&self, archive: &ArchiveRef, writer: &mut W) -> crate::Result<()> {
        let mut current_sector = archive.sector;
        let mut remaining = archive.length;
        let mut chunk = 0;

        loop {
            let offset = current_sector as usize * SECTOR_SIZE;
            
            if remaining >= SECTOR_DATA_SIZE {
                let data_block = &self.data[offset..offset + SECTOR_SIZE];
                
                match Sector::from_normal_header(data_block) {
                    Ok(sector) => {
                        sector.header.validate(archive.id, chunk, archive.index_id)?;
                        current_sector = sector.header.next;
                        writer.write_all(sector.data_block)?;
                    },
                    Err(_) => return Err(ParseError::Sector(archive.sector).into())
                };
                
                remaining -= SECTOR_DATA_SIZE;
            } else {
                if remaining == 0 { break; }
                
                let data_block = &self.data[offset..offset + SECTOR_HEADER_SIZE + remaining];
                
                match Sector::from_normal_header(data_block) {
                    Ok(sector) => {
                        sector.header.validate(archive.id, chunk, archive.index_id)?;
                        writer.write_all(sector.data_block)?;
                        break;
                    },
                    Err(_) => return Err(ParseError::Sector(archive.sector).into())
                };
            }
            
            chunk += 1;
        }

        Ok(())
    }
}

impl Store for MemoryStore {
    #[inline]
    fn new(mut main_file: File) -> crate::Result<Self> {
        let mut buffer = Vec::with_capacity(main_file.metadata()?.len() as usize);
        main_file.read_to_end(&mut buffer)?;
        
        Ok(Self { data: buffer })
    }

    #[inline]
    fn read(&self, archive: &ArchiveRef) -> crate::Result<Vec<u8>> {
        let mut data = Vec::with_capacity(archive.length);

        self.read_internal(archive, &mut data)?;

        Ok(data)
    }
}

impl ReadIntoWriter for MemoryStore {
    #[inline]
    fn read_into_writer<W: Write>(
        &self, 
        archive: &ArchiveRef, 
        writer: &mut W
    ) -> crate::Result<()> {
        self.read_internal(archive, writer)
    }
}