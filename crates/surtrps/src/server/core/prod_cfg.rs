use axum::Json;

use models::prod::{AnnouncementMeta, PreannouncementMeta, ProdAndroidNetwork, ProdAndroidRefresh, ProdAndroidRemote, ProdAndroidVersion};

pub async fn prod_version() -> Json<ProdAndroidVersion> {
    Json(ProdAndroidVersion::load())
}

pub async fn prod_refresh() -> Json<ProdAndroidRefresh> {
    Json(ProdAndroidRefresh::default())
}

pub async fn prod_network() -> Json<ProdAndroidNetwork> {
    Json(ProdAndroidNetwork::load())
}

pub async fn prod_remote() -> Json<ProdAndroidRemote> {
    Json(ProdAndroidRemote {})
}

pub async fn prod_announce() -> Json<AnnouncementMeta> {
    Json(AnnouncementMeta::load())
}

pub async fn prod_preannounce() -> Json<PreannouncementMeta> {
    Json(PreannouncementMeta::load())
}
