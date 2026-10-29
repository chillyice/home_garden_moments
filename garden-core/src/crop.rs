use crate::types::{CropConfig, StageConfig};
use crate::error::Error;

/// 作物配置（硬编码示例，后续从 JSON 加载）
pub fn get_crop_config(crop_code: &str) -> Result<CropConfig, Error> {
    match crop_code {
        // 番茄
        "tomato_v1" => Ok(CropConfig {
            code: "tomato_v1".to_string(),
            name_key: "garden.crop.tomato".to_string(),
            stages: vec![
                StageConfig { stage: 1, growth_threshold: 10.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 2, growth_threshold: 30.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 3, growth_threshold: 60.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 4, growth_threshold: 85.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 5, growth_threshold: 95.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 6, growth_threshold: 100.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
            ],
            variant_pool: vec!["red_round".to_string(), "yellow_round".to_string(), "purple_round".to_string()],
            variant_whitelist: vec!["red_round".to_string(), "yellow_round".to_string()],
            soil_preference: vec!["loam".to_string()],
            season_preference: vec!["spring".to_string(), "summer".to_string(), "autumn".to_string()],
            growth_base: 1.0,
            growth_per_minute: 0.5,
        }),

        // 胡萝卜
        "carrot_v1" => Ok(CropConfig {
            code: "carrot_v1".to_string(),
            name_key: "garden.crop.carrot".to_string(),
            stages: vec![
                StageConfig { stage: 1, growth_threshold: 15.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 2, growth_threshold: 40.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 3, growth_threshold: 70.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 4, growth_threshold: 90.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 5, growth_threshold: 98.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 6, growth_threshold: 100.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
            ],
            variant_pool: vec!["orange_round".to_string(), "purple_round".to_string()],
            variant_whitelist: vec!["orange_round".to_string()],
            soil_preference: vec!["loam".to_string(), "sand".to_string()],
            season_preference: vec!["spring".to_string(), "autumn".to_string()],
            growth_base: 1.0,
            growth_per_minute: 0.8,
        }),

        // 向日葵
        "sunflower_v1" => Ok(CropConfig {
            code: "sunflower_v1".to_string(),
            name_key: "garden.crop.sunflower".to_string(),
            stages: vec![
                StageConfig { stage: 1, growth_threshold: 20.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 2, growth_threshold: 45.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 3, growth_threshold: 75.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 4, growth_threshold: 90.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 5, growth_threshold: 98.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 6, growth_threshold: 100.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
            ],
            variant_pool: vec!["yellow_large".to_string(), "yellow_small".to_string(), "white_large".to_string()],
            variant_whitelist: vec!["yellow_large".to_string()],
            soil_preference: vec!["loam".to_string(), "silt".to_string()],
            season_preference: vec!["summer".to_string(), "autumn".to_string()],
            growth_base: 1.0,
            growth_per_minute: 0.4,
        }),

        // 玫瑰
        "rose_v1" => Ok(CropConfig {
            code: "rose_v1".to_string(),
            name_key: "garden.crop.rose".to_string(),
            stages: vec![
                StageConfig { stage: 1, growth_threshold: 25.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 2, growth_threshold: 55.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 3, growth_threshold: 80.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 4, growth_threshold: 92.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 5, growth_threshold: 97.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 6, growth_threshold: 100.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
            ],
            variant_pool: vec!["red_pink".to_string(), "white_pink".to_string(), "yellow_pink".to_string()],
            variant_whitelist: vec!["red_pink".to_string(), "white_pink".to_string()],
            soil_preference: vec!["loam".to_string(), "sandy_loam".to_string()],
            season_preference: vec!["spring".to_string(), "summer".to_string()],
            growth_base: 1.0,
            growth_per_minute: 0.3,
        }),

        // 小麦
        "wheat_v1" => Ok(CropConfig {
            code: "wheat_v1".to_string(),
            name_key: "garden.crop.wheat".to_string(),
            stages: vec![
                StageConfig { stage: 1, growth_threshold: 10.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 2, growth_threshold: 30.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 3, growth_threshold: 60.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 4, growth_threshold: 85.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 5, growth_threshold: 95.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
                StageConfig { stage: 6, growth_threshold: 100.0, yield_multiplier: 1.0, quality_multiplier: 1.0 },
            ],
            variant_pool: vec!["golden".to_string(), "white".to_string()],
            variant_whitelist: vec!["golden".to_string()],
            soil_preference: vec!["loam".to_string(), "clay".to_string()],
            season_preference: vec!["spring".to_string(), "summer".to_string()],
            growth_base: 1.0,
            growth_per_minute: 1.0,
        }),

        _ => Err(Error::ConfigNotFound),
    }
}

/// 获取所有支持的作物编码
pub fn get_all_crop_codes() -> Vec<String> {
    vec![
        "tomato_v1".to_string(),
        "carrot_v1".to_string(),
        "sunflower_v1".to_string(),
        "rose_v1".to_string(),
        "wheat_v1".to_string(),
    ]
}
