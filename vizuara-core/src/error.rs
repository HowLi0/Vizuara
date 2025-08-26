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
    
    #[error("Interactive error: {0}")]
    InteractiveError(String),
}

impl From<String> for VizuaraError {
    fn from(msg: String) -> Self {
        VizuaraError::InteractiveError(msg)
    }
}

impl From<&str> for VizuaraError {
    fn from(msg: &str) -> Self {
        VizuaraError::InteractiveError(msg.to_string())
    }
}

/// 结果类型的便捷别名
pub type Result<T> = std::result::Result<T, VizuaraError>;
