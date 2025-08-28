//! 平行坐标图实现
//!
//! 用于可视化多维数据，每个维度对应一个垂直轴

use crate::PlotArea;
use vizuara_core::{Color, Primitive, LinearScale, Scale, HorizontalAlign, VerticalAlign};
use nalgebra::Point2;

/// 平行坐标轴
#[derive(Debug, Clone)]
pub struct ParallelAxis {
    pub name: String,
    pub min_value: f32,
    pub max_value: f32,
    pub tick_count: usize,
    pub show_ticks: bool,
    pub show_labels: bool,
}

impl ParallelAxis {
    /// 创建新的坐标轴
    pub fn new<S: Into<String>>(name: S, min_value: f32, max_value: f32) -> Self {
        Self {
            name: name.into(),
            min_value,
            max_value,
            tick_count: 5,
            show_ticks: true,
            show_labels: true,
        }
    }

    /// 设置刻度数量
    pub fn tick_count(mut self, count: usize) -> Self {
        self.tick_count = count;
        self
    }

    /// 设置是否显示刻度
    pub fn show_ticks(mut self, show: bool) -> Self {
        self.show_ticks = show;
        self
    }

    /// 设置是否显示标签
    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }
}

/// 数据系列
#[derive(Debug, Clone)]
pub struct ParallelSeries {
    pub name: String,
    pub values: Vec<f32>,
    pub color: Color,
    pub line_width: f32,
    pub alpha: f32,
    pub highlighted: bool,
}

impl ParallelSeries {
    /// 创建新的数据系列
    pub fn new<S: Into<String>>(name: S, values: Vec<f32>) -> Self {
        Self {
            name: name.into(),
            values,
            color: Color::rgb(0.2, 0.6, 0.9),
            line_width: 1.5,
            alpha: 0.7,
            highlighted: false,
        }
    }

    /// 设置颜色
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// 设置线宽
    pub fn line_width(mut self, width: f32) -> Self {
        self.line_width = width;
        self
    }

    /// 设置透明度
    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }

    /// 设置是否高亮
    pub fn highlighted(mut self, highlighted: bool) -> Self {
        self.highlighted = highlighted;
        self
    }
}

/// 平行坐标图样式
#[derive(Debug, Clone)]
pub struct ParallelStyle {
    pub axis_color: Color,
    pub axis_width: f32,
    pub grid_color: Color,
    pub grid_width: f32,
    pub label_size: f32,
    pub label_color: Color,
    pub tick_size: f32,
    pub axis_spacing: f32,
    pub show_grid: bool,
}

impl Default for ParallelStyle {
    fn default() -> Self {
        Self {
            axis_color: Color::rgb(0.3, 0.3, 0.3),
            axis_width: 2.0,
            grid_color: Color::rgb(0.8, 0.8, 0.8),
            grid_width: 0.5,
            label_size: 12.0,
            label_color: Color::rgb(0.1, 0.1, 0.1),
            tick_size: 5.0,
            axis_spacing: 100.0,
            show_grid: true,
        }
    }
}

/// 平行坐标图
#[derive(Debug, Clone)]
pub struct ParallelCoordinates {
    axes: Vec<ParallelAxis>,
    series: Vec<ParallelSeries>,
    style: ParallelStyle,
    title: Option<String>,
    brushing_enabled: bool,
    selected_ranges: Vec<Option<(f32, f32)>>, // 每个轴的选择范围
}

impl ParallelCoordinates {
    /// 创建新的平行坐标图
    pub fn new() -> Self {
        Self {
            axes: Vec::new(),
            series: Vec::new(),
            style: ParallelStyle::default(),
            title: None,
            brushing_enabled: false,
            selected_ranges: Vec::new(),
        }
    }

    /// 添加坐标轴
    pub fn add_axis(mut self, axis: ParallelAxis) -> Self {
        self.axes.push(axis);
        self.selected_ranges.push(None);
        self
    }

    /// 批量添加坐标轴
    pub fn axes(mut self, axes: Vec<ParallelAxis>) -> Self {
        self.selected_ranges.resize(axes.len(), None);
        self.axes = axes;
        self
    }

