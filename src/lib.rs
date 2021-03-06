//! Basic linear and vector geometry for two-dimensional Euclidean geometry

#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    clippy::doc_markdown,
    clippy::unimplemented
)]
#![allow(clippy::result_unit_err)]

mod aabb;
pub use self::aabb::*;

mod radians;
pub use self::radians::*;

mod polygon;
pub use self::polygon::*;

mod vector;
pub use self::vector::*;

mod point;
pub use self::point::*;

mod convex_hull;
pub use self::convex_hull::*;

mod intersects;
pub use self::intersects::*;
