#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod auth;
pub mod bears;
pub mod equals;
pub mod i18n;

/// A simple transparent wrapper.
///
/// We use it as a certificate of a successful type-check.
/// Return it from a `witness` function, proving we have type-checked the value.
pub struct Certified<T>(T);
impl<T> Certified<T> {
    /// Create a new [Certified] value.
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T> std::ops::Deref for Certified<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for Certified<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Certified(self.0.clone())
    }
}
impl<T> Copy for Certified<T> where T: Copy {}
