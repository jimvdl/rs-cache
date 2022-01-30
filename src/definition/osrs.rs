#[allow(clippy::too_many_lines)]
mod item_def;
mod loc_def;
mod map_def;
mod npc_def;
#[allow(clippy::too_many_lines)]
mod obj_def;

pub use item_def::*;
pub use loc_def::*;
pub use map_def::*;
pub use npc_def::*;
pub use obj_def::*;

use std::collections::HashMap;

use crate::Cache;
use runefs::{ArchiveFileGroup, IndexMetadata, REFERENCE_TABLE_ID};

/// Marker trait for definitions.
pub trait Definition: Sized {
    fn new(id: u16, buffer: &[u8]) -> crate::Result<Self>;
}

/// Adds definition fetching from the cache to every struct that implements `Definition`.
///
/// The main difference between `fetch_from_index` and `fetch_from_archive`:
/// - `fetch_from_index` will get only 1 definition from each archive making it a 1:1 relation.
/// - `fetch_from_archive` will get multiple definitions from each archive making it a N:1 relation
/// where N is atleast 1.
pub trait FetchDefinition: Definition {
    // TODO: finish documentation with example.
    /// Fetches multiple definitions from every archive in the index.
    ///
    /// Note: every archive contains only one definition. (1:1)
    ///
    /// # Errors
    ///
    /// Can return multiple errors: if reading, decoding or parsing definition buffers fail.
    fn fetch_from_index<D>(cache: &Cache, index_id: u8) -> crate::Result<HashMap<u16, D>>
    where
        D: Definition,
    {
        let buffer = cache.read(REFERENCE_TABLE_ID, index_id as u32)?.decode()?;
        let archives = IndexMetadata::try_from(buffer)?;
        let mut definitions = HashMap::new();
        for archive in &archives {
            let buffer = cache.read(index_id, archive.id)?.decode()?;

            definitions.insert(archive.id as u16, D::new(archive.id as u16, &buffer)?);
        }

        Ok(definitions)
    }

    /// Fetches multiple definitions from a single archive.
    ///
    /// Note: every archive contains multiple definitions. (N:1)
    ///
    /// # Errors
    ///
    /// Can return multiple errors: if reading, decoding or parsing definition buffers fail.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use rscache::Cache;
    /// use rscache::definition::osrs::{
    ///     FetchDefinition,
    ///     ItemDefinition,
    /// };
    ///
    /// # fn main() -> rscache::Result<()> {
    /// # let cache = Cache::new("./data/osrs_cache")?;
    /// let index_id = 2; // Config index.
    /// let archive_id = 10; // Archive containing item definitions.
    ///
    /// let item_defs: HashMap<u16, ItemDefinition>
    ///     = ItemDefinition::fetch_from_archive(&cache, index_id, archive_id)?;
    /// # Ok(())
    /// # }
    /// ```
    fn fetch_from_archive<D>(
        cache: &Cache,
        index_id: u8,
        archive_id: u32,
    ) -> crate::Result<HashMap<u16, D>>
    where
        D: Definition,
    {
        let buffer = cache.read(REFERENCE_TABLE_ID, index_id as u32)?.decode()?;
        let archives = IndexMetadata::try_from(buffer)?;
        let entry_count = archives[archive_id as usize - 1].entry_count;
        let buffer = cache.read(index_id, archive_id)?.decode()?;

        let archive_group = ArchiveFileGroup::from_buffer(&buffer, entry_count);

        let mut definitions = HashMap::new();
        for archive_file in archive_group {
            definitions.insert(
                archive_file.id as u16,
                D::new(archive_file.id as u16, &archive_file.data)?,
            );
        }

        Ok(definitions)
    }
}

impl<D: Definition> FetchDefinition for D {}
