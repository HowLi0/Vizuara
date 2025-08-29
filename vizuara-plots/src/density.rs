//! 核密度估计图实现
//!
//! 用于显示数据的概率密度分布

use crate::PlotArea;
use nalgebra::Point2;
use std::f32::consts::PI;
use vizuara_core::{Color, HorizontalAlign, LinearScale, Primitive, Scale, VerticalAlign};

/// 密度图数据点
#[derive(Debug, Clone)]
pub struct DensityPoint {
    pub x: f32,
    pub y: f32,
    pub density: f32,
}

/// 密度图样式
#[derive(Debug, Clone)]
pub struct DensityStyle {
    pub line_color: Color,
    pub line_width: f32,
    pub fill_color: Option<Color>,
    pub bandwidth: f32,
    pub resolution: usize,
    pub show_area: bool,
    pub show_points: bool,
    pub point_size: f32,
}

impl Default for DensityStyle {
    fn default() -> Self {
        Self {
            line_color: Color::rgb(0.2, 0.6, 0.9),
            line_width: 2.0,
            fill_color: Some(Color::rgba(0.2, 0.6, 0.9, 0.3)),
            bandwidth: 1.0,
            resolution: 200,
            show_area: true,
            show_points: false,
            point_size: 3.0,
        }
    }
}

/// 密度图
#[derive(Debug, Clone)]
pub struct DensityPlot {
    data: Vec<f32>,
    style: DensityStyle,
    title: Option<String>,
    x_range: Option<(f32, f32)>,
    kernel_type: KernelType,
}

/// 核函数类型
#[derive(Debug, Clone, Copy)]
pub enum KernelType {
    Gaussian,
    Epanechnikov,
    Triangular,
    Uniform,
}

impl DensityPlot {
    /// 创建新的密度图
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            style: DensityStyle::default(),
            title: None,
            x_range: None,
            kernel_type: KernelType::Gaussian,
        }
    }

    /// 设置数据
    pub fn data(mut self, data: &[f32]) -> Self {
        self.data = data.to_vec();
        self
    }

    /// 设置标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置线条颜色
    pub fn line_color(mut self, color: Color) -> Self {
        self.style.line_color = color;
        self
    }

    /// 设置线条宽度
    pub fn line_width(mut self, width: f32) -> Self {
        self.style.line_width = width;
        self
    }

    /// 设置填充颜色
    pub fn fill_color(mut self, color: Option<Color>) -> Self {
        self.style.fill_color = color;
        self
    }

    /// 设置带宽（控制平滑程度）
    pub fn bandwidth(mut self, bandwidth: f32) -> Self {
        self.style.bandwidth = bandwidth;
        self
    }

    /// 设置分辨率
    pub fn resolution(mut self, resolution: usize) -> Self {
        self.style.resolution = resolution;
        self
    }

    /// 设置核函数类型
    pub fn kernel(mut self, kernel_type: KernelType) -> Self {
        self.kernel_type = kernel_type;
        self
    }

    /// 设置是否显示区域填充
    pub fn show_area(mut self, show: bool) -> Self {
        self.style.show_area = show;
        self
    }

    /// 设置是否显示数据点
    pub fn show_points(mut self, show: bool, size: f32) -> Self {
        self.style.show_points = show;
        self.style.point_size = size;
        self
    }

    /// 设置X轴范围
    pub fn x_range(mut self, min: f32, max: f32) -> Self {
        self.x_range = Some((min, max));
        self
    }

    /// 计算核密度估计
    fn compute_kde(&self) -> Vec<DensityPoint> {
        if self.data.is_empty() {
            return Vec::new();
        }

        let (x_min, x_max) = if let Some((min, max)) = self.x_range {
            (min, max)
        } else {
            let min = self.data.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            let max = self.data.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
            let range = max - min;
            (min - range * 0.1, max + range * 0.1)
        };

        let step = (x_max - x_min) / (self.style.resolution as f32);
        let mut points = Vec::new();

        for i in 0..=self.style.resolution {
            let x = x_min + i as f32 * step;
            let mut density = 0.0;

            for &data_point in &self.data {
                let u = (x - data_point) / self.style.bandwidth;
                density += self.kernel_function(u);
            }

            density /= (self.data.len() as f32) * self.style.bandwidth;
            points.push(DensityPoint { x, y: 0.0, density });
        }

        points
    }

    /// 核函数
    fn kernel_function(&self, u: f32) -> f32 {
        match self.kernel_type {
            KernelType::Gaussian => (1.0 / (2.0 * PI).sqrt()) * (-0.5 * u * u).exp(),
            KernelType::Epanechnikov => {
                if u.abs() <= 1.0 {
                    0.75 * (1.0 - u * u)
                } else {
                    0.0
                }
            }
            KernelType::Triangular => {
                if u.abs() <= 1.0 {
                    1.0 - u.abs()
                } else {
                    0.0
                }
            }
            KernelType::Uniform => {
                if u.abs() <= 1.0 {
                    0.5
                } else {
                    0.0
                }
            }
        }
    }

    /// 生成渲染图元
    pub fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.data.is_empty() {
            return primitives;
        }

        // 计算密度
        let density_points = self.compute_kde();
        if density_points.is_empty() {
            return primitives;
        }

        // 设置坐标尺度
        let x_min = density_points.first().unwrap().x;
        let x_max = density_points.last().unwrap().x;
        let y_max = density_points.iter().map(|p| p.density).fold(0.0, f32::max);

        let x_scale = LinearScale::new(x_min, x_max);
        let y_scale = LinearScale::new(0.0, y_max * 1.1);

        // 创建密度曲线的点
        let mut curve_points = Vec::new();
        for point in &density_points {
            let screen_x = plot_area.x + x_scale.normalize(point.x) * plot_area.width;
            let screen_y = plot_area.y + plot_area.height
                - y_scale.normalize(point.density) * plot_area.height;
            curve_points.push(Point2::new(screen_x, screen_y));
        }

        // 绘制填充区域
        if self.style.show_area && self.style.fill_color.is_some() {
            let mut fill_points = curve_points.clone();

            // 添加底部的点来形成封闭的多边形
            let baseline_y = plot_area.y + plot_area.height;
            fill_points.push(Point2::new(curve_points.last().unwrap().x, baseline_y));
            fill_points.push(Point2::new(curve_points.first().unwrap().x, baseline_y));

            primitives.push(Primitive::Polygon {
                points: fill_points,
                fill: self.style.fill_color.unwrap(),
                stroke: None,
            });
        }

        // 绘制密度曲线
        if curve_points.len() > 1 {
            primitives.push(Primitive::Polyline {
                points: curve_points,
                color: self.style.line_color,
                width: self.style.line_width,
            });
        }

        // 绘制数据点
        if self.style.show_points {
            for &data_point in &self.data {
                let screen_x = plot_area.x + x_scale.normalize(data_point) * plot_area.width;
                let baseline_y = plot_area.y + plot_area.height;

                primitives.push(Primitive::Circle {
                    center: Point2::new(screen_x, baseline_y - 5.0),
                    radius: self.style.point_size,
                });
            }
        }

        // 绘制标题
        if let Some(ref title) = self.title {
            primitives.push(Primitive::Text {
                position: Point2::new(plot_area.x + plot_area.width / 2.0, plot_area.y - 20.0),
                content: title.clone(),
                size: 14.0,
                color: Color::rgb(0.1, 0.1, 0.1),
                h_align: HorizontalAlign::Center,
                v_align: VerticalAlign::Bottom,
            });
        }

        primitives
    }

    /// 获取数据长度
    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    /// 获取数据统计信息
    pub fn statistics(&self) -> Option<DensityStatistics> {
        if self.data.is_empty() {
            return None;
        }

        let n = self.data.len() as f32;
        let mean = self.data.iter().sum::<f32>() / n;
        let variance = self.data.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / (n - 1.0);
        let std_dev = variance.sqrt();

        let mut sorted = self.data.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let median = if sorted.len() % 2 == 0 {
            (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
        } else {
            sorted[sorted.len() / 2]
        };

        Some(DensityStatistics {
            mean,
            median,
            std_dev,
            min: sorted[0],
            max: sorted[sorted.len() - 1],
            count: sorted.len(),
        })
    }
}

