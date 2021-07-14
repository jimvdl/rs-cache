# About

This document **(WIP)** contains specifications for the rs3 cache. 

# General Details

Main file containing all of the data: `main_file_cache.dat2`.\
Main music file containing all of the music data: `main_file_cache.dat2m`.

The cache consists of multiple indices ranging from 0 to 255.\
For example: 
- `main_file_cache.idx0`
- `main_file_cache.idx1`
- `main_file_cache.idx2`

Each index contains a list of archives.

Reference table: `main_file_cache.idx255`.

## Structs
| Structs | Size (bytes) | Variables (with size) |
| - |:-:| -:|
| Archive | 6 | `sector` - 3<br/>`length` - 3
| Sector | 520 | _See sector layout below_ |
| Sector header | 8 \| 10* | `archive_id` - 2 \| 4*<br/>`chunk` - 2<br/>`next` - 3<br/>`index_id` - 1 |
| Sector data | 512 \| 510* | `data`
\* See the `Sector layout` table for exact byte sizes.

| Sector layout | Archive id < 65535 | Archive id > 65535 |
| ------------- |:------------------:| ------------------:|
| Data size     | 512 bytes          | 510 bytes          |
| Header size   | 8 bytes            | 10 bytes           |

# Architecture

Cache instance allocation process:
 1. Load the main data file.
 2. Iterate over all indices.
 3. Load each index into a byte buffer.
 4. Load metadata of each archive contained within the current index.
 5. All the archives are parsed from this buffer.

Fetch buffer from cache:
 1. Call read on cache with `index_id` and `archive_id`.
 2. Cache does an internal hashmap lookup for the specified index id.
 3. Within this index does the same lookup for the archive id.
 4. Archive contains the starting `sector` and the `length`.
 5. Main file reader uses the fetched archive to read the corresponding data.
 6. Return the byte buffer.

Main file reading:
 1. Set read pointer to sector starting pointer from archive.
 2. Parse & validate the sector header.
 3. Copy sector data into the total byte buffer.
 4. Set the read pointer to the next sector (header of current sector contains the next sector).
 5. Increment chunk with 1.
 6. Repeat step 2 to 5 until the total byte buffer == the archive length.
