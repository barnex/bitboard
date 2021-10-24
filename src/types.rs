pub use std::fmt;
pub use std::ops::Index;
pub use std::str::FromStr;

#[allow(unused)]
pub type Result<T> = anyhow::Result<T>;

pub use anyhow::format_err;

#[allow(unused)]
pub type SmVec<T> = smallvec::SmallVec<[T; 32]>;

pub type Set<T> = fnv::FnvHashSet<T>;
