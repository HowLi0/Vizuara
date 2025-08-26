use vizuara_core::{Primitive, Color, Scale, LinearScale};
use nalgebra::Point2;

/// 柱状图数据点
#[derive(Debug, Clone)]
pub struct BarData {
    pub category: String,
    pub value: f32,
}

impl BarData {
    pub fn new<S: Into<String>>(category: S, value: f32) -> Self {
        Self {
            category: category.into(),
            value,
        }
    }
}

impl From<(&str, f32)> for BarData {
    fn from((category, value): (&str, f32)) -> Self {
        Self::new(category, value)
    }
}

impl From<(String, f32)> for BarData {
    fn from((category, value): (String, f32)) -> Self {
        Self::new(category, value)
    }
}

/// 柱状图样式配置
#[derive(Debug, Clone)]
pub struct BarStyle {
    pub fill_color: Color,
    pub stroke_color: Color,
    pub stroke_width: f32,
    pub bar_width: f32,  // 柱子宽度比例 (0.0-1.0)
}

impl Default for BarStyle {
    fn default() -> Self {
        Self {
            fill_color: Color::rgb(0.4, 0.6, 0.8),
            stroke_color: Color::rgb(0.2, 0.2, 0.2),
            stroke_width: 1.0,
            bar_width: 0.8, // 80% 宽度
        }
    }
}

/// 柱状图
#[derive(Debug, Clone)]
pub struct BarPlot {
    data: Vec<BarData>,
    style: BarStyle,
    y_scale: Option<LinearScale>,
    title: Option<String>,
}

impl BarPlot {
    /// 创建新的柱状图
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            style: BarStyle::default(),
            y_scale: None,
            title: None,
        }
    }

    /// 设置数据
    pub fn data<T: Into<BarData> + Clone>(mut self, data: &[T]) -> Self {
        self.data = data.iter().cloned().map(|d| d.into()).collect();
        self
    }

    /// 从分离的类别和数值设置数据
    pub fn categories_values(mut self, categories: &[&str], values: &[f32]) -> Self {
        assert_eq!(categories.len(), values.len(), "Categories and values must have the same length");
        
        self.data = categories
            .iter()
            .zip(values.iter())
            .map(|(&cat, &val)| BarData::new(cat, val))
            .collect();
        self
    }

    /// 设置样式
    pub fn style(mut self, style: BarStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置填充颜色
    pub fn fill_color(mut self, color: Color) -> Self {
        self.style.fill_color = color;
        self
    }

    /// 设置边框样式
    pub fn stroke(mut self, color: Color, width: f32) -> Self {
        self.style.stroke_color = color;
        self.style.stroke_width = width;
        self
    }

    /// 设置柱子宽度比例
    pub fn bar_width(mut self, width: f32) -> Self {
        self.style.bar_width = width.clamp(0.1, 1.0);
        self
    }

    /// 设置标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置 Y 轴比例尺
    pub fn y_scale(mut self, scale: LinearScale) -> Self {
        self.y_scale = Some(scale);
        self
    }

    /// 自动计算 Y 轴比例尺
    pub fn auto_scale(mut self) -> Self {
        if !self.data.is_empty() {
            let values: Vec<f32> = self.data.iter().map(|d| d.value).collect();
            let min_val = values.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            let max_val = values.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
            
            // 柱状图通常从 0 开始
            let domain_min = if min_val >= 0.0 { 0.0 } else { min_val * 1.1 };
            let domain_max = max_val * 1.1;
            
            self.y_scale = Some(LinearScale::new(domain_min, domain_max));
        }
        self
    }

    /// 生成渲染图元
    pub fn generate_primitives(&self, plot_area: super::PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.data.is_empty() {
            return primitives;
        }

        let y_scale = if let Some(ref scale) = self.y_scale {
            scale.clone()
        } else {
            let values: Vec<f32> = self.data.iter().map(|d| d.value).collect();
            LinearScale::from_data(&values)
        };

        let bar_count = self.data.len() as f32;
        let bar_spacing = plot_area.width / bar_count;
        let bar_width = bar_spacing * self.style.bar_width;
        let bar_gap = (bar_spacing - bar_width) / 2.0;

        // 计算基线位置（Y=0 的位置）
        let baseline_y = if y_scale.normalize(0.0) <= 1.0 && y_scale.normalize(0.0) >= 0.0 {
            plot_area.y + plot_area.height - y_scale.normalize(0.0) * plot_area.height
        } else {
            plot_area.y + plot_area.height // 如果 0 不在范围内，使用底部
        };

        // 生成每个柱子
        for (i, bar_data) in self.data.iter().enumerate() {
            let x = plot_area.x + bar_gap + i as f32 * bar_spacing;
            let value_normalized = y_scale.normalize(bar_data.value);
            let bar_height = value_normalized * plot_area.height;
            
            // 柱子顶部的 Y 坐标
            let bar_top_y = plot_area.y + plot_area.height - bar_height;
            
            // 创建柱子矩形（带样式）
            primitives.push(Primitive::RectangleStyled {
                min: Point2::new(x, bar_top_y.min(baseline_y)),
                max: Point2::new(x + bar_width, bar_top_y.max(baseline_y)),
                fill: self.style.fill_color,
                stroke: Some((self.style.stroke_color, self.style.stroke_width)),
            });

            // 添加数值标签（在柱子顶部）
            let label_y = if bar_data.value >= 0.0 {
                bar_top_y - 5.0 // 正值标签在柱子上方
            } else {
                baseline_y + 15.0 // 负值标签在基线下方
            };

            primitives.push(Primitive::Text {
                position: Point2::new(x + bar_width / 2.0, label_y),
                content: format!("{:.1}", bar_data.value),
                size: 10.0,
                color: Color::rgb(0.1, 0.1, 0.1),
                h_align: vizuara_core::HorizontalAlign::Center,
                v_align: if bar_data.value >= 0.0 { vizuara_core::VerticalAlign::Bottom } else { vizuara_core::VerticalAlign::Top },
            });

            // 添加类别标签（在 X 轴下方）
            primitives.push(Primitive::Text {
                position: Point2::new(x + bar_width / 2.0, plot_area.y + plot_area.height + 20.0),
                content: bar_data.category.clone(),
                size: 10.0,
                color: Color::rgb(0.1, 0.1, 0.1),
                h_align: vizuara_core::HorizontalAlign::Center,
                v_align: vizuara_core::VerticalAlign::Top,
            });
        }

        // 如果有基线（Y=0），绘制基线
        if baseline_y > plot_area.y && baseline_y < plot_area.y + plot_area.height {
            primitives.push(Primitive::Line {
                start: Point2::new(plot_area.x, baseline_y),
                end: Point2::new(plot_area.x + plot_area.width, baseline_y),
            });
        }

        primitives
    }

    /// 获取数据边界
    pub fn data_bounds(&self) -> Option<(f32, f32)> {
        if self.data.is_empty() {
            return None;
        }

        let values: Vec<f32> = self.data.iter().map(|d| d.value).collect();
        let min_val = values.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_val = values.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));

        Some((min_val, max_val))
    }

    /// 获取数据点数量
    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    /// 获取类别列表
    pub fn categories(&self) -> Vec<&str> {
        self.data.iter().map(|d| d.category.as_str()).collect()
    }
}