    /// 从数据自动创建坐标轴
    pub fn auto_axes(mut self, names: &[&str], data: &[Vec<f32>]) -> Self {
        if names.len() != data.len() {
            return self;
        }

        self.axes.clear();
        for (i, &name) in names.iter().enumerate() {
            if !data[i].is_empty() {
                let min_val = data[i].iter().fold(f32::INFINITY, |a, &b| a.min(b));
                let max_val = data[i].iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
                let range = max_val - min_val;
                let axis = ParallelAxis::new(
                    name,
                    min_val - range * 0.05,
                    max_val + range * 0.05,
                );
                self.axes.push(axis);
            }
        }

        self.selected_ranges.resize(self.axes.len(), None);
        self
    }

    /// 添加数据系列
    pub fn add_series(mut self, series: ParallelSeries) -> Self {
        self.series.push(series);
        self
    }

    /// 批量添加数据系列
    pub fn series(mut self, series: Vec<ParallelSeries>) -> Self {
        self.series = series;
        self
    }

    /// 从矩阵数据创建系列
    pub fn from_matrix(mut self, data: &[Vec<f32>], names: Option<&[&str]>) -> Self {
        self.series.clear();
        
        for (i, row) in data.iter().enumerate() {
            let name = names
                .and_then(|n| n.get(i))
                .map(|&s| s.to_string())
                .unwrap_or_else(|| format!("Series {}", i + 1));
            
            let hue = (i as f32 / data.len() as f32) * 360.0;
            let color = Color::rgb(
                (hue * std::f32::consts::PI / 180.0).sin() * 0.5 + 0.5,
                ((hue + 120.0) * std::f32::consts::PI / 180.0).sin() * 0.5 + 0.5,
                ((hue + 240.0) * std::f32::consts::PI / 180.0).sin() * 0.5 + 0.5,
            );

            let series = ParallelSeries::new(name, row.clone()).color(color);
            self.series.push(series);
        }

        self
    }

    /// 设置标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置轴间距
    pub fn axis_spacing(mut self, spacing: f32) -> Self {
        self.style.axis_spacing = spacing;
        self
    }

    /// 设置是否显示网格
    pub fn show_grid(mut self, show: bool) -> Self {
        self.style.show_grid = show;
        self
    }

    /// 启用刷选功能
    pub fn enable_brushing(mut self, enabled: bool) -> Self {
        self.brushing_enabled = enabled;
        self
    }

    /// 设置轴的选择范围
    pub fn set_axis_range(mut self, axis_index: usize, range: Option<(f32, f32)>) -> Self {
        if axis_index < self.selected_ranges.len() {
            self.selected_ranges[axis_index] = range;
        }
        self
    }

