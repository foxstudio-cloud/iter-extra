# iter-extra

[![https://crates.io/crates/iter-extra](https://img.shields.io/crates/v/iter-extra)](https://crates.io/crates/iter-extra)
[![https://docs.rs/crate/iter-extra/latest](https://img.shields.io/docsrs/iter-extra)](https://docs.rs/crate/iter-extra/latest)

Additional iterator methods for Rust. Provides utilities for partial ordering, distance calculations, and more.

```toml
[dependencies]
iter-extra = "0.2.2"
```

```rust
use iter_extra::prelude::*;

let numbers = vec![3.2, 1.5, 2.8, 0.9];
assert_eq!(numbers.iter().min_by_partial_key(|&x| x), Some(&0.9));

let deltas = vec![1, 1, 2, 2, 3, 3, 2, 3, 4];
assert_eq!(deltas.collect::<Vec<usize>>(), vec![0, 0, 2, 0, 4, 0, 2, 1, 8]);
```

See [documentation](https://docs.rs/crate/iter-extra/latest) for all available methods.

## License

MIT - see [LICENSE](LICENSE) file for details.
