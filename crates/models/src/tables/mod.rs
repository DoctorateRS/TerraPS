mod activity;
mod character;
mod charword;
mod crisisv2;
mod display_meta;
mod equip;
mod handbook;
mod retro;
mod skin;
mod stage;

pub use activity::*;
pub use character::*;
pub use charword::*;
pub use crisisv2::*;
pub use display_meta::*;
pub use equip::*;
pub use handbook::*;
pub use retro::*;
pub use skin::*;
pub use stage::*;

/// The trait which defines the loading of tables.
pub trait LoadTable: Sized {
    type Err;

    fn load() -> Result<Self, Self::Err>;
}
