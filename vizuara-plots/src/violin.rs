use nalgebra::Point2;
use vizuara_core::{Color, Primitive};

/// 核密度估计结果
#[derive(Debug, Clone)]
pub struct DensityEstimate {
    /// 分布点
    pub points: Vec<f32>,
    /// 对应的密度值
    pub densities: Vec<f32>,
    /// 最大密度值
    pub max_density: f32,
}

impl DensityEstimate {
    /// 使用高斯核进行核密度估计
    pub fn from_data(data: &[f32], bandwidth: Option<f32>) -> Self {
        if data.is_empty() {
            return Self {
                points: Vec::new(),
                densities: Vec::new(),
                max_density: 0.0,
            };
        }

        let data_min = data.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let data_max = data.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let range = data_max - data_min;
        
        // 自动计算带宽（Silverman's rule of thumb）
        let bw = bandwidth.unwrap_or_else(|| {
            let n = data.len() as f32;
            let std_dev = Self::calculate_std_dev(data);
            0.9 * std_dev * n.powf(-0.2)
        });

        // 创建评估点
        let num_points = 100;
        let margin = range * 0.2;
        let start = data_min - margin;
        let end = data_max + margin;
        let step = (end - start) / (num_points - 1) as f32;
        
        let mut points = Vec::new();
        let mut densities = Vec::new();
        let mut max_density: f32 = 0.0;

        for i in 0..num_points {
            let x = start + i as f32 * step;
            let density = Self::gaussian_kde(data, x, bw);
            
            points.push(x);
            densities.push(density);
            max_density = max_density.max(density);
        }

        Self {
            points,
            densities,
            max_density,
        }
    }

    fn calculate_std_dev(data: &[f32]) -> f32 {
        let mean = data.iter().sum::<f32>() / data.len() as f32;
        let variance = data.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f32>() / data.len() as f32;
        variance.sqrt()
    }

    fn gaussian_kde(data: &[f32], x: f32, bandwidth: f32) -> f32 {
        let n = data.len() as f32;
        let sum = data.iter()
            .map(|&xi| {
                let u = (x - xi) / bandwidth;
                (-0.5 * u * u).exp()
            })
            .sum::<f32>();
        
        sum / (n * bandwidth * (2.0 * std::f32::consts::PI).sqrt())
    }
}

/// 小提琴图统计数据
#[derive(Debug, Clone)]
pub struct ViolinStatistics {
    /// 最小值
    pub min: f32,
    /// 第一四分位数
    pub q1: f32,
    /// 中位数
    pub median: f32,
    /// 第三四分位数
    pub q3: f32,
    /// 最大值
    pub max: f32,
    /// 平均值
    pub mean: f32,
    /// 异常值
    pub outliers: Vec<f32>,
    /// 密度估计
    pub density: DensityEstimate,
}

impl ViolinStatistics {
    /// 从原始数据计算统计信息
    pub fn from_data(mut data: Vec<f32>) -> Self {
        if data.is_empty() {
            return Self {
                min: 0.0,
                q1: 0.0,
                median: 0.0,
                q3: 0.0,
                max: 0.0,
                mean: 0.0,
                outliers: Vec::new(),
                density: DensityEstimate::from_data(&[], None),
            };
        }

        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = data.len();

        let min = data[0];
        let max = data[len - 1];
        let median = if len % 2 == 0 {
            (data[len / 2 - 1] + data[len / 2]) / 2.0
        } else {
            data[len / 2]
        };

        let q1_idx = len / 4;
        let q3_idx = 3 * len / 4;
        let q1 = data[q1_idx];
        let q3 = data[q3_idx];

        let mean = data.iter().sum::<f32>() / len as f32;

        // 计算异常值 (使用 1.5 * IQR 规则)
        let iqr = q3 - q1;
        let lower_fence = q1 - 1.5 * iqr;
        let upper_fence = q3 + 1.5 * iqr;
        let outliers = data.iter()
            .filter(|&&x| x < lower_fence || x > upper_fence)
            .cloned()
            .collect();

        // 计算密度估计
        let density = DensityEstimate::from_data(&data, None);

        Self {
            min,
            q1,
            median,
            q3,
            max,
            mean,
            outliers,
            density,
        }
    }
}

/// 小提琴图数据组
#[derive(Debug, Clone)]
pub struct ViolinGroup {
    /// 组标签
    pub label: String,
    /// 统计数据
    pub statistics: ViolinStatistics,
}

impl ViolinGroup {
    pub fn new<S: Into<String>>(label: S, statistics: ViolinStatistics) -> Self {
        Self {
            label: label.into(),
            statistics,
        }
    }

    /// 从原始数据创建组
    pub fn from_data<S: Into<String>>(label: S, data: Vec<f32>) -> Self {
        Self {
            label: label.into(),
            statistics: ViolinStatistics::from_data(data),
        }
    }
}

