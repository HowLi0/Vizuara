use crate::{ExportError, ExportFormat, ExportOptions, ExportResult, Exporter};
use svg::node::element::{Circle, Line, Rectangle, Text as SvgText};
use svg::node::Text;
use svg::Document;
use vizuara_core::{Color, Primitive, Style};

/// SVG导出器
pub struct SvgExporter;

impl SvgExporter {
    /// 创建新的SVG导出器
    pub fn new() -> Self {
        Self
    }

    /// 将颜色转换为SVG颜色字符串
    fn color_to_svg(color: &Color) -> String {
        format!(
            "rgb({}, {}, {})",
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8
        )
    }

    /// 将原语转换为SVG元素
    fn primitive_to_svg(
        primitive: &Primitive,
        style: &Style,
        options: &ExportOptions,
    ) -> Result<Box<dyn svg::Node>, ExportError> {
        match primitive {
            Primitive::Circle { center, radius } => {
                let mut circle = Circle::new()
                    .set("cx", center.x)
                    .set("cy", center.y)
                    .set("r", *radius);

                // 应用样式
                if let Some(fill_color) = &style.fill_color {
                    circle = circle.set("fill", Self::color_to_svg(fill_color));
                } else {
                    circle = circle.set("fill", "none");
                }

                if let Some(stroke_color) = &style.stroke_color {
                    circle = circle
                        .set("stroke", Self::color_to_svg(stroke_color))
                        .set("stroke-width", style.stroke_width);
                } else {
                    circle = circle.set("stroke", "none");
                }

                if style.opacity < 1.0 {
                    circle = circle.set("opacity", style.opacity);
                }

                Ok(Box::new(circle))
            }

            Primitive::Rectangle { min, max } => {
                let width = max.x - min.x;
                let height = max.y - min.y;

                let mut rect = Rectangle::new()
                    .set("x", min.x)
                    .set("y", min.y)
                    .set("width", width)
                    .set("height", height);

                // 应用样式
                if let Some(fill_color) = &style.fill_color {
                    rect = rect.set("fill", Self::color_to_svg(fill_color));
                } else {
                    rect = rect.set("fill", "none");
                }

                if let Some(stroke_color) = &style.stroke_color {
                    rect = rect
                        .set("stroke", Self::color_to_svg(stroke_color))
                        .set("stroke-width", style.stroke_width);
                } else {
                    rect = rect.set("stroke", "none");
                }

                if style.opacity < 1.0 {
                    rect = rect.set("opacity", style.opacity);
                }

                Ok(Box::new(rect))
            }

            Primitive::Line { start, end } => {
                let mut line = Line::new()
                    .set("x1", start.x)
                    .set("y1", start.y)
                    .set("x2", end.x)
                    .set("y2", end.y);

                // 线条总是需要stroke
                if let Some(stroke_color) = &style.stroke_color {
                    line = line
                        .set("stroke", Self::color_to_svg(stroke_color))
                        .set("stroke-width", style.stroke_width);
                } else {
                    // 默认黑色
                    line = line
                        .set("stroke", "black")
                        .set("stroke-width", style.stroke_width);
                }

                if style.opacity < 1.0 {
                    line = line.set("opacity", style.opacity);
                }

                Ok(Box::new(line))
            }

            Primitive::Text {
                position,
                content,
                size,
                color,
                ..
            } => {
                let mut text = SvgText::new()
                    .set("x", position.x)
                    .set("y", position.y)
                    .set("font-size", *size)
                    .set("fill", Self::color_to_svg(color))
                    .add(Text::new(content.clone()));

                if style.opacity < 1.0 {
                    text = text.set("opacity", style.opacity);
                }

                Ok(Box::new(text))
            }

            Primitive::Point(position) => {
                // 将点渲染为小圆圈，使用配置的大小
                let radius = options.default_point_size;
                let mut circle = Circle::new()
                    .set("cx", position.x)
                    .set("cy", position.y)
                    .set("r", radius);

                // 应用样式
                if let Some(fill_color) = &style.fill_color {
                    circle = circle.set("fill", Self::color_to_svg(fill_color));
                } else {
                    circle = circle.set("fill", "black"); // 默认黑色
                }

                if let Some(stroke_color) = &style.stroke_color {
                    circle = circle
                        .set("stroke", Self::color_to_svg(stroke_color))
                        .set("stroke-width", style.stroke_width);
                }

                if style.opacity < 1.0 {
                    circle = circle.set("opacity", style.opacity);
                }

                Ok(Box::new(circle))
            }

            _ => Err(ExportError::SvgError(format!(
                "不支持的原语类型: {:?}",
                primitive
            ))),
        }
    }
}

