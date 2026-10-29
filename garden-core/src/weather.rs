use crate::types::{WeatherSnapshot, PlotStatus};
use crate::error::Error;

/// 判断状态（干旱 / 霜冻 / 灼叶 / 沤根 / 虫害 / 锦化）
pub fn check_plot_status(
    plot_id: u64,
    weather: &WeatherSnapshot,
    last_watered_at: i64,
    last_sun_at: i64,
    is_indoor: bool,
) -> Result<Option<PlotStatus>, Error> {
    const WATERING_COOLDOWN: i64 = 12 * 60 * 60;  // 12 小时（秒）
    const SUN_COOLDOWN: i64 = 12 * 60 * 60;      // 12 小时（秒）

    // 室内场域不触发状态
    if is_indoor {
        return Ok(None);
    }

    // 计算距离上次浇水的时间
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let time_since_watering = now - last_watered_at;

    // 1. 缺水判定
    if time_since_watering > WATERING_COOLDOWN {
        return Ok(Some(PlotStatus::Thirsty));
    }

    // 2. 灼叶判定（气温 ≥ 35℃ + 晴天 + 露天 + 无遮阳棚）
    if weather.temperature >= 35.0
        && weather.condition == "sunny"
        && !is_indoor
    {
        return Ok(Some(PlotStatus::Scorch));
    }

    // 3. 霜冻判定（气温 ≤ 0℃）
    if weather.temperature <= 0.0 {
        return Ok(Some(PlotStatus::Frost));
    }

    // 4. 沤根判定（雨后 24h 内又浇水）
    if time_since_watering <= 24 * 60 * 60 && weather.condition == "rainy" {
        return Ok(Some(PlotStatus::RootRot));
    }

    // 5. 虫害判定（气温 25~32℃ + 连续阴雨 ≥ 2 天）
    if (weather.temperature >= 25.0 && weather.temperature <= 32.0)
        && weather.condition == "rainy"
    {
        return Ok(Some(PlotStatus::Pest));
    }

    // 6. 锦化判定（无任何异常 + 距上次照料 ≤ 24h + 本茬照料 ≥ 3 次）
    let time_since_sun = now - last_sun_at;
    if time_since_sun <= 24 * 60 * 60 {
        // TODO: 实现本茬照料次数统计
        // 暂时返回 None，后续补充
        return Ok(None);
    }

    // 无异常
    Ok(None)
}

/// 判断是否干旱（用于成长计算）
pub fn is_drought(weather: &WeatherSnapshot, last_watered_at: i64) -> bool {
    const WATERING_COOLDOWN: i64 = 48 * 60 * 60;  // 48 小时（秒）

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let time_since_watering = now - last_watered_at;

    time_since_watering > WATERING_COOLDOWN && weather.condition == "sunny"
}

/// 判断是否霜冻（用于成长计算）
pub fn is_frost(weather: &WeatherSnapshot) -> bool {
    weather.temperature <= 0.0
}

/// 判断是否高温胁迫（用于成长计算）
pub fn is_heat_stress(weather: &WeatherSnapshot) -> bool {
    weather.temperature > 35.0
}

/// 判断是否低温胁迫（用于成长计算）
pub fn is_cold_stress(weather: &WeatherSnapshot, crop_min_temp: f32) -> bool {
    weather.temperature < crop_min_temp
}
