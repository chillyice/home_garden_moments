use napi::bindgen_prelude::*;
use garden_core::{GardenCore, PlotState, WeatherSnapshot, VariantRollResult, CrossCheckResult, HarvestResult, Error};
use serde::{Serialize, Deserialize};

/// 错误转换：Rust Error → String
#[derive(Debug, Serialize, Deserialize)]
struct ErrorWrapper {
    message: String,
}

impl From<Error> for ErrorWrapper {
    fn from(err: Error) -> Self {
        ErrorWrapper {
            message: err.to_string(),
        }
    }
}

/// GardenCore 原生绑定
#[napi]
pub struct GardenNative {
    core: GardenCore,
}

#[napi]
impl GardenNative {
    /// 创建新的 GardenCore 实例
    pub fn new() -> GardenNative {
        GardenNative {
            core: GardenCore::new(),
        }
    }

    /// 播种到地块
    pub fn plant(
        &self,
        plot_id: u64,
        crop_code: &str,
        stage: u8,
        variant: Option<String>,
    ) -> Result<PlotStateWrapper, ErrorWrapper> {
        let plot_state = self.core.plant(plot_id, crop_code, stage, variant)?;
        Ok(PlotStateWrapper(plot_state))
    }

    /// 浇水
    pub fn water(&self, plot_id: u64, now: i64) -> Result<PlotStateWrapper, ErrorWrapper> {
        let plot_state = self.core.water(plot_id, now)?;
        Ok(PlotStateWrapper(plot_state))
    }

    /// 晒太阳
    pub fn sun(
        &self,
        plot_id: u64,
        weather_json: String,
        now: i64,
    ) -> Result<PlotStateWrapper, ErrorWrapper> {
        let weather: WeatherSnapshot = serde_json::from_str(&weather_json).map_err(|e| {
            ErrorWrapper {
                message: format!("Failed to parse weather JSON: {}", e),
            }
        })?;
        let plot_state = self.core.sun(plot_id, &weather, now)?;
        Ok(PlotStateWrapper(plot_state))
    }

    /// 收获
    pub fn harvest(&self, plot_id: u64, now: i64) -> Result<HarvestResultWrapper, ErrorWrapper> {
        let harvest_result = self.core.harvest(plot_id, now)?;
        Ok(HarvestResultWrapper(harvest_result))
    }

    /// 计算成长
    pub fn calculate_growth(
        &self,
        plot_state_json: String,
        weather_json: String,
        time_delta: i64,
    ) -> Result<f64, ErrorWrapper> {
        let plot_state: PlotState = serde_json::from_str(&plot_state_json).map_err(|e| {
            ErrorWrapper {
                message: format!("Failed to parse plot state JSON: {}", e),
            }
        })?;
        let weather: WeatherSnapshot = serde_json::from_str(&weather_json).map_err(|e| {
            ErrorWrapper {
                message: format!("Failed to parse weather JSON: {}", e),
            }
        })?;
        let growth = self.core.calculate_growth(&plot_state, &weather, time_delta)?;
        Ok(growth as f64)
    }

    /// 变体 roll
    pub fn variant_roll(&self, crop_code: &str, seed: u64) -> Result<VariantRollResultWrapper, ErrorWrapper> {
        let result = self.core.variant_roll(crop_code, seed)?;
        Ok(VariantRollResultWrapper(result))
    }

    /// 重茬判定
    pub fn cross_check(
        &self,
        plot_id: u64,
        crop_code: &str,
        soil_type: &str,
        season: &str,
    ) -> Result<CrossCheckResultWrapper, ErrorWrapper> {
        let result = self.core.cross_check(plot_id, crop_code, soil_type, season)?;
        Ok(CrossCheckResultWrapper(result))
    }

    /// 状态判定
    pub fn check_status(
        &self,
        plot_id: u64,
        weather_json: String,
        now: i64,
    ) -> Result<Option<String>, ErrorWrapper> {
        let weather: WeatherSnapshot = serde_json::from_str(&weather_json).map_err(|e| {
            ErrorWrapper {
                message: format!("Failed to parse weather JSON: {}", e),
            }
        })?;
        let status = self.core.check_status(plot_id, &weather, now)?;
        Ok(status.map(|s| s.to_string()))
    }
}

/// PlotState 包装器（用于原生序列化）
#[derive(Debug, Serialize, Deserialize)]
pub struct PlotStateWrapper(pub PlotState);

/// HarvestResult 包装器（用于原生序列化）
#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestResultWrapper(pub HarvestResult);

/// VariantRollResult 包装器（用于原生序列化）
#[derive(Debug, Serialize, Deserialize)]
pub struct VariantRollResultWrapper(pub VariantRollResult);

/// CrossCheckResult 包装器（用于原生序列化）
#[derive(Debug, Serialize, Deserialize)]
pub struct CrossCheckResultWrapper(pub CrossCheckResult);
