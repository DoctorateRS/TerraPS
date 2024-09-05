//! Testing

#[cfg(test)]
mod tables {
    use std::{fs::File, io::Write};

    use crate::tables::CharacterTable;

    use common_utils::print_json;

    #[test]
    fn test_char_table() {
        let table = match CharacterTable::load() {
            Ok(t) => t,
            Err(e) => {
                let e = e.to_string();
                println!("{}", e);
                return;
            }
        };

        let mut f = File::create("F:/TerraPS/test/tables/chars.json").unwrap();

        f.write_all(print_json(table).unwrap().as_bytes());
    }
}
