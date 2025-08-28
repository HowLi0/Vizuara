use nalgebra::Point2;
use std::f32::consts::PI;
use vizuara_core::{Color, Primitive};

/// 饼图数据项
#[derive(Debug, Clone)]
pub struct PieData {
    pub label: String,
    pub value: f32,
    pub color: Option<Color>,
}

impl PieData {
    pub fn new<S: Into<String>>(label: S, value: f32) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    pub fn with_color<S: Into<String>>(label: S, value: f32, color: Color) -> Self {
        Self {
            label: label.into(),
            value,
            color: Some(color),
        }
    }
}

impl From<(&str, f32)> for PieData {
    fn from((label, value): (&str, f32)) -> Self {
        Self::new(label, value)
    }
}

impl From<(String, f32)> for PieData {
    fn from((label, value): (String, f32)) -> Self {
        Self::new(label, value)
    }
}

/// 饼图样式配置
#[derive(Debug, Clone)]
pub struct PieStyle {
    /// 内半径（0.0 = 饼图，>0.0 = 圆环图）
    pub inner_radius: f32,
    /// 外半径
    pub outer_radius: f32,
    /// 扇形间隙角度（弧度）
    pub gap_angle: f32,
    /// 边框宽度
    pub stroke_width: f32,
    /// 边框颜色
    pub stroke_color: Color,
    /// 是否显示标签
    pub show_labels: bool,
    /// 标签字体大小
    pub label_size: f32,
    /// 标签颜色
    pub label_color: Color,
    /// 标签距离中心的距离比例
    pub label_distance: f32,
    /// 是否显示百分比
    pub show_percentage: bool,
    /// 起始角度（弧度，0 = 右侧，PI/2 = 顶部）
    pub start_angle: f32,
}

impl Default for PieStyle {
    fn default() -> Self {
        Self {
            inner_radius: 0.0,
            outer_radius: 80.0,
            gap_angle: 0.0,
            stroke_width: 1.0,
            stroke_color: Color::rgb(1.0, 1.0, 1.0),
            show_labels: true,
            label_size: 12.0,
            label_color: Color::rgb(0.2, 0.2, 0.2),
            label_distance: 1.2,
            show_percentage: true,
            start_angle: -PI / 2.0, // 从顶部开始
        }
    }
}

/// 饼图
#[derive(Debug, Clone)]
pub struct PieChart {
    data: Vec<PieData>,
    style: PieStyle,
    center: Point2<f32>,
    title: Option<String>,
    default_colors: Vec<Color>,
}

impl PieChart {
    /// 创建新的饼图
    pub fn new() -> Self {
        let default_colors = vec![
            Color::rgb(0.2, 0.6, 0.9),   // 蓝色
            Color::rgb(0.9, 0.5, 0.2),   // 橙色
            Color::rgb(0.4, 0.8, 0.4),   // 绿色
            Color::rgb(0.9, 0.3, 0.3),   // 红色
            Color::rgb(0.7, 0.4, 0.9),   // 紫色
            Color::rgb(0.9, 0.9, 0.3),   // 黄色
            Color::rgb(0.3, 0.9, 0.9),   // 青色
            Color::rgb(0.9, 0.6, 0.8),   // 粉色
            Color::rgb(0.5, 0.7, 0.3),   // 橄榄绿
            Color::rgb(0.8, 0.4, 0.6),   // 暗粉色
        ];

        Self {
            data: Vec::new(),
            style: PieStyle::default(),
            center: Point2::new(200.0, 200.0),
            title: None,
            default_colors,
        }
    }

    /// 设置数据
    pub fn data<T: Into<PieData> + Clone>(mut self, data: &[T]) -> Self {
        self.data = data.iter().cloned().map(|d| d.into()).collect();
        self
    }

    /// 从分离的标签和数值设置数据
    pub fn labels_values(mut self, labels: &[&str], values: &[f32]) -> Self {
        assert_eq!(
            labels.len(),
            values.len(),
            "Labels and values must have the same length"
        );

        self.data = labels
            .iter()
            .zip(values.iter())
            .map(|(&label, &value)| PieData::new(label, value))
            .collect();
        self
    }

    /// 设置样式
    pub fn style(mut self, style: PieStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置中心点
    pub fn center(mut self, x: f32, y: f32) -> Self {
        self.center = Point2::new(x, y);
        self
    }

    /// 设置半径（饼图模式）
    pub fn radius(mut self, radius: f32) -> Self {
        self.style.outer_radius = radius;
        self.style.inner_radius = 0.0;
        self
    }

    /// 设置内外半径（圆环图模式）
    pub fn donut(mut self, inner_radius: f32, outer_radius: f32) -> Self {
        self.style.inner_radius = inner_radius;
        self.style.outer_radius = outer_radius;
        self
    }

    /// 设置扇形间隙
    pub fn gap_angle(mut self, gap_angle: f32) -> Self {
        self.style.gap_angle = gap_angle;
        self
    }

    /// 设置起始角度
    pub fn start_angle(mut self, start_angle: f32) -> Self {
        self.style.start_angle = start_angle;
        self
    }

    /// 设置边框样式
    pub fn stroke(mut self, color: Color, width: f32) -> Self {
        self.style.stroke_color = color;
        self.style.stroke_width = width;
        self
    }

    /// 设置标签样式
    pub fn labels(mut self, show: bool, size: f32, color: Color, distance: f32) -> Self {
        self.style.show_labels = show;
        self.style.label_size = size;
        self.style.label_color = color;
        self.style.label_distance = distance;
        self
    }

    /// 设置是否显示百分比
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.style.show_percentage = show;
        self
    }

