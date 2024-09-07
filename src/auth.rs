//! # example 2 : lift a value into type
//!
//! This is a pretty common usage.
//!
//! Constructing the [Auth] enum is done based on the [Session::user_id] value.
//! We are lifting a concrete value into type.
//!
//! ```rust
//! enum Auth<T> {
//!     Admin(T),
//!     User(T),
//! }
//!
//! struct Session {
//!     user_id: u32,
//! }
//! impl Session {
//!     fn new(user_id: u32) -> Self {
//!         Self { user_id }
//!     }
//!     fn auth(self) -> Auth<Self> {
//!         if self.user_id == 0 {
//!             Auth::Admin(self)
//!         } else {
//!             Auth::User(self)
//!         }
//!     }
//! }
//!
//! let session = Session::new(0);
//! assert!(matches!(session.auth(), Auth::Admin(session)));
//! let session = Session::new(1000);
//! assert!(matches!(session.auth(), Auth::User(session)));
//! ```

/// Route handler.
///
/// 1. get current session
/// 2. authenticate
/// 3. get_admin_page()
pub fn handler() -> Result<String, String> {
    let session = Session { user_id: 0 };
    let auth = authenticate(session);
    get_admin_page(&auth)
}

/// User session, identified by `user_id`.
pub struct Session { pub user_id: u32 }

/// Type witness for the [Session::user_id] value.
pub enum Auth<T> {
    /// [Auth::Admin] <=> ([Session::user_id] == 0)
    Admin(T),
    /// [Auth::User] <=> ([Session::user_id] != 0)
    User(T),
}

/// Authenticate a user [Session], the returned [Auth] is
/// a type witness for the [Session::user_id].
///
/// [Auth::Admin] <=> ([Session::user_id] == 0)
pub fn authenticate(session: Session) -> Auth<Session> {
    if session.user_id == 0 {
        Auth::Admin(session)
    } else {
        Auth::User(session)
    }
}

/// Return the admin page for [Auth::Admin], or 404.
pub fn get_admin_page(auth: &Auth<Session>) -> Result<String, String> {
    if let Auth::Admin(_) = auth {
        Ok("<html>admin</html>".to_string())
    } else {
        Err("404".to_string())
    }
}
