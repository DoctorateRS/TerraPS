use crate::{
    core::{
        asset, general_v1_server_time, prod,
        user::{self, app, business_card},
    },
    game::{
        account, background, building, campaignv2,
        char_manager::{char, char_build, charm},
        crisis_manager::crisis_v2,
        deep_sea,
        gacha::{advanced, normal},
        mail, online, pay,
        quest_manager::{april_fools, bossrush, quest, story_review},
        rlv2, sandboxv2, shop, social, story, tower,
    },
    utils::json::JSON,
};
use axum::{
    routing::{get, post},
    Json, Router,
};

use serde_json::json;
use tower_http::trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnResponse, TraceLayer};
use tracing::Level;

#[tracing::instrument]
pub fn app() -> Router {
    let trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::default().level(Level::INFO))
        .on_eos(())
        .on_request(())
        .on_failure(DefaultOnFailure::default().level(Level::ERROR))
        .on_response(DefaultOnResponse::default().level(Level::DEBUG));

    Router::new()
        .nest("/app", app_routes())
        .nest("/account", account_routes())
        .nest("/activity", activity_routes())
        .nest("/act25side", act25side_routes())
        .nest("/aprilFool", april_fools_routes())
        .nest("/building", building_routes())
        .nest("/businessCard", business_card_routes())
        .nest("/campaignV2", campaignv2_routes())
        .nest("/char", char_routes())
        .nest("/charBuild", char_build_routes())
        .nest("/config/prod", config_routes())
        .nest("/crisisV2", crisis_v2_routes())
        .nest("/deepSea", deep_sea_routes())
        .nest("/gacha", gacha_routes())
        .nest("/mail", mail_routes())
        .nest("/online", online_routes())
        .nest("/quest", quest_routes())
        .nest("/retro", retro_routes())
        .nest("/rlv2", rlv2_routes())
        .nest("/sandboxPerm/sandboxV2", sandbox_routes())
        .nest("/shop", shop_routes())
        .nest("/social", social_routes())
        .nest("/story", story_routes())
        .nest("/storyreview", story_review_routes())
        .nest("/tower", tower_routes())
        .nest("/u8", u8_routes())
        .nest("/user", user_routes())
        .nest("/debug", debug_routes())
        .merge(misc_routes())
        .fallback(fallback)
        .layer(trace)
}

fn app_routes() -> Router {
    Router::new()
        .route("/v1/config", get(app::app_v1_config))
        .route("/getSettings", post(app::app_get_settings))
        .route("/getCode", post(app::app_get_code))
}

fn account_routes() -> Router {
    Router::new()
        .route("/login", post(account::account_login))
        .route("/syncStatus", post(account::account_sync_status))
        .route("/syncData", post(account::account_sync_data))
        .route("/yostar_auth_request", post(account::account_yostar_auth_req))
        .route("/yostar_auth_submit", post(account::account_yostar_auth_submit))
}

fn activity_routes() -> Router {
    Router::new()
        .route("/bossRush/relicSelect", post(bossrush::relic_select))
        .route("/bossRush/battleStart", post(quest::quest_battle_start))
        .route("/act24side/battleStart", post(quest::quest_battle_start))
        .route("/act24side/battleFinish", post(quest::quest_battle_finish))
        .route("/act24side/setTool", post(quest::set_tool))
}

fn act25side_routes() -> Router {
    Router::new()
        .route("/battleStart", post(quest::quest_battle_start))
        .route("/battleFinish", post(quest::quest_battle_finish))
}

fn april_fools_routes() -> Router {
    Router::new()
        .route("/act5fun/battleStart", post(quest::quest_battle_start))
        .route("/act5fun/battleFinish", post(april_fools::act5_fun_battle_finish))
        .route("/act4fun/battleStart", post(quest::quest_battle_start))
        .route("/act4fun/battleFinish", post(april_fools::act4_fun_battle_finish))
        .route("/act4fun/liveSettle", post(april_fools::act4_fun_live_settle))
        .route("/act3fun/battleStart", post(quest::quest_battle_start))
        .route("/act3fun/battleFinish", post(quest::quest_battle_finish))
}

fn building_routes() -> Router {
    Router::new()
        .route("/sync", post(building::building_sync))
        .route("/getRecentVisitors", post(building::building_get_recent_visitors))
        .route("/getInfoShareVisitorsNum", post(building::building_get_info_share_visitor_num))
        .route("/getAssistReport", post(building::building_get_assist_report))
        .route("/changeDiySolution", post(building::building_change_diy_solution))
        .route("/assignChar", post(building::building_assign_char))
        .route("/setBuildingAssist", post(building::building_set_building_assist))
}

fn business_card_routes() -> Router {
    Router::new()
        .route("/changeNameCardComponent", post(business_card::change_name_component))
        .route("/changeNameCardSkin", post(business_card::change_card_skin))
}

