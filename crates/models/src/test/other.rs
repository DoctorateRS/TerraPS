use std::{fs::File, io::Write};

use common_utils::print_json;

use crate::mail::MailManager;

#[test]
fn test_mail() {
    let mails = MailManager::load().unwrap();

    let mut f = File::create("../../test/mails.json").unwrap();
    let _ = f.write_all(print_json(mails).unwrap().as_bytes());
}
