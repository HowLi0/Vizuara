use nalgebra::Point2;
use vizuara_core::{Color, Primitive};

/// 热力图数据点
#[derive(Debug, Clone)]
pub struct HeatmapCell {
    /// X坐标索引
    pub x_index: usize,
    /// Y坐标索引  
    pub y_index: usize,
    /// 数值
    pub value: f32,
}

impl HeatmapCell {
    pub fn new(x_index: usize, y_index: usize, value: f32) -> Self {
        Self {
            x_index,
            y_index,
            value,
        }
    }
}

/// 颜色映射策略
#[derive(Debug, Clone)]
pub enum ColorMap {
    /// 蓝-白-红渐变 (常用于温度数据)
    BlueWhiteRed,
    /// 蓝绿渐变 (常用于深度数据)
    BlueGreen,
    /// 灰度 (黑白)
    Grayscale,
    /// 彩虹色谱
    Rainbow,
    /// 自定义渐变 (起始色 -> 结束色)
    Custom(Color, Color),
}

impl Default for ColorMap {
    fn default() -> Self {
        Self::BlueWhiteRed
    }
}

impl ColorMap {
    /// 根据标准化值 (0.0-1.0) 获取对应颜色
    pub fn get_color(&self, normalized_value: f32) -> Color {
        let t = normalized_value.clamp(0.0, 1.0);

        match self {
            ColorMap::BlueWhiteRed => {
                if t < 0.5 {
                    // 蓝色到白色
                    let factor = t * 2.0;
                    Color::rgb(factor, factor, 1.0)
                } else {
                    // 白色到红色
                    let factor = (t - 0.5) * 2.0;
                    Color::rgb(1.0, 1.0 - factor, 1.0 - factor)
                }
            }
            ColorMap::BlueGreen => Color::rgb(0.0, t, 1.0 - t),
            ColorMap::Grayscale => Color::rgb(t, t, t),
            ColorMap::Rainbow => {
                // HSV彩虹映射: H = t * 300° (避免回到红色)
                let h = t * 300.0; // 色相角度
                let s = 1.0; // 饱和度
                let v = 1.0; // 明度
                hsv_to_rgb(h, s, v)
            }
            ColorMap::Custom(start, end) => Color::rgb(
                start.r + t * (end.r - start.r),
                start.g + t * (end.g - start.g),
                start.b + t * (end.b - start.b),
            ),
        }
    }
}

/// HSV到RGB颜色空间转换
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = match h_prime as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    Color::rgb(r_prime + m, g_prime + m, b_prime + m)
}

/// 热力图样式配置
#[derive(Debug, Clone)]
pub struct HeatmapStyle {
    /// 颜色映射
    pub color_map: ColorMap,
    /// 是否显示网格线
    pub show_grid: bool,
    /// 网格线颜色
    pub grid_color: Color,
    /// 网格线宽度
    pub grid_width: f32,
    /// 是否显示数值标签
    pub show_values: bool,
    /// 标签字体大小
    pub label_size: f32,
    /// 标签颜色
    pub label_color: Color,
}

impl Default for HeatmapStyle {
    fn default() -> Self {
        Self {
            color_map: ColorMap::default(),
            show_grid: true,
            grid_color: Color::rgb(0.8, 0.8, 0.8),
            grid_width: 1.0,
            show_values: false,
            label_size: 10.0,
            label_color: Color::rgb(0.2, 0.2, 0.2),
        }
    }
}

/// 热力图
#[derive(Debug, Clone)]
pub struct Heatmap {
    /// 2D数据矩阵 (按行存储: data[row][col])
    data: Vec<Vec<f32>>,
    /// X轴标签 (列标签)
    x_labels: Vec<String>,
    /// Y轴标签 (行标签)
    y_labels: Vec<String>,
    /// 样式配置
    style: HeatmapStyle,
    /// 数值范围 (用于颜色映射)
    value_range: Option<(f32, f32)>,
}

