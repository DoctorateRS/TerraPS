#[macro_export]
macro_rules! impl_load {
    ($table:ident, $path:ident) => {
        impl $table {
            pub fn load() -> $table {
                use std::io::Read;
                serde_json::from_str(&{
                    let mut ct = String::new();
                    let mut f = std::fs::File::open($path).unwrap();
                    f.read_to_string(&mut ct).unwrap();
                    ct
                })
                .unwrap()
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
