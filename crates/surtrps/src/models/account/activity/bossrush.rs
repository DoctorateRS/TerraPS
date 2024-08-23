use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Bossrush {
    pub relic: Relic,
}

#[derive(Serialize, Deserialize)]
pub struct Relic {
    pub level: HashMap<String, u8>,
    pub select: String,
}

impl Bossrush {
    pub fn new(act: u8) -> Self {
        let mut relic_levels = HashMap::new();

        let relics = [
            format!("act{}bossrush_relic_01", act),
            format!("act{}bossrush_relic_02", act),
            format!("act{}bossrush_relic_03", act),
            format!("act{}bossrush_relic_04", act),
        ];

        for relic in relics {
            relic_levels.insert(relic, 3);
        }

        Self {
            relic: Relic {
                level: relic_levels,
                select: format!("act{}bossrush_relic_01", act),
            },
        }
    }

    pub fn change_relic(&mut self, id: u8) {
        if id > 5 || id == 0 {
            return;
        }

        let _ = self.relic.select.pop();

        match id {
            1 => self.relic.select.push('1'),
            2 => self.relic.select.push('2'),
            3 => self.relic.select.push('3'),
            4 => self.relic.select.push('4'),
            _ => (),
        }
    }
}
