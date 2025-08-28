use nalgebra::Point2;
use std::f32::consts::PI;
use vizuara_core::{Color, Primitive};

/// 雷达图维度
#[derive(Debug, Clone)]
pub struct RadarDimension {
    /// 维度名称
    pub name: String,
    /// 最小值
    pub min_value: f32,
    /// 最大值
    pub max_value: f32,
    /// 显示标签
    pub show_label: bool,
}

impl RadarDimension {
    pub fn new<S: Into<String>>(name: S, min_value: f32, max_value: f32) -> Self {
        Self {
            name: name.into(),
            min_value,
            max_value,
            show_label: true,
        }
    }

    /// 标准化数值到 [0, 1] 范围
    pub fn normalize(&self, value: f32) -> f32 {
        if self.max_value == self.min_value {
            0.5
        } else {
            ((value - self.min_value) / (self.max_value - self.min_value)).clamp(0.0, 1.0)
        }
    }
}

/// 雷达图数据系列
#[derive(Debug, Clone)]
pub struct RadarSeries {
    /// 系列名称
    pub name: String,
    /// 数据点值
    pub values: Vec<f32>,
    /// 填充颜色
    pub fill_color: Color,
    /// 线条颜色
    pub line_color: Color,
    /// 线条宽度
    pub line_width: f32,
    /// 填充透明度
    pub fill_alpha: f32,
    /// 是否显示数据点
    pub show_points: bool,
    /// 数据点大小
    pub point_size: f32,
}

impl RadarSeries {
    pub fn new<S: Into<String>>(name: S, values: Vec<f32>) -> Self {
        Self {
            name: name.into(),
            values,
            fill_color: Color::rgba(0.2, 0.6, 0.9, 0.3),
            line_color: Color::rgb(0.2, 0.6, 0.9),
            line_width: 2.0,
            fill_alpha: 0.3,
            show_points: true,
            point_size: 4.0,
        }
    }

    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    pub fn line_color(mut self, color: Color) -> Self {
        self.line_color = color;
        self
    }

    pub fn line_width(mut self, width: f32) -> Self {
        self.line_width = width;
        self
    }

    pub fn fill_alpha(mut self, alpha: f32) -> Self {
        self.fill_alpha = alpha.clamp(0.0, 1.0);
        self
    }

    pub fn show_points(mut self, show: bool, size: f32) -> Self {
        self.show_points = show;
        self.point_size = size;
        self
    }
}

/// 雷达图样式配置
#[derive(Debug, Clone)]
pub struct RadarStyle {
    /// 网格线颜色
    pub grid_color: Color,
    /// 网格线宽度
    pub grid_width: f32,
    /// 轴线颜色
    pub axis_color: Color,
    /// 轴线宽度
    pub axis_width: f32,
    /// 标签字体大小
    pub label_size: f32,
    /// 标签颜色
    pub label_color: Color,
    /// 标签距离中心的距离比例
    pub label_distance: f32,
    /// 网格层数
    pub grid_levels: usize,
    /// 是否显示刻度值
    pub show_scale_values: bool,
    /// 刻度值字体大小
    pub scale_value_size: f32,
    /// 刻度值颜色
    pub scale_value_color: Color,
}

impl Default for RadarStyle {
    fn default() -> Self {
        Self {
            grid_color: Color::rgb(0.7, 0.7, 0.7),
            grid_width: 1.0,
            axis_color: Color::rgb(0.5, 0.5, 0.5),
            axis_width: 1.0,
            label_size: 12.0,
            label_color: Color::rgb(0.2, 0.2, 0.2),
            label_distance: 1.15,
            grid_levels: 5,
            show_scale_values: true,
            scale_value_size: 10.0,
            scale_value_color: Color::rgb(0.4, 0.4, 0.4),
        }
    }
}

/// 雷达图
#[derive(Debug, Clone)]
pub struct RadarChart {
    /// 维度定义
    dimensions: Vec<RadarDimension>,
    /// 数据系列
    series: Vec<RadarSeries>,
    /// 样式配置
    style: RadarStyle,
    /// 中心点
    center: Point2<f32>,
    /// 半径
    radius: f32,
    /// 起始角度（弧度，0 = 右侧，-PI/2 = 顶部）
    start_angle: f32,
    /// 标题
    title: Option<String>,
    /// 默认颜色
    default_colors: Vec<(Color, Color)>, // (fill, line)
}

