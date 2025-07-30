# iter-extra

A Rust crate providing additional iterator methods for working with partial ordering, particularly useful when dealing with floating-point numbers and other types that implement `PartialOrd` but not `Ord`.

## Features

- `min_by_partial_key` - Find the minimum element using a key function with partial ordering
- `max_by_partial_key` - Find the maximum element using a key function with partial ordering
- Handles NaN values gracefully by treating them as equal during comparisons
- Works with any iterator type through a blanket implementation
- Gradually extended with things we don't want to re-implement

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
iter-extra = "0.1.0"
```

## Usage

```rust
use iter_extra::prelude::*;

// Basic usage with floating-point numbers
let numbers = vec![3.2, 1.5, 2.8, 0.9];
assert_eq!(numbers.iter().min_by_partial_key(|&x| x), Some(&0.9));

let max = numbers.iter().max_by_partial_key(|&x| x);
assert_eq!(numbers.iter().max_by_partial_key(|&x| x), Some(&3.2));

// Works with NaN values
let with_nan = vec![1.0, f64::NAN, 2.0, 0.5];
let min = with_nan.iter().min_by_partial_key(|&x| x);
assert_eq!(min, Some(&0.5));
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
