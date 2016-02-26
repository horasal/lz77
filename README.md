## LZ77 - LZ77 compress/decompress module

### Get Started

Add the following line to the `dependencies` section in your `Cargo.toml` file:

```toml
lz77 = "0.1"
```

Then you are able to compress/decompress lz77 data:

```rust
use lz77::{lz77_compress, lz77_decompress};

let mut decompressed= Vec::new();
lz77_decompress(compressed, &mut decompressed);
let mut recompressed  = Vec::new();
lz77_compress(decompressed, &mut recompressed);
```

### Documentation

[Online documentation](http://zhaihj.github.io/doc/lz77/index.html)

You can also run `cargo doc` to get a local copy of documentation.

### LICENSE

MIT
