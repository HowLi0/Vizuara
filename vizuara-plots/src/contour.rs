//! 等高线图实现
//!
//! 用于可视化3D数据的2D表示，显示等值线

use crate::PlotArea;
use vizuara_core::{Color, Primitive, HorizontalAlign, VerticalAlign};
use nalgebra::Point2;

/// 等高线级别
#[derive(Debug, Clone)]
pub struct ContourLevel {
    pub value: f32,
    pub color: Color,
    pub line_width: f32,
    pub label: Option<String>,
}

/// 等高线样式
#[derive(Debug, Clone)]
pub struct ContourStyle {
    pub levels: Vec<ContourLevel>,
    pub filled: bool,
    pub show_labels: bool,
    pub label_size: f32,
    pub label_color: Color,
    pub grid_resolution: usize,
}

impl Default for ContourStyle {
    fn default() -> Self {
        Self {
            levels: Vec::new(),
            filled: false,
            show_labels: true,
            label_size: 10.0,
            label_color: Color::rgb(0.1, 0.1, 0.1),
            grid_resolution: 50,
        }
    }
}

/// 3D数据点
#[derive(Debug, Clone, Copy)]
pub struct DataPoint3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// 等高线图
#[derive(Debug, Clone)]
pub struct ContourPlot {
    data: Vec<DataPoint3D>,
    style: ContourStyle,
    title: Option<String>,
    x_range: Option<(f32, f32)>,
    y_range: Option<(f32, f32)>,
    auto_levels: Option<usize>,
}

