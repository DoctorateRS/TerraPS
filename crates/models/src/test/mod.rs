//! Testing

#[cfg(test)]
mod other;

#[cfg(test)]
mod tables {
    use std::{fs::File, io::Write};

    use crate::tables::*;

    use common_utils::print_json;

    #[test]
    fn test_char_table() {
        let table = match CharacterTable::load() {
            Ok(t) => t,
            Err(e) => {
                panic!("failed to load: {}", e);
            }
        };

        let mut f = File::create("../../test/tables/chars.json").unwrap();

        let _ = f.write_all(print_json(table).unwrap().as_bytes());
    }

    #[test]
    fn test_act_table() {
        let table = match ActivityTable::load() {
            Ok(t) => t,
            Err(e) => {
                panic!("failed to load: {}", e);
            }
        };

        let mut f = File::create("../../test/tables/acts.json").unwrap();

        let _ = f.write_all(print_json(table).unwrap().as_bytes());
    }

    #[test]
    fn test_skin_table() {
        let table = match SkinTable::load() {
            Ok(t) => t,
            Err(e) => {
                panic!("failed to load: {}", e);
            }
        };

        let mut f = File::create("../../test/tables/skins.json").unwrap();

        let _ = f.write_all(print_json(table).unwrap().as_bytes());
    }

    #[test]
    fn test_stage_table() {
        let table = match StageTable::load() {
            Ok(t) => t,
            Err(e) => {
                panic!("failed to load: {}", e);
            }
        };

        let mut f = File::create("../../test/tables/stages.json").unwrap();

        let _ = f.write_all(print_json(table).unwrap().as_bytes());
    }

    #[test]
    fn test_crisisv2_table() {
        let table = match CrisisV2Table::load() {
            Ok(t) => t,
            Err(e) => {
                panic!("failed to load: {}", e);
            }
        };

        let mut f = File::create("../../test/tables/crisis_v2.json").unwrap();

        let _ = f.write_all(print_json(table).unwrap().as_bytes());
    }

    #[test]
    fn test_handbook_info_table() {
        let table = match HandbookInfoTable::load() {
            Ok(t) => t,
            Err(e) => {
                panic!("failed to load: {}", e);
            }
        };

        let mut f = File::create("../../test/tables/handbookinfos.json").unwrap();

        let _ = f.write_all(print_json(table).unwrap().as_bytes());
    }

    #[test]
    fn test_enemy_handbook_table() {
        let table = match HandbookEnemyTable::load() {
            Ok(t) => t,
            Err(e) => {
                panic!("failed to load: {}", e);
            }
        };

        let mut f = File::create("../../test/tables/handbookenemies.json").unwrap();

        let _ = f.write_all(print_json(table).unwrap().as_bytes());
    }

    #[test]
    fn test_equip_table() {
        let table = match EquipTable::load() {
            Ok(t) => t,
            Err(e) => {
                panic!("failed to load: {}", e);
            }
        };

        let mut f = File::create("../../test/tables/equips.json").unwrap();

        let _ = f.write_all(print_json(table).unwrap().as_bytes());
    }

    #[test]
    fn test_battle_equip_table() {
        let table = match BattleEquipTable::load() {
            Ok(t) => t,
            Err(e) => {
                panic!("failed to load: {}", e);
            }
        };

        let mut f = File::create("../../test/tables/battle_equips.json").unwrap();

        let _ = f.write_all(print_json(table).unwrap().as_bytes());
    }

    #[test]
    fn test_retro_table() {
        let table = match RetroTable::load() {
            Ok(t) => t,
            Err(e) => {
                panic!("failed to load: {}", e);
            }
        };

        let mut f = File::create("../../test/tables/retro.json").unwrap();

        let _ = f.write_all(print_json(table).unwrap().as_bytes());
    }

    #[test]
    fn test_display_meta_table() {
        let table = match DisplayMetaTable::load() {
            Ok(t) => t,
            Err(e) => {
                panic!("failed to load: {}", e);
            }
        };

        let mut f = File::create("../../test/tables/display_meta.json").unwrap();

        let _ = f.write_all(print_json(table).unwrap().as_bytes());
    }

    #[test]
    fn test_story_table() {
        let table = match StoryTable::load() {
            Ok(t) => t,
            Err(e) => {
                panic!("failed to load: {}", e);
            }
        };

        let mut f = File::create("../../test/tables/story.json").unwrap();

        let _ = f.write_all(print_json(table).unwrap().as_bytes());
    }

    #[test]
    fn test_story_review_table() {
        let table = match StoryReviewTable::load() {
            Ok(t) => t,
            Err(e) => {
                panic!("failed to load: {}", e);
            }
        };

        let mut f = File::create("../../test/tables/story_review.json").unwrap();

        let _ = f.write_all(print_json(table).unwrap().as_bytes());
    }

    #[test]
    fn test_story_review_meta_table() {
        let table = match StoryReviewMetaTable::load() {
            Ok(t) => t,
            Err(e) => {
                panic!("failed to load: {}", e);
            }
        };

        let mut f = File::create("../../test/tables/story_review_meta.json").unwrap();

        let _ = f.write_all(print_json(table).unwrap().as_bytes());
    }
}
