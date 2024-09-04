#[macro_export]
macro_rules! impl_load {
    ($table:ident, $ct:ident) => {
        impl $table {
            pub fn load() -> $table {
                serde_json::from_str($ct).unwrap()
            }
        }
    };
}

mod activity;
mod character;
mod crisisv2;
mod stage;

pub use activity::*;
pub use character::*;
