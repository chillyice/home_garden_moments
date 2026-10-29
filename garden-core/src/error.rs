use serde::{Serialize, Deserialize};
use std::io;

/// 核心错误类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
    PlotOccupied,           // 地块已占用
    CropNotUnlocked,        // 作物未解锁
    StageInvalid,           // 阶段无效
    PlotNotFound,           // 地块不存在
    AlreadyWatered,         // 已浇水 < 12h
    AlreadySun,             // 已晒太阳 < 12h
    NutrientFull,           // 营养已满
    NotMature,              // 未成熟
    NotPlanted,             // 未播种
    CrossCheckFailed,       // 重茬判定失败
    StatusCheckFailed,      // 状态判定失败
    ConfigNotFound,         // 配置不存在
    WeatherNotFound,        // 天气数据缺失
    IoError(String),        // IO 错误信息
    JsonError(String),      // JSON 错误信息
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PlotOccupied => write!(f, "地块已占用"),
            Error::CropNotUnlocked => write!(f, "作物未解锁"),
            Error::StageInvalid => write!(f, "阶段无效"),
            Error::PlotNotFound => write!(f, "地块不存在"),
            Error::AlreadyWatered => write!(f, "已浇水，12h 后才能再次浇水"),
            Error::AlreadySun => write!(f, "已晒太阳，12h 后才能再次晒太阳"),
            Error::NutrientFull => write!(f, "营养已满"),
            Error::NotMature => write!(f, "作物未成熟，无法收获"),
            Error::NotPlanted => write!(f, "地块未播种"),
            Error::CrossCheckFailed => write!(f, "重茬判定失败"),
            Error::StatusCheckFailed => write!(f, "状态判定失败"),
            Error::ConfigNotFound => write!(f, "作物配置不存在"),
            Error::WeatherNotFound => write!(f, "天气数据缺失"),
            Error::IoError(msg) => write!(f, "IO 错误: {}", msg),
            Error::JsonError(msg) => write!(f, "JSON 错误: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// 错误转换：Rust Error → String（供前端使用）
impl From<Error> for String {
    fn from(err: Error) -> Self {
        err.to_string()
    }
}

/// 错误转换：std::io::Error → Error
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err.to_string())
    }
}

/// 错误转换：serde_json::Error → Error
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err.to_string())
    }
}
