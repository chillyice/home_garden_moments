pub mod types;
pub mod crop;
pub mod weather;
pub mod growth;
pub mod storage;
pub mod error;

use crate::types::{PlotState, NutrientState, CropConfig, WeatherSnapshot, VariantRollResult, CrossCheckResult, HarvestResult, PlotStatus, CropStage};
use crate::storage::{GardenSave, GardenStorage, MemoryStorage, FileStorage};
use crate::crop::get_crop_config;
use crate::weather::check_plot_status;
use crate::growth::{calculate_growth, is_mature};
use crate::error::Error;
use std::io;

/// GardenCore 核心库
pub struct GardenCore {
    storage: Box<dyn GardenStorage>,
    // TODO: 注入 WeatherService 和其他依赖
}

impl GardenCore {
    /// 创建新的 GardenCore 实例（使用内存存储）
    pub fn new() -> Self {
        Self {
            storage: Box::new(MemoryStorage::new()),
        }
    }

    /// 创建新的 GardenCore 实例（指定存储层）
    pub fn with_storage(storage: Box<dyn GardenStorage>) -> Self {
        Self { storage }
    }

    /// 创建新的 GardenCore 实例（使用文件存储）
    pub fn with_file_storage(file_path: &str) -> Result<Self, Error> {
        let mut storage = Box::new(FileStorage::new(file_path));
        storage.load_from_file()?;
        Ok(Self { storage })
    }

    // ========== 播种 ==========

    /// 播种到地块
    ///
    /// # 参数
    /// - `plot_id`: 地块 ID
    /// - `crop_code`: 作物编码
    /// - `stage`: 初始阶段（1–6）
    /// - `variant`: 可选变体（从 variant_pool 中 roll）
    ///
    /// # 返回
    /// - `Ok(PlotState)`: 新的地块状态
    /// - `Err(Error)`: 错误（如地块已占用 / 作物未解锁 / 阶段无效）
    pub fn plant(
        &self,
        plot_id: u64,
        crop_code: &str,
        stage: u8,
        variant: Option<String>,
    ) -> Result<PlotState, Error> {
        // 检查阶段是否有效
        if stage < 1 || stage > 6 {
            return Err(Error::StageInvalid);
        }
        let _ = CropStage::try_from(stage);

        // 加载作物配置
        let crop_config = get_crop_config(crop_code)?;

        // 检查阶段是否在配置中
        let stage_config = crop_config.stages
            .iter()
            .find(|s| s.stage == stage)
            .ok_or(Error::StageInvalid)?;

        // Roll 变体（如果未提供）
        let final_variant = if let Some(v) = variant {
            v
        } else {
            // 从 variant_pool 中随机选择一个变体
            let pool = &crop_config.variant_pool;
            let index = (plot_id + stage as u64) % pool.len() as u64;
            pool[index as usize].clone()
        };

        // 创建新的地块状态
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let plot_state = PlotState {
            id: plot_id,
            crop_code: crop_config.code.clone(),
            stage,
            variant: Some(final_variant),
            soil_type: "loam".to_string(),  // TODO: 从地块配置读取
            facility: None,                 // TODO: 从地块配置读取
            last_watered_at: now,
            last_sun_at: now,
            growth: 0.0,
            nutrients: NutrientState {
                nitrogen: 50.0,
                phosphorus: 50.0,
                potassium: 50.0,
                decay_at: now + 72 * 60 * 60,  // 72 小时后衰减
            },
            status: None,
        };

        // 保存到存储层
        self.storage.set_plot(&plot_state)?;

        Ok(plot_state)
    }

    // ========== 照料 ==========

