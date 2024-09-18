//! Testing

#[cfg(test)]
mod tables {
    use std::{fs::File, io::Write};

    use crate::tables::{ActivityTable, CharacterTable, HandbookInfoTable, LoadTable, SkinTable, StageTable};

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
}
