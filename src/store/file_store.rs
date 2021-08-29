use std::{ 
    fs::File,
    io::{ Read, Seek, SeekFrom, BufReader },
    cell::RefCell,
    io::Write,
};

use crate::{
    arc::ArchiveRef,
    sec::{ Sector, SectorHeaderSize },
    error::ParseError,
    sec::SECTOR_SIZE,
};

use super::{ Store, ReadIntoWriter };

/// Cache inner reading using a file handle.
#[derive(Debug)]
pub struct FileStore {
    handle: RefCell<BufReader<File>>
}

impl FileStore {
    #[inline]
    fn read_internal<W: Write>(&self, archive: &ArchiveRef, writer: &mut W) -> crate::Result<()> {
        let header_size = SectorHeaderSize::from_archive(archive);
        let (header_len, data_len) = header_size.clone().into();
        let mut current_sector = archive.sector;
        let mut remaining = archive.length;
        let mut data_block = vec![0; SECTOR_SIZE];
        let mut chunk = 0;

        loop {
            let offset = current_sector as usize * SECTOR_SIZE;
            
            if remaining >= data_len {
                let mut handle = self.handle.borrow_mut();
                handle.seek(SeekFrom::Start(offset as u64))?;
                handle.read_exact(&mut data_block)?;
                
                match Sector::new(&data_block, &header_size) {
                    Ok(sector) => {
                        sector.header.validate(archive.id, chunk, archive.index_id)?;
                        current_sector = sector.header.next;
                        writer.write_all(sector.data_block)?;
                    },
                    Err(_) => return Err(ParseError::Sector(archive.sector).into())
                };

                remaining -= data_len;
            } else {
                if remaining == 0 { break; }

                let mut data_block = vec![0; remaining + header_len];

                let mut handle = self.handle.borrow_mut();
                handle.seek(SeekFrom::Start(offset as u64))?;
                handle.read_exact(&mut data_block)?;

                match Sector::new(&data_block, &header_size) {
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

impl Store for FileStore {
    #[inline]
    fn new(main_file: File) -> crate::Result<Self> {
        Ok(Self { handle: RefCell::new(BufReader::new(main_file)) })
    }

    #[inline]
    fn read(&self, archive: &ArchiveRef) -> crate::Result<Vec<u8>> {
        let mut data = Vec::with_capacity(archive.length);

        self.read_internal(archive, &mut data)?;

        Ok(data)
    }
}

impl ReadIntoWriter for FileStore {
    #[inline]
    fn read_into_writer<W: Write>(
        &self, 
        archive: &ArchiveRef, 
        writer: &mut W
    ) -> crate::Result<()> {
        self.read_internal(archive, writer)
    }
}