impl RadarChart {
    /// 创建新的雷达图
    pub fn new() -> Self {
        let default_colors = vec![
            (Color::rgba(0.2, 0.6, 0.9, 0.3), Color::rgb(0.2, 0.6, 0.9)),   // 蓝色
            (Color::rgba(0.9, 0.5, 0.2, 0.3), Color::rgb(0.9, 0.5, 0.2)),   // 橙色
            (Color::rgba(0.4, 0.8, 0.4, 0.3), Color::rgb(0.4, 0.8, 0.4)),   // 绿色
            (Color::rgba(0.9, 0.3, 0.3, 0.3), Color::rgb(0.9, 0.3, 0.3)),   // 红色
            (Color::rgba(0.7, 0.4, 0.9, 0.3), Color::rgb(0.7, 0.4, 0.9)),   // 紫色
            (Color::rgba(0.9, 0.9, 0.3, 0.3), Color::rgb(0.9, 0.9, 0.3)),   // 黄色
        ];

        Self {
            dimensions: Vec::new(),
            series: Vec::new(),
            style: RadarStyle::default(),
            center: Point2::new(200.0, 200.0),
            radius: 150.0,
            start_angle: -PI / 2.0, // 从顶部开始
            title: None,
            default_colors,
        }
    }

    /// 设置维度
    pub fn dimensions(mut self, dimensions: Vec<RadarDimension>) -> Self {
        self.dimensions = dimensions;
        self
    }

    /// 添加维度
    pub fn add_dimension(mut self, dimension: RadarDimension) -> Self {
        self.dimensions.push(dimension);
        self
    }

    /// 快速创建维度（所有维度使用相同的范围）
    pub fn simple_dimensions(mut self, names: &[&str], min_value: f32, max_value: f32) -> Self {
        self.dimensions = names.iter()
            .map(|&name| RadarDimension::new(name, min_value, max_value))
            .collect();
        self
    }

    /// 添加数据系列
    pub fn add_series(mut self, series: RadarSeries) -> Self {
        self.series.push(series);
        self
    }

    /// 快速添加数据系列
    pub fn add_data(mut self, name: &str, values: Vec<f32>) -> Self {
        let series = RadarSeries::new(name, values);
        self.series.push(series);
        self
    }