impl Default for BarPlot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PlotArea;

    #[test]
    fn test_bar_plot_creation() {
        let plot = BarPlot::new();
        assert_eq!(plot.data_len(), 0);
    }

    #[test]
    fn test_bar_plot_with_data() {
        let data = vec![("A", 10.0), ("B", 20.0), ("C", 15.0)];
        let plot = BarPlot::new().data(&data);
        
        assert_eq!(plot.data_len(), 3);
        assert_eq!(plot.categories(), vec!["A", "B", "C"]);
    }

    #[test]
    fn test_bar_plot_categories_values() {
        let categories = ["类别1", "类别2", "类别3"];
        let values = [5.0, 10.0, 7.5];
        let plot = BarPlot::new().categories_values(&categories, &values);
        
        assert_eq!(plot.data_len(), 3);
        assert_eq!(plot.categories(), vec!["类别1", "类别2", "类别3"]);
    }

    #[test]
    fn test_bar_plot_data_bounds() {
        let data = vec![("A", 10.0), ("B", 20.0), ("C", 5.0)];
        let plot = BarPlot::new().data(&data);
        
        let bounds = plot.data_bounds().unwrap();
        assert_eq!(bounds.0, 5.0);  // min
        assert_eq!(bounds.1, 20.0); // max
    }

    #[test]
    fn test_bar_plot_with_negative_values() {
        let data = vec![("A", -5.0), ("B", 10.0), ("C", -2.0)];
        let plot = BarPlot::new().data(&data).auto_scale();
        
        let bounds = plot.data_bounds().unwrap();
        assert_eq!(bounds.0, -5.0);
        assert_eq!(bounds.1, 10.0);
    }

    #[test]
    fn test_bar_plot_primitive_generation() {
        let data = vec![("A", 10.0), ("B", 20.0)];
        let plot = BarPlot::new().data(&data).auto_scale();
        
        let plot_area = PlotArea::new(100.0, 100.0, 400.0, 300.0);
        let primitives = plot.generate_primitives(plot_area);
        
        // 应该包含：2个矩形 + 2个数值标签 + 2个类别标签 + 可能的基线 = 至少6个图元
        assert!(primitives.len() >= 6);
    }

    #[test]
    fn test_bar_plot_styling() {
        let plot = BarPlot::new()
            .fill_color(Color::rgb(1.0, 0.0, 0.0))
            .stroke(Color::rgb(0.0, 0.0, 1.0), 2.0)
            .bar_width(0.6)
            .title("Test Chart");
        
        assert_eq!(plot.style.fill_color, Color::rgb(1.0, 0.0, 0.0));
        assert_eq!(plot.style.stroke_color, Color::rgb(0.0, 0.0, 1.0));
        assert_eq!(plot.style.stroke_width, 2.0);
        assert_eq!(plot.style.bar_width, 0.6);
        assert_eq!(plot.title, Some("Test Chart".to_string()));
    }
}
