pub mod normal {
    use serde_json::{json, Value};

    use crate::{
        constants::url::GACHA_TABLE_URL,
        utils::{
            game::update_data,
            json::{get_keys, JSON},
            random::sample,
        },
    };

    pub async fn get_tags() -> Value {
        let gacha_table = update_data(GACHA_TABLE_URL).await;
        let mut all_tags = Vec::new();
        for data in gacha_table["gachaTags"].as_array().unwrap() {
            let tag_id = data["tagId"].as_u64().unwrap();
            if tag_id < 1000 {
                all_tags.push(tag_id);
            }
        }
        let tags = sample(all_tags, 5);
        json!(tags)
    }

    pub async fn build_tag_char_set(mode: &str) {
        let mut tag_char_set = json!({});
    }
}
