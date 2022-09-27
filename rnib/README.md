# Overview

`rnib` crate provides Rust wrappers for `protobuf` modules defined in `ric-plt/nodeb-rnib` repository.

## Getting Started

To get started with using `rnib` add a dependency to this crate and start using the definitions defined in this crate.
For example

```rust

use rnib;

let cell = rnib::entitities::Cell::new();
let _ = cell.encode();

```

