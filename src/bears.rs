//! # example 1 : trait check without type erasure
//!
//! This is a simple example of constructing a type witness without erasing type information.
//!
//! ```
//! # use bear_witness::bears::*;
//! #
//! let animal = BrownBear;
//! let certified_bear = bear_witness(animal); // -> Certified<BrownBear>
//! // we can call [Bear#growl]
//! println!("{}", certified_bear.growl());
//! // we can also call [BrownBear#do_brown_bear_things],
//! // because the [bear_witness] call didn't erase any type information
//! println!("{}", certified_bear.do_brown_bear_things());
//!
//! let animal = PolarBear;
//! let certified_bear = bear_witness(animal); // -> Certified<PolarBear>
//! // we can call [Bear#growl]
//! println!("{}", certified_bear.growl());
//! // cannot call [BrownBear#do_brown_bear_things], because this is still a [PolarBear]
//! // println!("{}", certified_bear.do_brown_bear_things());
//! ```
//!
//! But the type system won't let us call [bear_witness] on a [Dog].
//! [Dog] does not implement [Bear], so it won't pass the trait check.
//! ```compile_fail
//! # use bear_witness::bears::*;
//! #
//! let animal = Dog;
//! let certified_bear = bear_witness(animal);
//! // error: the trait `Bear` is not implemented for `Dog`
//! ```

use crate::Certified;

// Define the type witness function.

/// Type check a trait bound without erasing type information.
///
/// We wrap the return value in [Certified] to signify that
/// this value has been type-checked.
pub fn bear_witness<T: Bear>(bear: T) -> Certified<T> {
    Certified::new(bear)
}

// Define the [Bear] trait.

/// [Bear] trait, implemented on [BrownBear] and [PolarBear], but not on [Dog].
pub trait Bear {
    /// Growling is something all [Bear]s do.
    fn growl(&self) -> &str;
}

// Define some concrete types.

/// A [BrownBear], does impl [Bear].
pub struct BrownBear;
impl BrownBear {
    /// This method is defined directly on the [BrownBear] type.
    ///
    /// Using `Box<dyn Bear>` would erase the type information.
    /// We still want to be able to call this directly after verifying this is a [Bear].
    pub fn do_brown_bear_things(&self) -> &str {
        "eating loads of honey"
    }
}
impl Bear for BrownBear {
    fn growl(&self) -> &str {
        "<brown bear growl>"
    }
}

/// A [PolarBear], does impl [Bear].
pub struct PolarBear;
impl Bear for PolarBear {
    fn growl(&self) -> &str {
        "<menacing polar bear growl>"
    }
}

/// A [Dog], so it does not impl [Bear].
pub struct Dog;