fn campaignv2_routes() -> Router {
    Router::new()
        .route("/battleStart", post(campaignv2::campaignv2_battle_start))
        .route("/battleFinish", post(campaignv2::campaignv2_battle_finish))
        .route("/battleSweep", post(campaignv2::campaignv2_battle_sweep))
}

fn char_routes() -> Router {
    Router::new().route("/changeMarkStar", post(char::char_change_mark_star))
}

fn char_build_routes() -> Router {
    Router::new()
        .route("/addonStage/battleStart", post(quest::quest_battle_start))
        .route("/addonStage/battleFinish", post(quest::quest_battle_finish))
        .route("/addonStory/unlock", post(char_build::char_build_addon_story_unlock))
        .route("/batchSetCharVoiceLan", post(char_build::char_build_batch_set_char_voice_lan))
        .route("/setCharVoiceLan", post(char_build::char_build_set_char_voice_lan))
        .route("/setDefaultSkill", post(char_build::char_build_set_char_default_skill))
        .route("/changeCharSkin", post(char_build::char_build_change_char_skin))
        .route("/setEquipment", post(char_build::char_build_set_char_equipment))
        .route("/changeCharTemplate", post(char_build::char_build_change_char_template))
}

fn config_routes() -> Router {
    Router::new()
        .route("/official/Android/version", get(prod::prod_android_version))
        .route("/official/network_config", get(prod::prod_network_config))
        .route("/official/remote_config", get(prod::prod_remote_config))
        .route("/official/refresh_config", get(prod::prod_refresh_config))
        .route("/announce_meta/Android/announcement.meta.jsons", get(prod::prod_announcement))
        .route("/announce_meta/Android/preannouncement.meta.json", get(prod::prod_pre_announcement))
}

fn crisis_v2_routes() -> Router {
    Router::new()
        .route("/getInfo", post(crisis_v2::crisis_v2_get_info))
        .route("/battleStart", post(crisis_v2::crisis_v2_battle_start))
        .route("/battleFinish", post(crisis_v2::crisis_v2_battle_finish))
        .route("/getSnapshot", post(crisis_v2::crisis_v2_get_snapshot))
}

fn deep_sea_routes() -> Router {
    Router::new()
        .route("/branch", post(deep_sea::deep_sea_branch))
        .route("/event", post(deep_sea::deep_sea_event))
}

fn gacha_routes() -> Router {
    Router::new()
        .route("/normalGacha", post(normal::normal_gacha))
        .route("/syncNormalGacha", post(normal::sync_normal_gacha))
        .route("/boostNormalGacha", post(normal::boost_normal_gacha))
        .route("/finishNormalGacha", post(normal::finish_normal_gacha))
        .route("/getPoolDetail", post(advanced::get_pool_detail))
        .route("/advancedGacha", post(advanced::advanced_gacha))
        .route("/tenAdvancedGacha", post(advanced::ten_advanced_gacha))
}

fn mail_routes() -> Router {
    Router::new()
        .route("/getMetaInfoList", post(mail::mail_get_meta_info_list))
        .route("/listMailBox", post(mail::mail_list_mail_box))
        .route("/receiveMail", post(mail::mail_receive_mail))
        .route("/receiveAllMail", post(mail::mail_receive_all_mail))
        .route("/removeAllReceivedMail", post(mail::mail_delete_all_received_mails))
}

fn online_routes() -> Router {
    Router::new()
        .route("/v1/ping", post(online::online_v1_ping))
        .route("/v1/loginout", post(online::online_v1_login_out))
}

fn quest_routes() -> Router {
    Router::new()
        .route("/battleStart", post(quest::quest_battle_start))
        .route("/battleContinue", post(quest::quest_battle_continue))
        .route("/battleFinish", post(quest::quest_battle_finish))
        .route("/changeSquadName", post(quest::squad_change_name))
        .route("/squadFormation", post(quest::squad_set_formation))
        .route("/saveBattleReplay", post(quest::quest_save_battle_replay))
        .route("/getBattleReplay", post(quest::quest_get_battle_replay))
}

fn retro_routes() -> Router {
    Router::new()
        .route("/typeAct20side/competitionStart", post(quest::act_20_competition_start))
        .route("/typeAct20side/competitionFinish", post(quest::act_20_competition_finish))
}

fn rlv2_routes() -> Router {
    Router::new()
        .route("/createGame", post(rlv2::rlv2_create_game))
        .route("/giveUpGame", post(rlv2::rlv2_give_up_game))
        .route("/selectChoice", post(rlv2::rlv2_choice_select))
        .route("/chooseInitialRelic", post(rlv2::rlv2_choose_init_relic))
        .route("/chooseInitialRecruitSet", post(rlv2::rlv2_choose_init_recruit_set))
        .route("/activeRecruitTicket", post(rlv2::rlv2_activate_recruit_tkt))
        .route("/recruitChar", post(rlv2::rlv2_recruit_char))
        .route("/closeRecruitTicket", post(rlv2::rlv2_close_tkt))
        .route("/finishEvent", post(rlv2::rlv2_finish_event))
        .route("/moveAndBattleStart", post(rlv2::rlv2_mv_n_battle_start))
        .route("/battleFinish", post(rlv2::rlv2_battle_finish))
        .route("/finishBattleReward", post(rlv2::rlv2_finish_battle_reward))
        .route("/moveTo", post(rlv2::rlv2_mv_to))
        .route("/buyGoods", post(rlv2::rlv2_buy_good))
        .route("/leaveShop", post(rlv2::rlv2_leave_shop))
        .route("/chooseBattleReward", post(rlv2::rlv2_choose_battle_reward))
}

