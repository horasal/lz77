//! ## Description
//! LZ77 Compress/Decompress module.
//! This package contains three functions:
//!
//! ```text
//! lz77_decompress(&[u8], &mut Vec<u8>)
//! lz77_compress(&[u8], &mut Vec<u8>)
//! lz77_compress_dummy(&[u8], &mut Vec<u8>)
//! ```
//!
//! ## Example
//!
//! ```text
//! extern crate lz77;
//! use lz77::{lz77_compress, lz77_decompress};
//!
//! let data = [0u8; 50];
//! let mut comp = Vec::new();
//! lz77_compress(&data, &mut comp);
//! let mut dec = Vec::new();
//! lz77_decompress(&comp, &mut dec);
//! assert!(comp.len() < data.len());
//! assert_eq!(data.to_vec(), dec);
//! ```

pub mod lz77;

#[cfg(test)]
mod tests {
    use lz77::{lz77_compress, lz77_compress_dummy, lz77_decompress};

    #[test]
    fn test_lz77_compress() {
        {
            let data = [0u8; 50];
            let mut comp = Vec::new();
            lz77_compress(&data, &mut comp);
            let mut dec = Vec::new();
            lz77_decompress(&comp, &mut dec);
            assert!(comp.len() < data.len());
            assert_eq!(data.to_vec(), dec);
        }
        {
            let mut data = [0u8; 0x80];
            for i in 0..0x80 {
                data[i] = i as u8 % 0x10u8;
            }
            let mut comp = Vec::new();
            lz77_compress(&data, &mut comp);
            let mut dec = Vec::new();
            lz77_decompress(&comp, &mut dec);
            assert!(comp.len() < data.len());
            assert_eq!(data.to_vec(), dec);
        }
        {
            let mut data = [0u8; 0x3000];
            for i in 0..0x3000 {
                data[i] = i as u8 / 0x10u8;
            }
            let mut comp = Vec::new();
            lz77_compress(&data, &mut comp);
            let mut dec = Vec::new();
            lz77_decompress(&comp, &mut dec);
            assert!(comp.len() < data.len());
            assert_eq!(data.to_vec(), dec);
        }
    }

    #[test]
    fn test_lz77_compress_dummy() {
        {
            let data = [0u8; 50];
            let mut comp = Vec::new();
            lz77_compress_dummy(&data, &mut comp);
            let mut dec = Vec::new();
            lz77_decompress(&comp, &mut dec);
            assert_eq!(data.to_vec(), dec);
        }
        {
            let data = [0u8; 80];
            let mut comp = Vec::new();
            lz77_compress_dummy(&data, &mut comp);
            let mut dec = Vec::new();
            lz77_decompress(&comp, &mut dec);
            assert_eq!(data.to_vec(), dec);
        }
        {
            let mut data = [0u8; 0x3000];
            for i in 0..0x3000 {
                data[i] = i as u8 / 0x10u8;
            }
            let mut comp = Vec::new();
            lz77_compress_dummy(&data, &mut comp);
            let mut dec = Vec::new();
            lz77_decompress(&comp, &mut dec);
            assert_eq!(data.to_vec(), dec);
        }
    }
}
