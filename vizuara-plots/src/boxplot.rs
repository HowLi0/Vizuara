use nalgebra::Point2;
use vizuara_core::{Color, Primitive};

/// 箱线图统计数据
#[derive(Debug, Clone)]
pub struct BoxStatistics {
    /// 最小值 (不包括异常值)
    pub min: f32,
    /// 第一四分位数 (Q1)
    pub q1: f32,
    /// 中位数 (Q2)
    pub median: f32,
    /// 第三四分位数 (Q3)
    pub q3: f32,
    /// 最大值 (不包括异常值)
    pub max: f32,
    /// 异常值列表
    pub outliers: Vec<f32>,
}

impl BoxStatistics {
    /// 从原始数据计算箱线图统计量
    pub fn from_data(mut data: Vec<f32>) -> Self {
        if data.is_empty() {
            return Self {
                min: 0.0,
                q1: 0.0,
                median: 0.0,
                q3: 0.0,
                max: 0.0,
                outliers: Vec::new(),
            };
        }

        // 排序数据
        data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let n = data.len();

        // 计算四分位数
        let q1 = percentile(&data, 25.0);
        let median = percentile(&data, 50.0);
        let q3 = percentile(&data, 75.0);

        // 计算四分位距 (IQR)
        let iqr = q3 - q1;

        // 计算须线范围 (1.5 * IQR)
        let lower_fence = q1 - 1.5 * iqr;
        let upper_fence = q3 + 1.5 * iqr;

        // 找到须线的实际端点 (在数据范围内)
        let min = data
            .iter()
            .find(|&&x| x >= lower_fence)
            .copied()
            .unwrap_or(data[0]);

        let max = data
            .iter()
            .rev()
            .find(|&&x| x <= upper_fence)
            .copied()
            .unwrap_or(data[n - 1]);

        // 识别异常值
        let outliers: Vec<f32> = data
            .iter()
            .filter(|&&x| x < lower_fence || x > upper_fence)
            .copied()
            .collect();

        Self {
            min,
            q1,
            median,
            q3,
            max,
            outliers,
        }
    }

    /// 手动创建箱线图统计量
    pub fn new(min: f32, q1: f32, median: f32, q3: f32, max: f32) -> Self {
        Self {
            min,
            q1,
            median,
            q3,
            max,
            outliers: Vec::new(),
        }
    }

    /// 添加异常值
    pub fn with_outliers(mut self, outliers: Vec<f32>) -> Self {
        self.outliers = outliers;
        self
    }
}

/// 计算百分位数
fn percentile(sorted_data: &[f32], p: f32) -> f32 {
    if sorted_data.is_empty() {
        return 0.0;
    }

    let n = sorted_data.len();
    let index = (p / 100.0) * (n - 1) as f32;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;

    if lower == upper {
        sorted_data[lower]
    } else {
        let weight = index - lower as f32;
        sorted_data[lower] * (1.0 - weight) + sorted_data[upper] * weight
    }
}

/// 箱线图样式配置
#[derive(Debug, Clone)]
pub struct BoxPlotStyle {
    /// 箱子填充颜色
    pub box_fill_color: Color,
    /// 箱子边框颜色
    pub box_stroke_color: Color,
    /// 箱子边框宽度
    pub box_stroke_width: f32,
    /// 中位数线颜色
    pub median_color: Color,
    /// 中位数线宽度
    pub median_width: f32,
    /// 须线颜色
    pub whisker_color: Color,
    /// 须线宽度
    pub whisker_width: f32,
    /// 异常值颜色
    pub outlier_color: Color,
    /// 异常值大小
    pub outlier_size: f32,
    /// 箱子宽度 (相对于间距的比例)
    pub box_width: f32,
}

impl Default for BoxPlotStyle {
    fn default() -> Self {
        Self {
            box_fill_color: Color::rgb(0.7, 0.9, 1.0),
            box_stroke_color: Color::rgb(0.2, 0.4, 0.8),
            box_stroke_width: 2.0,
            median_color: Color::rgb(0.8, 0.2, 0.2),
            median_width: 3.0,
            whisker_color: Color::rgb(0.3, 0.3, 0.3),
            whisker_width: 1.5,
            outlier_color: Color::rgb(0.9, 0.3, 0.3),
            outlier_size: 4.0,
            box_width: 0.6,
        }
    }
}

/// 箱线图数据组
#[derive(Debug, Clone)]
pub struct BoxPlotGroup {
    /// 组标签
    pub label: String,
    /// 统计数据
    pub statistics: BoxStatistics,
}

