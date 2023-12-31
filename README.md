[![ci](https://github.com/so-schen/serde-bytes-ng/actions/workflows/ci.yml/badge.svg)](https://github.com/so-schen/serde-bytes-ng/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/serde_bytes_ng.svg)](https://crates.io/crates/serde_bytes_ng)

# From `serde_bytes` v0.11.13, array of any size is supported.

# crate DEPRECATED

## serde-bytes-ng
Integrate [`serde_bytes`](https://crates.io/crates/serde_bytes) and generic byte array support.

The goal is the contribute to upstream `serde_bytes`, but because lack of response from its author, 
create a separate crate with generic byte array support plus latest `serde_bytes` codes so that can
be released as one crate.

### Example

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Efficient<'a> {
    #[serde(with = "serde_bytes_ng")]
    bytes: &'a [u8],

    #[serde(with = "serde_bytes_ng")]
    byte_buf: Vec<u8>,

    #[serde(with = "serde_bytes_ng")]
    byte_array: [u8; 188],

    #[serde(with = "serde_bytes_ng", borrow)]
    byte_array_ref: &'a [u8; 188],

}
```
