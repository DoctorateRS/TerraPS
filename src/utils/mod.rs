pub mod battle_data;
pub mod battle_replay;
pub mod comp;
pub mod crypto;
pub mod game;
pub mod json;
pub mod mail;
pub mod random;
pub mod rlutils;
pub mod server;

pub fn zipper<T: IntoIterator, U: IntoIterator>(a: T, b: U) -> Vec<(T::Item, U::Item)> {
    a.into_iter().zip(b).collect()
}

pub fn enumerate<T: IntoIterator>(a: T) -> Vec<(usize, T::Item)> {
    a.into_iter().enumerate().collect()
}
