mod activity;
mod character;
mod charword;
mod crisisv2;
mod display_meta;
mod equip;
mod handbook;
mod retro;
mod rogue;
mod sandbox;
mod skin;
mod stage;
mod story;
mod story_review;
mod tower;

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
pub use story::*;
pub use story_review::*;

/// The trait which defines the loading of tables.
pub trait LoadTable: Sized {
    type Err;

    fn load() -> Result<Self, Self::Err>;
}
