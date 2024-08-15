use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Social {
    #[serde(rename = "assistCharList")]
    pub assist_char_list: [(); 3],
}
