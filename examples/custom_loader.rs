use std::{ 
    collections::HashMap,
    io::{ self, BufReader, },
};
use rscache::{ 
    Cache,
    def::osrs::{ Definition, FetchDefinition },
    ext::ReadExt,
};

fn main() -> rscache::Result<()> {
    let cache = Cache::new("./data/osrs_cache")?;
    
    let custom_ldr = CustomLoader::new(&cache)?;

    // Try to change the id here to see different item names.
    // Note: some names might not display correctly due to the parser
    // not being fully correctly implemented.
    let some_def = custom_ldr.load(1042);

    if let Some(def) = some_def {
        println!("id: {}, name: {}", def.id, def.name);
    }
    
    Ok(())
}

// Newtype defining the loader.
struct CustomLoader(HashMap<u16, CustomDefinition>);

// Your definition with all the required fields. (in this example it's just a ItemDefinition)
#[derive(Default)]
struct CustomDefinition {
    pub id: u16,
    pub name: String,
}

impl Definition for CustomDefinition {
    fn new(id: u16, buffer: &[u8]) -> rscache::Result<Self> {
        let mut reader = BufReader::new(buffer);
        let def = decode_buffer(id, &mut reader)?;

        Ok(def)
    }
}

fn decode_buffer(id: u16, reader: &mut BufReader<&[u8]>) -> io::Result<CustomDefinition> {
    // Parse the buffer into a definition.
    let mut def = CustomDefinition {
        id,
        .. CustomDefinition::default()
    };

    loop {
        let opcode = reader.read_u8()?;

        match opcode {
            // 0 indicates to stop parsing. Skipping for now because we are
            // only interested in the name.
            // 0 => break,
            2 => { def.name = reader.read_string()?; break; },
            // Skipping the rest of the buffer for the sake of the example,
            // every opcode should be parsed into values of the definition.
            _ => { if reader.buffer().len() == 0 { break; } }
            // Should normally be:
            // _ => unreachable!()
        }
    }

    Ok(def)
}

impl CustomLoader {
    fn new(cache: &Cache) -> rscache::Result<Self> {
        // Some definitions are all contained within one archive.
        // Other times one archive only contains one definition, but in most cases
        // for OSRS they are stored as multiple definitions per archive.

        let index_id = 2; // Config index.
        let archive_id = 10; // Contains all ItemDefinitions.

        let map = CustomDefinition::fetch_from_archive(cache, index_id, archive_id)?;

        Ok(Self(map))
    }

    // Simple HashMap lookup.
    fn load(&self, id: u16) -> Option<&CustomDefinition> {
        self.0.get(&id)
    }
}