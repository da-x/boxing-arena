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
