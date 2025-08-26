use vizuara_core::{Primitive, Color, Scale, LinearScale};
use nalgebra::Point2;

/// 散点图数据点
#[derive(Debug, Clone)]
pub struct DataPoint {
    pub x: f32,
    pub y: f32,
}

impl DataPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for DataPoint {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

/// 散点图配置
#[derive(Debug, Clone)]
pub struct ScatterStyle {
    pub color: Color,
    pub size: f32,
    pub alpha: f32,
}

impl Default for ScatterStyle {
    fn default() -> Self {
        Self {
            color: Color::rgb(0.2, 0.4, 0.8),
            size: 5.0,
            alpha: 1.0,
        }
    }
}

/// 散点图
#[derive(Debug, Clone)]
pub struct ScatterPlot {
    data: Vec<DataPoint>,
    style: ScatterStyle,
    x_scale: Option<LinearScale>,
    y_scale: Option<LinearScale>,
}

impl ScatterPlot {
    /// 创建新的散点图
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            style: ScatterStyle::default(),
            x_scale: None,
            y_scale: None,
        }
    }

    /// 设置数据（接受各种格式）
    pub fn data<T: Into<DataPoint> + Clone>(mut self, data: &[T]) -> Self {
        self.data = data.iter().cloned().map(|d| d.into()).collect();
        self
    }

    /// 从两个向量设置 X 和 Y 数据
    pub fn xy_data(mut self, x_data: &[f32], y_data: &[f32]) -> Self {
        assert_eq!(x_data.len(), y_data.len(), "X and Y data must have the same length");
        
        self.data = x_data
            .iter()
            .zip(y_data.iter())
            .map(|(&x, &y)| DataPoint::new(x, y))
            .collect();
        self
    }

    /// 设置样式
    pub fn style(mut self, style: ScatterStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置颜色
    pub fn color(mut self, color: Color) -> Self {
        self.style.color = color;
        self
    }

    /// 设置点大小
    pub fn size(mut self, size: f32) -> Self {
        self.style.size = size;
        self
    }

    /// 设置 X 轴比例尺
    pub fn x_scale(mut self, scale: LinearScale) -> Self {
        self.x_scale = Some(scale);
        self
    }

    /// 设置 Y 轴比例尺
    pub fn y_scale(mut self, scale: LinearScale) -> Self {
        self.y_scale = Some(scale);
        self
    }

    /// 自动计算比例尺
    pub fn auto_scale(mut self) -> Self {
        if !self.data.is_empty() {
            let x_values: Vec<f32> = self.data.iter().map(|p| p.x).collect();
            let y_values: Vec<f32> = self.data.iter().map(|p| p.y).collect();
            
            self.x_scale = Some(LinearScale::from_data(&x_values));
            self.y_scale = Some(LinearScale::from_data(&y_values));
        }
        self
    }

    /// 生成渲染图元
    pub fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.data.is_empty() {
            return primitives;
        }

        // 获取或创建比例尺
        let x_scale = if let Some(ref scale) = self.x_scale {
            scale.clone()
        } else {
            let x_values: Vec<f32> = self.data.iter().map(|p| p.x).collect();
            LinearScale::from_data(&x_values)
        };
        
        let y_scale = if let Some(ref scale) = self.y_scale {
            scale.clone()
        } else {
            let y_values: Vec<f32> = self.data.iter().map(|p| p.y).collect();
            LinearScale::from_data(&y_values)
        };

        // 转换数据点到屏幕坐标
        let screen_points: Vec<Point2<f32>> = self
            .data
            .iter()
            .map(|point| {
                let x_norm = x_scale.normalize(point.x);
                let y_norm = y_scale.normalize(point.y);
                
                // 将归一化坐标映射到绘图区域
                let screen_x = plot_area.x + x_norm * plot_area.width;
                // Y轴翻转：屏幕坐标系是从上到下，而数据坐标系是从下到上
                let screen_y = plot_area.y + plot_area.height - y_norm * plot_area.height;
                
                Point2::new(screen_x, screen_y)
            })
            .collect();

        // 创建点的图元
        if !screen_points.is_empty() {
            primitives.push(Primitive::Points(screen_points));
        }

        primitives
    }

    /// 获取数据的边界
    pub fn data_bounds(&self) -> Option<(DataPoint, DataPoint)> {
        if self.data.is_empty() {
            return None;
        }

        let mut min_x = self.data[0].x;
        let mut max_x = self.data[0].x;
        let mut min_y = self.data[0].y;
        let mut max_y = self.data[0].y;

        for point in &self.data {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }

        Some((
            DataPoint::new(min_x, min_y),
            DataPoint::new(max_x, max_y),
        ))
    }

    /// 获取数据点数量
    pub fn data_len(&self) -> usize {
        self.data.len()
    }
}

/// 绘图区域定义
#[derive(Debug, Clone, Copy)]
pub struct PlotArea {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl PlotArea {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
}

impl Default for ScatterPlot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scatter_plot_creation() {
        let plot = ScatterPlot::new();
        assert_eq!(plot.data_len(), 0);
    }

    #[test]
    fn test_scatter_plot_with_data() {
        let data = vec![(1.0, 2.0), (2.0, 3.0), (3.0, 1.0)];
        let plot = ScatterPlot::new().data(&data);
        
        assert_eq!(plot.data_len(), 3);
    }

    #[test]
    fn test_scatter_plot_xy_data() {
        let x_data = vec![1.0, 2.0, 3.0];
        let y_data = vec![2.0, 3.0, 1.0];
        let plot = ScatterPlot::new().xy_data(&x_data, &y_data);
        
        assert_eq!(plot.data_len(), 3);
    }

    #[test]
    fn test_data_bounds() {
        let data = vec![(1.0, 2.0), (3.0, 4.0), (0.0, 1.0)];
        let plot = ScatterPlot::new().data(&data);
        
        let bounds = plot.data_bounds().unwrap();
        assert_eq!(bounds.0.x, 0.0);
        assert_eq!(bounds.0.y, 1.0);
        assert_eq!(bounds.1.x, 3.0);
        assert_eq!(bounds.1.y, 4.0);
    }

    #[test]
    fn test_primitive_generation() {
        let data = vec![(1.0, 2.0), (2.0, 3.0)];
        let plot = ScatterPlot::new().data(&data).auto_scale();
        
        let plot_area = PlotArea::new(100.0, 100.0, 400.0, 300.0);
        let primitives = plot.generate_primitives(plot_area);
        
        assert_eq!(primitives.len(), 1); // 应该有一个 Points 图元
    }
}