    /// 设置样式
    pub fn style(mut self, style: RadarStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置中心点和半径
    pub fn center_radius(mut self, center_x: f32, center_y: f32, radius: f32) -> Self {
        self.center = Point2::new(center_x, center_y);
        self.radius = radius;
        self
    }

    /// 设置起始角度
    pub fn start_angle(mut self, angle: f32) -> Self {
        self.start_angle = angle;
        self
    }

    /// 设置网格样式
    pub fn grid_style(mut self, color: Color, width: f32, levels: usize) -> Self {
        self.style.grid_color = color;
        self.style.grid_width = width;
        self.style.grid_levels = levels;
        self
    }

    /// 设置标签样式
    pub fn label_style(mut self, size: f32, color: Color, distance: f32) -> Self {
        self.style.label_size = size;
        self.style.label_color = color;
        self.style.label_distance = distance;
        self
    }

    /// 设置标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 获取维度数量
    pub fn dimension_count(&self) -> usize {
        self.dimensions.len()
    }

    /// 获取系列数量
    pub fn series_count(&self) -> usize {
        self.series.len()
    }

    /// 生成渲染图元
    pub fn generate_primitives(&self, _plot_area: super::PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.dimensions.is_empty() {
            return primitives;
        }

        // 绘制网格
        self.draw_grid(&mut primitives);

        // 绘制轴线
        self.draw_axes(&mut primitives);

        // 绘制标签
        self.draw_labels(&mut primitives);

        // 绘制数据系列
        self.draw_series(&mut primitives);

        // 添加标题
        if let Some(ref title) = self.title {
            primitives.push(Primitive::Text {
                position: Point2::new(self.center.x, self.center.y - self.radius - 40.0),
                content: title.clone(),
                size: 16.0,
                color: Color::rgb(0.2, 0.2, 0.2),
                h_align: vizuara_core::HorizontalAlign::Center,
                v_align: vizuara_core::VerticalAlign::Middle,
            });
        }

        primitives
    }

    fn draw_grid(&self, primitives: &mut Vec<Primitive>) {
        let dim_count = self.dimensions.len();
        if dim_count < 3 {
            return; // 至少需要3个维度才能形成多边形
        }

        // 绘制同心多边形网格
        for level in 1..=self.style.grid_levels {
            let level_radius = self.radius * (level as f32) / (self.style.grid_levels as f32);
            let mut grid_points = Vec::new();

            for i in 0..dim_count {
                let angle = self.start_angle + (i as f32) * 2.0 * PI / (dim_count as f32);
                let x = self.center.x + level_radius * angle.cos();
                let y = self.center.y + level_radius * angle.sin();
                grid_points.push(Point2::new(x, y));
            }

            // 闭合多边形
            if let Some(first_point) = grid_points.first().cloned() {
                grid_points.push(first_point);
            }

            if grid_points.len() >= 2 {
                primitives.push(Primitive::Polyline {
                    points: grid_points,
                    color: self.style.grid_color,
                    width: self.style.grid_width,
                });
            }

            // 添加刻度值标签
            if self.style.show_scale_values && level < self.style.grid_levels {
                let value_ratio = level as f32 / self.style.grid_levels as f32;
                let label_x = self.center.x + level_radius * self.start_angle.cos() + 5.0;
                let label_y = self.center.y + level_radius * self.start_angle.sin();

                primitives.push(Primitive::Text {
                    position: Point2::new(label_x, label_y),
                    content: format!("{:.1}", value_ratio),
                    size: self.style.scale_value_size,
                    color: self.style.scale_value_color,
                    h_align: vizuara_core::HorizontalAlign::Left,
                    v_align: vizuara_core::VerticalAlign::Middle,
                });
            }
        }
    }

    fn draw_axes(&self, primitives: &mut Vec<Primitive>) {
        let dim_count = self.dimensions.len();

        for i in 0..dim_count {
            let angle = self.start_angle + (i as f32) * 2.0 * PI / (dim_count as f32);
            let end_x = self.center.x + self.radius * angle.cos();
            let end_y = self.center.y + self.radius * angle.sin();

            primitives.push(Primitive::Line {
                start: self.center,
                end: Point2::new(end_x, end_y),
            });
        }
    }

    fn draw_labels(&self, primitives: &mut Vec<Primitive>) {
        let dim_count = self.dimensions.len();

        for (i, dimension) in self.dimensions.iter().enumerate() {
            if !dimension.show_label {
                continue;
            }

            let angle = self.start_angle + (i as f32) * 2.0 * PI / (dim_count as f32);
            let label_radius = self.radius * self.style.label_distance;
            let label_x = self.center.x + label_radius * angle.cos();
            let label_y = self.center.y + label_radius * angle.sin();

            // 根据角度调整文本对齐方式
            let h_align = if angle.cos() > 0.1 {
                vizuara_core::HorizontalAlign::Left
            } else if angle.cos() < -0.1 {
                vizuara_core::HorizontalAlign::Right
            } else {
                vizuara_core::HorizontalAlign::Center
            };

            let v_align = if angle.sin() > 0.1 {
                vizuara_core::VerticalAlign::Bottom
            } else if angle.sin() < -0.1 {
                vizuara_core::VerticalAlign::Top
            } else {
                vizuara_core::VerticalAlign::Middle
            };

            primitives.push(Primitive::Text {
                position: Point2::new(label_x, label_y),
                content: dimension.name.clone(),
                size: self.style.label_size,
                color: self.style.label_color,
                h_align,
                v_align,
            });
        }
    }

    fn draw_series(&self, primitives: &mut Vec<Primitive>) {
        let dim_count = self.dimensions.len();
        if dim_count < 3 {
            return;
        }

        for (series_idx, series) in self.series.iter().enumerate() {
            if series.values.len() != dim_count {
                continue; // 跳过维度数量不匹配的系列
            }

            // 选择颜色
            let (default_fill, default_line) = self.default_colors[series_idx % self.default_colors.len()];
            let fill_color = if series.fill_color == Color::rgba(0.2, 0.6, 0.9, 0.3) {
                default_fill
            } else {
                series.fill_color
            };
            let line_color = if series.line_color == Color::rgb(0.2, 0.6, 0.9) {
                default_line
            } else {
                series.line_color
            };

            // 计算数据点坐标
            let mut data_points = Vec::new();
            for (i, &value) in series.values.iter().enumerate() {
                let dimension = &self.dimensions[i];
                let normalized_value = dimension.normalize(value);
                let point_radius = self.radius * normalized_value;
                
                let angle = self.start_angle + (i as f32) * 2.0 * PI / (dim_count as f32);
                let x = self.center.x + point_radius * angle.cos();
                let y = self.center.y + point_radius * angle.sin();
                data_points.push(Point2::new(x, y));
            }

            // 闭合多边形
            if let Some(first_point) = data_points.first().cloned() {
                data_points.push(first_point);
            }

            // 绘制填充区域
            if data_points.len() >= 4 {
                let mut fill_points = data_points.clone();
                fill_points.pop(); // 移除重复的最后一个点

                primitives.push(Primitive::Polygon {
                    points: fill_points,
                    fill: fill_color,
                    stroke: None,
                });
            }

            // 绘制边界线
            if data_points.len() >= 2 {
                primitives.push(Primitive::Polyline {
                    points: data_points.clone(),
                    color: line_color,
                    width: series.line_width,
                });
            }

            // 绘制数据点
            if series.show_points {
                for (i, &value) in series.values.iter().enumerate() {
                    let dimension = &self.dimensions[i];
                    let normalized_value = dimension.normalize(value);
                    let point_radius = self.radius * normalized_value;
                    
                    let angle = self.start_angle + (i as f32) * 2.0 * PI / (dim_count as f32);
                    let x = self.center.x + point_radius * angle.cos();
                    let y = self.center.y + point_radius * angle.sin();

                    primitives.push(Primitive::Circle {
                        center: Point2::new(x, y),
                        radius: series.point_size,
                    });
                }
            }
        }
    }
}

impl Default for RadarChart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radar_chart_creation() {
        let chart = RadarChart::new();
        assert_eq!(chart.dimension_count(), 0);
        assert_eq!(chart.series_count(), 0);
    }

