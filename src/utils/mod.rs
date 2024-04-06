pub mod battle_record;
pub mod comp;
pub mod crypto;
pub mod game;
pub mod json;
pub mod server;

pub fn zipper<T: IntoIterator, U: IntoIterator>(a: T, b: U) -> Vec<(T::Item, U::Item)> {
    a.into_iter().zip(b).collect()
}