fn sandbox_routes() -> Router {
    Router::new()
        .route("/createGame", post(sandboxv2::create_game))
        .route("/battleStart", post(sandboxv2::sandbox_battle_start))
        .route("/battleFinish", post(sandboxv2::sandbox_battle_finish))
        .route("/setSquad", post(sandboxv2::set_squad))
        .route("/homeBuildSave", post(sandboxv2::home_build_save))
        .route("/settleGame", post(sandboxv2::settle_game))
        .route("/eatFood", post(sandboxv2::eat_food))
        .route("/exploreMode", post(sandboxv2::explore_mode))
        .route("/monthBattleStart", post(sandboxv2::month_battle_start))
        .route("/monthBattleFinish", post(sandboxv2::month_battle_finish))
}

fn shop_routes() -> Router {
    Router::new().route("/getSkinGoodList", post(shop::pay_get_unconfirmed_order_id_list))
}

fn social_routes() -> Router {
    Router::new()
        .route("/setAssistCharList", post(social::social_set_assist_char_list))
        .route("/setCardShowMedal", post(social::social_set_card_medal))
        .route("/getSortListInfo", post(social::social_get_sort_list_info))
        .route("/searchPlayer", post(social::social_search_player))
}

fn story_routes() -> Router {
    Router::new().route("/finishStory", post(story::story_finish_story))
}

fn story_review_routes() -> Router {
    Router::new()
        .route("/markStoryAcceKnown", post(story_review::mark_story_acce_known))
        .route("/readStory", post(story_review::read_story))
}

fn tower_routes() -> Router {
    Router::new()
        .route("/createGame", post(tower::tower_create_game))
        .route("/initGodCard", post(tower::tower_init_godcard))
        .route("/initGame", post(tower::tower_init_game))
        .route("/initCard", post(tower::tower_init_card))
        .route("/battleStart", post(tower::tower_battle_start))
        .route("/battleFinish", post(tower::tower_battle_finish))
        .route("/recruit", post(tower::tower_recruit))
        .route("/chooseSubGodCard", post(tower::tower_choose_sub_godcard))
        .route("/settleGame", post(tower::tower_settle_game))
}

fn u8_routes() -> Router {
    Router::new()
        .route("/user/auth/v1/agreement_version", get(user::misc::agreement_version))
        .route("/user/v1/getToken", post(user::user_v1_get_token))
        .route("/pay/getAllProductList", post(pay::pay_get_all_prod_list))
}

fn user_routes() -> Router {
    Router::new()
        .route("/auth", post(user::user_auth))
        .route("/auth/v1/token_by_phone_password", post(user::auth_v1_token_by_phone_password))
        .route("/agreement", get(user::user_agreement))
        .route("/checkIn", get(user::user_check_in))
        .route("/changeResume", post(user::user_change_resume))
        .route("/changeAvatar", post(user::user_change_avatar))
        .route("/changeSecretary", post(user::user_change_secretary))
        .route("/info/v1/need_cloud_auth", post(user::user_need_cloud_auth))
        .route("/info/v1/basic", get(user::misc::info_v1_basic))
        .route("/login", post(user::user_login))
        .route("/oauth2/v1/grant", post(user::user_oauth2_v1_grant))
        .route("/oauth2/v2/grant", post(user::user_oauth2_v2_grant))
        .route("/yostar_createlogin", post(user::user_yostar_create_login))
}

fn misc_routes() -> Router {
    Router::new()
        .route("/general/v1/server_time", get(general_v1_server_time))
        .route("/pay/getUnconfirmedOrderIdList", post(pay::pay_get_unconfirmed_order_id_list))
        .route("/background/setBackground", post(background::background_set_bg))
        .route("/homeTheme/change", post(background::home_theme_change))
        .route("/charm/setSquad", post(charm::charm_set_squad))
        .route("/car/confirmBattleCar", post(quest::confirm_battle_car))
        .route("/templateTrap/setTrapSquad", post(quest::set_trap_squad))
        .route("/assetbundle/official/Android/assets/:hash/:name", get(asset::get_file))
}

fn debug_routes() -> Router {
    Router::new()
}

async fn fallback() -> JSON {
    Json(json!({
        "playerDataDelta": {
            "deleted": {},
            "modified": {}
        }
    }))
}
