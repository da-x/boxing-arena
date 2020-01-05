# boxing-arena &emsp; [![Build Status]][travis] [![Latest Version]][crates.io] [![Docs badge]][Docs link] [![License badge]][License link]

[Build Status]: https://api.travis-ci.org/da-x/boxing-arena.svg?branch=master
[travis]: https://travis-ci.org/da-x/boxing-arena
[Latest Version]: https://img.shields.io/crates/v/boxing-arena.svg
[crates.io]: https://crates.io/crates/boxing-arena
[License badge]: https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg
[License link]: https://travis-ci.org/da-x/boxing-arena
[Docs badge]: https://docs.rs/boxing-arena/badge.svg
[Docs link]: https://docs.rs/boxing-arena

The `boxing-arena` crate provides a very simply reuse of `Box` allocation by
keeping a vector a reusable `Box` allocations, that can be used when wanting to
wrap a value in `Box`.

It would be sometimes easier to introduce `boxing-arena` in code bases where
`Box` is already used extensively than to use an arena crate that affects type
and life-time semantics more drastically.

Basic usage demonstration:

```rust
// Prepare an long-lived arena:
let mut ba = BoxingArena::new();

// ... per allocation ... :

// In place of using `Box::new` directly, we do:
let boxed_big_value = ba.rebox(big_value);

// Instead of letting Rust drop and deallocate the Box, we do:
ba.unbox(boxed_big_value);
```

## License

`boxing-arena` is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `boxing-arena` by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
