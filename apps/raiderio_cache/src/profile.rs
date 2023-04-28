use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    pub name: String,
    pub race: String,
    pub class: String,
    #[serde(rename = "active_spec_name")]
    pub active_spec_name: String,
    #[serde(rename = "active_spec_role")]
    pub active_spec_role: String,
    pub gender: String,
    pub faction: String,
    #[serde(rename = "achievement_points")]
    pub achievement_points: i64,
    #[serde(rename = "honorable_kills")]
    pub honorable_kills: i64,
    #[serde(rename = "thumbnail_url")]
    pub thumbnail_url: String,
    pub region: String,
    pub realm: String,
    #[serde(rename = "last_crawled_at")]
    pub last_crawled_at: String,
    #[serde(rename = "profile_url")]
    pub profile_url: String,
    #[serde(rename = "profile_banner")]
    pub profile_banner: String,
    #[serde(rename = "mythic_plus_scores_by_season")]
    pub mythic_plus_scores_by_season: Vec<MythicPlusScoresBySeason>,
    #[serde(rename = "mythic_plus_ranks")]
    pub mythic_plus_ranks: MythicPlusRanks,
    #[serde(rename = "mythic_plus_recent_runs")]
    pub mythic_plus_recent_runs: Vec<MythicPlusRecentRun>,
    #[serde(rename = "mythic_plus_highest_level_runs")]
    pub mythic_plus_highest_level_runs: Vec<MythicPlusHighestLevelRun>,
    pub gear: Gear,
    #[serde(rename = "raid_progression")]
    pub raid_progression: RaidProgression,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RaidProgression {
    #[serde(rename = "vault-of-the-incarnates")]
    pub vault_of_the_incarnates: Vault,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vault {
    summary: String,
    total_bosses: i64,
    normal_bosses_killed: i64,
    heroic_bosses_killed: i64,
    mythic_bosses_killed: i64,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MythicPlusScoresBySeason {
    pub season: String,
    pub scores: Scores,
    pub segments: Segments,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scores {
    pub all: f64,
    pub dps: f64,
    pub healer: f64,
    pub tank: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segments {
    pub all: All,
    pub dps: Dps,
    pub healer: Healer,
    pub tank: Tank,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct All {
    pub score: f64,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dps {
    pub score: f64,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Healer {
    pub score: f64,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tank {
    pub score: f64,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec0 {
    pub score: f64,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec1 {
    pub score: f64,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec2 {
    pub score: f64,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec3 {
    pub score: f64,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MythicPlusRanks {
    pub overall: Overall,
    pub class: Class,
    #[serde(rename = "faction_overall")]
    pub faction_overall: FactionOverall,
    #[serde(rename = "faction_class")]
    pub faction_class: FactionClass,
    pub tank: Option<Tank2>,
    #[serde(rename = "class_tank")]
    pub class_tank: Option<ClassTank>,
    #[serde(rename = "faction_tank")]
    pub faction_tank: Option<FactionTank>,
    #[serde(rename = "faction_class_tank")]
    pub faction_class_tank: Option<FactionClassTank>,
    pub healer: Option<Healer2>,
    #[serde(rename = "class_healer")]
    pub class_healer: Option<ClassHealer>,
    #[serde(rename = "faction_healer")]
    pub faction_healer: Option<FactionHealer>,
    #[serde(rename = "faction_class_healer")]
    pub faction_class_healer: Option<FactionClassHealer>,
    pub dps: Dps2,
    #[serde(rename = "class_dps")]
    pub class_dps: ClassDps,
    #[serde(rename = "faction_dps")]
    pub faction_dps: FactionDps,
    #[serde(rename = "faction_class_dps")]
    pub faction_class_dps: FactionClassDps,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overall {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionOverall {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionClass {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tank2 {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassTank {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionTank {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionClassTank {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Healer2 {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassHealer {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionHealer {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionClassHealer {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dps2 {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassDps {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionDps {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionClassDps {
    pub world: i64,
    pub region: i64,
    pub realm: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MythicPlusRecentRun {
    pub dungeon: String,
    #[serde(rename = "short_name")]
    pub short_name: String,
    #[serde(rename = "mythic_level")]
    pub mythic_level: i64,
    #[serde(rename = "completed_at")]
    pub completed_at: String,
    #[serde(rename = "clear_time_ms")]
    pub clear_time_ms: i64,
    #[serde(rename = "par_time_ms")]
    pub par_time_ms: i64,
    #[serde(rename = "num_keystone_upgrades")]
    pub num_keystone_upgrades: i64,
    #[serde(rename = "map_challenge_mode_id")]
    pub map_challenge_mode_id: i64,
    #[serde(rename = "zone_id")]
    pub zone_id: i64,
    pub score: f64,
    pub affixes: Vec<Affix>,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Affix {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub icon: String,
    #[serde(rename = "wowhead_url")]
    pub wowhead_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MythicPlusHighestLevelRun {
    pub dungeon: String,
    #[serde(rename = "short_name")]
    pub short_name: String,
    #[serde(rename = "mythic_level")]
    pub mythic_level: i64,
    #[serde(rename = "completed_at")]
    pub completed_at: String,
    #[serde(rename = "clear_time_ms")]
    pub clear_time_ms: i64,
    #[serde(rename = "par_time_ms")]
    pub par_time_ms: i64,
    #[serde(rename = "num_keystone_upgrades")]
    pub num_keystone_upgrades: i64,
    #[serde(rename = "map_challenge_mode_id")]
    pub map_challenge_mode_id: i64,
    #[serde(rename = "zone_id")]
    pub zone_id: i64,
    pub score: f64,
    pub affixes: Vec<Affix2>,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Affix2 {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub icon: String,
    #[serde(rename = "wowhead_url")]
    pub wowhead_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gear {
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "item_level_equipped")]
    pub item_level_equipped: i64,
    #[serde(rename = "item_level_total")]
    pub item_level_total: i64,
}
