use crate::{
    core::{general_v1_server_time, prod, user},
    game::{
        account, background, building,
        char_manager::{char, char_build, charm},
        crisis_manager::crisis_v2,
        online, social,
    },
};
use axum::{
    extract::Request,
    http::Uri,
    routing::{get, post},
    Router,
};
use reqwest::StatusCode;
use tower_http::trace::{DefaultMakeSpan as DefMakeSpan, TraceLayer as Tracer};
use tracing::{debug, Span};

pub fn routes() -> Router {
    let trace_layer = Tracer::new_for_http()
        .make_span_with(DefMakeSpan::default())
        .on_request(|req: &Request<_>, _span: &Span| {
            debug!("Received request: {:?}", req.uri());
        })
        .on_response(())
        .on_failure(())
        .on_eos(());

    Router::new()
        .nest("/app", app_routes())
        .nest("/account", account_routes())
        .nest("/building", building_routes())
        .nest("/businessCard", business_card_routes())
        .nest("/char", char_routes())
        .nest("/charBuild", char_build_routes())
        .nest("/config/prod", config_routes())
        .nest("/crisisV2", crisis_v2_routes())
        .nest("/online", online_routes())
        .nest("/quest", quest_routes())
        .nest("/social", social_routes())
        .nest("/user", user_routes())
        .merge(misc_routes())
        .fallback(fallback)
        .layer(trace_layer)
}

fn app_routes() -> Router {
    Router::new().route("/v1/config", get(user::app_v1_config))
}

fn account_routes() -> Router {
    Router::new()
        .route("/login", post(account::account_login))
        .route("/syncStatus", post(account::account_sync_status))
        .route("/syncData", post(account::account_sync_data))
        .route("/yostar_auth_request", post(account::account_yostar_auth_req))
        .route("/yostar_auth_submit", post(account::account_yostar_auth_submit))
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
        .route("/changeNameCardComponent", post(user::business_card::change_name_component))
        .route("/changeNameCardSkin", post(user::business_card::change_card_skin))
}

fn char_routes() -> Router {
    Router::new().route("/changeMarkStar", post(char::char_change_mark_star))
}

fn char_build_routes() -> Router {
    Router::new()
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

fn online_routes() -> Router {
    Router::new()
        .route("/v1/ping", post(online::online_v1_ping))
        .route("/v1/loginout", post(online::online_v1_login_out))
}

fn quest_routes() -> Router {
    Router::new()
        .route("/getInfo", post(crisis_v2::crisis_v2_get_info))
        .route("/battleStart", post(crisis_v2::crisis_v2_battle_start))
        .route("/battleFinish", post(crisis_v2::crisis_v2_battle_finish))
        .route("/getSnapshot", post(crisis_v2::crisis_v2_get_snapshot))
}

fn social_routes() -> Router {
    Router::new()
        .route("/setAssistCharList", post(social::social_set_assist_char_list))
        .route("/setCardShowMedal", post(social::social_set_card_medal))
}

fn user_routes() -> Router {
    Router::new()
        .route("/auth", post(user::user_auth))
        .route("/agreement", get(user::user_agreement))
        .route("/checkIn", get(user::user_check_in))
        .route("/changeAvatar", post(user::user_change_avatar))
        .route("/changeSecretary", post(user::user_change_secretary))
        .route("/info/v1/basic", get(user::info_v1_basic))
}

fn misc_routes() -> Router {
    Router::new()
        .route("/general/v1/server_time", get(general_v1_server_time))
        .route("/u8/user/auth/v1/agreement_version", get(user::agreement_version))
        .route("/background/setBackground", post(background::background_set_bg))
        .route("/homeTheme/change", post(background::home_theme_change))
        .route("/charm/setSquad", post(charm::charm_set_squad))
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("ERROR: {} not found", uri))
}