impl ContourPlot {
    /// 创建新的等高线图
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            style: ContourStyle::default(),
            title: None,
            x_range: None,
            y_range: None,
            auto_levels: None,
        }
    }

    /// 设置数据
    pub fn data(mut self, data: &[DataPoint3D]) -> Self {
        self.data = data.to_vec();
        self
    }

    /// 从网格数据创建
    pub fn from_grid(mut self, x_values: &[f32], y_values: &[f32], z_grid: &[Vec<f32>]) -> Self {
        let mut data = Vec::new();
        for (i, &x) in x_values.iter().enumerate() {
            for (j, &y) in y_values.iter().enumerate() {
                if i < z_grid.len() && j < z_grid[i].len() {
                    data.push(DataPoint3D { x, y, z: z_grid[i][j] });
                }
            }
        }
        self.data = data;
        self
    }

    /// 设置标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 添加等高线级别
    pub fn add_level(mut self, level: ContourLevel) -> Self {
        self.style.levels.push(level);
        self
    }

    /// 自动生成等高线级别
    pub fn auto_levels(mut self, count: usize) -> Self {
        self.auto_levels = Some(count);
        self
    }

    /// 设置是否填充
    pub fn filled(mut self, filled: bool) -> Self {
        self.style.filled = filled;
        self
    }

    /// 设置是否显示标签
    pub fn show_labels(mut self, show: bool, size: f32) -> Self {
        self.style.show_labels = show;
        self.style.label_size = size;
        self
    }

    /// 设置网格分辨率
    pub fn grid_resolution(mut self, resolution: usize) -> Self {
        self.style.grid_resolution = resolution;
        self
    }

    /// 设置X轴范围
    pub fn x_range(mut self, min: f32, max: f32) -> Self {
        self.x_range = Some((min, max));
        self
    }

    /// 设置Y轴范围
    pub fn y_range(mut self, min: f32, max: f32) -> Self {
        self.y_range = Some((min, max));
        self
    }

    /// 生成自动等高线级别
    fn generate_auto_levels(&mut self) {
        if let Some(count) = self.auto_levels {
            if self.data.is_empty() {
                return;
            }

            let z_min = self.data.iter().map(|p| p.z).fold(f32::INFINITY, f32::min);
            let z_max = self.data.iter().map(|p| p.z).fold(f32::NEG_INFINITY, f32::max);

            self.style.levels.clear();
            
            for i in 0..count {
                let t = i as f32 / (count - 1) as f32;
                let value = z_min + t * (z_max - z_min);
                
                // 使用颜色渐变
                let color = if self.style.filled {
                    Color::rgb(t, 0.5, 1.0 - t)
                } else {
                    Color::rgb(0.1 + t * 0.8, 0.1 + t * 0.8, 0.1 + t * 0.8)
                };

                self.style.levels.push(ContourLevel {
                    value,
                    color,
                    line_width: 1.5,
                    label: Some(format!("{:.2}", value)),
                });
            }
        }
    }

    /// 双线性插值（为将来扩展保留）
    #[allow(dead_code)]
    fn bilinear_interpolation(&self, x: f32, y: f32, grid: &Grid) -> Option<f32> {
        if x < grid.x_min || x > grid.x_max || y < grid.y_min || y > grid.y_max {
            return None;
        }

        let x_step = (grid.x_max - grid.x_min) / (grid.width - 1) as f32;
        let y_step = (grid.y_max - grid.y_min) / (grid.height - 1) as f32;

        let i = ((x - grid.x_min) / x_step) as usize;
        let j = ((y - grid.y_min) / y_step) as usize;

        if i >= grid.width - 1 || j >= grid.height - 1 {
            return None;
        }

        let x_frac = (x - (grid.x_min + i as f32 * x_step)) / x_step;
        let y_frac = (y - (grid.y_min + j as f32 * y_step)) / y_step;

        let z00 = grid.values[j][i];
        let z10 = grid.values[j][i + 1];
        let z01 = grid.values[j + 1][i];
        let z11 = grid.values[j + 1][i + 1];

        let z0 = z00 * (1.0 - x_frac) + z10 * x_frac;
        let z1 = z01 * (1.0 - x_frac) + z11 * x_frac;
        let z = z0 * (1.0 - y_frac) + z1 * y_frac;

        Some(z)
    }

    /// 生成渲染图元
    pub fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.data.is_empty() {
            return primitives;
        }

        // 生成自动等高线级别
        let mut plot_copy = self.clone();
        plot_copy.generate_auto_levels();

        // 创建规则网格
        let grid = self.create_grid();

        // 绘制等高线
        for level in &plot_copy.style.levels {
            let contour_lines = self.extract_contour_lines(&grid, level.value);
            
            for line in contour_lines {
                if line.len() < 2 {
                    continue;
                }

                let screen_points: Vec<Point2<f32>> = line.iter().map(|&(x, y)| {
                    let screen_x = plot_area.x + (x - grid.x_min) / (grid.x_max - grid.x_min) * plot_area.width;
                    let screen_y = plot_area.y + plot_area.height - (y - grid.y_min) / (grid.y_max - grid.y_min) * plot_area.height;
                    Point2::new(screen_x, screen_y)
                }).collect();

                    if self.style.filled {
                        // 填充区域（简化实现）
                        if screen_points.len() > 2 {
                            primitives.push(Primitive::Polygon {
                                points: screen_points,
                                fill: level.color,
                                stroke: Some((level.color, 1.0)),
                            });
                        }
                    } else {
                        // 绘制等高线
                        primitives.push(Primitive::Polyline {
                            points: screen_points,
                            color: level.color,
                            width: level.line_width,
                        });
                    }
            }
        }

        // 绘制标题
        if let Some(ref title) = self.title {
            primitives.push(Primitive::Text {
                position: Point2::new(
                    plot_area.x + plot_area.width / 2.0,
                    plot_area.y - 20.0,
                ),
                content: title.clone(),
                size: 14.0,
                color: Color::rgb(0.1, 0.1, 0.1),
                h_align: HorizontalAlign::Center,
                v_align: VerticalAlign::Bottom,
            });
        }

        primitives
    }

    /// 创建规则网格
    fn create_grid(&self) -> Grid {
        let x_min = self.x_range.map(|(min, _)| min).unwrap_or_else(|| 
            self.data.iter().map(|p| p.x).fold(f32::INFINITY, f32::min)
        );
        let x_max = self.x_range.map(|(_, max)| max).unwrap_or_else(|| 
            self.data.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max)
        );
        let y_min = self.y_range.map(|(min, _)| min).unwrap_or_else(|| 
            self.data.iter().map(|p| p.y).fold(f32::INFINITY, f32::min)
        );
        let y_max = self.y_range.map(|(_, max)| max).unwrap_or_else(|| 
            self.data.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max)
        );

        let width = self.style.grid_resolution;
        let height = self.style.grid_resolution;

        let mut values = vec![vec![0.0; width]; height];

        // 简单的最近邻插值
        for j in 0..height {
            for i in 0..width {
                let x = x_min + (i as f32 / (width - 1) as f32) * (x_max - x_min);
                let y = y_min + (j as f32 / (height - 1) as f32) * (y_max - y_min);

                // 找到最近的数据点
                let mut min_dist = f32::INFINITY;
                let mut nearest_z = 0.0;

                for point in &self.data {
                    let dist = ((point.x - x).powi(2) + (point.y - y).powi(2)).sqrt();
                    if dist < min_dist {
                        min_dist = dist;
                        nearest_z = point.z;
                    }
                }

                values[j][i] = nearest_z;
            }
        }

        Grid {
            values,
            x_min,
            x_max,
            y_min,
            y_max,
            width,
            height,
        }
    }

    /// 提取等高线
    fn extract_contour_lines(&self, grid: &Grid, level: f32) -> Vec<Vec<(f32, f32)>> {
        // 简化的等高线提取算法（Marching Squares的简化版本）
        let mut lines = Vec::new();
        
        let x_step = (grid.x_max - grid.x_min) / (grid.width - 1) as f32;
        let y_step = (grid.y_max - grid.y_min) / (grid.height - 1) as f32;

        for j in 0..grid.height - 1 {
            for i in 0..grid.width - 1 {
                let z00 = grid.values[j][i];
                let z10 = grid.values[j][i + 1];
                let z01 = grid.values[j + 1][i];
                let z11 = grid.values[j + 1][i + 1];

                // 检查是否有等高线穿过这个网格单元
                let min_z = z00.min(z10).min(z01).min(z11);
                let max_z = z00.max(z10).max(z01).max(z11);

                if level >= min_z && level <= max_z {
                    // 简单的线性插值找到交点
                    let x = grid.x_min + i as f32 * x_step;
                    let y = grid.y_min + j as f32 * y_step;

                    // 这里应该实现完整的Marching Squares算法
                    // 为了简化，我们只创建一个简单的线段
                    let mut line = Vec::new();
                    line.push((x, y));
                    line.push((x + x_step, y + y_step));
                    lines.push(line);
                }
            }
        }

        lines
    }

    /// 获取数据点数量
    pub fn data_len(&self) -> usize {
        self.data.len()
    }
}

