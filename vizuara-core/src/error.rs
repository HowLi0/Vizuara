use thiserror::Error;

/// Vizuara 库的错误类型
#[derive(Error, Debug)]
pub enum VizuaraError {
    #[error("Invalid color format: {0}")]
    InvalidColor(String),
    
    #[error("Invalid data: {0}")]
    InvalidData(String),
    
    #[error("Rendering error: {0}")]
    RenderError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
}

/// 结果类型的便捷别名
pub type Result<T> = std::result::Result<T, VizuaraError>;