impl BoxPlotGroup {
    pub fn new<S: Into<String>>(label: S, statistics: BoxStatistics) -> Self {
        Self {
            label: label.into(),
            statistics,
        }
    }

    /// 从原始数据创建组
    pub fn from_data<S: Into<String>>(label: S, data: Vec<f32>) -> Self {
        Self {
            label: label.into(),
            statistics: BoxStatistics::from_data(data),
        }
    }
}

/// 箱线图
#[derive(Debug, Clone)]
pub struct BoxPlot {
    /// 数据组
    groups: Vec<BoxPlotGroup>,
    /// 样式配置
    style: BoxPlotStyle,
    /// 数值范围 (用于 Y 轴缩放)
    value_range: Option<(f32, f32)>,
}

impl BoxPlot {
    /// 创建新的箱线图
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            style: BoxPlotStyle::default(),
            value_range: None,
        }
    }

    /// 添加数据组
    pub fn add_group(mut self, group: BoxPlotGroup) -> Self {
        self.groups.push(group);
        self.compute_value_range();
        self
    }

    /// 从多组原始数据创建箱线图
    pub fn from_data_groups(mut self, data_groups: &[(&str, Vec<f32>)]) -> Self {
        for (label, data) in data_groups {
            self.groups
                .push(BoxPlotGroup::from_data(*label, data.clone()));
        }
        self.compute_value_range();
        self
    }

    /// 设置样式
    pub fn style(mut self, style: BoxPlotStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置箱子颜色
    pub fn box_color(mut self, fill: Color, stroke: Color) -> Self {
        self.style.box_fill_color = fill;
        self.style.box_stroke_color = stroke;
        self
    }

    /// 设置中位数线样式
    pub fn median_style(mut self, color: Color, width: f32) -> Self {
        self.style.median_color = color;
        self.style.median_width = width;
        self
    }

    /// 设置异常值样式
    pub fn outlier_style(mut self, color: Color, size: f32) -> Self {
        self.style.outlier_color = color;
        self.style.outlier_size = size;
        self
    }

    /// 设置数值范围
    pub fn value_range(mut self, min: f32, max: f32) -> Self {
        self.value_range = Some((min, max));
        self
    }

    /// 自动计算数值范围
    pub fn auto_range(mut self) -> Self {
        self.compute_value_range();
        self
    }

    /// 计算数值范围
    fn compute_value_range(&mut self) {
        if self.groups.is_empty() {
            self.value_range = Some((0.0, 1.0));
            return;
        }

        let mut min_val = f32::INFINITY;
        let mut max_val = f32::NEG_INFINITY;

        for group in &self.groups {
            let stats = &group.statistics;

            // 考虑所有数据点 (包括异常值)
            min_val = min_val.min(stats.min);
            max_val = max_val.max(stats.max);

            for &outlier in &stats.outliers {
                min_val = min_val.min(outlier);
                max_val = max_val.max(outlier);
            }
        }

        // 添加一些边距
        let range = max_val - min_val;
        let margin = range * 0.1;

        self.value_range = Some((min_val - margin, max_val + margin));
    }

    /// 获取组数量
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    /// 获取指定组的统计数据
    pub fn get_group(&self, index: usize) -> Option<&BoxPlotGroup> {
        self.groups.get(index)
    }

    /// 生成渲染图元
    pub fn generate_primitives(&self, plot_area: crate::PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.groups.is_empty() {
            return primitives;
        }

        let (min_val, max_val) = self.value_range.unwrap_or((0.0, 1.0));
        let group_count = self.groups.len();
        let group_width = plot_area.width / group_count as f32;
        let box_width = group_width * self.style.box_width;

        for (i, group) in self.groups.iter().enumerate() {
            let center_x = plot_area.x + (i as f32 + 0.5) * group_width;
            let stats = &group.statistics;

            // 转换数值到屏幕 Y 坐标
            let normalize_y = |value: f32| -> f32 {
                let normalized = (value - min_val) / (max_val - min_val);
                plot_area.y + plot_area.height - normalized * plot_area.height
            };

            let min_y = normalize_y(stats.min);
            let q1_y = normalize_y(stats.q1);
            let median_y = normalize_y(stats.median);
            let q3_y = normalize_y(stats.q3);
            let max_y = normalize_y(stats.max);

            // 绘制箱子 (Q1 到 Q3)
            primitives.push(Primitive::RectangleStyled {
                min: Point2::new(center_x - box_width / 2.0, q3_y),
                max: Point2::new(center_x + box_width / 2.0, q1_y),
                fill: self.style.box_fill_color,
                stroke: Some((self.style.box_stroke_color, self.style.box_stroke_width)),
            });

            // 绘制中位数线
            primitives.push(Primitive::Line {
                start: Point2::new(center_x - box_width / 2.0, median_y),
                end: Point2::new(center_x + box_width / 2.0, median_y),
            });

            // 绘制上须线 (Q3 到 max)
            primitives.push(Primitive::Line {
                start: Point2::new(center_x, q3_y),
                end: Point2::new(center_x, max_y),
            });

            // 绘制上须线帽
            let whisker_cap_width = box_width * 0.3;
            primitives.push(Primitive::Line {
                start: Point2::new(center_x - whisker_cap_width / 2.0, max_y),
                end: Point2::new(center_x + whisker_cap_width / 2.0, max_y),
            });

            // 绘制下须线 (Q1 到 min)
            primitives.push(Primitive::Line {
                start: Point2::new(center_x, q1_y),
                end: Point2::new(center_x, min_y),
            });

            // 绘制下须线帽
            primitives.push(Primitive::Line {
                start: Point2::new(center_x - whisker_cap_width / 2.0, min_y),
                end: Point2::new(center_x + whisker_cap_width / 2.0, min_y),
            });

            // 绘制异常值
            for &outlier in &stats.outliers {
                let outlier_y = normalize_y(outlier);
                primitives.push(Primitive::Circle {
                    center: Point2::new(center_x, outlier_y),
                    radius: self.style.outlier_size,
                });
            }

            // 添加组标签
            primitives.push(Primitive::Text {
                position: Point2::new(center_x, plot_area.y + plot_area.height + 20.0),
                content: group.label.clone(),
                size: 12.0,
                color: Color::rgb(0.2, 0.2, 0.2),
                h_align: vizuara_core::HorizontalAlign::Center,
                v_align: vizuara_core::VerticalAlign::Top,
            });
        }

        primitives
    }
}

