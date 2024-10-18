use axum::Json;

pub async fn get_unconfirmed_order_id_list() -> Json<()> {
    Json(())
}
