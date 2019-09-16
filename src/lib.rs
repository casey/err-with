//! This crate provides a single trait, `ErrWith`, with one method,
//! `err_with`. `ErrWith` is implemented for `Result<T, E>`, with
//! `result.err_with(w)` transforming an `Err(e)` to an `Err((e,w))`,
//! and leaving an `Ok(...)` unchanged.
//!
//! This is not particularly useful on its own, but can be used to
//! define conversions from `(E, W)` into your custom error types, so
//! error contexts can be recorded and reported later.
//!
//! For example:
//!
//! ```no_run
//! use std::{fs, io, path::{Path, PathBuf}};
//!
//! use err_with::ErrWith;
//!
//! // Given this custom error type:
//! #[derive(Debug)]
//! enum Error {
//!   Io { io_error: io::Error, path: PathBuf },
//! }
//!
//! // We can define a conversion from `(io::Error, AsRef<Path>)` to our
//! // custom error type:
//! impl<P: AsRef<Path>> From<(io::Error, P)> for Error {
//!     fn from((io_error, path): (io::Error, P)) -> Error {
//!         Error::Io {
//!             path: path.as_ref().to_owned(),
//!             io_error,
//!         }
//!     }
//! }
//!
//! // Which allows us to attach the path of an I/O error and convert
//! // the error into our custom error type in an ergonomic fashion:
//! fn main() -> Result<(), Error> {
//!     fs::read_to_string("foo/bar").err_with("foo/bar")?;
//!     Ok(())
//! }
//! ```
//!

pub trait ErrWith<T, E> {
  fn err_with<W>(self, with: W) -> Result<T, (E, W)>;
}

impl<T, E> ErrWith<T, E> for Result<T, E> {
  fn err_with<W>(self, with: W) -> Result<T, (E, W)> {
    match self {
      Ok(ok) => Ok(ok),
      Err(error) => Err((error, with)),
    }
  }
}
