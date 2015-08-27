//! Newtype wrappers for floats.

#![deny(missing_docs)]

/// Newtypes that do not admit `NaN`.
pub mod nonnan;

/// Newtypes that explicitly set ordering for `NaN`s.
pub mod expord;
