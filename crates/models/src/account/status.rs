use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use common_utils::time;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStatus {
    pub nick_name: String,
    pub nick_number: String,
    pub level: u16,
    pub exp: u8,
    pub social_point: u16,
    pub gacha_ticket: u64,
    pub classic_gacha_ticket: u64,
    pub ten_gacha_ticket: u64,
    pub classic_ten_gacha_ticket: u64,
    pub instant_ticket: u64,
    pub hgg_shard: u64,
    pub lgg_shard: u64,
    pub classic_shard: u64,
    pub recruit_license: u64,
    pub progress: u32,
    pub buy_ap_remain_times: u8,
    pub ap_limit_up_flag: u8,
    pub uid: String,
    pub flags: HashMap<String, u8>,
    pub ap: u16,
    pub max_ap: u16,
    pub android_diamond: u64,
    pub ios_diamond: u64,
    pub pay_diamond: u64,
    pub free_diamond: u64,
    pub diamond_shard: u64,
    pub gold: u64,
    pub practice_ticket: u64,
    pub last_refresh_ts: u64,
    pub last_ap_add_time: u64,
    pub main_stage_progress: (),
    pub register_ts: u64,
    pub last_online_ts: u64,
    pub server_name: String,
    pub avatar_id: String,
    pub resume: String,
    pub friend_num_limit: u16,
    pub monthly_subscription_start_time: u64,
    pub monthly_subscription_end_time: u64,
    pub secretary: String,
    pub secretary_skin_id: String,
    pub tip_monthly_card_expire_ts: u64,
    pub avatar: Avatar,
    pub global_voice_lan: String,
}

#[derive(Serialize, Deserialize)]
pub struct Avatar {
    #[serde(rename = "type")]
    pub t: String,
    pub id: String,
}

impl Default for PlayerStatus {
    fn default() -> Self {
        let time = time(-1);
        Self {
            nick_name: String::from("Terra"),
            nick_number: String::from("1111"),
            level: 999,
            exp: 0,
            social_point: 300,
            gacha_ticket: 9999999,
            classic_gacha_ticket: 9999999,
            ten_gacha_ticket: 9999999,
            classic_ten_gacha_ticket: 9999999,
            instant_ticket: 9999999,
            hgg_shard: 9999999,
            lgg_shard: 9999999,
            classic_shard: 9999999,
            recruit_license: 9999999,
            progress: 30000,
            buy_ap_remain_times: 99,
            ap_limit_up_flag: 0,
            uid: String::from("1"),
            flags: HashMap::new(),
            ap: 999,
            max_ap: 999,
            android_diamond: 9999999,
            ios_diamond: 9999999,
            pay_diamond: 9999999,
            free_diamond: 9999999,
            diamond_shard: 9999999,
            gold: 9999999,
            practice_ticket: 999,
            last_refresh_ts: time,
            last_ap_add_time: time,
            main_stage_progress: (),
            register_ts: time,
            last_online_ts: time,
            server_name: String::from("Terra"),
            avatar_id: String::from("0"),
            resume: String::from("TerraPS"),
            friend_num_limit: 999,
            monthly_subscription_start_time: 0,
            monthly_subscription_end_time: 0,
            secretary: String::from("char_377_gdglow"),
            secretary_skin_id: String::from("char_377_gdglow@sanrio#1"),
            tip_monthly_card_expire_ts: 0,
            avatar: Avatar {
                t: String::from("ICON"),
                id: String::from("avatar_activity_RI"),
            },
            global_voice_lan: String::from("JP"),
        }
    }
}

impl PlayerStatus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_flag(&mut self, key: String) {
        self.flags.insert(key, 1);
    }
}
