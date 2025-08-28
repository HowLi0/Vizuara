use nalgebra::Point2;
use vizuara_core::{Color, LinearScale, Primitive, Scale};

/// 面积图数据点
#[derive(Debug, Clone)]
pub struct AreaDataPoint {
    pub x: f32,
    pub y: f32,
}

impl AreaDataPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for AreaDataPoint {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

/// 面积图数据系列
#[derive(Debug, Clone)]
pub struct AreaSeries {
    pub label: String,
    pub data: Vec<AreaDataPoint>,
    pub fill_color: Color,
    pub line_color: Color,
    pub line_width: f32,
    pub alpha: f32,
}

impl AreaSeries {
    pub fn new<S: Into<String>>(label: S) -> Self {
        Self {
            label: label.into(),
            data: Vec::new(),
            fill_color: Color::rgb(0.2, 0.6, 0.9),
            line_color: Color::rgb(0.1, 0.4, 0.7),
            line_width: 2.0,
            alpha: 0.6,
        }
    }

    pub fn data<T: Into<AreaDataPoint> + Clone>(mut self, data: &[T]) -> Self {
        self.data = data.iter().cloned().map(|d| d.into()).collect();
        self
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

    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }
}

/// 面积图填充模式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AreaFillMode {
    /// 填充到零基线
    ToZero,
    /// 填充到下一系列（堆叠面积图）
    Stacked,
    /// 填充到指定基线值
    ToBaseline(f32),
}

/// 面积图样式配置
#[derive(Debug, Clone)]
pub struct AreaStyle {
    /// 填充模式
    pub fill_mode: AreaFillMode,
    /// 是否显示数据点
    pub show_points: bool,
    /// 数据点大小
    pub point_size: f32,
    /// 数据点颜色
    pub point_color: Color,
    /// 是否平滑曲线
    pub smooth: bool,
    /// 平滑强度
    pub smooth_factor: f32,
}

impl Default for AreaStyle {
    fn default() -> Self {
        Self {
            fill_mode: AreaFillMode::ToZero,
            show_points: false,
            point_size: 4.0,
            point_color: Color::rgb(0.2, 0.2, 0.2),
            smooth: false,
            smooth_factor: 0.5,
        }
    }
}

/// 面积图
#[derive(Debug, Clone)]
pub struct AreaChart {
    series: Vec<AreaSeries>,
    style: AreaStyle,
    x_scale: Option<LinearScale>,
    y_scale: Option<LinearScale>,
    title: Option<String>,
    default_colors: Vec<(Color, Color)>, // (fill_color, line_color)
}

impl AreaChart {
    /// 创建新的面积图
    pub fn new() -> Self {
        let default_colors = vec![
            (Color::rgba(0.2, 0.6, 0.9, 0.6), Color::rgb(0.1, 0.4, 0.7)),   // 蓝色
            (Color::rgba(0.9, 0.5, 0.2, 0.6), Color::rgb(0.7, 0.3, 0.1)),   // 橙色
            (Color::rgba(0.4, 0.8, 0.4, 0.6), Color::rgb(0.2, 0.6, 0.2)),   // 绿色
            (Color::rgba(0.9, 0.3, 0.3, 0.6), Color::rgb(0.7, 0.1, 0.1)),   // 红色
            (Color::rgba(0.7, 0.4, 0.9, 0.6), Color::rgb(0.5, 0.2, 0.7)),   // 紫色
            (Color::rgba(0.9, 0.9, 0.3, 0.6), Color::rgb(0.7, 0.7, 0.1)),   // 黄色
        ];

        Self {
            series: Vec::new(),
            style: AreaStyle::default(),
            x_scale: None,
            y_scale: None,
            title: None,
            default_colors,
        }
    }

    /// 添加数据系列
    pub fn add_series(mut self, series: AreaSeries) -> Self {
        self.series.push(series);
        self
    }

    /// 创建单系列面积图
    pub fn single_series<T: Into<AreaDataPoint> + Clone>(
        mut self,
        label: &str,
        data: &[T],
    ) -> Self {
        let series = AreaSeries::new(label).data(data);
        self.series.push(series);
        self
    }

