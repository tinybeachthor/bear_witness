# bear_witness

This crates provides examples of type witness in rust.

> [!WARNING]
> This crate is not intended to be depended on, just to provide examples.

There are 3 main categories of a type witness usage:
1. trait check without type erasure ([bears])
2. lift a value into type ([auth])
3. convert a type into value
4. bonus: type isomorphism ([equals])

## example 1 : trait check without type erasure

This example is implemented in the [bears] module.

Let's say we have some concrete types: `BrownBear`, `PolarBear`, and `Dog`.
And a `Bear` trait implemented on `BrownBear` and `PolarBear`.

```rust
struct BrownBear;
impl BrownBear {
    fn do_brown_bear_things(&self) -> &str {
        "< eating lots of honey, yum! >"
    }
}
struct PolarBear;
struct Dog;

trait Bear {
    fn growl(&self) -> &str {
        "< bear growling >"
    }
}
impl Bear for BrownBear {}
impl Bear for PolarBear {}
```

We want to be sure our animal is a `Bear`, but we don't want to erase the concrete type.
Wrapping in `Box<dyn Bear>` won't work, we wouldn't be able to call `BrownBear::do_brown_bear_things` on it.

Instead we will construct a type witness: forcing a type check on the `Bear` trait, but returning the same type.
```rust
# use bear_witness::bears::*;
#
fn witness<T: Bear>(bear: T) -> T {
    bear
}
let animal = BrownBear;
let bear = witness(animal);
```

This checks out, but we don't retain any information about having checked the type.
```rust
# use bear_witness::bears::*;
#
# fn witness<T: Bear>(bear: T) -> T {
#     bear
# }
#
fn certified_only<T: Bear>(bear: T) -> bool {
    true
}
assert!(certified_only(witness(BrownBear)));
assert!(certified_only(BrownBear)); // yikes, this one was not verified
```

We want to tag the type with something to show we have checked it.
We can use a simple transparent wrapper [Certified] for this.
```rust
# use bear_witness::bears::*;
# use bear_witness::Certified;
#
fn certified_witness<T: Bear>(bear: T) -> Certified<T> {
    Certified::new(bear)
}
let animal = BrownBear;
let bear = certified_witness(animal); // -> Certified<BrownBear>

fn certified_only_fixed<T: Bear>(bear: Certified<T>) -> bool {
    true
}
assert!(certified_only_fixed(certified_witness(BrownBear)));
// assert!(certified_only_fixed(BrownBear)); // does not compile anymore
```
