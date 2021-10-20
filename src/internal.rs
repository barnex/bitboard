pub use std::fmt;

#[allow(unused)]
pub type Result<T> = anyhow::Result<T>;

#[allow(unused)]
pub type SmallVec<T> = smallvec::SmallVec<[T; 32]>;
