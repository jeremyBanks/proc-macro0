# proc-macro0

A fork of [proc-macro2](https://github.com/dtolnay/proc-macro2) with mutually
exclusive feature flags for different use cases:

- **`proc-macro`** — For writing procedural macros. Wraps the compiler's
  `proc_macro` types when available.
- **`sync`** — For multi-threaded tools. All types are `Send + Sync`.

By default, neither feature is enabled, providing a basic fallback
implementation that is neither proc-macro compatible nor thread-safe.

## Features

### `proc-macro` (for procedural macros)

```toml
[dependencies]
proc-macro0 = { version = "1.0", features = ["proc-macro"] }
```

When enabled, proc-macro0 behaves identically to proc-macro2:
- Wraps real `proc_macro::TokenStream` when running inside a procedural macro
- Falls back to a pure-Rust implementation otherwise
- Types are `!Send + !Sync` (matching the compiler's types)

Use this when writing procedural macros with syn/quote.

### `sync` (for multi-threaded tools)

```toml
[dependencies]
proc-macro0 = { version = "1.0", features = ["sync"] }
```

When enabled:
- Always uses the fallback implementation (never wraps real `proc_macro`)
- Uses `Arc` instead of `Rc` for reference counting
- Uses `RwLock` instead of `RefCell` for the source map
- All types are `Send + Sync`

Use this for code analysis tools, formatters, or other multi-threaded
applications that need to parse Rust code from multiple threads.

### Why mutually exclusive?

The compiler's `proc_macro` types are `!Send + !Sync` because they use
thread-local storage. If proc-macro0 wraps these types, it cannot be
thread-safe. Enabling both features results in a compile error:

```
error: The `proc-macro` and `sync` features are mutually exclusive.
```

## Usage

For procedural macros (same as proc-macro2):

```rust
extern crate proc_macro;

#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro0::TokenStream::from(input);

    let output: proc_macro0::TokenStream = {
        /* transform input */
    };

    proc_macro::TokenStream::from(output)
}
```

For multi-threaded parsing:

```rust
use proc_macro0::TokenStream;
use std::thread;

fn main() {
    let handles: Vec<_> = sources.into_iter().map(|src| {
        thread::spawn(move || {
            src.parse::<TokenStream>().unwrap()
        })
    }).collect();

    let results: Vec<_> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
}
```

## Other Features

### `span-locations`

Expose methods `Span::start` and `Span::end` which give the line/column
location of a token.

```toml
[dependencies]
proc-macro0 = { version = "1.0", features = ["sync", "span-locations"] }
```

## Unstable features

The `procmacro2_semver_exempt` config flag enables unstable APIs:

```
RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo build
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
