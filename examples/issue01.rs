extern crate konami_lz77;
use konami_lz77::*;

fn main() {
    let data = [
        185, 254, 185, 254, 185, 254, 185, 254, 33, 33, 33, 43, 42, 35, 38, 42, 40, 35, 38, 40, 42,
        35, 38, 40, 42, 35, 64,
    ];
    let comp = compress(&data);
    let dec = decompress(&comp);

    //assertion raised!!!!!!!
    //assert!(comp.len() < data.len());
    println!("{} vs {}", comp.len(), data.len());
    println!("{:#?}", data);
    println!("{:#?}", comp);
    assert_eq!(data.to_vec(), dec);
}