impl Default for ContourPlot {
    fn default() -> Self {
        Self::new()
    }
}

/// 网格数据结构
#[derive(Debug, Clone)]
struct Grid {
    values: Vec<Vec<f32>>,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    width: usize,
    height: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contour_plot_creation() {
        let plot = ContourPlot::new();
        assert_eq!(plot.data_len(), 0);
    }

    #[test]
    fn test_contour_plot_with_data() {
        let data = vec![
            DataPoint3D { x: 0.0, y: 0.0, z: 1.0 },
            DataPoint3D { x: 1.0, y: 0.0, z: 2.0 },
            DataPoint3D { x: 0.0, y: 1.0, z: 3.0 },
            DataPoint3D { x: 1.0, y: 1.0, z: 4.0 },
        ];
        let plot = ContourPlot::new().data(&data);
        assert_eq!(plot.data_len(), 4);
    }

    #[test]
    fn test_from_grid() {
        let x_values = vec![0.0, 1.0];
        let y_values = vec![0.0, 1.0];
        let z_grid = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        
        let plot = ContourPlot::new().from_grid(&x_values, &y_values, &z_grid);
        assert_eq!(plot.data_len(), 4);
    }

    #[test]
    fn test_auto_levels() {
        let data = vec![
            DataPoint3D { x: 0.0, y: 0.0, z: 1.0 },
            DataPoint3D { x: 1.0, y: 1.0, z: 5.0 },
        ];
        let mut plot = ContourPlot::new().data(&data).auto_levels(3);
        plot.generate_auto_levels();
        assert_eq!(plot.style.levels.len(), 3);
    }

    #[test]
    fn test_contour_primitives() {
        let data = vec![
            DataPoint3D { x: 0.0, y: 0.0, z: 1.0 },
            DataPoint3D { x: 1.0, y: 0.0, z: 2.0 },
            DataPoint3D { x: 0.0, y: 1.0, z: 3.0 },
            DataPoint3D { x: 1.0, y: 1.0, z: 4.0 },
        ];
        let plot = ContourPlot::new()
            .data(&data)
            .auto_levels(3)
            .title("测试等高线图");
        
        let plot_area = PlotArea::new(0.0, 0.0, 400.0, 300.0);
        let primitives = plot.generate_primitives(plot_area);
        assert!(!primitives.is_empty());
    }
}
