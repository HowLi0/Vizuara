use crate::{ExportError, ExportFormat, ExportOptions, ExportResult, Exporter};
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Rect, Shader, Stroke, Transform};
use vizuara_core::{Color, Primitive, Style};

/// PNG导出器
pub struct PngExporter;

impl PngExporter {
    /// 创建新的PNG导出器
    pub fn new() -> Self {
        Self
    }

    /// 将颜色转换为tiny-skia颜色
    fn color_to_skia(color: &Color, alpha: f32) -> tiny_skia::Color {
        tiny_skia::Color::from_rgba(
            color.r.clamp(0.0, 1.0),
            color.g.clamp(0.0, 1.0),
            color.b.clamp(0.0, 1.0),
            alpha.clamp(0.0, 1.0),
        )
        .unwrap_or(tiny_skia::Color::BLACK)
    }

    /// 渲染原语到pixmap
    fn render_primitive(
        pixmap: &mut Pixmap,
        primitive: &Primitive,
        style: &Style,
        options: &ExportOptions,
    ) -> Result<(), ExportError> {
        match primitive {
            Primitive::Circle { center, radius } => {
                Self::render_circle(pixmap, center, *radius, style)?;
            }
            Primitive::Rectangle { min, max } => {
                Self::render_rectangle(pixmap, min, max, style)?;
            }
            Primitive::Line { start, end } => {
                Self::render_line(pixmap, start, end, style)?;
            }
            Primitive::Text {
                position,
                content,
                size,
                color,
                ..
            } => {
                Self::render_text(pixmap, position, content, *size, color, style)?;
            }
            Primitive::Point(position) => {
                Self::render_point(pixmap, position, style, options)?;
            }
            _ => {
                return Err(ExportError::PngError(format!(
                    "不支持的原语类型: {:?}",
                    primitive
                )));
            }
        }
        Ok(())
    }

