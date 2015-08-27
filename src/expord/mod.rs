//! Explicit ordering for `NaN`s in floats.

/// Order `NaN`s less than all other floats.
pub mod less;

/// Order `NaN`s greater than all other floats.
pub mod greater;
