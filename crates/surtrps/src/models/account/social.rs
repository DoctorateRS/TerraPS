use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Social {
    #[serde(rename = "assistCharList")]
    pub assist_char_list: [(); 3],
}

pub const SOCIAL_STATIC: Social = Social { assist_char_list: [(); 3] };
