use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Social {
    #[serde(rename = "assistCharList")]
    assist_char_list: [(); 3],
}
