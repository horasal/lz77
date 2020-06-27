//! ## Description
//! LZ77 Compress/Decompress module.
//! This package contains three functions:
//!
//! ```text
//! lz77_decompress(&[u8]) -> Vec<u8>
//! lz77_compress(&[u8]) -> Vec<u8>
//! lz77_compress_dummy(&[u8]) -> Vec<u8>
//! ```
//!
//! ## Example
//!
//! ```text
//! extern crate lz77;
//! use lz77::{lz77_compress, lz77_decompress};
//!
//! let data = [0u8; 50];
//! let comp = lz77_compress(&data);
//! let mut dec = Vec::new();
//! let dec = lz77_decompress(&comp);
//! assert!(comp.len() < data.len());
//! assert_eq!(data.to_vec(), dec);
//! ```

pub mod lz77;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod tests {
    use lz77::{lz77_compress, lz77_compress_dummy, lz77_decompress};

    #[test]
    fn test_lz77_compress() {
        {
            let data = [0u8; 50];
            let comp = lz77_compress(&data);
            let dec = lz77_decompress(&comp);
            assert!(comp.len() < data.len());
            assert_eq!(data.to_vec(), dec);
        }
        {
            let mut data = [0u8; 0x80];
            for (i, byte) in data.iter_mut().enumerate() {
                *byte = i as u8 % 0x10u8;
            }
            let comp = lz77_compress(&data);
            let dec = lz77_decompress(&comp);
            assert!(comp.len() < data.len());
            assert_eq!(data.to_vec(), dec);
        }
        {
            let mut data = [0u8; 0x3000];
            for (i, byte) in data.iter_mut().enumerate() {
                *byte = i as u8 % 0x10u8;
            }
            let comp = lz77_compress(&data);
            let dec = lz77_decompress(&comp);
            assert!(comp.len() < data.len());
            assert_eq!(data.to_vec(), dec);
        }
    }

    #[test]
    fn test_lz77_compress_dummy() {
        {
            let data = [0u8; 50];
            let comp = lz77_compress_dummy(&data);
            let dec = lz77_decompress(&comp);
            assert_eq!(data.to_vec(), dec);
        }
        {
            let data = [0u8; 80];
            let comp = lz77_compress_dummy(&data);
            let dec = lz77_decompress(&comp);
            assert_eq!(data.to_vec(), dec);
        }
        {
            let mut data = [0u8; 0x3000];
            for (i, byte) in data.iter_mut().enumerate() {
                *byte = i as u8 % 0x10u8;
            }
            let comp = lz77_compress_dummy(&data);
            let dec = lz77_decompress(&comp);
            assert_eq!(data.to_vec(), dec);
        }
    }

    #[quickcheck]
    fn test_lz77_compress_prop(data: Vec<u8>) -> bool {
        lz77_decompress(&lz77_compress(&data)) == data
    }

    #[quickcheck]
    fn test_lz77_compress_dummy_prop(data: Vec<u8>) -> bool {
        lz77_decompress(&lz77_compress_dummy(&data)) == data
    }
}
