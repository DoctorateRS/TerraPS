use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::time::time;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStatus {
    nick_name: String,
    nick_number: String,
    level: u16,
    exp: u8,
    social_point: u16,
    gacha_ticket: u64,
    classic_gacha_ticket: u64,
    ten_gacha_ticket: u64,
    classic_ten_gacha_ticket: u64,
    instant_ticket: u64,
    hgg_shard: u64,
    lgg_shard: u64,
    classic_shard: u64,
    recruit_license: u64,
    progress: u32,
    buy_ap_remain_times: u8,
    ap_limit_up_flag: u8,
    uid: String,
    flags: HashMap<String, u8>,
    ap: u16,
    max_ap: u16,
    android_diamond: u64,
    ios_diamond: u64,
    pay_diamond: u64,
    free_diamond: u64,
    diamond_shard: u64,
    gold: u64,
    practice_ticket: u64,
    last_refresh_ts: u64,
    last_ap_add_time: u64,
    main_stage_progress: (),
    register_ts: u64,
    last_online_ts: u64,
    server_name: String,
    avatar_id: String,
    resume: String,
    friend_num_limit: u16,
    monthly_subscription_start_time: u64,
    monthly_subscription_end_time: u64,
    secretary: String,
    secretary_skin_id: String,
    tip_monthly_card_expire_ts: u64,
    avatar: Avatar,
    global_voice_lan: String,
}

#[derive(Serialize, Deserialize)]
struct Avatar {
    #[serde(rename = "type")]
    t: String,
    id: String,
}

impl Default for PlayerStatus {
    fn default() -> Self {
        let time = time();
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
