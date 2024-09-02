use crate::{
    cnst::table_paths::{ACTIVITY_TABLE_PATH, CHARACTER_TABLE_PATH},
    models::tables::{activity::ActivityTable, character::CharacterTable},
};

#[test]
fn activity() {
    dbg!(ActivityTable::load());
}

#[test]
fn character() {
    dbg!(CharacterTable::load());
}
