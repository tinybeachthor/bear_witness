# bear_witness

This crate provides examples of type witness in rust.

If you are familiar with the concept and just want to see a cool thing: [i18n]

> [!WARNING]
> This crate is not intended to be depended on, just to provide examples.

---

There are 3 main categories of a type witness usage:
1. trait check without type erasure ([bears])
2. lift a value into type ([auth])
3. convert a type into value ([i18n])
4. bonus: type isomorphism ([equals])

## type witness

Any program is also a proof. A simple addition calculator constructs a proof that 2 numbers can be added.
The result of the operation is also the proof. If you try dividing by `0`, the calculator won't give you a result at all.

Consider the following function:
```rust
fn first_character(s: String) -> char {
    s.chars().next().unwrap()
}
let s = "Hello".to_string();
assert_eq!(first_character(s), 'H');
```
If I call this function over some `x` and get back a [char], I will know that `x` must be a [String].
The fact that `first_character` can be called on some value proves that the value is a [String].
We actually don't even need to run the function, the type-checker can verifies this during compilation.
```rust,compile_fail
# fn first_character(s: String) -> char {
#     s.chars().next().unwrap()
# }
first_character(42);
// error: expected `String`, found integer
```
```rust,should_panic
# fn first_character(s: String) -> char {
#     s.chars().next().unwrap()
# }
// compiles because the input is indeed a [String] but panics at runtime
// sidenote: to type check this we would need a type for a non-empty String,
//           e.g. `(char, String)`
first_character("".to_string());
// panic: called `Option::unwrap()` on a `None` value
```

A type witness works similarly, we construct a type and it's existence is used to verify some property.
The main advantage is that it's all done at compile time - so no runtime overhead at all.
Is this ever useful? Sometimes.
There are some examples of type witness usage: example 2 ([auth]) is a pretty common pattern.
The most interesting one is example 3.

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
