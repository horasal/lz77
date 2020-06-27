//! ## Description
//! LZ77 Compress/Decompress module.
//! This package contains three functions:
//!
//! ```text
//! decompress(&[u8]) -> Vec<u8>
//! compress(&[u8]) -> Vec<u8>
//! compress_dummy(&[u8]) -> Vec<u8>
//! ```
//!
//! ## Example
//!
//! ```text
//! extern crate lz77;
//! use lz77::{compress, decompress};
//!
//! let data = [0u8; 50];
//! let comp = compress(&data);
//! let mut dec = Vec::new();
//! let dec = decompress(&comp);
//! assert!(comp.len() < data.len());
//! assert_eq!(data.to_vec(), dec);
//! ```

mod lz77;
pub use lz77::{compress, compress_dummy, decompress};

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        {
            let data = [0u8; 50];
            let comp = compress(&data);
            let dec = decompress(&comp);
            assert!(comp.len() < data.len());
            assert_eq!(data.to_vec(), dec);
        }
        {
            let mut data = [0u8; 0x80];
            for (i, byte) in data.iter_mut().enumerate() {
                *byte = i as u8 % 0x10u8;
            }
            let comp = compress(&data);
            let dec = decompress(&comp);
            assert!(comp.len() < data.len());
            assert_eq!(data.to_vec(), dec);
        }
        {
            let mut data = [0u8; 0x3000];
            for (i, byte) in data.iter_mut().enumerate() {
                *byte = i as u8 % 0x10u8;
            }
            let comp = compress(&data);
            let dec = decompress(&comp);
            assert!(comp.len() < data.len());
            assert_eq!(data.to_vec(), dec);
        }
    }

    #[test]
    fn test_compress_dummy() {
        {
            let data = [0u8; 50];
            let comp = compress_dummy(&data);
            let dec = decompress(&comp);
            assert_eq!(data.to_vec(), dec);
        }
        {
            let data = [0u8; 80];
            let comp = compress_dummy(&data);
            let dec = decompress(&comp);
            assert_eq!(data.to_vec(), dec);
        }
        {
            let mut data = [0u8; 0x3000];
            for (i, byte) in data.iter_mut().enumerate() {
                *byte = i as u8 % 0x10u8;
            }
            let comp = compress_dummy(&data);
            let dec = decompress(&comp);
            assert_eq!(data.to_vec(), dec);
        }
    }

    #[quickcheck]
    fn test_compress_prop(data: Vec<u8>) -> bool {
        decompress(&compress(&data)) == data
    }

    #[quickcheck]
    fn test_compress_dummy_prop(data: Vec<u8>) -> bool {
        decompress(&compress_dummy(&data)) == data
    }
}
