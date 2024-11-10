use std::{collections::HashMap, fs::exists};

use axum::{http::HeaderMap, response::IntoResponse, Json};
use common_utils::{time, write_json};
use reqwest::StatusCode;
use uuid::Uuid;

use anyhow::Result;

use models::{
    account::{
        character::chara::{Char as Chara, Equip, Skill, VoiceLan},
        sync::{AccountLogin, AccountSyncStatus},
        User, UserData,
    },
    tables::{BattleEquipTable, CharType, CharWordTable, CharacterTable, CharmTable, DisplayMetaTable, EquipTable, LoadTable, RetroTable, SkinTable},
    MiscResponse, BATCH_EVENT,
};

use crate::{cnst::user::USER_JSON_PATH, USER_CONFIG};

pub async fn login(header: HeaderMap) -> Json<AccountLogin> {
    let uid = header
        .get("Uid")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    Json(AccountLogin::new(uid))
}

pub async fn sync_data() -> impl IntoResponse {
    fn sync_data_inner() -> Result<()> {
        if !exists(USER_JSON_PATH)? {
            write_json(USER_JSON_PATH, User::default())?;
        }

        let mut player_data = UserData::default();

        let skin_table = SkinTable::load()?;
        let char_table = CharacterTable::load()?;
        let equid_table = EquipTable::load()?;
        let battle_equip_table = BattleEquipTable::load()?;
        let display_meta_table = DisplayMetaTable::load()?;
        let charword_table = CharWordTable::load()?;
        let charm_table = CharmTable::load()?;
        let retro_table = RetroTable::load()?;

        let ts = time(USER_CONFIG.fake_time);

        let mut temp_skins = HashMap::new();

        for (skin_key, skin) in &skin_table.char_skins {
            if !skin_key.contains("@") {
                continue;
            }

            player_data.user.skin.add_skin(skin_key.clone());

            if !temp_skins.contains_key(&skin.char_id) || skin.display_skin.on_year > skin_table.char_skins[&temp_skins[&skin.char_id]].display_skin.on_year {
                temp_skins.insert(skin.char_id.clone(), skin.skin_id.clone());
            };
        }

        for (char_key, chara) in &char_table.table {
            if !char_key.contains("char") {
                continue;
            }

            if let CharType::Character(char) = chara {
                let mut character = Chara::new(char_key.clone());

                let evolve_phase = char.phases.len() - 1;
                character.set_elite_status(evolve_phase as u8);
                character.set_level(char.phases[evolve_phase].max_level);

                let vlan = if charword_table.char_default_type_dict.contains_key(char_key) {
                    charword_table.char_default_type_dict[char_key].clone()
                } else {
                    "JP".into()
                };

                character.set_voice_lan(vlan.parse::<VoiceLan>().unwrap_or(VoiceLan::Jp));
                character.set_default_skill_index(char.skills.len() as i8 - 1);

                match &**char_key {
                    "char_508_aguard" | "char_509_acast" | "char_510_amedic" | "char_511_asnipe" => (),
                    key => character.set_skin(format!("{}#2", key)),
                }

                if let Some(skin) = temp_skins.get(char_key) {
                    character.set_skin(skin.clone());
                }
            }
        }

        Ok(())
    }

    if let Ok(_userdata) = sync_data_inner() {
        // PLACEHOLDER
        (StatusCode::ACCEPTED, "").into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "internal error occurred").into_response()
    }
}

pub async fn sync_status() -> Json<AccountSyncStatus> {
    Json(AccountSyncStatus::default())
}

pub async fn sync_push_data<'a>() -> Json<MiscResponse<'a>> {
    Json(BATCH_EVENT)
}
