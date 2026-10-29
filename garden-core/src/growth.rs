use crate::types::{PlotState, NutrientState, CropConfig, StageConfig, WeatherSnapshot};
use crate::error::Error;
use crate::weather::{is_drought, is_frost, is_heat_stress, is_cold_stress};

/// 计算成长增量
///
/// # 参数
/// - `plot_state`: 地块状态
/// - `crop_config`: 作物配置
/// - `weather`: 天气快照
/// - `time_delta`: 时间增量（分钟）
/// - `soil_multiplier`: 土壤匹配倍率
/// - `season_multiplier`: 季节倍率
/// - `temperature_multiplier`: 气温适配倍率
/// - `nutrient_multiplier`: 营养速率倍率
/// - `gardener_level`: 园丁等级（用于永久加成）
/// - `collection_percentage`: 图鉴百分比（用于永久加成）
/// - `is_indoor`: 是否室内场域
///
/// # 返回
/// - `Ok(f32)`: 成长增量
/// - `Err(Error)`: 错误
pub fn calculate_growth(
    plot_state: &PlotState,
    crop_config: &CropConfig,
    weather: &WeatherSnapshot,
    time_delta: i64,
    soil_multiplier: f32,
    season_multiplier: f32,
    temperature_multiplier: f32,
    nutrient_multiplier: f32,
    gardener_level: u32,
    collection_percentage: f32,
    is_indoor: bool,
) -> Result<f32, Error> {
    // 1. 检查是否成熟
    if plot_state.growth >= 100.0 {
        return Ok(0.0);
    }

    // 2. 检查是否处于霜冻状态（冻结）
    if is_frost(weather) {
        return Ok(0.0);
    }

    // 3. 判断状态异常（干旱、灼叶、沤根、虫害）
    let status_multiplier = match plot_state.status.as_deref() {
        Some("thirsty") | Some("缺水") => 0.7,
        Some("scorch") | Some("灼叶") => 0.6,
        Some("root_rot") | Some("沤根") => 0.5,
        Some("pest") | Some("虫害") => 0.8,
        _ => 1.0,
    };

    // 4. 判断是否干旱（用于成长计算）
    let drought_multiplier = if is_drought(weather, plot_state.last_watered_at) {
        0.7
    } else {
        1.0
    };

    // 5. 判断是否高温胁迫（用于成长计算）
    let heat_stress_multiplier = if is_heat_stress(weather) {
        0.7
    } else {
        1.0
    };

    // 6. 判断是否低温胁迫（用于成长计算）
    let cold_stress_multiplier = 1.0;
    // TODO: 实现作物耐寒下限，从 crop_config 读取

    // 7. 计算基础成长
    let base_growth = crop_config.growth_per_minute * (time_delta as f32) * crop_config.growth_base;

    // 8. 计算所有乘子
    let total_multiplier = 1.0
        * soil_multiplier
        * season_multiplier
        * temperature_multiplier
        * nutrient_multiplier
        * status_multiplier
        * drought_multiplier
        * heat_stress_multiplier
        * cold_stress_multiplier
        * (1.0 + 0.05 * (gardener_level - 1) as f32)      // 园丁等级加成
        * (1.0 + collection_percentage / 100.0)          // 图鉴加成
        * 1.0;                                           // 设施修正（暂时硬编码为 1.0）

    // 9. 应用乘数硬顶 ×3.0
    let capped_multiplier = total_multiplier.min(3.0);

    // 10. 计算成长增量
    let growth_increment = base_growth * capped_multiplier;

    // 11. 应用状态异常对产量的影响（暂时在 harvest() 中处理）
    // TODO: 在 harvest() 中应用状态产量系数

    Ok(growth_increment)
}

/// 计算照料加成
///
/// # 参数
/// - `plot_state`: 地块状态
/// - `time_delta`: 时间增量（分钟）
///
/// # 返回
/// - `Ok(f32)`: 照料加成
pub fn calculate_tending_bonus(plot_state: &PlotState, time_delta: i64) -> Result<f32, Error> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let mut bonus = 0.0;

    // 浇水加成（每次 +120 分钟）
    let time_since_watering = now - plot_state.last_watered_at;
    let watering_count = (time_since_watering / (120 * 60)) as f32;
    bonus += watering_count * 120.0;

    // 晒太阳加成（每次 +120 分钟）
    let time_since_sun = now - plot_state.last_sun_at;
    let sun_count = (time_since_sun / (120 * 60)) as f32;
    bonus += sun_count * 120.0;

    // 施肥加成（每次 +360 分钟）
    // TODO: 实现施肥次数统计
    // let fertilize_count = plot_state.fertilize_count;
    // bonus += fertilize_count * 360.0;

    Ok(bonus)
}

/// 判断作物是否成熟
///
/// # 参数
/// - `plot_state`: 地块状态
/// - `crop_config`: 作物配置
///
/// # 返回
/// - `Ok(bool)`: 是否成熟
pub fn is_mature(plot_state: &PlotState, crop_config: &CropConfig) -> bool {
    plot_state.growth >= 100.0
}

/// 获取当前阶段
///
/// # 参数
/// - `plot_state`: 地块状态
/// - `crop_config`: 作物配置
///
/// # 返回
/// - `Ok(u8)`: 当前阶段（1–6）
pub fn get_current_stage(plot_state: &PlotState, crop_config: &CropConfig) -> Result<u8, Error> {
    // 根据成长进度计算阶段
    for stage_config in &crop_config.stages {
        if plot_state.growth < stage_config.growth_threshold {
            return Ok(stage_config.stage - 1);
        }
    }
    Ok(6) // 成熟
}
