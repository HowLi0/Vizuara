//! Vizuara 导出功能模块
//!
//! 提供将可视化内容导出为各种格式的功能：
//! - SVG（矢量格式）
//! - PNG（位图格式）
//! - 其他格式支持

pub mod common;
pub mod error;
pub mod png;
pub mod svg;

pub use common::{ExportFormat, ExportOptions};
pub use error::{ExportError, ExportResult};

use vizuara_core::{Primitive, Style};

/// 导出器特征
pub trait Exporter {
    /// 导出到指定路径
    fn export_to_file(
        &self,
        primitives: &[Primitive],
        styles: &[Style],
        width: u32,
        height: u32,
        path: &str,
        options: &ExportOptions,
    ) -> ExportResult<()>;

    /// 导出到字节数组
    fn export_to_bytes(
        &self,
        primitives: &[Primitive],
        styles: &[Style],
        width: u32,
        height: u32,
        options: &ExportOptions,
    ) -> ExportResult<Vec<u8>>;

    /// 获取支持的格式
    fn supported_format(&self) -> ExportFormat;
}

/// 导出管理器
pub struct ExportManager;

impl ExportManager {
    /// 创建新的导出管理器
    pub fn new() -> Self {
        Self
    }

    /// 导出为SVG格式
    pub fn export_svg(
        primitives: &[Primitive],
        styles: &[Style],
        width: u32,
        height: u32,
        path: &str,
        options: Option<ExportOptions>,
    ) -> ExportResult<()> {
        let exporter = svg::SvgExporter::new();
        let opts = options.unwrap_or_default();
        exporter.export_to_file(primitives, styles, width, height, path, &opts)
    }

    /// 导出为PNG格式
    pub fn export_png(
        primitives: &[Primitive],
        styles: &[Style],
        width: u32,
        height: u32,
        path: &str,
        options: Option<ExportOptions>,
    ) -> ExportResult<()> {
        let exporter = png::PngExporter::new();
        let opts = options.unwrap_or_default();
        exporter.export_to_file(primitives, styles, width, height, path, &opts)
    }

    /// 自动检测格式并导出
    pub fn export_auto(
        primitives: &[Primitive],
        styles: &[Style],
        width: u32,
        height: u32,
        path: &str,
        options: Option<ExportOptions>,
    ) -> ExportResult<()> {
        let format = ExportFormat::from_extension(path)?;
        let opts = options.unwrap_or_default();

        match format {
            ExportFormat::Svg => {
                Self::export_svg(primitives, styles, width, height, path, Some(opts))
            }
            ExportFormat::Png => {
                Self::export_png(primitives, styles, width, height, path, Some(opts))
            }
        }
    }
}

impl Default for ExportManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point2;
    use tempfile::tempdir;
    use vizuara_core::{Color, Primitive};

    #[test]
    fn test_export_manager_creation() {
        let _manager = ExportManager::new();
        let _also_manager = ExportManager::new();
        // 基本创建测试
    }

    #[test]
    fn test_export_format_detection() {
        assert_eq!(
            ExportFormat::from_extension("test.svg").unwrap(),
            ExportFormat::Svg
        );
        assert_eq!(
            ExportFormat::from_extension("test.png").unwrap(),
            ExportFormat::Png
        );
        assert!(ExportFormat::from_extension("test.txt").is_err());
    }

    #[test]
    fn test_simple_svg_export() -> ExportResult<()> {
        let primitives = vec![Primitive::Circle {
            center: Point2::new(50.0, 50.0),
            radius: 20.0,
        }];
        let styles = vec![vizuara_core::Style::new().fill_color(Color::rgb(1.0, 0.0, 0.0))];

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.svg");

        ExportManager::export_svg(
            &primitives,
            &styles,
            100,
            100,
            file_path.to_str().unwrap(),
            None,
        )?;

        assert!(file_path.exists());
        Ok(())
    }
}
