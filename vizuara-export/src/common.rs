use crate::error::{ExportError, ExportResult};

/// 导出格式枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportFormat {
    /// SVG矢量格式
    Svg,
    /// PNG位图格式
    Png,
}

impl ExportFormat {
    /// 从文件扩展名推断格式
    pub fn from_extension(path: &str) -> ExportResult<Self> {
        let extension = std::path::Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| ExportError::UnsupportedFormat("无法确定文件扩展名".to_string()))?
            .to_lowercase();

        match extension.as_str() {
            "svg" => Ok(ExportFormat::Svg),
            "png" => Ok(ExportFormat::Png),
            _ => Err(ExportError::UnsupportedFormat(format!(
                "不支持的格式: {}",
                extension
            ))),
        }
    }

    /// 获取文件扩展名
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Svg => "svg",
            ExportFormat::Png => "png",
        }
    }

    /// 获取MIME类型
    pub fn mime_type(&self) -> &'static str {
        match self {
            ExportFormat::Svg => "image/svg+xml",
            ExportFormat::Png => "image/png",
        }
    }
}

/// 导出选项
#[derive(Debug, Clone)]
pub struct ExportOptions {
    /// 背景颜色 (None表示透明)
    pub background_color: Option<vizuara_core::Color>,
    /// DPI设置（仅对位图格式有效）
    pub dpi: f32,
    /// 质量设置（0.0-1.0，仅对部分格式有效）
    pub quality: f32,
    /// 是否包含元数据
    pub include_metadata: bool,
    /// 自定义属性
    pub custom_attributes: std::collections::HashMap<String, String>,
    /// 抗锯齿设置
    pub anti_aliasing: bool,
    /// 视图框边距（像素）
    pub margin: f32,
    /// 是否启用压缩（对支持的格式）
    pub compression: bool,
    /// 默认点大小（用于Point原语渲染）
    pub default_point_size: f32,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            background_color: None, // 透明背景
            dpi: 300.0,             // 高DPI适合打印
            quality: 0.95,          // 高质量
            include_metadata: true,
            custom_attributes: std::collections::HashMap::new(),
            anti_aliasing: true,
            margin: 0.0,
            compression: true,
            default_point_size: 2.0,
        }
    }
}

impl ExportOptions {
    /// 创建新的导出选项
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置背景颜色
    pub fn with_background(mut self, color: vizuara_core::Color) -> Self {
        self.background_color = Some(color);
        self
    }

    /// 设置透明背景
    pub fn with_transparent_background(mut self) -> Self {
        self.background_color = None;
        self
    }

    /// 设置DPI
    pub fn with_dpi(mut self, dpi: f32) -> Self {
        self.dpi = dpi;
        self
    }

    /// 设置质量
    pub fn with_quality(mut self, quality: f32) -> Self {
        self.quality = quality.clamp(0.0, 1.0);
        self
    }

    /// 设置是否包含元数据
    pub fn with_metadata(mut self, include: bool) -> Self {
        self.include_metadata = include;
        self
    }

    /// 添加自定义属性
    pub fn with_custom_attribute(mut self, key: String, value: String) -> Self {
        self.custom_attributes.insert(key, value);
        self
    }

    /// 设置抗锯齿
    pub fn with_anti_aliasing(mut self, enabled: bool) -> Self {
        self.anti_aliasing = enabled;
        self
    }

    /// 设置边距
    pub fn with_margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    /// 设置压缩
    pub fn with_compression(mut self, enabled: bool) -> Self {
        self.compression = enabled;
        self
    }

    /// 设置默认点大小
    pub fn with_point_size(mut self, size: f32) -> Self {
        self.default_point_size = size;
        self
    }

    /// 高质量设置预设（适合打印）
    pub fn high_quality() -> Self {
        Self::default()
            .with_dpi(300.0)
            .with_quality(1.0)
            .with_anti_aliasing(true)
            .with_compression(false)
    }

    /// 网络友好设置预设
    pub fn web_friendly() -> Self {
        Self::default()
            .with_dpi(96.0)
            .with_quality(0.8)
            .with_compression(true)
    }
}

/// 坐标变换辅助函数
pub fn scale_primitive(
    primitive: &vizuara_core::Primitive,
    scale_x: f32,
    scale_y: f32,
) -> vizuara_core::Primitive {
    use nalgebra::Point2;
    use vizuara_core::Primitive;

    match primitive {
        Primitive::Point(position) => {
            Primitive::Point(Point2::new(position.x * scale_x, position.y * scale_y))
        }
        Primitive::Line { start, end } => Primitive::Line {
            start: Point2::new(start.x * scale_x, start.y * scale_y),
            end: Point2::new(end.x * scale_x, end.y * scale_y),
        },
        Primitive::Rectangle { min, max } => Primitive::Rectangle {
            min: Point2::new(min.x * scale_x, min.y * scale_y),
            max: Point2::new(max.x * scale_x, max.y * scale_y),
        },
        Primitive::Circle { center, radius } => Primitive::Circle {
            center: Point2::new(center.x * scale_x, center.y * scale_y),
            radius: *radius * scale_x.min(scale_y), // 使用较小的缩放比例保持圆形
        },
        Primitive::Text {
            position,
            content,
            size,
            color,
            h_align,
            v_align,
        } => Primitive::Text {
            position: Point2::new(position.x * scale_x, position.y * scale_y),
            content: content.clone(),
            size: *size * scale_x.min(scale_y),
            color: *color,
            h_align: *h_align,
            v_align: *v_align,
        },
        // 对于复杂的原语，暂时返回原始值
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_format_from_extension() {
        assert_eq!(
            ExportFormat::from_extension("test.svg").unwrap(),
            ExportFormat::Svg
        );
        assert_eq!(
            ExportFormat::from_extension("test.SVG").unwrap(),
            ExportFormat::Svg
        );
        assert_eq!(
            ExportFormat::from_extension("test.png").unwrap(),
            ExportFormat::Png
        );
        assert_eq!(
            ExportFormat::from_extension("test.PNG").unwrap(),
            ExportFormat::Png
        );

        assert!(ExportFormat::from_extension("test.txt").is_err());
        assert!(ExportFormat::from_extension("test").is_err());
    }

    #[test]
    fn test_export_options_builder() {
        let options = ExportOptions::new()
            .with_background(vizuara_core::Color::rgb(1.0, 1.0, 1.0))
            .with_dpi(150.0)
            .with_quality(0.8)
            .with_metadata(false)
            .with_custom_attribute("author".to_string(), "Vizuara".to_string());

        assert!(options.background_color.is_some());
        assert_eq!(options.dpi, 150.0);
        assert_eq!(options.quality, 0.8);
        assert!(!options.include_metadata);
        assert_eq!(
            options.custom_attributes.get("author"),
            Some(&"Vizuara".to_string())
        );
    }

    #[test]
    fn test_scale_primitive() {
        use nalgebra::Point2;
        use vizuara_core::Primitive;

        let circle = Primitive::Circle {
            center: Point2::new(10.0, 20.0),
            radius: 5.0,
        };

        let scaled = scale_primitive(&circle, 2.0, 3.0);
        if let Primitive::Circle { center, radius } = scaled {
            assert_eq!(center, Point2::new(20.0, 60.0));
            assert_eq!(radius, 10.0); // 使用min(2.0, 3.0) = 2.0
        } else {
            panic!("Expected Circle primitive");
        }
    }
}
