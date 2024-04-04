pub mod comp;
pub mod game;
pub mod json;

pub fn contains<T: PartialEq>(val: &T, vec: Vec<T>) -> bool {
    for item in vec {
        if item == *val {
            return true;
        }
    }
    false
}

pub fn zipper<T: IntoIterator, U: IntoIterator>(a: T, b: U) -> Vec<(T::Item, U::Item)> {
    a.into_iter().zip(b).collect()
}