    /// 浇水
    ///
    /// # 参数
    /// - `plot_id`: 地块 ID
    /// - `now`: 当前时间戳（Unix 秒）
    ///
    /// # 返回
    /// - `Ok(PlotState)`: 更新后的地块状态
    /// - `Err(Error)`: 错误（如地块未播种 / 已浇水 < 12h）
    pub fn water(
        &self,
        plot_id: u64,
        now: i64,
    ) -> Result<PlotState, Error> {
        // 从存储层读取地块状态
        let plot_state = self.storage.get_plot(plot_id)?
            .ok_or(Error::NotPlanted)?;

        // 检查冷却时间（12h）
        let time_since_watering = now - plot_state.last_watered_at;
        if time_since_watering < 12 * 60 * 60 {
            return Err(Error::AlreadyWatered);
        }

        // 更新浇水时间
        let mut updated_state = plot_state;
        updated_state.last_watered_at = now;

        // 判断状态
        let weather = WeatherSnapshot {
            condition: "sunny".to_string(),
            temperature: 25.0,
            wind: 5.0,
            event: None,
        };
        let status = check_plot_status(plot_id, &weather, updated_state.last_watered_at, updated_state.last_sun_at, false)?;
        updated_state.status = status.map(|s| s.to_string());

        // 保存到存储层
        self.storage.set_plot(&updated_state)?;

        Ok(updated_state)
    }

    /// 晒太阳
    ///
    /// # 参数
    /// - `plot_id`: 地块 ID
    /// - `weather`: 天气快照
    /// - `now`: 当前时间戳（Unix 秒）
    ///
    /// # 返回
    /// - `Ok(PlotState)`: 更新后的地块状态
    /// - `Err(Error)`: 错误（如地块未播种 / 已晒太阳 < 12h）
    pub fn sun(
        &self,
        plot_id: u64,
        weather: &WeatherSnapshot,
        now: i64,
    ) -> Result<PlotState, Error> {
        // 从存储层读取地块状态
        let plot_state = self.storage.get_plot(plot_id)?
            .ok_or(Error::NotPlanted)?;

        // 检查冷却时间（12h）
        let time_since_sun = now - plot_state.last_sun_at;
        if time_since_sun < 12 * 60 * 60 {
            return Err(Error::AlreadySun);
        }

        // 更新晒太阳时间
        let mut updated_state = plot_state;
        updated_state.last_sun_at = now;

        // 判断状态
        let status = check_plot_status(plot_id, weather, updated_state.last_watered_at, updated_state.last_sun_at, false)?;
        updated_state.status = status.map(|s| s.to_string());

        // 保存到存储层
        self.storage.set_plot(&updated_state)?;

        Ok(updated_state)
    }

    // ========== 收获 ==========

    /// 收获
    ///
    /// # 参数
    /// - `plot_id`: 地块 ID
    /// - `now`: 当前时间戳（Unix 秒）
    ///
    /// # 返回
    /// - `Ok(HarvestResult)`: 收获结果
    /// - `Err(Error)`: 错误（如地块未成熟 / 未播种）
    pub fn harvest(
        &self,
        plot_id: u64,
        now: i64,
    ) -> Result<HarvestResult, Error> {
        // 从存储层读取地块状态
        let plot_state = self.storage.get_plot(plot_id)?
            .ok_or(Error::NotPlanted)?;

        // 检查是否成熟
        let crop_config = get_crop_config(&plot_state.crop_code)?;
        if !is_mature(&plot_state, &crop_config) {
            return Err(Error::NotMature);
        }

        // 计算产量（基于成长进度、状态、土壤等）
        let base_quantity = 1.0;
        let yield_multiplier = crop_config.stages[5].yield_multiplier;  // 成熟阶段
        let quality_multiplier = crop_config.stages[5].quality_multiplier;

        // 应用状态异常对产量的影响
        let status_multiplier = match plot_state.status.as_deref() {
            Some("thirsty") | Some("缺水") => 0.7,
            Some("scorch") | Some("灼叶") => 0.6,
            Some("root_rot") | Some("沤根") => 0.5,
            Some("pest") | Some("虫害") => 0.8,
            _ => 1.0,
        };

        let quantity = base_quantity * yield_multiplier * quality_multiplier * status_multiplier;
        let quality = 80.0 + (quantity * 2.0) as f32;  // 品质 = 80 + 产量系数

        // 计算园币收入
        let coin_income = 12;  // TODO: 从配置读取

        // 计算园丁 XP
        let exp_gain = 30;  // TODO: 从配置读取

        // 触发的成就
        let achievement_triggers = vec!["first_harvest".to_string()];

        Ok(HarvestResult {
            crop_code: plot_state.crop_code.clone(),
            stage: plot_state.stage,
            variant: plot_state.variant.clone(),
            quantity,
            quality,
            coin_income,
            exp_gain,
            achievement_triggers,
        })
    }