    /// 设置标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 获取数据项数量
    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    /// 计算总值
    pub fn total_value(&self) -> f32 {
        self.data.iter().map(|item| item.value).sum()
    }

    /// 生成饼图的渲染图元
    pub fn generate_primitives(&self, _plot_area: super::PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.data.is_empty() {
            return primitives;
        }

        let total = self.total_value();
        if total <= 0.0 {
            return primitives;
        }

        let mut current_angle = self.style.start_angle;
        let gap_per_segment = if self.data.len() > 1 {
            self.style.gap_angle / self.data.len() as f32
        } else {
            0.0
        };

        // 生成扇形
        for (i, item) in self.data.iter().enumerate() {
            if item.value <= 0.0 {
                continue;
            }

            let percentage = item.value / total;
            let sector_angle = percentage * 2.0 * PI - gap_per_segment;

            // 选择颜色
            let color = item.color.unwrap_or_else(|| {
                self.default_colors[i % self.default_colors.len()]
            });

            // 生成扇形图元
            if self.style.inner_radius > 0.0 {
                // 圆环模式
                primitives.push(Primitive::ArcRing {
                    center: self.center,
                    inner_radius: self.style.inner_radius,
                    outer_radius: self.style.outer_radius,
                    start_angle: current_angle,
                    end_angle: current_angle + sector_angle,
                    fill: color,
                    stroke: Some((self.style.stroke_color, self.style.stroke_width)),
                });
            } else {
                // 饼图模式
                primitives.push(Primitive::ArcSector {
                    center: self.center,
                    radius: self.style.outer_radius,
                    start_angle: current_angle,
                    end_angle: current_angle + sector_angle,
                    fill: color,
                    stroke: Some((self.style.stroke_color, self.style.stroke_width)),
                });
            }

            // 添加标签
            if self.style.show_labels {
                let label_angle = current_angle + sector_angle / 2.0;
                let label_radius = self.style.outer_radius * self.style.label_distance;
                let label_x = self.center.x + label_radius * label_angle.cos();
                let label_y = self.center.y + label_radius * label_angle.sin();

                let label_text = if self.style.show_percentage {
                    format!("{}\n{:.1}%", item.label, percentage * 100.0)
                } else {
                    item.label.clone()
                };

                primitives.push(Primitive::Text {
                    position: Point2::new(label_x, label_y),
                    content: label_text,
                    size: self.style.label_size,
                    color: self.style.label_color,
                    h_align: vizuara_core::HorizontalAlign::Center,
                    v_align: vizuara_core::VerticalAlign::Middle,
                });
            }

            current_angle += sector_angle + gap_per_segment;
        }

        // 添加标题
        if let Some(ref title) = self.title {
            primitives.push(Primitive::Text {
                position: Point2::new(self.center.x, self.center.y - self.style.outer_radius - 30.0),
                content: title.clone(),
                size: 16.0,
                color: Color::rgb(0.2, 0.2, 0.2),
                h_align: vizuara_core::HorizontalAlign::Center,
                v_align: vizuara_core::VerticalAlign::Middle,
            });
        }

        primitives
    }
}

impl Default for PieChart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pie_chart_creation() {
        let chart = PieChart::new();
        assert_eq!(chart.data_len(), 0);
        assert_eq!(chart.total_value(), 0.0);
    }

    #[test]
    fn test_pie_chart_data() {
        let data = [("A", 30.0), ("B", 20.0), ("C", 50.0)];
        let chart = PieChart::new().data(&data);
        
        assert_eq!(chart.data_len(), 3);
        assert_eq!(chart.total_value(), 100.0);
    }

    #[test]
    fn test_pie_chart_labels_values() {
        let labels = ["苹果", "香蕉", "橙子"];
        let values = [40.0, 30.0, 30.0];
        let chart = PieChart::new().labels_values(&labels, &values);
        
        assert_eq!(chart.data_len(), 3);
        assert_eq!(chart.total_value(), 100.0);
    }

    #[test]
    fn test_donut_configuration() {
        let chart = PieChart::new()
            .donut(30.0, 80.0)
            .gap_angle(0.1);
        
        assert_eq!(chart.style.inner_radius, 30.0);
        assert_eq!(chart.style.outer_radius, 80.0);
        assert_eq!(chart.style.gap_angle, 0.1);
    }

    #[test]
    fn test_empty_data_primitives() {
        let chart = PieChart::new();
        let primitives = chart.generate_primitives(super::PlotArea::new(0.0, 0.0, 400.0, 400.0));
        assert!(primitives.is_empty());
    }

    #[test]
    fn test_single_item_primitives() {
        let data = [("全部", 100.0)];
        let chart = PieChart::new().data(&data);
        let primitives = chart.generate_primitives(super::PlotArea::new(0.0, 0.0, 400.0, 400.0));
        assert!(!primitives.is_empty());
    }
}
