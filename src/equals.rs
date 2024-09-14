//! example 4 : type equality
//!
//! Two types are equal if we can define an isomorphism over them.
//! Any value of the first type has to be representable as the other type and vice-versa.
//!
//! ## Type equality of `String` <-> `Result<String, Infallible>`
//!
//! ```
//! # pub use std::convert::Infallible;
//! #
//! /// Type equality proof by construction.
//! trait Equals {
//!     /// [Equals::Target] has to also impl [Equals].
//!     type Target: Equals<Target = Self>;
//!
//!     fn iso(self) -> Self::Target;
//! }
//!
//! impl Equals for Result<String, Infallible> {
//!     type Target = String;
//!
//!     fn iso(self) -> Self::Target {
//!         match self {
//!             Ok(string) => string,
//!             Err(never) => match never {},
//!         }
//!     }
//! }
//! impl Equals for String {
//!     type Target = Result<String, Infallible>;
//!
//!     fn iso(self) -> Self::Target {
//!         Ok(self)
//!     }
//! }
//!
//! let a = String::from("isomorphic");
//! let b = Equals::iso(a.clone());
//! assert_eq!(Equals::iso(b), a);
//! ```
//!
//! This will work, but we can only implement the trait once.
//!
//! A better way to do it is to implement a trait generic over 2 types.
//!
//! ## Equality witness trait
//!
//! ```
//! trait EqualsWitness<A, B> {
//!     fn is_iso() {}
//! }
//! impl<A, B> EqualsWitness<A, B> for A
//! where
//!     A: From<B>,
//!     B: From<A>,
//! {}
//! // This impl also covers identity A <-> A, because types impl `From<Self>`.
//! ```
//!
//! If we have an implentation of `EqualsWitness`, it is our type witness for the
//! equality between these 2 types.
//! We can simply check if the `is_iso` can be called.
//!
//! ```
//! # use bear_witness::equals::EqualsWitness;
//! // pair and array of size 2
//! <[u8; 2] as EqualsWitness::<[u8; 2], (u8, u8)>>::is_iso();
//!
//! // any type is isomophic with itself
//! <u32 as EqualsWitness::<u32, u32>>::is_iso();
//! <&str as EqualsWitness::<&str, &str>>::is_iso();
//! <Vec<[u8; 10]> as EqualsWitness::<Vec<[u8; 10]>, Vec<[u8; 10]>>>::is_iso();
//! ```
//!
//! ```compile_fail
//! # use bear_witness::equals::EqualsWitness;
//! <u32 as EqualsWitness::<u32, String>>::is_iso();
//! // the trait `From<String>` is not implemented for `u32`, which is required by `u32: EqualsWitness<u32, String>`
//! ```

/// Type equality witness trait
pub trait EqualsWitness<A, B> {
    /// Are the types isomorphic?
    fn is_iso() {}
}
impl<A, B> EqualsWitness<A, B> for A
where
    A: From<B>,
    B: From<A>,
{
}