/// 小提琴图样式配置
#[derive(Debug, Clone)]
pub struct ViolinStyle {
    /// 小提琴填充颜色
    pub violin_fill_color: Color,
    /// 小提琴边框颜色
    pub violin_stroke_color: Color,
    /// 小提琴边框宽度
    pub violin_stroke_width: f32,
    /// 中位数线颜色
    pub median_color: Color,
    /// 中位数线宽度
    pub median_width: f32,
    /// 平均值点颜色
    pub mean_color: Color,
    /// 平均值点大小
    pub mean_size: f32,
    /// 四分位数线颜色
    pub quartile_color: Color,
    /// 四分位数线宽度
    pub quartile_width: f32,
    /// 异常值颜色
    pub outlier_color: Color,
    /// 异常值大小
    pub outlier_size: f32,
    /// 小提琴宽度 (相对于间距的比例)
    pub violin_width: f32,
    /// 是否显示箱线图
    pub show_box: bool,
    /// 箱线图宽度比例
    pub box_width: f32,
    /// 箱线图颜色
    pub box_color: Color,
    /// 是否显示数据点
    pub show_points: bool,
    /// 数据点颜色
    pub point_color: Color,
    /// 数据点大小
    pub point_size: f32,
    /// 数据点透明度
    pub point_alpha: f32,
}

impl Default for ViolinStyle {
    fn default() -> Self {
        Self {
            violin_fill_color: Color::rgba(0.2, 0.6, 0.9, 0.6),
            violin_stroke_color: Color::rgb(0.1, 0.4, 0.7),
            violin_stroke_width: 1.5,
            median_color: Color::rgb(0.0, 0.0, 0.0),
            median_width: 2.0,
            mean_color: Color::rgb(1.0, 0.0, 0.0),
            mean_size: 4.0,
            quartile_color: Color::rgb(0.3, 0.3, 0.3),
            quartile_width: 1.0,
            outlier_color: Color::rgb(1.0, 0.5, 0.0),
            outlier_size: 3.0,
            violin_width: 0.8,
            show_box: true,
            box_width: 0.1,
            box_color: Color::rgb(0.0, 0.0, 0.0),
            show_points: false,
            point_color: Color::rgba(0.2, 0.2, 0.2, 0.6),
            point_size: 2.0,
            point_alpha: 0.6,
        }
    }
}

/// 小提琴图
#[derive(Debug, Clone)]
pub struct ViolinPlot {
    /// 数据组
    groups: Vec<ViolinGroup>,
    /// 样式配置
    style: ViolinStyle,
    /// 数值范围 (用于 Y 轴缩放)
    value_range: Option<(f32, f32)>,
    /// 标题
    title: Option<String>,
}