    /// 检查系列是否在选择范围内
    fn is_series_selected(&self, series: &ParallelSeries) -> bool {
        if !self.brushing_enabled {
            return true;
        }

        for (i, range) in self.selected_ranges.iter().enumerate() {
            if let Some((min, max)) = range {
                if let Some(&value) = series.values.get(i) {
                    if value < *min || value > *max {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// 生成渲染图元
    pub fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.axes.is_empty() {
            return primitives;
        }

        let axis_count = self.axes.len();
        let total_width = plot_area.width - 40.0; // 左右边距
        let axis_spacing = if axis_count > 1 {
            total_width / (axis_count - 1) as f32
        } else {
            0.0
        };

        let axis_height = plot_area.height - 80.0; // 上下边距
        let axis_start_y = plot_area.y + 40.0;

        // 计算轴位置
        let mut axis_positions = Vec::new();
        for i in 0..axis_count {
            let x = plot_area.x + 20.0 + i as f32 * axis_spacing;
            axis_positions.push(x);
        }

        // 绘制坐标轴
        for (i, (axis, &x)) in self.axes.iter().zip(axis_positions.iter()).enumerate() {
            // 绘制轴线
            primitives.push(Primitive::Polyline {
                points: vec![
                    Point2::new(x, axis_start_y),
                    Point2::new(x, axis_start_y + axis_height)
                ],
                color: self.style.axis_color,
                width: self.style.axis_width,
            });

            // 绘制轴标签
            primitives.push(Primitive::Text {
                position: Point2::new(x, axis_start_y - 10.0),
                content: axis.name.clone(),
                size: self.style.label_size,
                color: self.style.label_color,
                h_align: HorizontalAlign::Center,
                v_align: VerticalAlign::Bottom,
            });

            // 绘制刻度和刻度标签
            if axis.show_ticks && axis.tick_count > 0 {
                for j in 0..=axis.tick_count {
                    let t = j as f32 / axis.tick_count as f32;
                    let y = axis_start_y + axis_height - t * axis_height;
                    let value = axis.min_value + t * (axis.max_value - axis.min_value);

                    // 刻度线
                    primitives.push(Primitive::Polyline {
                        points: vec![
                            Point2::new(x - self.style.tick_size, y),
                            Point2::new(x + self.style.tick_size, y)
                        ],
                        color: self.style.axis_color,
                        width: 1.0,
                    });

                    // 刻度标签
                    if axis.show_labels {
                        primitives.push(Primitive::Text {
                            position: Point2::new(x + 15.0, y),
                            content: format!("{:.1}", value),
                            size: self.style.label_size * 0.8,
                            color: self.style.label_color,
                            h_align: HorizontalAlign::Left,
                            v_align: VerticalAlign::Middle,
                        });
                    }
                }
            }

            // 绘制网格线
            if self.style.show_grid && i > 0 {
                let prev_x = axis_positions[i - 1];
                for j in 0..=axis.tick_count {
                    let t = j as f32 / axis.tick_count as f32;
                    let y = axis_start_y + axis_height - t * axis_height;

                    primitives.push(Primitive::Polyline {
                        points: vec![
                            Point2::new(prev_x, y),
                            Point2::new(x, y)
                        ],
                        color: self.style.grid_color,
                        width: self.style.grid_width,
                    });
                }
            }
        }

        // 绘制数据线
        for series in &self.series {
            if series.values.len() != axis_count {
                continue;
            }

            let is_selected = self.is_series_selected(series);
            let alpha = if is_selected {
                series.alpha
            } else {
                series.alpha * 0.3 // 未选中的线条变淡
            };

            let line_color = Color::rgba(series.color.r, series.color.g, series.color.b, alpha);
            let line_width = if series.highlighted { series.line_width * 2.0 } else { series.line_width };

            // 创建折线的点
            let mut line_points = Vec::new();
            for (j, (&value, &x)) in series.values.iter().zip(axis_positions.iter()).enumerate() {
                if let Some(axis) = self.axes.get(j) {
                    let scale = LinearScale::new(axis.min_value, axis.max_value);
                    let normalized = scale.normalize(value);
                    let y = axis_start_y + axis_height - normalized * axis_height;
                    line_points.push(Point2::new(x, y));
                }
            }

            // 绘制折线
            if line_points.len() > 1 {
                primitives.push(Primitive::Polyline {
                    points: line_points.clone(),
                    color: line_color,
                    width: line_width,
                });
            }

            // 绘制数据点（如果高亮）
            if series.highlighted {
                for point in &line_points {
                    primitives.push(Primitive::Circle {
                        center: *point,
                        radius: 3.0,
                    });
                }
            }
        }

        // 绘制选择范围
        if self.brushing_enabled {
            for (i, range) in self.selected_ranges.iter().enumerate() {
                if let Some((min, max)) = range {
                    if let (Some(axis), Some(&x)) = (self.axes.get(i), axis_positions.get(i)) {
                        let scale = LinearScale::new(axis.min_value, axis.max_value);
                        let normalized_max = scale.normalize(*max);
                        let normalized_min = scale.normalize(*min);
                        let y_min = axis_start_y + axis_height - normalized_max * axis_height;
                        let y_max = axis_start_y + axis_height - normalized_min * axis_height;

                        // 绘制选择区域
                        primitives.push(Primitive::Rectangle {
                            min: Point2::new(x - 5.0, y_min),
                            max: Point2::new(x + 5.0, y_max),
                        });
                    }
                }
            }
        }

        // 绘制标题
        if let Some(ref title) = self.title {
            primitives.push(Primitive::Text {
                position: Point2::new(
                    plot_area.x + plot_area.width / 2.0,
                    plot_area.y + 15.0,
                ),
                content: title.clone(),
                size: 16.0,
                color: Color::rgb(0.1, 0.1, 0.1),
                h_align: HorizontalAlign::Center,
                v_align: VerticalAlign::Top,
            });
        }

        primitives
    }

    /// 获取轴数量
    pub fn axis_count(&self) -> usize {
        self.axes.len()
    }

    /// 获取系列数量
    pub fn series_count(&self) -> usize {
        self.series.len()
    }

    /// 获取数据维度统计
    pub fn get_statistics(&self) -> Vec<(String, f32, f32, f32)> {
        let mut stats = Vec::new();
        
        for (i, axis) in self.axes.iter().enumerate() {
            let values: Vec<f32> = self.series.iter()
                .filter_map(|s| s.values.get(i).copied())
                .collect();
            
            if !values.is_empty() {
                let min = values.iter().fold(f32::INFINITY, |a, &b| a.min(b));
                let max = values.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
                let mean = values.iter().sum::<f32>() / values.len() as f32;
                
                stats.push((axis.name.clone(), min, max, mean));
            }
        }
        
        stats
    }
}

impl Default for ParallelCoordinates {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_coordinates_creation() {
        let pc = ParallelCoordinates::new();
        assert_eq!(pc.axis_count(), 0);
        assert_eq!(pc.series_count(), 0);
    }

    #[test]
    fn test_auto_axes() {
        let names = ["X", "Y", "Z"];
        let data = [
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        
        let pc = ParallelCoordinates::new().auto_axes(&names, &data);
        assert_eq!(pc.axis_count(), 3);
    }

    #[test]
    fn test_from_matrix() {
        let data = [
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ];
        let names = ["Series1", "Series2"];
        
        let pc = ParallelCoordinates::new().from_matrix(&data, Some(&names));
        assert_eq!(pc.series_count(), 2);
    }

    #[test]
    fn test_brushing() {
        let mut pc = ParallelCoordinates::new()
            .enable_brushing(true);
        
        pc = pc.set_axis_range(0, Some((1.0, 5.0)));
        assert_eq!(pc.selected_ranges[0], Some((1.0, 5.0)));
    }

    #[test]
    fn test_parallel_coordinates_primitives() {
        let axes = vec![
            ParallelAxis::new("X", 0.0, 10.0),
            ParallelAxis::new("Y", 0.0, 20.0),
            ParallelAxis::new("Z", 0.0, 30.0),
        ];
        
        let series = vec![
            ParallelSeries::new("Data1", vec![5.0, 10.0, 15.0]),
            ParallelSeries::new("Data2", vec![3.0, 15.0, 25.0]),
        ];
        
        let pc = ParallelCoordinates::new()
            .axes(axes)
            .series(series)
            .title("测试平行坐标图");
        
        let plot_area = PlotArea::new(0.0, 0.0, 600.0, 400.0);
        let primitives = pc.generate_primitives(plot_area);
        assert!(!primitives.is_empty());
    }

    #[test]
    fn test_statistics() {
        let axes = vec![
            ParallelAxis::new("X", 0.0, 10.0),
            ParallelAxis::new("Y", 0.0, 20.0),
        ];
        
        let series = vec![
            ParallelSeries::new("Data1", vec![1.0, 10.0]),
            ParallelSeries::new("Data2", vec![9.0, 20.0]),
        ];
        
        let pc = ParallelCoordinates::new()
            .axes(axes)
            .series(series);
        
        let stats = pc.get_statistics();
        assert_eq!(stats.len(), 2);
        assert_eq!(stats[0].0, "X");
        assert_eq!(stats[0].1, 1.0); // min
        assert_eq!(stats[0].2, 9.0); // max
        assert_eq!(stats[0].3, 5.0); // mean
    }
}