impl Default for DensityPlot {
    fn default() -> Self {
        Self::new()
    }
}

/// 密度图统计信息
#[derive(Debug, Clone)]
pub struct DensityStatistics {
    pub mean: f32,
    pub median: f32,
    pub std_dev: f32,
    pub min: f32,
    pub max: f32,
    pub count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_plot_creation() {
        let plot = DensityPlot::new();
        assert_eq!(plot.data_len(), 0);
    }

    #[test]
    fn test_density_plot_with_data() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let plot = DensityPlot::new().data(&data);
        assert_eq!(plot.data_len(), 5);
    }

    #[test]
    fn test_kde_computation() {
        let data = vec![1.0, 2.0, 3.0];
        let plot = DensityPlot::new().data(&data).bandwidth(1.0);
        let kde = plot.compute_kde();
        assert!(!kde.is_empty());
    }

    #[test]
    fn test_kernel_functions() {
        let plot = DensityPlot::new();

        // 测试高斯核
        assert!(plot.kernel_function(0.0) > 0.0);

        // 测试Epanechnikov核
        let plot_epan = DensityPlot::new().kernel(KernelType::Epanechnikov);
        assert!(plot_epan.kernel_function(0.0) > 0.0);
        assert_eq!(plot_epan.kernel_function(2.0), 0.0);
    }

    #[test]
    fn test_statistics() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let plot = DensityPlot::new().data(&data);
        let stats = plot.statistics().unwrap();

        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.median, 3.0);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.count, 5);
    }

    #[test]
    fn test_density_primitives() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let plot = DensityPlot::new().data(&data).title("测试密度图");

        let plot_area = PlotArea::new(0.0, 0.0, 400.0, 300.0);
        let primitives = plot.generate_primitives(plot_area);
        assert!(!primitives.is_empty());
    }
}