impl Heatmap {
    /// 创建新的热力图
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            x_labels: Vec::new(),
            y_labels: Vec::new(),
            style: HeatmapStyle::default(),
            value_range: None,
        }
    }

    /// 从2D数组设置数据
    pub fn data(mut self, data: &[Vec<f32>]) -> Self {
        self.data = data.to_vec();
        self.auto_generate_labels();
        self.compute_value_range();
        self
    }

    /// 从1D数组和维度设置数据
    pub fn data_matrix(mut self, data: &[f32], rows: usize, cols: usize) -> Self {
        assert_eq!(
            data.len(),
            rows * cols,
            "Data length must match rows * cols"
        );

        self.data = (0..rows)
            .map(|row| (0..cols).map(|col| data[row * cols + col]).collect())
            .collect();

        self.auto_generate_labels();
        self.compute_value_range();
        self
    }

    /// 设置X轴标签
    pub fn x_labels(mut self, labels: &[&str]) -> Self {
        self.x_labels = labels.iter().map(|&s| s.to_string()).collect();
        self
    }

    /// 设置Y轴标签
    pub fn y_labels(mut self, labels: &[&str]) -> Self {
        self.y_labels = labels.iter().map(|&s| s.to_string()).collect();
        self
    }

    /// 设置样式
    pub fn style(mut self, style: HeatmapStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置颜色映射
    pub fn color_map(mut self, color_map: ColorMap) -> Self {
        self.style.color_map = color_map;
        self
    }

    /// 设置是否显示网格
    pub fn show_grid(mut self, show: bool) -> Self {
        self.style.show_grid = show;
        self
    }

    /// 设置是否显示数值
    pub fn show_values(mut self, show: bool) -> Self {
        self.style.show_values = show;
        self
    }

    /// 设置数值范围 (用于颜色映射)
    pub fn value_range(mut self, min: f32, max: f32) -> Self {
        self.value_range = Some((min, max));
        self
    }

    /// 自动计算数值范围
    pub fn auto_range(mut self) -> Self {
        self.compute_value_range();
        self
    }

    /// 自动生成标签
    fn auto_generate_labels(&mut self) {
        if self.data.is_empty() {
            return;
        }

        let rows = self.data.len();
        let cols = self.data[0].len();

        if self.x_labels.is_empty() {
            self.x_labels = (0..cols).map(|i| format!("C{}", i)).collect();
        }

        if self.y_labels.is_empty() {
            self.y_labels = (0..rows).map(|i| format!("R{}", i)).collect();
        }
    }

    /// 计算数值范围
    fn compute_value_range(&mut self) {
        if self.data.is_empty() {
            self.value_range = Some((0.0, 1.0));
            return;
        }

        let mut min_val = f32::INFINITY;
        let mut max_val = f32::NEG_INFINITY;

        for row in &self.data {
            for &value in row {
                min_val = min_val.min(value);
                max_val = max_val.max(value);
            }
        }

        // 避免除零
        if (max_val - min_val).abs() < 1e-6 {
            max_val = min_val + 1.0;
        }

        self.value_range = Some((min_val, max_val));
    }

    /// 获取数据维度
    pub fn dimensions(&self) -> (usize, usize) {
        if self.data.is_empty() {
            (0, 0)
        } else {
            (self.data.len(), self.data[0].len())
        }
    }

    /// 获取指定位置的数值
    pub fn get_value(&self, row: usize, col: usize) -> Option<f32> {
        self.data.get(row)?.get(col).copied()
    }

    /// 生成渲染图元
    pub fn generate_primitives(&self, plot_area: crate::PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.data.is_empty() {
            return primitives;
        }

        let (rows, cols) = self.dimensions();
        let (min_val, max_val) = self.value_range.unwrap_or((0.0, 1.0));

        // 计算每个单元格的大小
        let cell_width = plot_area.width / cols as f32;
        let cell_height = plot_area.height / rows as f32;

        // 为每个单元格创建矩形
        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                // 计算单元格位置
                let x = plot_area.x + col_idx as f32 * cell_width;
                let y = plot_area.y + row_idx as f32 * cell_height;

                // 标准化数值到 [0, 1]
                let normalized_value = (value - min_val) / (max_val - min_val);
                let color = self.style.color_map.get_color(normalized_value);

                // 创建填充矩形
                primitives.push(Primitive::RectangleStyled {
                    min: Point2::new(x, y),
                    max: Point2::new(x + cell_width, y + cell_height),
                    fill: color,
                    stroke: if self.style.show_grid {
                        Some((self.style.grid_color, self.style.grid_width))
                    } else {
                        None
                    },
                });

                // 添加数值标签
                if self.style.show_values {
                    let label_x = x + cell_width / 2.0;
                    let label_y = y + cell_height / 2.0;

                    primitives.push(Primitive::Text {
                        position: Point2::new(label_x, label_y),
                        content: format!("{:.1}", value),
                        size: self.style.label_size,
                        color: self.style.label_color,
                        h_align: vizuara_core::HorizontalAlign::Center,
                        v_align: vizuara_core::VerticalAlign::Middle,
                    });
                }
            }
        }

        // 添加轴标签
        self.add_axis_labels(&mut primitives, plot_area, cell_width, cell_height);

        primitives
    }

    /// 添加轴标签
    fn add_axis_labels(
        &self,
        primitives: &mut Vec<Primitive>,
        plot_area: crate::PlotArea,
        cell_width: f32,
        cell_height: f32,
    ) {
        // X轴标签 (列标签)
        for (i, label) in self.x_labels.iter().enumerate() {
            let x = plot_area.x + (i as f32 + 0.5) * cell_width;
            let y = plot_area.y + plot_area.height + 15.0;

            primitives.push(Primitive::Text {
                position: Point2::new(x, y),
                content: label.clone(),
                size: 10.0,
                color: Color::rgb(0.2, 0.2, 0.2),
                h_align: vizuara_core::HorizontalAlign::Center,
                v_align: vizuara_core::VerticalAlign::Top,
            });
        }

        // Y轴标签 (行标签)
        for (i, label) in self.y_labels.iter().enumerate() {
            let x = plot_area.x - 15.0;
            let y = plot_area.y + (i as f32 + 0.5) * cell_height;

            primitives.push(Primitive::Text {
                position: Point2::new(x, y),
                content: label.clone(),
                size: 10.0,
                color: Color::rgb(0.2, 0.2, 0.2),
                h_align: vizuara_core::HorizontalAlign::Right,
                v_align: vizuara_core::VerticalAlign::Middle,
            });
        }
    }
}

