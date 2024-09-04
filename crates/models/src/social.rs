use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::PlayerDataDeltaStatic;

#[derive(Deserialize, Serialize)]
pub struct PlayerSocial {
    players: [PlayerSocialProfile; 1],
    #[serde(rename = "friendStatusList")]
    friend_status_list: [u8; 1],
    #[serde(rename = "resultIdList")]
    result_id_list: [String; 1],
    #[serde(rename = "playerDataDelta")]
    player_data_delta: PlayerDataDeltaStatic,
}

#[derive(Deserialize, Serialize)]
struct PlayerSocialProfile {
    #[serde(rename = "nickName")]
    nick_name: String,
    #[serde(rename = "nickNumber")]
    nick_number: String,
    uid: String,
    #[serde(rename = "friendNumLimit")]
    friend_num_limit: u8,
    #[serde(rename = "serverName")]
    server_name: String,
    level: u8,
    #[serde(rename = "avatarId")]
    avatar_id: String,
    avatar: HashMap<(), ()>,
    assist_char_list: [(); 1],
    last_online_time: u8,
    medal_board: MedalBoard,
    skin: Skin,
}

#[derive(Deserialize, Serialize)]
struct MedalBoard {
    #[serde(rename = "type")]
    t: String,
    custom: (),
    template: (),
}

#[derive(Deserialize, Serialize)]
struct Skin {
    selected: String,
    state: HashMap<(), ()>,
}

impl PlayerSocialProfile {
    pub fn new(name: &str) -> Self {
        Self {
            nick_name: String::from(name),
            nick_number: String::from("6666"),
            uid: String::from("66666666"),
            friend_num_limit: 50,
            server_name: String::from("泰拉"),
            level: 120,
            avatar_id: String::from("0"),
            avatar: HashMap::default(),
            assist_char_list: [()],
            last_online_time: 0,
            medal_board: MedalBoard {
                t: String::from("EMPTY"),
                custom: (),
                template: (),
            },
            skin: Skin {
                selected: String::from("nc_rhodes_default"),
                state: HashMap::default(),
            },
        }
    }
}

impl PlayerSocial {
    pub fn new(name: &str) -> Self {
        Self {
            players: [PlayerSocialProfile::new(name)],
            friend_status_list: [0],
            result_id_list: [String::from("66666666")],
            player_data_delta: PlayerDataDeltaStatic::default(),
        }
    }
}