impl Default for BoxPlot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_statistics_calculation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let stats = BoxStatistics::from_data(data);

        assert_eq!(stats.median, 5.5);
        assert_eq!(stats.q1, 3.25);
        assert_eq!(stats.q3, 7.75);
    }

    #[test]
    fn test_box_statistics_with_outliers() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 100.0]; // 100 是异常值
        let stats = BoxStatistics::from_data(data);

        assert!(!stats.outliers.is_empty());
        assert!(stats.outliers.contains(&100.0));
    }

    #[test]
    fn test_percentile_calculation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        assert_eq!(percentile(&data, 0.0), 1.0);
        assert_eq!(percentile(&data, 50.0), 3.0);
        assert_eq!(percentile(&data, 100.0), 5.0);
    }

    #[test]
    fn test_box_plot_creation() {
        let boxplot = BoxPlot::new();
        assert_eq!(boxplot.group_count(), 0);
    }

    #[test]
    fn test_box_plot_with_groups() {
        let data_groups = &[
            ("Group A", vec![1.0, 2.0, 3.0, 4.0, 5.0]),
            ("Group B", vec![2.0, 3.0, 4.0, 5.0, 6.0]),
        ];

        let boxplot = BoxPlot::new().from_data_groups(data_groups);
        assert_eq!(boxplot.group_count(), 2);

        let group_a = boxplot.get_group(0).unwrap();
        assert_eq!(group_a.label, "Group A");
    }

    #[test]
    fn test_box_plot_styling() {
        let style = BoxPlotStyle {
            box_fill_color: Color::rgb(1.0, 0.0, 0.0),
            median_color: Color::rgb(0.0, 1.0, 0.0),
            ..Default::default()
        };

        let boxplot = BoxPlot::new().style(style.clone());
        assert_eq!(boxplot.style.box_fill_color, Color::rgb(1.0, 0.0, 0.0));
        assert_eq!(boxplot.style.median_color, Color::rgb(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_primitive_generation() {
        let data_groups = &[("Test", vec![1.0, 2.0, 3.0, 4.0, 5.0])];
        let boxplot = BoxPlot::new().from_data_groups(data_groups).auto_range();

        let plot_area = crate::PlotArea::new(0.0, 0.0, 100.0, 100.0);
        let primitives = boxplot.generate_primitives(plot_area);

        // 应该包含箱子、须线、中位数线等图元
        assert!(!primitives.is_empty());
    }

    #[test]
    fn test_empty_data() {
        let stats = BoxStatistics::from_data(vec![]);
        assert_eq!(stats.median, 0.0);
        assert_eq!(stats.q1, 0.0);
        assert_eq!(stats.q3, 0.0);
    }
}
