use crate::types::{PlotState, NutrientState, CropConfig, WeatherSnapshot, VariantRollResult, CrossCheckResult, HarvestResult, PlotStatus};
use crate::error::Error;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

// ========== 存档格式（GardenSave v1）==========

/// 存档头
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GardenSaveHeader {
    pub version: String,          // "1.0"
    pub content_version: String,  // content/*.json 的 SHA256 哈希
    pub world_id: String,         // ULID
    pub created_at: i64,          // Unix timestamp（秒）
    pub last_synced: i64,         // Unix timestamp（秒）
}

/// 存档数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GardenSaveData {
    pub plots: HashMap<u64, PlotState>,  // plot_id -> PlotState
    pub members: Vec<Member>,            // 成员列表
    pub codex: HashMap<String, CodexEntry>,  // crop_code -> CodexEntry
    pub achievements: HashMap<String, u64>,  // code -> timestamp
    pub settings: Settings,              // 世界设置
}

/// 成员
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub player_uid: String,
    pub nickname: String,
    pub specialties: Vec<String>,
    pub last_active_date: i64,
}

/// 图鉴条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexEntry {
    pub crop_code: String,
    pub collected_variants: Vec<String>,
    pub total_harvests: u32,
}

/// 世界设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub time_ratio: f32,         // 时间比例
    pub daily_points_cap: u32,   // 每日积分上限
    pub ai_suggest_enabled: bool,
}

/// 完整存档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GardenSave {
    pub header: GardenSaveHeader,
    pub data: GardenSaveData,
}

// ========== 存储层接口 ==========

/// 存储层接口
pub trait GardenStorage: Send + Sync {
    /// 获取地块状态
    fn get_plot(&self, plot_id: u64) -> Result<Option<PlotState>, Error>;

    /// 设置地块状态
    fn set_plot(&self, plot_state: &PlotState) -> Result<(), Error>;

    /// 删除地块
    fn delete_plot(&self, plot_id: u64) -> Result<(), Error>;

    /// 列出所有地块
    fn list_plots(&self) -> Result<Vec<PlotState>, Error>;

    /// 导出存档
    fn export_save(&self) -> Result<GardenSave, Error>;

    /// 导入存档
    fn import_save(&self, save: &GardenSave) -> Result<(), Error>;

    /// 保存存档（根据模式选择文件存储或数据库存储）
    fn save(&self) -> Result<(), Error>;

    /// 加载存档
    fn load(&self) -> Result<Option<GardenSave>, Error>;
}

// ========== 内存存储（MemoryStorage）==========

/// 内存存储（用于测试）
pub struct MemoryStorage {
    plots: Arc<Mutex<HashMap<u64, PlotState>>>,
    save: Arc<Mutex<Option<GardenSave>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            plots: Arc::new(Mutex::new(HashMap::new())),
            save: Arc::new(Mutex::new(None)),
        }
    }

    pub fn with_save(save: GardenSave) -> Self {
        Self {
            plots: Arc::new(Mutex::new(save.data.plots.clone())),
            save: Arc::new(Mutex::new(Some(save))),
        }
    }
}

impl GardenStorage for MemoryStorage {
    fn get_plot(&self, plot_id: u64) -> Result<Option<PlotState>, Error> {
        Ok(self.plots.lock().unwrap().get(&plot_id).cloned())
    }

    fn set_plot(&self, plot_state: &PlotState) -> Result<(), Error> {
        self.plots.lock().unwrap().insert(plot_state.id, plot_state.clone());
        Ok(())
    }

    fn delete_plot(&self, plot_id: u64) -> Result<(), Error> {
        self.plots.lock().unwrap().remove(&plot_id);
        Ok(())
    }

    fn list_plots(&self) -> Result<Vec<PlotState>, Error> {
        Ok(self.plots.lock().unwrap().values().cloned().collect())
    }

    fn export_save(&self) -> Result<GardenSave, Error> {
        let save = self.save.lock().unwrap();
        save.as_ref().cloned().ok_or(Error::ConfigNotFound)
    }

    fn import_save(&self, save: &GardenSave) -> Result<(), Error> {
        self.plots.lock().unwrap().clear();
        self.plots.lock().unwrap().extend(save.data.plots.clone());
        *self.save.lock().unwrap() = Some(save.clone());
        Ok(())
    }

    fn save(&self) -> Result<(), Error> {
        let save = self.save.lock().unwrap();
        let save = save.as_ref().ok_or(Error::ConfigNotFound)?;
        self.plots.lock().unwrap().clear();
        self.plots.lock().unwrap().extend(save.data.plots.clone());
        Ok(())
    }

    fn load(&self) -> Result<Option<GardenSave>, Error> {
        Ok(self.save.lock().unwrap().as_ref().cloned())
    }
}

// ========== 文件存储（FileStorage）==========

/// 文件存储（用于 Solo 模式）
pub struct FileStorage {
    file_path: String,
    plots: Arc<Mutex<HashMap<u64, PlotState>>>,
    save: Arc<Mutex<Option<GardenSave>>>,
}