impl ViolinPlot {
    /// 创建新的小提琴图
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            style: ViolinStyle::default(),
            value_range: None,
            title: None,
        }
    }

    /// 添加数据组
    pub fn add_group(mut self, group: ViolinGroup) -> Self {
        self.groups.push(group);
        self
    }

    /// 从多组原始数据创建小提琴图
    pub fn from_data_groups(mut self, data_groups: &[(&str, Vec<f32>)]) -> Self {
        for &(label, ref data) in data_groups {
            let group = ViolinGroup::from_data(label, data.clone());
            self.groups.push(group);
        }
        self
    }

    /// 设置样式
    pub fn style(mut self, style: ViolinStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置小提琴颜色
    pub fn violin_color(mut self, fill: Color, stroke: Color) -> Self {
        self.style.violin_fill_color = fill;
        self.style.violin_stroke_color = stroke;
        self
    }

    /// 设置中位数线样式
    pub fn median_style(mut self, color: Color, width: f32) -> Self {
        self.style.median_color = color;
        self.style.median_width = width;
        self
    }

    /// 设置平均值点样式
    pub fn mean_style(mut self, color: Color, size: f32) -> Self {
        self.style.mean_color = color;
        self.style.mean_size = size;
        self
    }

    /// 设置是否显示箱线图
    pub fn show_box(mut self, show: bool, width: f32) -> Self {
        self.style.show_box = show;
        self.style.box_width = width;
        self
    }

    /// 设置是否显示数据点
    pub fn show_points(mut self, show: bool, size: f32, alpha: f32) -> Self {
        self.style.show_points = show;
        self.style.point_size = size;
        self.style.point_alpha = alpha;
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

    /// 设置标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 计算数值范围
    fn compute_value_range(&mut self) {
        if self.groups.is_empty() {
            return;
        }

        let mut min_val = f32::INFINITY;
        let mut max_val = f32::NEG_INFINITY;

        for group in &self.groups {
            let stats = &group.statistics;
            min_val = min_val.min(stats.min);
            max_val = max_val.max(stats.max);
        }

        let margin = (max_val - min_val) * 0.1;
        self.value_range = Some((min_val - margin, max_val + margin));
    }

    /// 获取组数量
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    /// 生成渲染图元
    pub fn generate_primitives(&self, plot_area: super::PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.groups.is_empty() {
            return primitives;
        }

        let (min_val, max_val) = self.value_range.unwrap_or((0.0, 1.0));
        let group_count = self.groups.len();
        let group_width = plot_area.width / group_count as f32;
        let violin_width = group_width * self.style.violin_width;

        for (i, group) in self.groups.iter().enumerate() {
            let center_x = plot_area.x + (i as f32 + 0.5) * group_width;
            let stats = &group.statistics;

            // 转换数值到屏幕 Y 坐标
            let normalize_y = |value: f32| -> f32 {
                let normalized = (value - min_val) / (max_val - min_val);
                plot_area.y + plot_area.height - normalized * plot_area.height
            };

            // 绘制小提琴形状
            self.draw_violin_shape(&mut primitives, center_x, violin_width, stats, &normalize_y);

            // 绘制箱线图（如果启用）
            if self.style.show_box {
                self.draw_box_plot(&mut primitives, center_x, group_width * self.style.box_width, stats, &normalize_y);
            }

            // 绘制中位数线
            let median_y = normalize_y(stats.median);
            let box_half_width = group_width * self.style.box_width / 2.0;
            primitives.push(Primitive::Line {
                start: Point2::new(center_x - box_half_width, median_y),
                end: Point2::new(center_x + box_half_width, median_y),
            });

            // 绘制平均值点
            let mean_y = normalize_y(stats.mean);
            primitives.push(Primitive::Circle {
                center: Point2::new(center_x, mean_y),
                radius: self.style.mean_size,
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

        // 添加标题
        if let Some(ref title) = self.title {
            primitives.push(Primitive::Text {
                position: Point2::new(plot_area.x + plot_area.width / 2.0, plot_area.y - 20.0),
                content: title.clone(),
                size: 16.0,
                color: Color::rgb(0.2, 0.2, 0.2),
                h_align: vizuara_core::HorizontalAlign::Center,
                v_align: vizuara_core::VerticalAlign::Bottom,
            });
        }

        primitives
    }

    fn draw_violin_shape<F>(
        &self,
        primitives: &mut Vec<Primitive>,
        center_x: f32,
        violin_width: f32,
        stats: &ViolinStatistics,
        normalize_y: F,
    ) where
        F: Fn(f32) -> f32,
    {
        if stats.density.points.is_empty() {
            return;
        }

        let max_density = stats.density.max_density;
        if max_density <= 0.0 {
            return;
        }

        // 创建小提琴轮廓点
        let mut left_points = Vec::new();
        let mut right_points = Vec::new();

        for (point, density) in stats.density.points.iter().zip(&stats.density.densities) {
            let y = normalize_y(*point);
            let width_factor = density / max_density;
            let half_width = violin_width * width_factor / 2.0;

            left_points.push(Point2::new(center_x - half_width, y));
            right_points.push(Point2::new(center_x + half_width, y));
        }

        // 合并成完整的小提琴轮廓
        let mut violin_points = left_points;
        right_points.reverse();
        violin_points.extend(right_points);

        if violin_points.len() >= 3 {
            primitives.push(Primitive::Polygon {
                points: violin_points,
                fill: self.style.violin_fill_color,
                stroke: Some((self.style.violin_stroke_color, self.style.violin_stroke_width)),
            });
        }
    }

    fn draw_box_plot<F>(
        &self,
        primitives: &mut Vec<Primitive>,
        center_x: f32,
        box_width: f32,
        stats: &ViolinStatistics,
        normalize_y: F,
    ) where
        F: Fn(f32) -> f32,
    {
        let q1_y = normalize_y(stats.q1);
        let q3_y = normalize_y(stats.q3);
        let half_width = box_width / 2.0;

        // 绘制箱子
        primitives.push(Primitive::RectangleStyled {
            min: Point2::new(center_x - half_width, q3_y),
            max: Point2::new(center_x + half_width, q1_y),
            fill: Color::rgba(1.0, 1.0, 1.0, 0.8),
            stroke: Some((self.style.box_color, 1.0)),
        });

        // 绘制四分位数线
        primitives.push(Primitive::Line {
            start: Point2::new(center_x - half_width, q1_y),
            end: Point2::new(center_x + half_width, q1_y),
        });
        primitives.push(Primitive::Line {
            start: Point2::new(center_x - half_width, q3_y),
            end: Point2::new(center_x + half_width, q3_y),
        });
    }
}

impl Default for ViolinPlot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_violin_plot_creation() {
        let plot = ViolinPlot::new();
        assert_eq!(plot.group_count(), 0);
    }

    #[test]
    fn test_violin_statistics() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = ViolinStatistics::from_data(data);
        
        assert_eq!(stats.median, 3.0);
        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
    }

    #[test]
    fn test_density_estimation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let density = DensityEstimate::from_data(&data, Some(1.0));
        
        assert!(!density.points.is_empty());
        assert!(!density.densities.is_empty());
        assert!(density.max_density > 0.0);
    }

    #[test]
    fn test_violin_from_data() {
        let data_groups = [
            ("组A", vec![1.0, 2.0, 3.0, 4.0, 5.0]),
            ("组B", vec![2.0, 3.0, 4.0, 5.0, 6.0]),
        ];
        
        let plot = ViolinPlot::new().from_data_groups(&data_groups);
        assert_eq!(plot.group_count(), 2);
    }
}
