//! Pre-computed tables for writing float strings.

#![doc(hidden)]

// Re-export all the feature-specific files.
pub use crate::table_large::*;
#[cfg(not(feature = "compact"))]
pub use crate::table_small::*;
