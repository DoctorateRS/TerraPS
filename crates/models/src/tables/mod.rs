mod activity;
mod character;
mod charword;
mod crisisv2;
mod stage;

pub use activity::*;
pub use character::*;
pub use charword::*;
pub use crisisv2::*;
pub use stage::*;

pub trait LoadTable: Sized {
    type Err;

    fn load() -> Result<Self, Self::Err>;
}