impl Default for SvgExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Exporter for SvgExporter {
    fn export_to_file(
        &self,
        primitives: &[Primitive],
        styles: &[Style],
        width: u32,
        height: u32,
        path: &str,
        options: &ExportOptions,
    ) -> ExportResult<()> {
        let svg_content = self.export_to_bytes(primitives, styles, width, height, options)?;
        std::fs::write(path, svg_content)?;
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
        let mut document = Document::new()
            .set("viewBox", (0, 0, width, height))
            .set("width", width)
            .set("height", height)
            .set("xmlns", "http://www.w3.org/2000/svg");

        // 添加背景
        if let Some(bg_color) = &options.background_color {
            let background = Rectangle::new()
                .set("x", 0)
                .set("y", 0)
                .set("width", width)
                .set("height", height)
                .set("fill", Self::color_to_svg(bg_color));
            document = document.add(background);
        }

        // 添加元数据
        if options.include_metadata {
            // 将作者信息作为注释添加到文档中
            if let Some(author) = options.custom_attributes.get("author") {
                document = document.set("data-author", author.as_str());
            }
        }

        // 转换所有原语
        for (primitive, style) in primitives.iter().zip(styles.iter()) {
            match Self::primitive_to_svg(primitive, style, options) {
                Ok(element) => {
                    document = document.add(element);
                }
                Err(e) => {
                    // 记录错误但继续处理其他元素
                    eprintln!("Warning: 跳过无法转换的原语: {}", e);
                }
            }
        }

        // 转换为字符串
        let svg_string = document.to_string();
        Ok(svg_string.into_bytes())
    }

    fn supported_format(&self) -> ExportFormat {
        ExportFormat::Svg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point2;
    use tempfile::tempdir;

    #[test]
    fn test_svg_exporter_creation() {
        let exporter = SvgExporter::new();
        assert_eq!(exporter.supported_format(), ExportFormat::Svg);
    }

    #[test]
    fn test_color_conversion() {
        let color = Color::rgb(1.0, 0.5, 0.0);
        let svg_color = SvgExporter::color_to_svg(&color);
        assert_eq!(svg_color, "rgb(255, 127, 0)");
    }

    #[test]
    fn test_simple_circle_export() -> ExportResult<()> {
        let exporter = SvgExporter::new();
        let primitives = vec![Primitive::Circle {
            center: Point2::new(50.0, 50.0),
            radius: 25.0,
        }];
        let styles = vec![Style::new().fill_color(Color::rgb(1.0, 0.0, 0.0))];

        let bytes =
            exporter.export_to_bytes(&primitives, &styles, 100, 100, &ExportOptions::default())?;
        let svg_string = String::from_utf8(bytes).unwrap();

        assert!(svg_string.contains("<svg"));
        assert!(svg_string.contains("<circle"));
        assert!(svg_string.contains("cx=\"50\""));
        assert!(svg_string.contains("cy=\"50\""));
        assert!(svg_string.contains("r=\"25\""));
        assert!(svg_string.contains("rgb(255, 0, 0)"));

        Ok(())
    }

    #[test]
    fn test_export_to_file() -> ExportResult<()> {
        let exporter = SvgExporter::new();
        let primitives = vec![Primitive::Rectangle {
            min: Point2::new(10.0, 10.0),
            max: Point2::new(90.0, 90.0),
        }];
        let styles = vec![Style::new()
            .fill_color(Color::rgb(0.0, 1.0, 0.0))
            .stroke(Color::rgb(0.0, 0.0, 1.0), 2.0)];

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_rect.svg");

        exporter.export_to_file(
            &primitives,
            &styles,
            100,
            100,
            file_path.to_str().unwrap(),
            &ExportOptions::default(),
        )?;

        assert!(file_path.exists());

        let content = std::fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("<rect"));
        assert!(content.contains("width=\"80\""));
        assert!(content.contains("height=\"80\""));

        Ok(())
    }

    #[test]
    fn test_background_color() -> ExportResult<()> {
        let exporter = SvgExporter::new();
        let primitives = vec![];
        let styles = vec![];

        let options = ExportOptions::new().with_background(Color::rgb(0.9, 0.9, 0.9));

        let bytes = exporter.export_to_bytes(&primitives, &styles, 100, 100, &options)?;
        let svg_string = String::from_utf8(bytes).unwrap();

        assert!(svg_string.contains("rgb(229, 229, 229)")); // 0.9 * 255 = 229

        Ok(())
    }
}
