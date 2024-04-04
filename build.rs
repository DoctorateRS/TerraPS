extern crate include_dir;
extern crate std;

use include_dir::include_dir;

fn main() {
    include_dir!("./config");
    include_dir!("./data");

    include_bytes!("./syncData.json");

    include_bytes!("./config/config.json");
    include_bytes!("./config/assists.json");
    include_bytes!("./config/mails.json");
    include_bytes!("./config/multiUserConfig.json");
    include_bytes!("./config/rlv2Config.json");
    include_bytes!("./config/squads.json");

    include_bytes!("./data/announce/announcement.meta.json");
    include_bytes!("./data/announce/preannouncement.meta.json");

    include_bytes!("./data/crisisv2/cc1.json");
    include_bytes!("./data/crisisv2/cc2.json");

    include_bytes!("./data/excel/activity_table.json");
    include_bytes!("./data/excel/audio_data.json");
    include_bytes!("./data/excel/battle_equip_table.json");
    include_bytes!("./data/excel/charm_table.json");
    include_bytes!("./data/excel/charword_table.json");
    include_bytes!("./data/excel/checkin_table.json");
    include_bytes!("./data/excel/climb_tower_table.json");
    include_bytes!("./data/excel/clue_data.json");
    include_bytes!("./data/excel/crisis_table.json");
    include_bytes!("./data/excel/crisis_v2_table.json");
    include_bytes!("./data/excel/data_version.txt");
    include_bytes!("./data/excel/display_meta_table.json");
    include_bytes!("./data/excel/enemy_handbook_table.json");
    include_bytes!("./data/excel/favor_table.json");
    include_bytes!("./data/excel/gacha_table.json");
    include_bytes!("./data/excel/gamedata_const.json");
    include_bytes!("./data/excel/handbook_info_table.json");
    include_bytes!("./data/excel/handbook_table.json");
    include_bytes!("./data/excel/handbook_team_table.json");
    include_bytes!("./data/excel/item_table.json");
    include_bytes!("./data/excel/medal_table.json");
    include_bytes!("./data/excel/mission_table.json");
    include_bytes!("./data/excel/open_server_table.json");
    include_bytes!("./data/excel/player_avatar_table.json");
    include_bytes!("./data/excel/range_table.json");
    include_bytes!("./data/excel/replicate_table.json");
    include_bytes!("./data/excel/retro_table.json");
    include_bytes!("./data/excel/roguelike_table.json");
    include_bytes!("./data/excel/roguelike_topic_table.json");
    include_bytes!("./data/excel/sandbox_perm_table.json");
    include_bytes!("./data/excel/sandbox_table.json");
    include_bytes!("./data/excel/shop_client_table.json");
    include_bytes!("./data/excel/skill_table.json");
    include_bytes!("./data/excel/skin_table.json");
    include_bytes!("./data/excel/stage_table.json");
    include_bytes!("./data/excel/story_review_meta_table.json");
    include_bytes!("./data/excel/story_review_table.json");
    include_bytes!("./data/excel/story_table.json");
    include_bytes!("./data/excel/tech_buff_table.json");
    include_bytes!("./data/excel/tip_table.json");
    include_bytes!("./data/excel/token_table.json");
    include_bytes!("./data/excel/uniequip_data.json");
    include_bytes!("./data/excel/uniequip_table.json");
    include_bytes!("./data/excel/zone_table.json");
    include_bytes!("./data/excel/vc/vc_config.json");

    include_bytes!("./data/rlv2/choiceBuffs.json");
    include_bytes!("./data/rlv2/recruitGroups.json");
    include_bytes!("./data/rlv2/nodesInfo.json");

    include_bytes!("./data/gacha/pool.json");
    include_bytes!("./data/user/user.json");

    include_bytes!("./data/tower/towerData.json");
}
