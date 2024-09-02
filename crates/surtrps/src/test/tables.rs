use crate::models::tables::{ActivityTable, CharacterTable};

#[test]
fn activity() {
    dbg!(ActivityTable::load());
}

#[test]
fn character() {
    dbg!(CharacterTable::load());
}