    fn render_circle(
        pixmap: &mut Pixmap,
        center: &nalgebra::Point2<f32>,
        radius: f32,
        style: &Style,
    ) -> Result<(), ExportError> {
        let mut path = PathBuilder::new();
        path.push_circle(center.x, center.y, radius);
        let path = path
            .finish()
            .ok_or_else(|| ExportError::PngError("无法创建圆形路径".to_string()))?;

        // 填充
        if let Some(fill_color) = &style.fill_color {
            let color = Self::color_to_skia(fill_color, style.opacity);
            let paint = Paint {
                shader: Shader::SolidColor(color),
                ..Paint::default()
            };
            pixmap.fill_path(
                &path,
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }

        // 描边
        if let Some(stroke_color) = &style.stroke_color {
            if style.stroke_width > 0.0 {
                let color = Self::color_to_skia(stroke_color, style.opacity);
                let paint = Paint {
                    shader: Shader::SolidColor(color),
                    ..Paint::default()
                };
                let stroke = Stroke {
                    width: style.stroke_width,
                    ..Stroke::default()
                };
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }
        }

        Ok(())
    }

    fn render_rectangle(
        pixmap: &mut Pixmap,
        min: &nalgebra::Point2<f32>,
        max: &nalgebra::Point2<f32>,
        style: &Style,
    ) -> Result<(), ExportError> {
        let rect = Rect::from_ltrb(min.x, min.y, max.x, max.y)
            .ok_or_else(|| ExportError::PngError("无效的矩形坐标".to_string()))?;

        // 填充
        if let Some(fill_color) = &style.fill_color {
            let color = Self::color_to_skia(fill_color, style.opacity);
            let paint = Paint {
                shader: Shader::SolidColor(color),
                ..Paint::default()
            };
            pixmap.fill_rect(rect, &paint, Transform::identity(), None);
        }

        // 描边
        if let Some(stroke_color) = &style.stroke_color {
            if style.stroke_width > 0.0 {
                let color = Self::color_to_skia(stroke_color, style.opacity);
                let paint = Paint {
                    shader: Shader::SolidColor(color),
                    ..Paint::default()
                };
                let stroke = Stroke {
                    width: style.stroke_width,
                    ..Stroke::default()
                };

                // 创建矩形路径用于描边
                let mut path = PathBuilder::new();
                path.push_rect(rect);
                if let Some(path) = path.finish() {
                    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
                }
            }
        }

        Ok(())
    }

    fn render_line(
        pixmap: &mut Pixmap,
        start: &nalgebra::Point2<f32>,
        end: &nalgebra::Point2<f32>,
        style: &Style,
    ) -> Result<(), ExportError> {
        let mut path = PathBuilder::new();
        path.move_to(start.x, start.y);
        path.line_to(end.x, end.y);
        let path = path
            .finish()
            .ok_or_else(|| ExportError::PngError("无法创建线条路径".to_string()))?;

        let default_color = Color::rgb(0.0, 0.0, 0.0);
        let stroke_color = style.stroke_color.as_ref().unwrap_or(&default_color); // 默认黑色
        let color = Self::color_to_skia(stroke_color, style.opacity);
        let paint = Paint {
            shader: Shader::SolidColor(color),
            ..Paint::default()
        };
        let stroke = Stroke {
            width: style.stroke_width.max(1.0), // 最小1像素宽度
            ..Stroke::default()
        };

        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        Ok(())
    }

    fn render_text(
        pixmap: &mut Pixmap,
        position: &nalgebra::Point2<f32>,
        _content: &str,
        _size: f32,
        _color: &Color,
        _style: &Style,
    ) -> Result<(), ExportError> {
        // 简单的文本渲染占位符
        // 在实际实现中，您可能需要使用字体渲染库如 fontdue 或 rusttype

        // 暂时渲染一个小圆圈作为文本占位符
        let mut path = PathBuilder::new();
        path.push_circle(position.x, position.y, 3.0);
        if let Some(path) = path.finish() {
            let color = Self::color_to_skia(&Color::rgb(0.0, 0.0, 0.0), 1.0);
            let paint = Paint {
                shader: Shader::SolidColor(color),
                ..Paint::default()
            };
            pixmap.fill_path(
                &path,
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }

        Ok(())
    }

    fn render_point(
        pixmap: &mut Pixmap,
        position: &nalgebra::Point2<f32>,
        style: &Style,
        options: &ExportOptions,
    ) -> Result<(), ExportError> {
        // 将点渲染为小圆圈，使用配置的大小
        let radius = options.default_point_size;
        let mut path = PathBuilder::new();
        path.push_circle(position.x, position.y, radius);
        let path = path
            .finish()
            .ok_or_else(|| ExportError::PngError("无法创建点路径".to_string()))?;

        // 应用填充颜色
        if let Some(fill_color) = &style.fill_color {
            let color = Self::color_to_skia(fill_color, style.opacity);
            let paint = Paint {
                shader: Shader::SolidColor(color),
                ..Paint::default()
            };
            pixmap.fill_path(
                &path,
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        } else {
            // 默认黑色填充
            let color = Self::color_to_skia(&Color::rgb(0.0, 0.0, 0.0), style.opacity);
            let paint = Paint {
                shader: Shader::SolidColor(color),
                ..Paint::default()
            };
            pixmap.fill_path(
                &path,
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }

        // 描边（如果有）
        if let Some(stroke_color) = &style.stroke_color {
            if style.stroke_width > 0.0 {
                let color = Self::color_to_skia(stroke_color, style.opacity);
                let paint = Paint {
                    shader: Shader::SolidColor(color),
                    ..Paint::default()
                };
                let stroke = Stroke {
                    width: style.stroke_width,
                    ..Stroke::default()
                };
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }
        }

        Ok(())
    }
}

impl Default for PngExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Exporter for PngExporter {
    fn export_to_file(
        &self,
        primitives: &[Primitive],
        styles: &[Style],
        width: u32,
        height: u32,
        path: &str,
        options: &ExportOptions,
    ) -> ExportResult<()> {
        let png_data = self.export_to_bytes(primitives, styles, width, height, options)?;
        std::fs::write(path, png_data)?;
        Ok(())
    }

    fn export_to_bytes(
        &self,
        primitives: &[Primitive],
        styles: &[Style],
        width: u32,
        height: u32,
        options: &ExportOptions,
    ) -> ExportResult<Vec<u8>> {
        let mut pixmap = Pixmap::new(width, height)
            .ok_or_else(|| ExportError::PngError("无法创建像素画布".to_string()))?;

        // 设置背景
        if let Some(bg_color) = &options.background_color {
            let bg = Self::color_to_skia(bg_color, 1.0);
            pixmap.fill(bg);
        }
        // 如果没有设置背景颜色，pixmap默认是透明的

        // 渲染所有原语
        for (primitive, style) in primitives.iter().zip(styles.iter()) {
            if let Err(e) = Self::render_primitive(&mut pixmap, primitive, style, options) {
                eprintln!("Warning: 跳过无法渲染的原语: {}", e);
            }
        }

        // 编码为PNG
        pixmap
            .encode_png()
            .map_err(|e| ExportError::PngError(format!("PNG编码失败: {}", e)))
    }

    fn supported_format(&self) -> ExportFormat {
        ExportFormat::Png
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point2;
    use tempfile::tempdir;

    #[test]
    fn test_png_exporter_creation() {
        let exporter = PngExporter::new();
        assert_eq!(exporter.supported_format(), ExportFormat::Png);
    }

    #[test]
    fn test_color_conversion() {
        let color = Color::rgb(1.0, 0.5, 0.0);
        let skia_color = PngExporter::color_to_skia(&color, 0.8);

        // tiny_skia::Color 的具体实现可能会有精度差异，这里只检查大致正确
        // 我们主要确保转换没有崩溃
        assert!(skia_color.red() > 0.9);
        assert!(skia_color.green() > 0.4 && skia_color.green() < 0.6);
        assert!(skia_color.blue() < 0.1);
        assert!(skia_color.alpha() > 0.7 && skia_color.alpha() < 0.9);
    }

    #[test]
    fn test_simple_circle_export() -> ExportResult<()> {
        let exporter = PngExporter::new();
        let primitives = vec![Primitive::Circle {
            center: Point2::new(50.0, 50.0),
            radius: 25.0,
        }];
        let styles = vec![Style::new().fill_color(Color::rgb(1.0, 0.0, 0.0))];

        let bytes =
            exporter.export_to_bytes(&primitives, &styles, 100, 100, &ExportOptions::default())?;

        // 验证这是有效的PNG数据
        assert!(bytes.len() > 100); // PNG头部信息就需要几十字节
        assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]); // PNG魔数

        Ok(())
    }

    #[test]
    fn test_export_to_file() -> ExportResult<()> {
        let exporter = PngExporter::new();
        let primitives = vec![Primitive::Rectangle {
            min: Point2::new(10.0, 10.0),
            max: Point2::new(90.0, 90.0),
        }];
        let styles = vec![Style::new()
            .fill_color(Color::rgb(0.0, 1.0, 0.0))
            .stroke(Color::rgb(0.0, 0.0, 1.0), 2.0)];

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_rect.png");

        exporter.export_to_file(
            &primitives,
            &styles,
            100,
            100,
            file_path.to_str().unwrap(),
            &ExportOptions::default(),
        )?;

        assert!(file_path.exists());

        let metadata = std::fs::metadata(&file_path).unwrap();
        assert!(metadata.len() > 100); // 确保文件不是空的

        Ok(())
    }

    #[test]
    fn test_background_color() -> ExportResult<()> {
        let exporter = PngExporter::new();
        let primitives = vec![];
        let styles = vec![];

        let options = ExportOptions::new().with_background(Color::rgb(0.9, 0.9, 0.9));

        let bytes = exporter.export_to_bytes(&primitives, &styles, 10, 10, &options)?;

        // 验证这是有效的PNG
        assert!(bytes.len() > 50);
        assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);

        Ok(())
    }
}
