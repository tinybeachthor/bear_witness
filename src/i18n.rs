//! # example 3 : type into value
//!
//! We have a template [Context] we want localize and render.
//! ```
//! struct Context {
//!     pub who: String,
//! }
//!
//! enum Language {
//!     English,
//!     French,
//!     German,
//! }
//! ```
//!
//! We could simply define a render method and match on the [Language]:
//! ```should_panic
//! # use bear_witness::i18n::*;
//! #
//! fn render(context: Context, language: Language) -> String {
//!     match language {
//!         Language::English => format!("Hello {}", context.who),
//!         Language::French => format!("Bonjour {}", context.who),
//!         _ => unimplemented!(),
//!     }
//! }
//!
//! let context = Context { who: "World".to_string() };
//! render(context, Language::German);
//! // panic: not implemented
//! ```
//!
//! If we don't support all the languages, we will have to silently fallback or return a `Result`.
//! But we should be able to type check this, since this is all static information, adding a new
//! language or translation would involve a recompilation anyway.
//!
//! ## [Localize] trait
//!
//! Define concrete types so we can type check against these.
//! ```
//! trait TypedLang {}
//!
//! struct English;
//! impl TypedLang for English {}
//! struct French;
//! impl TypedLang for French {}
//! struct German;
//! impl TypedLang for German {}
//! ```
//! > sidenote: once `const generics` work over enums, we won't need this
//!
//! Define [Localized] wrapper for a value.
//! ```
//! # use bear_witness::i18n::*;
//! #
//! enum Localized<T> {
//!     English(T),
//!     French(T),
//!     German(T),
//! }
//!
//! trait Localize<L: TypedLang> {
//!     fn localize(self, lang: L) -> Localized<Self>
//!         where Self: Sized;
//! }
//! ```
//!
//! ## Wire it all together
//!
//! Impl [Localize] for languages we support.
//! ```
//! # use bear_witness::i18n::*;
//! #
//! # struct Context {
//! #     pub who: String,
//! # }
//! impl Localize<English> for Context {
//!     fn localize(self, _lang: English) -> Localized<Self> {
//!         Localized::English(self)
//!     }
//! }
//! impl Localize<German> for Context {
//!     fn localize(self, _lang: German) -> Localized<Self> {
//!         Localized::German(self)
//!     }
//! }
//! ```
//!
//!
//! ## The panic becomes a type error
//!
//! ```
//! # use bear_witness::i18n::*;
//! #
//! fn render(localized: Localized<Context>) -> String {
//!     match localized {
//!         Localized::English(context) => format!("Hello {}", context.who),
//!         Localized::French(context) => format!("Bonjour {}", context.who),
//!         _ => unimplemented!(),
//!     }
//! }
//!
//! let context = Context { who: "World".to_string() };
//! assert_eq!(render(context.localize(English)), "Hello World");
//! ```
//!
//! ```compile_fail
//! # use bear_witness::i18n::*;
//! #
//! # fn render(localized: Localized<Context>) -> String {
//! #     match localized {
//! #         Localized::English(context) => format!("Hello {}", context.who),
//! #         Localized::French(context) => format!("Bonjour {}", context.who),
//! #         _ => unimplemented!(),
//! #     }
//! # }
//! #
//! # let context = Context { who: "World".to_string() };
//! render(context.localize(German));
//! // error: the trait `Localize<German>` is not implemented for `Context`
//! ```

/// The context for rendering localized message.
pub struct Context {
    /// Who do we want to greet?
    pub who: String,
}

/// Language enum.
pub enum Language {
    /// English
    English,
    /// French
    French,
    /// German
    German,
}

/// Typed language trait, so we can pass a typed language to functions.
pub trait TypedLang {}

/// Typed English
pub struct English;
impl TypedLang for English {}
/// Typed French
pub struct French;
impl TypedLang for French {}
/// Typed German
pub struct German;
impl TypedLang for German {}

/// Localized value into a language.
pub enum Localized<T: Sized> {
    /// English
    English(T),
    /// French
    French(T),
    /// German
    German(T),
}

/// Localize -> [Localized]
pub trait Localize<L: TypedLang> {
    /// Turn a value into [Localized] for a given language.
    fn localize(self, lang: L) -> Localized<Self>
    where
        Self: Sized;
}

// impl Localize for Context

impl Localize<English> for Context {
    fn localize(self, _lang: English) -> Localized<Self> {
        Localized::English(self)
    }
}
impl Localize<French> for Context {
    fn localize(self, _lang: French) -> Localized<Self> {
        Localized::French(self)
    }
}