    /// 设置样式
    pub fn style(mut self, style: AreaStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置填充模式
    pub fn fill_mode(mut self, mode: AreaFillMode) -> Self {
        self.style.fill_mode = mode;
        self
    }

    /// 设置为堆叠面积图
    pub fn stacked(mut self) -> Self {
        self.style.fill_mode = AreaFillMode::Stacked;
        self
    }

    /// 显示数据点
    pub fn show_points(mut self, show: bool, size: f32) -> Self {
        self.style.show_points = show;
        self.style.point_size = size;
        self
    }

    /// 设置平滑曲线
    pub fn smooth(mut self, smooth: bool, factor: f32) -> Self {
        self.style.smooth = smooth;
        self.style.smooth_factor = factor;
        self
    }

    /// 设置X轴比例尺
    pub fn x_scale(mut self, scale: LinearScale) -> Self {
        self.x_scale = Some(scale);
        self
    }

    /// 设置Y轴比例尺
    pub fn y_scale(mut self, scale: LinearScale) -> Self {
        self.y_scale = Some(scale);
        self
    }

    /// 自动计算比例尺
    pub fn auto_scale(mut self) -> Self {
        if self.series.is_empty() {
            return self;
        }

        let mut x_min = f32::INFINITY;
        let mut x_max = f32::NEG_INFINITY;
        let mut y_min = f32::INFINITY;
        let mut y_max = f32::NEG_INFINITY;

        for series in &self.series {
            for point in &series.data {
                x_min = x_min.min(point.x);
                x_max = x_max.max(point.x);
                y_min = y_min.min(point.y);
                y_max = y_max.max(point.y);
            }
        }

        // 添加边距
        let x_margin = (x_max - x_min) * 0.05;
        let y_margin = (y_max - y_min) * 0.1;

        self.x_scale = Some(LinearScale::new(x_min - x_margin, x_max + x_margin));
        
        // 对于面积图，Y轴最小值通常设为0或数据最小值
        let y_bottom = match self.style.fill_mode {
            AreaFillMode::ToZero => 0.0,
            AreaFillMode::ToBaseline(baseline) => baseline.min(y_min),
            AreaFillMode::Stacked => 0.0,
        };
        
        self.y_scale = Some(LinearScale::new(y_bottom - y_margin, y_max + y_margin));
        self
    }

    /// 设置标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 获取系列数量
    pub fn series_count(&self) -> usize {
        self.series.len()
    }

    /// 生成渲染图元
    pub fn generate_primitives(&self, plot_area: super::PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.series.is_empty() {
            return primitives;
        }

        let default_x_scale = LinearScale::new(0.0, 1.0);
        let default_y_scale = LinearScale::new(0.0, 1.0);
        let x_scale = self.x_scale.as_ref().unwrap_or(&default_x_scale);
        let y_scale = self.y_scale.as_ref().unwrap_or(&default_y_scale);

        match self.style.fill_mode {
            AreaFillMode::Stacked => {
                self.generate_stacked_areas(&mut primitives, plot_area, x_scale, y_scale);
            }
            _ => {
                self.generate_individual_areas(&mut primitives, plot_area, x_scale, y_scale);
            }
        }

        primitives
    }

    fn generate_individual_areas(
        &self,
        primitives: &mut Vec<Primitive>,
        plot_area: super::PlotArea,
        x_scale: &LinearScale,
        y_scale: &LinearScale,
    ) {
        for (i, series) in self.series.iter().enumerate() {
            if series.data.is_empty() {
                continue;
            }

            // 选择默认颜色
            let (default_fill, default_line) = self.default_colors[i % self.default_colors.len()];
            let fill_color = if series.fill_color == Color::rgb(0.2, 0.6, 0.9) {
                default_fill
            } else {
                series.fill_color
            };
            let line_color = if series.line_color == Color::rgb(0.1, 0.4, 0.7) {
                default_line
            } else {
                series.line_color
            };

            // 确定基线Y坐标
            let baseline_y = match self.style.fill_mode {
                AreaFillMode::ToZero => {
                    let zero_norm = y_scale.normalize(0.0);
                    plot_area.y + plot_area.height - zero_norm * plot_area.height
                }
                AreaFillMode::ToBaseline(baseline) => {
                    let baseline_norm = y_scale.normalize(baseline);
                    plot_area.y + plot_area.height - baseline_norm * plot_area.height
                }
                _ => plot_area.y + plot_area.height,
            };

            // 生成面积多边形
            let mut area_points = Vec::new();
            
            // 添加数据点
            for point in &series.data {
                let x_norm = x_scale.normalize(point.x);
                let y_norm = y_scale.normalize(point.y);
                let screen_x = plot_area.x + x_norm * plot_area.width;
                let screen_y = plot_area.y + plot_area.height - y_norm * plot_area.height;
                area_points.push(Point2::new(screen_x, screen_y));
            }

            // 添加基线点（从右到左）
            if !area_points.is_empty() {
                let last_point = *area_points.last().unwrap();
                let first_point = *area_points.first().unwrap();
                area_points.push(Point2::new(last_point.x, baseline_y));
                area_points.push(Point2::new(first_point.x, baseline_y));
            }

            // 创建面积多边形
            if area_points.len() >= 3 {
                primitives.push(Primitive::Polygon {
                    points: area_points.clone(),
                    fill: fill_color,
                    stroke: None,
                });
            }

            // 生成边界线
            let mut line_points = Vec::new();
            for point in &series.data {
                let x_norm = x_scale.normalize(point.x);
                let y_norm = y_scale.normalize(point.y);
                let screen_x = plot_area.x + x_norm * plot_area.width;
                let screen_y = plot_area.y + plot_area.height - y_norm * plot_area.height;
                line_points.push(Point2::new(screen_x, screen_y));
            }

            if line_points.len() >= 2 {
                primitives.push(Primitive::Polyline {
                    points: line_points,
                    color: line_color,
                    width: series.line_width,
                });
            }

            // 添加数据点
            if self.style.show_points {
                for point in &series.data {
                    let x_norm = x_scale.normalize(point.x);
                    let y_norm = y_scale.normalize(point.y);
                    let screen_x = plot_area.x + x_norm * plot_area.width;
                    let screen_y = plot_area.y + plot_area.height - y_norm * plot_area.height;

                    primitives.push(Primitive::Circle {
                        center: Point2::new(screen_x, screen_y),
                        radius: self.style.point_size,
                    });
                }
            }
        }
    }

    fn generate_stacked_areas(
        &self,
        primitives: &mut Vec<Primitive>,
        plot_area: super::PlotArea,
        x_scale: &LinearScale,
        y_scale: &LinearScale,
    ) {
        // 堆叠面积图实现
        // 需要计算每个点的累积值
        if self.series.is_empty() {
            return;
        }

        // 获取所有X坐标的并集并排序
        let mut all_x_values = std::collections::BTreeSet::new();
        for series in &self.series {
            for point in &series.data {
                all_x_values.insert((point.x * 1000.0).round() as i32);
            }
        }
        let sorted_x: Vec<f32> = all_x_values.into_iter().map(|x| x as f32 / 1000.0).collect();

        // 为每个系列创建堆叠面积
        let mut cumulative_values = vec![0.0; sorted_x.len()];

        for (series_idx, series) in self.series.iter().enumerate() {
            let (default_fill, _default_line) = self.default_colors[series_idx % self.default_colors.len()];
            let fill_color = if series.fill_color == Color::rgb(0.2, 0.6, 0.9) {
                default_fill
            } else {
                series.fill_color
            };

            // 插值获取每个X位置的Y值
            let mut current_layer_points = Vec::new();
            let mut previous_layer_points = Vec::new();

            for (i, &x) in sorted_x.iter().enumerate() {
                // 在当前系列中查找对应的Y值（简单线性插值）
                let y_value = self.interpolate_y_value(series, x);
                
                let new_cumulative = cumulative_values[i] + y_value;
                
                let x_norm = x_scale.normalize(x);
                let current_y_norm = y_scale.normalize(new_cumulative);
                let previous_y_norm = y_scale.normalize(cumulative_values[i]);
                
                let screen_x = plot_area.x + x_norm * plot_area.width;
                let current_screen_y = plot_area.y + plot_area.height - current_y_norm * plot_area.height;
                let previous_screen_y = plot_area.y + plot_area.height - previous_y_norm * plot_area.height;
                
                current_layer_points.push(Point2::new(screen_x, current_screen_y));
                previous_layer_points.push(Point2::new(screen_x, previous_screen_y));
                
                cumulative_values[i] = new_cumulative;
            }

            // 创建当前层的多边形
            let mut polygon_points = current_layer_points;
            
            // 添加下层边界（反向）
            previous_layer_points.reverse();
            polygon_points.extend(previous_layer_points);

            if polygon_points.len() >= 3 {
                primitives.push(Primitive::Polygon {
                    points: polygon_points,
                    fill: fill_color,
                    stroke: None,
                });
            }
        }
    }

    fn interpolate_y_value(&self, series: &AreaSeries, target_x: f32) -> f32 {
        if series.data.is_empty() {
            return 0.0;
        }

        // 查找最接近的点或进行线性插值
        for window in series.data.windows(2) {
            let p1 = &window[0];
            let p2 = &window[1];
            
            if target_x >= p1.x && target_x <= p2.x {
                // 线性插值
                let t = (target_x - p1.x) / (p2.x - p1.x);
                return p1.y + t * (p2.y - p1.y);
            }
        }

        // 如果在范围外，使用最近的点
        if target_x < series.data[0].x {
            series.data[0].y
        } else {
            series.data.last().unwrap().y
        }
    }
}

impl Default for AreaChart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_chart_creation() {
        let chart = AreaChart::new();
        assert_eq!(chart.series_count(), 0);
    }

    #[test]
    fn test_single_series() {
        let data = [(0.0, 10.0), (1.0, 20.0), (2.0, 15.0)];
        let chart = AreaChart::new().single_series("测试", &data);
        
        assert_eq!(chart.series_count(), 1);
    }

    #[test]
    fn test_multiple_series() {
        let series1 = AreaSeries::new("系列1").data(&[(0.0, 10.0), (1.0, 15.0)]);
        let series2 = AreaSeries::new("系列2").data(&[(0.0, 5.0), (1.0, 8.0)]);
        
        let chart = AreaChart::new()
            .add_series(series1)
            .add_series(series2);
        
        assert_eq!(chart.series_count(), 2);
    }

    #[test]
    fn test_stacked_mode() {
        let chart = AreaChart::new().stacked();
        assert_eq!(chart.style.fill_mode, AreaFillMode::Stacked);
    }

    #[test]
    fn test_area_interpolation() {
        let chart = AreaChart::new();
        let series = AreaSeries::new("test").data(&[(0.0, 0.0), (2.0, 20.0)]);
        
        // 测试中点插值
        let interpolated = chart.interpolate_y_value(&series, 1.0);
        assert_eq!(interpolated, 10.0);
    }
}
