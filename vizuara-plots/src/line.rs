use vizuara_core::{Primitive, Color, Scale, LinearScale};
use nalgebra::Point2;

/// 折线图数据点（重用 scatter 的 DataPoint）
pub use crate::scatter::DataPoint;

/// 折线图样式
#[derive(Debug, Clone)]
pub struct LinePlotStyle {
    pub color: Color,
    pub width: f32,
    pub style: vizuara_core::LineStyle,
    pub alpha: f32,
}

impl Default for LinePlotStyle {
    fn default() -> Self {
        Self {
            color: Color::rgb(0.2, 0.4, 0.8),
            width: 2.0,
            style: vizuara_core::LineStyle::Solid,
            alpha: 1.0,
        }
    }
}

/// 折线图
#[derive(Debug, Clone)]
pub struct LinePlot {
    data: Vec<DataPoint>,
    style: LinePlotStyle,
    x_scale: Option<LinearScale>,
    y_scale: Option<LinearScale>,
    smooth: bool,
}

impl LinePlot {
    /// 创建新的折线图
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            style: LinePlotStyle::default(),
            x_scale: None,
            y_scale: None,
            smooth: false,
        }
    }

    /// 设置数据（接受各种格式）
    pub fn data<T: Into<DataPoint> + Clone>(mut self, data: &[T]) -> Self {
        self.data = data.iter().cloned().map(|d| d.into()).collect();
        // 按 X 坐标排序，确保线条连接正确
        self.data.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));
        self
    }

    /// 从两个向量设置 X 和 Y 数据
    pub fn xy_data(mut self, x_data: &[f32], y_data: &[f32]) -> Self {
        assert_eq!(x_data.len(), y_data.len(), "X and Y data must have the same length");
        
        let mut combined: Vec<_> = x_data
            .iter()
            .zip(y_data.iter())
            .map(|(&x, &y)| DataPoint::new(x, y))
            .collect();
        
        // 按 X 坐标排序
        combined.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));
        
        self.data = combined;
        self
    }

    /// 设置线条样式
    pub fn style(mut self, style: LinePlotStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置颜色
    pub fn color(mut self, color: Color) -> Self {
        self.style.color = color;
        self
    }

    /// 设置线宽
    pub fn line_width(mut self, width: f32) -> Self {
        self.style.width = width;
        self
    }

    /// 设置线条样式
    pub fn line_style(mut self, style: vizuara_core::LineStyle) -> Self {
        self.style.style = style;
        self
    }

    /// 设置是否平滑
    pub fn smooth(mut self, smooth: bool) -> Self {
        self.smooth = smooth;
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
    pub fn generate_primitives(&self, plot_area: crate::PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.data.len() < 2 {
            return primitives; // 需要至少2个点才能画线
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

        // 创建线条图元
        if screen_points.len() >= 2 {
            primitives.push(Primitive::LineStrip(screen_points));
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

impl Default for LinePlot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_plot_creation() {
        let plot = LinePlot::new();
        assert_eq!(plot.data_len(), 0);
    }

    #[test]
    fn test_line_plot_with_data() {
        let data = vec![(1.0, 2.0), (3.0, 1.0), (2.0, 3.0)]; // 故意乱序
        let plot = LinePlot::new().data(&data);
        
        assert_eq!(plot.data_len(), 3);
        
        // 验证数据已按 X 坐标排序
        assert_eq!(plot.data[0].x, 1.0);
        assert_eq!(plot.data[1].x, 2.0);
        assert_eq!(plot.data[2].x, 3.0);
    }

    #[test]
    fn test_line_plot_xy_data() {
        let x_data = vec![3.0, 1.0, 2.0]; // 故意乱序
        let y_data = vec![6.0, 2.0, 4.0];
        let plot = LinePlot::new().xy_data(&x_data, &y_data);
        
        assert_eq!(plot.data_len(), 3);
        
        // 验证数据已按 X 坐标排序
        assert_eq!(plot.data[0].x, 1.0);
        assert_eq!(plot.data[0].y, 2.0);
        assert_eq!(plot.data[1].x, 2.0);
        assert_eq!(plot.data[1].y, 4.0);
        assert_eq!(plot.data[2].x, 3.0);
        assert_eq!(plot.data[2].y, 6.0);
    }

    #[test]
    fn test_line_plot_bounds() {
        let data = vec![(1.0, 2.0), (3.0, 4.0), (0.0, 1.0)];
        let plot = LinePlot::new().data(&data);
        
        let bounds = plot.data_bounds().unwrap();
        assert_eq!(bounds.0.x, 0.0);
        assert_eq!(bounds.0.y, 1.0);
        assert_eq!(bounds.1.x, 3.0);
        assert_eq!(bounds.1.y, 4.0);
    }

    #[test]
    fn test_line_plot_primitive_generation() {
        let data = vec![(1.0, 2.0), (2.0, 3.0), (3.0, 1.0)];
        let plot = LinePlot::new().data(&data).auto_scale();
        
        let plot_area = crate::PlotArea::new(100.0, 100.0, 400.0, 300.0);
        let primitives = plot.generate_primitives(plot_area);
        
        assert_eq!(primitives.len(), 1); // 应该有一个 LineStrip 图元
        
        match &primitives[0] {
            Primitive::LineStrip(points) => {
                assert_eq!(points.len(), 3); // 3个点的线条
            }
            _ => panic!("Expected LineStrip primitive"),
        }
    }

    #[test]
    fn test_line_plot_insufficient_data() {
        // 测试数据点不足的情况
        let plot = LinePlot::new().data(&[(1.0, 2.0)]);
        
        let plot_area = crate::PlotArea::new(100.0, 100.0, 400.0, 300.0);
        let primitives = plot.generate_primitives(plot_area);
        
        assert_eq!(primitives.len(), 0); // 少于2个点，不应该生成线条
    }

    #[test]
    fn test_line_plot_styling() {
        let plot = LinePlot::new()
            .color(Color::rgb(1.0, 0.0, 0.0))
            .line_width(3.0)
            .line_style(vizuara_core::LineStyle::Dashed);
        
        assert_eq!(plot.style.color, Color::rgb(1.0, 0.0, 0.0));
        assert_eq!(plot.style.width, 3.0);
        assert_eq!(plot.style.style, vizuara_core::LineStyle::Dashed);
    }
}