impl FileStorage {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
            plots: Arc::new(Mutex::new(HashMap::new())),
            save: Arc::new(Mutex::new(None)),
        }
    }

    /// 从文件加载
    pub fn load_from_file(&self) -> Result<(), Error> {
        if Path::new(&self.file_path).exists() {
            let content = fs::read_to_string(&self.file_path)?;
            let save: GardenSave = serde_json::from_str(&content)?;
            self.plots.lock().unwrap().clear();
            self.plots.lock().unwrap().extend(save.data.plots.clone());
            *self.save.lock().unwrap() = Some(save);
        }
        Ok(())
    }

    /// 保存到文件
    pub fn save_to_file(&self) -> Result<(), Error> {
        let save = self.save.lock().unwrap();
        if let Some(save) = save.as_ref() {
            let content = serde_json::to_string_pretty(save)?;
            fs::write(&self.file_path, content)?;
        }
        Ok(())
    }
}

impl GardenStorage for FileStorage {
    fn get_plot(&self, plot_id: u64) -> Result<Option<PlotState>, Error> {
        Ok(self.plots.lock().unwrap().get(&plot_id).cloned())
    }

    fn set_plot(&self, plot_state: &PlotState) -> Result<(), Error> {
        self.plots.lock().unwrap().insert(plot_state.id, plot_state.clone());
        Ok(())
    }

    fn delete_plot(&self, plot_id: u64) -> Result<(), Error> {
        self.plots.lock().unwrap().remove(&plot_id);
        Ok(())
    }

    fn list_plots(&self) -> Result<Vec<PlotState>, Error> {
        Ok(self.plots.lock().unwrap().values().cloned().collect())
    }

    fn export_save(&self) -> Result<GardenSave, Error> {
        let save = self.save.lock().unwrap();
        save.as_ref().cloned().ok_or(Error::ConfigNotFound)
    }

    fn import_save(&self, save: &GardenSave) -> Result<(), Error> {
        self.plots.lock().unwrap().clear();
        self.plots.lock().unwrap().extend(save.data.plots.clone());
        *self.save.lock().unwrap() = Some(save.clone());
        Ok(())
    }

    fn save(&self) -> Result<(), Error> {
        self.save_to_file()
    }

    fn load(&self) -> Result<Option<GardenSave>, Error> {
        if Path::new(&self.file_path).exists() {
            let content = fs::read_to_string(&self.file_path)?;
            let save: GardenSave = serde_json::from_str(&content)?;
            self.plots.lock().unwrap().clear();
            self.plots.lock().unwrap().extend(save.data.plots.clone());
            *self.save.lock().unwrap() = Some(save);
        }
        Ok(None)
    }
}

// ========== 数据库存储（DatabaseStorage）==========

/// 数据库存储（用于 Server 模式）
pub struct DatabaseStorage {
    // TODO: 集成 MySQL / PostgreSQL
    // pub db: sqlx::MySqlPool,
}

impl DatabaseStorage {
    pub fn new() -> Self {
        Self {}
    }
}

impl GardenStorage for DatabaseStorage {
    fn get_plot(&self, plot_id: u64) -> Result<Option<PlotState>, Error> {
        // TODO: 从数据库读取
        Ok(None)
    }

    fn set_plot(&self, plot_state: &PlotState) -> Result<(), Error> {
        // TODO: 写入数据库
        Ok(())
    }

    fn delete_plot(&self, plot_id: u64) -> Result<(), Error> {
        // TODO: 从数据库删除
        Ok(())
    }

    fn list_plots(&self) -> Result<Vec<PlotState>, Error> {
        // TODO: 从数据库读取所有地块
        Ok(Vec::new())
    }

    fn export_save(&self) -> Result<GardenSave, Error> {
        // TODO: 从数据库导出
        Err(Error::ConfigNotFound)
    }

    fn import_save(&self, save: &GardenSave) -> Result<(), Error> {
        // TODO: 导入到数据库
        Ok(())
    }

    fn save(&self) -> Result<(), Error> {
        // TODO: 保存到数据库
        Ok(())
    }

    fn load(&self) -> Result<Option<GardenSave>, Error> {
        // TODO: 从数据库加载
        Ok(None)
    }
}

// ========== 辅助函数 ==========

/// 生成 ULID
pub fn generate_ulid() -> String {
    ulid::Ulid::new().to_string()
}

/// 获取当前时间戳（Unix 秒）
pub fn current_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// 创建新存档
pub fn create_save(content_version: &str, world_id: &str) -> GardenSave {
    let now = current_timestamp();
    GardenSave {
        header: GardenSaveHeader {
            version: "1.0".to_string(),
            content_version: content_version.to_string(),
            world_id: world_id.to_string(),
            created_at: now,
            last_synced: now,
        },
        data: GardenSaveData {
            plots: HashMap::new(),
            members: Vec::new(),
            codex: HashMap::new(),
            achievements: HashMap::new(),
            settings: Settings {
                time_ratio: 1.0,
                daily_points_cap: 40,
                ai_suggest_enabled: true,
            },
        },
    }
}
