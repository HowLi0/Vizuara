use thiserror::Error;

/// 导出错误类型
#[derive(Error, Debug)]
pub enum ExportError {
    /// IO错误
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    /// 格式不支持
    #[error("不支持的格式: {0}")]
    UnsupportedFormat(String),

    /// SVG生成错误
    #[error("SVG生成错误: {0}")]
    SvgError(String),

    /// PNG生成错误
    #[error("PNG生成错误: {0}")]
    PngError(String),

    /// 渲染错误
    #[error("渲染错误: {0}")]
    RenderError(String),

    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),
}

/// 导出结果类型
pub type ExportResult<T> = Result<T, ExportError>;
