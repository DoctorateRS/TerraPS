use serde::{Deserialize, Serialize};

use super::character::chara::VoiceLan;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NpcAudio {
    pub npc_show_audio_info_flag: VoiceLan,
}