    #[test]
    fn test_radar_dimensions() {
        let dimensions = vec![
            RadarDimension::new("维度1", 0.0, 100.0),
            RadarDimension::new("维度2", 0.0, 100.0),
            RadarDimension::new("维度3", 0.0, 100.0),
        ];

        let chart = RadarChart::new().dimensions(dimensions);
        assert_eq!(chart.dimension_count(), 3);
    }

    #[test]
    fn test_radar_series() {
        let chart = RadarChart::new()
            .add_data("系列1", vec![80.0, 60.0, 90.0])
            .add_data("系列2", vec![70.0, 85.0, 75.0]);
        
        assert_eq!(chart.series_count(), 2);
    }

    #[test]
    fn test_dimension_normalize() {
        let dim = RadarDimension::new("测试", 0.0, 100.0);
        assert_eq!(dim.normalize(50.0), 0.5);
        assert_eq!(dim.normalize(0.0), 0.0);
        assert_eq!(dim.normalize(100.0), 1.0);
        assert_eq!(dim.normalize(-10.0), 0.0); // 应该被限制在 [0, 1]
        assert_eq!(dim.normalize(110.0), 1.0); // 应该被限制在 [0, 1]
    }

    #[test]
    fn test_simple_dimensions() {
        let names = ["速度", "力量", "技巧"];
        let chart = RadarChart::new().simple_dimensions(&names, 0.0, 10.0);
        
        assert_eq!(chart.dimension_count(), 3);
        assert_eq!(chart.dimensions[0].name, "速度");
        assert_eq!(chart.dimensions[0].min_value, 0.0);
        assert_eq!(chart.dimensions[0].max_value, 10.0);
    }
}
