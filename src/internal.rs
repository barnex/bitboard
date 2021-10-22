pub use std::fmt;

#[allow(unused)]
pub type Result<T> = anyhow::Result<T>;

pub use anyhow::format_err;

#[allow(unused)]
pub type SmallVec<T> = smallvec::SmallVec<[T; 32]>;

pub type Set<T> = fnv::FnvHashSet<T>;