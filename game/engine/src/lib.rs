//! # ENGINE
//!
//! `engine` это движок игры, в нём содержаться различные модули игры, такие как
//! `sounds`, `physics`, `graphics`, а также некоторые полезные функции
pub mod graphics;
pub mod physics;
pub mod sounds;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
