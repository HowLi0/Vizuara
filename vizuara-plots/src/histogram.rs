use vizuara_core::{Primitive, Color, Scale, LinearScale};
use nalgebra::Point2;

/// 直方图数据桶
#[derive(Debug, Clone)]
pub struct HistogramBin {
    /// 桶的起始值
    pub start: f32,
    /// 桶的结束值  
    pub end: f32,
    /// 桶中数据点的数量
    pub count: usize,
    /// 桶的频率密度 (count / bin_width)
    pub density: f32,
}

impl HistogramBin {
    pub fn new(start: f32, end: f32, count: usize) -> Self {
        let width = end - start;
        let density = if width > 0.0 { count as f32 / width } else { 0.0 };
        Self { start, end, count, density }
    }

    /// 获取桶的中心位置
    pub fn center(&self) -> f32 {
        (self.start + self.end) / 2.0
    }

    /// 获取桶的宽度
    pub fn width(&self) -> f32 {
        self.end - self.start
    }
}

/// 直方图样式配置
#[derive(Debug, Clone)]
pub struct HistogramStyle {
    /// 柱子填充颜色
    pub fill_color: Color,
    /// 柱子边框颜色
    pub stroke_color: Color,
    /// 边框宽度
    pub stroke_width: f32,
    /// 柱子透明度
    pub alpha: f32,
}

impl Default for HistogramStyle {
    fn default() -> Self {
        Self {
            fill_color: Color::rgb(0.3, 0.7, 0.9),
            stroke_color: Color::rgb(0.1, 0.3, 0.5),
            stroke_width: 1.0,
            alpha: 0.8,
        }
    }
}

/// 直方图分桶策略
#[derive(Debug, Clone)]
pub enum BinningStrategy {
    /// 固定桶数量
    FixedCount(usize),
    /// 固定桶宽度
    FixedWidth(f32),
    /// 自动选择 (使用 Sturges 规则)
    Auto,
}

impl Default for BinningStrategy {
    fn default() -> Self {
        Self::Auto
    }
}

/// 直方图
#[derive(Debug, Clone)]
pub struct Histogram {
    /// 原始数据
    data: Vec<f32>,
    /// 分桶策略
    binning: BinningStrategy,
    /// 样式配置
    style: HistogramStyle,
    /// 计算得到的桶数据
    bins: Vec<HistogramBin>,
    /// X轴比例尺
    x_scale: Option<LinearScale>,
    /// Y轴比例尺 
    y_scale: Option<LinearScale>,
}

