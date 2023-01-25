//! # ENGINE
//!
//! `engine` это движок игры, в нём содержаться различные модули игры, такие как
//! `sounds`, `physics`, `graphics`, а также некоторые полезные функции
pub mod graphics;
pub mod physics;
pub mod sounds;
pub mod rect;
pub mod time;
pub mod component;
pub mod unit;
pub mod group;
pub mod traits;
pub mod loader;
pub mod programs;

pub use rect::Rect;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
}
