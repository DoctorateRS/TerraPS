use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignV2 {
    pub campaign_current_fee: u16,
    pub campaign_total_fee: u16,
    pub last_refresh_ts: u64,
    pub instances: HashMap<String, CampaignV2Instance>,
    pub open: CampaignV2Open,
    pub missions: HashMap<String, u8>,
    pub sweep_max_kills: HashMap<String, u16>,
}

impl CampaignV2 {
    pub fn new() -> Self {
        Self {
            campaign_total_fee: 1800,
            campaign_current_fee: 1800,
            last_refresh_ts: 0,
            instances: HashMap::new(),
            open: CampaignV2Open::new(),
            missions: HashMap::new(),
            sweep_max_kills: HashMap::new(),
        }
    }

    pub fn set_campaign(&mut self, camp_id: String) {
        self.instances.insert(camp_id, CampaignV2Instance::default());
    }

    pub fn set_mission(&mut self, mission_id: String) {
        self.missions.insert(mission_id, 2);
    }

    pub fn set_sweep_max_kills(&mut self, camp_id: String) {
        self.sweep_max_kills.insert(camp_id, 400);
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignV2Instance {
    pub max_kills: u16,
    pub reward_status: [u8; 8],
}

impl Default for CampaignV2Instance {
    fn default() -> Self {
        Self { max_kills: 400, reward_status: [1; 8] }
    }
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CampaignV2Open {
    pub permanent: Vec<String>,
    rotate: String,
    r_group: String,
    pub training: Vec<String>,
    t_group: String,
    t_all_open: String,
}

impl CampaignV2Open {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_perm(&mut self, camp_id: String) {
        self.permanent.push(camp_id);
    }

    pub fn set_rotate(&mut self, camp_id: String) {
        self.rotate = camp_id;
    }

    pub fn set_rotate_group(&mut self, group_id: String) {
        self.r_group = group_id;
    }

    pub fn add_training(&mut self, camp_id: String) {
        self.training.push(camp_id);
    }

    pub fn set_training_group(&mut self, group_id: String) {
        self.t_group = group_id;
    }

    pub fn set_training_all_open(&mut self, group_id: String) {
        self.t_all_open = group_id;
    }
}