impl Histogram {
    /// 创建新的直方图
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            binning: BinningStrategy::default(),
            style: HistogramStyle::default(),
            bins: Vec::new(),
            x_scale: None,
            y_scale: None,
        }
    }

    /// 设置数据
    pub fn data(mut self, data: &[f32]) -> Self {
        self.data = data.to_vec();
        self.compute_bins();
        self
    }

    /// 设置分桶策略
    pub fn binning(mut self, strategy: BinningStrategy) -> Self {
        self.binning = strategy;
        if !self.data.is_empty() {
            self.compute_bins();
        }
        self
    }

    /// 设置样式
    pub fn style(mut self, style: HistogramStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置填充颜色
    pub fn fill_color(mut self, color: Color) -> Self {
        self.style.fill_color = color;
        self
    }

    /// 设置边框颜色
    pub fn stroke_color(mut self, color: Color) -> Self {
        self.style.stroke_color = color;
        self
    }

    /// 自动计算比例尺
    pub fn auto_scale(mut self) -> Self {
        if self.bins.is_empty() {
            return self;
        }

        // 计算X轴范围 (数据值范围)
        let min_val = self.bins.first().unwrap().start;
        let max_val = self.bins.last().unwrap().end;
        
        // 计算Y轴范围 (频次范围)
        let max_count = self.bins.iter().map(|b| b.count).max().unwrap_or(0);

        self.x_scale = Some(LinearScale::new(min_val, max_val));
        self.y_scale = Some(LinearScale::new(0.0, max_count as f32));
        
        self
    }

    /// 计算桶数据
    fn compute_bins(&mut self) {
        if self.data.is_empty() {
            self.bins.clear();
            return;
        }

        // 计算数据范围
        let min_val = self.data.iter().cloned().fold(f32::INFINITY, f32::min);
        let max_val = self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        
        if min_val >= max_val {
            self.bins.clear();
            return;
        }

        // 确定桶的数量
        let bin_count = match self.binning {
            BinningStrategy::FixedCount(n) => n,
            BinningStrategy::FixedWidth(width) => {
                ((max_val - min_val) / width).ceil() as usize
            },
            BinningStrategy::Auto => {
                // 使用 Sturges 规则: ceil(log2(n)) + 1
                let n = self.data.len();
                if n <= 1 { 1 } else { (n as f32).log2().ceil() as usize + 1 }
            }
        };

        let bin_count = bin_count.max(1); // 至少1个桶
        let bin_width = (max_val - min_val) / bin_count as f32;

        // 初始化桶
        let mut bins = Vec::with_capacity(bin_count);
        for i in 0..bin_count {
            let start = min_val + i as f32 * bin_width;
            let end = if i == bin_count - 1 {
                max_val // 最后一个桶包含最大值
            } else {
                min_val + (i + 1) as f32 * bin_width
            };
            bins.push(HistogramBin::new(start, end, 0));
        }

        // 分配数据到桶中
        for &value in &self.data {
            let bin_index = if value >= max_val {
                bin_count - 1 // 最大值放入最后一个桶
            } else {
                ((value - min_val) / bin_width) as usize
            };
            
            if bin_index < bins.len() {
                bins[bin_index].count += 1;
                // 重新计算密度
                let width = bins[bin_index].width();
                bins[bin_index].density = if width > 0.0 { 
                    bins[bin_index].count as f32 / width 
                } else { 
                    0.0 
                };
            }
        }

        self.bins = bins;
    }

    /// 获取桶数据
    pub fn bins(&self) -> &[HistogramBin] {
        &self.bins
    }

    /// 生成渲染图元
    pub fn generate_primitives(&self, plot_area: super::PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.bins.is_empty() {
            return primitives;
        }

        // 获取或创建比例尺
        let x_scale = if let Some(ref scale) = self.x_scale {
            scale.clone()
        } else {
            let min_val = self.bins.first().unwrap().start;
            let max_val = self.bins.last().unwrap().end;
            LinearScale::new(min_val, max_val)
        };
        
        let y_scale = if let Some(ref scale) = self.y_scale {
            scale.clone()
        } else {
            let max_count = self.bins.iter().map(|b| b.count).max().unwrap_or(0);
            LinearScale::new(0.0, max_count as f32)
        };

        // 为每个桶创建矩形
        for bin in &self.bins {
            if bin.count == 0 {
                continue; // 跳过空桶
            }

            // 计算屏幕坐标
            let x_norm_start = x_scale.normalize(bin.start);
            let x_norm_end = x_scale.normalize(bin.end);
            let y_norm = y_scale.normalize(bin.count as f32);

            let screen_x_start = plot_area.x + x_norm_start * plot_area.width;
            let screen_x_end = plot_area.x + x_norm_end * plot_area.width;
            let screen_y_bottom = plot_area.y + plot_area.height; // 底部
            let screen_y_top = plot_area.y + plot_area.height - y_norm * plot_area.height;

            // 创建矩形图元
            primitives.push(Primitive::RectangleStyled {
                min: Point2::new(screen_x_start, screen_y_top),
                max: Point2::new(screen_x_end, screen_y_bottom),
                fill: self.style.fill_color,
                stroke: Some((self.style.stroke_color, self.style.stroke_width)),
            });
        }

        primitives
    }
}

impl Default for Histogram {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_histogram_creation() {
        let hist = Histogram::new();
        assert!(hist.data.is_empty());
        assert!(hist.bins.is_empty());
    }

    #[test]
    fn test_histogram_with_data() {
        let data = vec![1.0, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0];
        let hist = Histogram::new()
            .data(&data)
            .binning(BinningStrategy::FixedCount(4))
            .auto_scale();
        
        assert_eq!(hist.data.len(), 8);
        assert_eq!(hist.bins.len(), 4);
        
        // 验证桶的总数据点数等于原始数据数量
        let total_count: usize = hist.bins.iter().map(|b| b.count).sum();
        assert_eq!(total_count, data.len());
    }

    #[test]
    fn test_auto_binning() {
        let data: Vec<f32> = (0..100).map(|i| i as f32).collect();
        let hist = Histogram::new()
            .data(&data)
            .binning(BinningStrategy::Auto);
        
        // Sturges 规则: log2(100) + 1 ≈ 8
        assert!(hist.bins.len() >= 7 && hist.bins.len() <= 9);
    }

    #[test]
    fn test_fixed_width_binning() {
        let data = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let hist = Histogram::new()
            .data(&data)
            .binning(BinningStrategy::FixedWidth(1.0));
        
        assert_eq!(hist.bins.len(), 5); // 0-1, 1-2, 2-3, 3-4, 4-5
    }

    #[test]
    fn test_bin_density_calculation() {
        let data = vec![1.0, 1.5, 2.0, 2.5];
        let hist = Histogram::new()
            .data(&data)
            .binning(BinningStrategy::FixedCount(2));
        
        for bin in &hist.bins {
            if bin.count > 0 {
                let expected_density = bin.count as f32 / bin.width();
                assert!((bin.density - expected_density).abs() < 1e-6);
            }
        }
    }

    #[test]
    fn test_primitive_generation() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let hist = Histogram::new()
            .data(&data)
            .binning(BinningStrategy::FixedCount(2))
            .auto_scale();
        
        let plot_area = crate::PlotArea::new(100.0, 100.0, 400.0, 300.0);
        let primitives = hist.generate_primitives(plot_area);
        
        // 应该有2个桶的矩形 (如果两个桶都有数据)
        assert!(primitives.len() >= 1 && primitives.len() <= 2);
    }
}
