use serde::{Serialize, Deserialize};
use crate::error::Error;

/// 作物状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotState {
    pub id: u64,
    pub crop_code: String,
    pub stage: u8,          // 1–6
    pub variant: Option<String>,
    pub soil_type: String,
    pub facility: Option<String>,
    pub last_watered_at: i64,  // Unix timestamp（秒）
    pub last_sun_at: i64,
    pub growth: f32,        // 0.0 – 100.0
    pub nutrients: NutrientState,
    pub status: Option<String>,  // 干旱 / 霜冻 / 失水 / 正常
}

/// 营养状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientState {
    pub nitrogen: f32,
    pub phosphorus: f32,
    pub potassium: f32,
    pub decay_at: i64,      // Unix timestamp（秒）
}

/// 作物配置（从 content/*.json 加载）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropConfig {
    pub code: String,
    pub name_key: String,
    pub stages: Vec<StageConfig>,
    pub variant_pool: Vec<String>,
    pub variant_whitelist: Vec<String>,
    pub soil_preference: Vec<String>,
    pub season_preference: Vec<String>,
    pub growth_base: f32,   // 基础成长倍率
    pub growth_per_minute: f32,  // 每分钟成长
}

/// 阶段配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageConfig {
    pub stage: u8,
    pub growth_threshold: f32,  // 成熟阈值（0.0–100.0）
    pub yield_multiplier: f32,  // 产量倍率
    pub quality_multiplier: f32, // 品质倍率
}

/// 天气快照（由调用方注入）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherSnapshot {
    pub condition: String,   // sunny / cloudy / rainy / snowy
    pub temperature: f32,    // 摄氏度
    pub wind: f32,           // 米/秒
    pub event: Option<String>, // drought / frost / heatwave
}

/// 变体 roll 结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantRollResult {
    pub variant: String,
    pub rarity: String,      // common / rare / epic / legendary
    pub probability: f32,    // 概率
}

/// 重茬判定结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossCheckResult {
    pub is_cross: bool,
    pub penalty_multiplier: f32,  // 惩罚倍率（×0.5 / ×0.7 / ×0.9）
    pub reason: String,        // 原因说明
}

/// 收获结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestResult {
    pub crop_code: String,
    pub stage: u8,
    pub variant: Option<String>,
    pub quantity: f32,        // 产量
    pub quality: f32,         // 品质（0.0–100.0）
    pub coin_income: i32,     // 园币收入
    pub exp_gain: u32,        // 园丁 XP
    pub achievement_triggers: Vec<String>,  // 触发的成就码
}

/// 作物阶段名称
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CropStage {
    Seed,
    Sprout,
    Seedling,
    Bud,
    Flower,
    Fruit,
}

impl TryFrom<u8> for CropStage {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CropStage::Seed),
            2 => Ok(CropStage::Sprout),
            3 => Ok(CropStage::Seedling),
            4 => Ok(CropStage::Bud),
            5 => Ok(CropStage::Flower),
            6 => Ok(CropStage::Fruit),
            _ => Err(Error::StageInvalid),
        }
    }
}

impl From<CropStage> for u8 {
    fn from(value: CropStage) -> Self {
        value as u8
    }
}

impl std::fmt::Display for CropStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CropStage::Seed => write!(f, "种子"),
            CropStage::Sprout => write!(f, "发芽"),
            CropStage::Seedling => write!(f, "幼苗"),
            CropStage::Bud => write!(f, "花苞"),
            CropStage::Flower => write!(f, "开花"),
            CropStage::Fruit => write!(f, "果实"),
        }
    }
}

/// 状态码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlotStatus {
    Normal,
    Thirsty,
    Frost,
    Scorch,
    RootRot,
    Pest,
    Chimeric,
}

impl std::fmt::Display for PlotStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlotStatus::Normal => write!(f, "正常"),
            PlotStatus::Thirsty => write!(f, "缺水"),
            PlotStatus::Frost => write!(f, "霜冻"),
            PlotStatus::Scorch => write!(f, "灼叶"),
            PlotStatus::RootRot => write!(f, "沤根"),
            PlotStatus::Pest => write!(f, "虫害"),
            PlotStatus::Chimeric => write!(f, "锦化"),
        }
    }
}

impl From<PlotStatus> for String {
    fn from(status: PlotStatus) -> Self {
        status.to_string()
    }
}

impl TryFrom<&str> for PlotStatus {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "normal" | "正常" => Ok(PlotStatus::Normal),
            "thirsty" | "缺水" => Ok(PlotStatus::Thirsty),
            "frost" | "霜冻" => Ok(PlotStatus::Frost),
            "scorch" | "灼叶" => Ok(PlotStatus::Scorch),
            "root_rot" | "沤根" => Ok(PlotStatus::RootRot),
            "pest" | "虫害" => Ok(PlotStatus::Pest),
            "chimeric" | "锦化" => Ok(PlotStatus::Chimeric),
            _ => Err(Error::StatusCheckFailed),
        }
    }
}