    /// 计算成长
    ///
    /// # 参数
    /// - `plot_state`: 地块状态
    /// - `weather`: 天气快照
    /// - `time_delta`: 时间增量（分钟）
    ///
    /// # 返回
    /// - `Ok(f32)`: 成长增量
    pub fn calculate_growth(
        &self,
        plot_state: &PlotState,
        weather: &WeatherSnapshot,
        time_delta: i64,
    ) -> Result<f32, Error> {
        // 加载作物配置
        let crop_config = get_crop_config(&plot_state.crop_code)?;

        // 调用成长计算函数
        calculate_growth(
            plot_state,
            &crop_config,
            weather,
            time_delta,
            1.0,  // 土壤匹配（默认 loam）
            1.0,  // 季节倍率
            1.0,  // 气温适配
            1.0,  // 营养速率
            1,    // 园丁等级
            0.0,  // 图鉴百分比
            false, // 是否室内场域
        )
    }

    // ========== 判定 ==========

    /// 变体 roll
    ///
    /// # 参数
    /// - `crop_code`: 作物编码
    /// - `seed`: 种子（用于确定性 roll，如 plot_id + harvest_count）
    ///
    /// # 返回
    /// - `Ok(VariantRollResult)`: 变体 roll 结果
    pub fn variant_roll(
        &self,
        crop_code: &str,
        seed: u64,
    ) -> Result<VariantRollResult, Error> {
        let crop_config = get_crop_config(crop_code)?;

        let pool = &crop_config.variant_pool;
        let index = seed % pool.len() as u64;
        let variant = pool[index as usize].clone();

        // 简单的稀有度分配
        let rarity = if index % 10 == 0 {
            "legendary".to_string()
        } else if index % 3 == 0 {
            "epic".to_string()
        } else if index % 2 == 0 {
            "rare".to_string()
        } else {
            "common".to_string()
        };

        let probability = 1.0 / pool.len() as f32;

        Ok(VariantRollResult {
            variant,
            rarity,
            probability,
        })
    }

    /// 重茬判定
    ///
    /// # 参数
    /// - `plot_id`: 地块 ID
    /// - `crop_code`: 作物编码
    /// - `soil_type`: 土壤类型
    /// - `season`: 当前季节
    ///
    /// # 返回
    /// - `Ok(CrossCheckResult)`: 重茬判定结果
    pub fn cross_check(
        &self,
        plot_id: u64,
        crop_code: &str,
        soil_type: &str,
        season: &str,
    ) -> Result<CrossCheckResult, Error> {
        // TODO: 实现重茬判定逻辑
        // 简单实现：总是返回正常
        Ok(CrossCheckResult {
            is_cross: false,
            penalty_multiplier: 1.0,
            reason: "无重茬".to_string(),
        })
    }

    /// 状态判定
    ///
    /// # 参数
    /// - `plot_id`: 地块 ID
    /// - `weather`: 天气快照
    /// - `now`: 当前时间戳（Unix 秒）
    ///
    /// # 返回
    /// - `Ok(Option<String>)`: 状态码（如 Some("drought") / Some("frost") / None）
    pub fn check_status(
        &self,
        plot_id: u64,
        weather: &WeatherSnapshot,
        now: i64,
    ) -> Result<Option<String>, Error> {
        // 从存储层读取地块状态
        let plot_state = self.storage.get_plot(plot_id)?
            .ok_or(Error::NotPlanted)?;

        let status = check_plot_status(plot_id, weather, plot_state.last_watered_at, plot_state.last_sun_at, false)?;

        Ok(status.map(|s| s.to_string()))
    }

    /// 导出存档
    pub fn export_save(&self) -> Result<GardenSave, Error> {
        self.storage.export_save()
    }

    /// 导入存档
    pub fn import_save(&mut self, save: &GardenSave) -> Result<(), Error> {
        self.storage.import_save(save)
    }

    /// 保存存档
    pub fn save(&mut self) -> Result<(), Error> {
        self.storage.save()
    }

    /// 加载存档
    pub fn load(&mut self) -> Result<Option<GardenSave>, Error> {
        self.storage.load()
    }
}
