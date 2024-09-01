use std::fmt::{Formatter, Result as FmtResult};

use serde::{
    de::{Error as SerdeDeErr, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub rarity: RarityTier,
    pub phases: Vec<CharacterPhase>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterPhase {
    pub max_level: u8,
}

#[derive(Clone, Copy)]
pub enum RarityTier {
    Tier1,
    Tier2,
    Tier3,
    Tier4,
    Tier5,
    Tier6,
}

impl<'de> Deserialize<'de> for RarityTier {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RarityTierVisitor;

        impl<'de> Visitor<'de> for RarityTierVisitor {
            type Value = RarityTier;

            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                formatter.write_str("a string")
            }

            fn visit_str<E: SerdeDeErr>(self, v: &str) -> Result<Self::Value, E> {
                match v {
                    "TIER_1" => Ok(RarityTier::Tier1),
                    "TIER_2" => Ok(RarityTier::Tier2),
                    "TIER_3" => Ok(RarityTier::Tier3),
                    "TIER_4" => Ok(RarityTier::Tier4),
                    "TIER_5" => Ok(RarityTier::Tier5),
                    "TIER_6" => Ok(RarityTier::Tier6),
                    _ => Err(E::custom("wrong string")),
                }
            }
        }

        deserializer.deserialize_str(RarityTierVisitor)
    }
}

impl Serialize for RarityTier {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            RarityTier::Tier1 => "TIER_1",
            RarityTier::Tier2 => "TIER_2",
            RarityTier::Tier3 => "TIER_3",
            RarityTier::Tier4 => "TIER_4",
            RarityTier::Tier5 => "TIER_5",
            RarityTier::Tier6 => "TIER_6",
        })
    }
}
