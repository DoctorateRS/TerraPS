use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::time::time;

#[derive(Serialize, Deserialize)]
pub struct PlayerStatus {
    #[serde(rename = "nickName")]
    nick_name: String,
    #[serde(rename = "nickNumber")]
    nick_number: String,
    level: u16,
    exp: u8,
    #[serde(rename = "socialPoint")]
    social_point: u16,
    #[serde(rename = "gachaTicket")]
    gacha_ticket: u64,
    #[serde(rename = "classicGachaTicket")]
    classic_gacha_ticket: u64,
    #[serde(rename = "tenGachaTicket")]
    ten_gacha_ticket: u64,
    #[serde(rename = "classicTenGachaTicket")]
    classic_ten_gacha_ticket: u64,
    #[serde(rename = "instantFinishTicket")]
    instant_ticket: u64,
    #[serde(rename = "hggShard")]
    hgg_shard: u64,
    #[serde(rename = "lggShard")]
    lgg_shard: u64,
    #[serde(rename = "classicShard")]
    classic_shard: u64,
    #[serde(rename = "recruitLicense")]
    recruit_license: u64,
    progress: u32,
    #[serde(rename = "buyApRemainTimes")]
    buy_ap_remain_times: u8,
    #[serde(rename = "apLimitUpFlag")]
    ap_lim_up_flag: u8,
    uid: String,
    flags: HashMap<String, u8>,
    ap: u16,
    #[serde(rename = "maxAp")]
    max_ap: u16,
    #[serde(rename = "androidDiamond")]
    android: u64,
    #[serde(rename = "iosDiamond")]
    ios: u64,
    #[serde(rename = "payDiamond")]
    pay: u64,
    #[serde(rename = "freeDiamond")]
    free: u64,
    #[serde(rename = "diamondShard")]
    shard: u64,
    gold: u64,
    #[serde(rename = "practiceTicket")]
    practice_ticket: u64,
    #[serde(rename = "lastRefreshTs")]
    last_refresh_ts: u64,
    #[serde(rename = "lastApAddTime")]
    last_ap_add_time: u64,
    #[serde(rename = "mainStageProgress")]
    main_stage_progress: (),
    #[serde(rename = "registerTs")]
    register_ts: u64,
    #[serde(rename = "lastOnlineTs")]
    last_online_ts: u64,
    #[serde(rename = "serverName")]
    server_name: String,
    #[serde(rename = "avatarId")]
    avatar_id: String,
    #[serde(rename = "resume")]
    resume: String,
    #[serde(rename = "friendNumLimit")]
    friend_num_limit: u16,
    #[serde(rename = "monthlySubscriptionStartTime")]
    monthly_subscription_start_time: u64,
    #[serde(rename = "monthlySubscriptionEndTime")]
    monthly_subscription_end_time: u64,
    secretary: String,
    #[serde(rename = "secretarySkinId")]
    secretary_skin_id: String,
    #[serde(rename = "tipMonthlyCardExpireTs")]
    tip_monthly_card_expire_ts: u64,
    avatar: Avatar,
    #[serde(rename = "globalVoiceLan")]
    global_voice_lan: String,
}

#[derive(Serialize, Deserialize)]
struct Avatar {
    #[serde(rename = "type")]
    t: String,
    id: String,
}

impl PlayerStatus {
    pub fn default() -> Self {
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
            ap_lim_up_flag: 0,
            uid: String::from("1"),
            flags: HashMap::new(),
            ap: 999,
            max_ap: 999,
            android: 9999999,
            ios: 9999999,
            pay: 9999999,
            free: 9999999,
            shard: 9999999,
            gold: 9999999,
            practice_ticket: 999,
            last_refresh_ts: time(),
            last_ap_add_time: time(),
            main_stage_progress: (),
            register_ts: time(),
            last_online_ts: time(),
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

    pub fn set_flag(&mut self, key: String) {
        self.flags.insert(key, 1);
    }
}
