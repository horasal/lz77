## LZ77 - LZ77 compress/decompress module

### Get Started

Add the following line to the `dependencies` section in your `Cargo.toml` file:

```toml
konami_lz77 = "0.2"
```

Then you are able to compress/decompress lz77 data:

```rust
use konami_lz77::{compress, decompress};

let compressed = ... // read your data here
let decompressed = decompress(compressed, &mut decompressed);
let recompressed = compress(decompressed, &mut recompressed);
```

### Documentation

[Online documentation](http://zhaihj.github.io/doc/lz77/index.html)

You can also run `cargo doc` to get a local copy of documentation.

A simple description of algorithms and basic ideas can be found [here](http://zhaihj.github.io/description-of-lz77.html). 

### LICENSE

MIT

This crate is not affiliated with Konami.
