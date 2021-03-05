# Integer cube root

This module contains the single trait `IntegerCubeRoot` and implements it 
for primitive integer types.

Based on [integer-sqrt-rs](https://github.com/derekdreery/integer-sqrt-rs).

![ci badge](https://github.com/maurolacy/integer-cbrt-rs/workflows/Continuous%20integration/badge.svg)

## Example

```rust
// `use` trait to get functionality
use integer_cbrt::IntegerCubeRoot;

assert_eq!(8u8.integer_cbrt(), 2);
```

