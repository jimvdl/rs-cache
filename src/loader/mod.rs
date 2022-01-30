//! Loaders for definitions.
//!
//! # Custom loaders
//!
//! If you need a certain loader and this crate doesn't provide it you can use the below
//! example to help you make your own loader if you desperately need it.
//!
//! ```
//! use std::{collections::HashMap, io::{ self, BufReader, }};
//! use rscache::{
//!     Cache, extension::ReadExt,
//!     definition::osrs::{ Definition, FetchDefinition },
//! };
//! 
//! fn main() -> Result<(), rscache::Error> {
//!     let cache = Cache::new("./data/osrs_cache")?;
//!     let custom_loader = CustomLoader::new(&cache)?;
//!     let definition = custom_loader.load(1042);
//!
//!     if let Some(def) = definition {
//!         println!("id: {}, name: {}", def.id, def.name);
//!     }
//!
//!     Ok(())
//! }
//!
//! // Newtype defining the loader.
//! struct CustomLoader(HashMap<u16, CustomDefinition>);
//! 
//! impl CustomLoader {
//!     fn new(cache: &Cache) -> Result<Self, rscache::Error> {
//!         // Some definitions are all contained within one archive.
//!         // Other times one archive only contains one definition, but in most cases
//!         // for OSRS they are stored as multiple definitions per archive.
//!
//!         let index_id = 2; // Config index.
//!         let archive_id = 10; // Contains all ItemDefinitions.
//!
//!         let map = CustomDefinition::fetch_from_archive(cache, index_id, archive_id)?;
//!
//!         Ok(Self(map))
//!     }
//!
//!     // Simple HashMap lookup.
//!     fn load(&self, id: u16) -> Option<&CustomDefinition> {
//!         self.0.get(&id)
//!     }
//! }
//!
//! // Your definition with all the required fields. (in this example it's just a ItemDefinition)
//! #[derive(Default)]
//! struct CustomDefinition {
//!     pub id: u16,
//!     pub name: String,
//! }
//!
//! impl Definition for CustomDefinition {
//!     fn new(id: u16, buffer: &[u8]) -> Result<Self, rscache::Error> {
//!         let mut reader = BufReader::new(buffer);
//!         let def = decode_buffer(id, &mut reader)?;
//!
//!         Ok(def)
//!     }
//! }
//! 
//! fn decode_buffer(id: u16, reader: &mut BufReader<&[u8]>) -> io::Result<CustomDefinition> {
//!     // Parse the buffer into a definition.
//!     let mut def = CustomDefinition {
//!         id,
//!         .. CustomDefinition::default()
//!     };
//!
//!     loop {
//!         let opcode = reader.read_u8()?;
//!
//!         match opcode {
//!             // 0 indicates to stop parsing. Skipping for now because we are
//!             // only interested in the name.
//!             // 0 => break,
//!             2 => { def.name = reader.read_string()?; break; },
//!             // Skipping the rest of the buffer for the sake of the example,
//!             // every opcode should be parsed into values of the definition.
//!             _ => { if reader.buffer().len() == 0 { break; } }
//!             // Should normally be:
//!             // _ => unreachable!()
//!         }
//!     }
//!
//!     Ok(def)
//! }
//! ```

/// OSRS loaders.
pub mod osrs;
/// RS3 loaders.
#[cfg(feature = "rs3")]
#[cfg_attr(docsrs, doc(cfg(feature = "rs3")))]
pub mod rs3;