impl Default for Heatmap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heatmap_creation() {
        let heatmap = Heatmap::new();
        assert_eq!(heatmap.dimensions(), (0, 0));
    }

    #[test]
    fn test_heatmap_with_2d_data() {
        let data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];

        let heatmap = Heatmap::new().data(&data);
        assert_eq!(heatmap.dimensions(), (2, 3));
        assert_eq!(heatmap.get_value(0, 1), Some(2.0));
        assert_eq!(heatmap.get_value(1, 2), Some(6.0));
    }

    #[test]
    fn test_heatmap_with_matrix_data() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let heatmap = Heatmap::new().data_matrix(&data, 2, 3);

        assert_eq!(heatmap.dimensions(), (2, 3));
        assert_eq!(heatmap.get_value(0, 0), Some(1.0));
        assert_eq!(heatmap.get_value(1, 2), Some(6.0));
    }

    #[test]
    fn test_color_mapping() {
        let color_map = ColorMap::BlueWhiteRed;

        // 测试边界值
        let blue = color_map.get_color(0.0);
        let white = color_map.get_color(0.5);
        let red = color_map.get_color(1.0);

        // 蓝色 (0.0): 应该是蓝色分量高
        assert!(blue.b > 0.8);
        // 白色 (0.5): 所有分量都应该高
        assert!(white.r > 0.8 && white.g > 0.8 && white.b > 0.8);
        // 红色 (1.0): 应该是红色分量高
        assert!(red.r > 0.8);
    }

    #[test]
    fn test_auto_labels() {
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];

        let heatmap = Heatmap::new().data(&data);
        assert_eq!(heatmap.x_labels, vec!["C0", "C1"]);
        assert_eq!(heatmap.y_labels, vec!["R0", "R1", "R2"]);
    }

    #[test]
    fn test_custom_labels() {
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];

        let heatmap = Heatmap::new()
            .data(&data)
            .x_labels(&["Week 1", "Week 2"])
            .y_labels(&["Product A", "Product B"]);

        assert_eq!(heatmap.x_labels, vec!["Week 1", "Week 2"]);
        assert_eq!(heatmap.y_labels, vec!["Product A", "Product B"]);
    }

    #[test]
    fn test_value_range() {
        let data = vec![vec![1.0, 5.0], vec![2.0, 8.0]];

        let heatmap = Heatmap::new().data(&data).auto_range();
        assert_eq!(heatmap.value_range, Some((1.0, 8.0)));
    }

    #[test]
    fn test_primitive_generation() {
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let heatmap = Heatmap::new().data(&data).auto_range();

        let plot_area = crate::PlotArea::new(0.0, 0.0, 100.0, 100.0);
        let primitives = heatmap.generate_primitives(plot_area);

        // 应该有4个单元格矩形 + 标签
        assert!(primitives.len() >= 4);
    }
}
