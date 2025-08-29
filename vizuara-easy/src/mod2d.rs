use vizuara_core::{Color, Result};
use vizuara_plots::{
    AreaChart, BarPlot, BoxPlot, ContourPlot, DensityPlot, Heatmap, Histogram,
    LinePlot, PieChart, PlotArea, RadarChart, ScatterPlot, ViolinPlot
};
use vizuara_scene::{Figure, Scene};
use vizuara_window::show_figure;

/// 2D Figure 便捷封装
/// 
/// 提供类似 Matplotlib 的简易绘图接口，支持多种图表类型和子图布局
pub struct Figure2D {
    figure: Figure,
    current_scene: Option<Scene>,
    grid: Option<(usize, usize)>,
    cell_index: usize,
    // 当前子图的轴设置
    current_xlabel: Option<String>,
    current_ylabel: Option<String>,
    current_title: Option<String>,
    current_xlim: Option<(f32, f32)>,
    current_ylim: Option<(f32, f32)>,
}

impl Figure2D {
    pub fn new(width: f32, height: f32) -> Self {
        Self { 
            figure: Figure::new(width, height), 
            current_scene: None, 
            grid: None, 
            cell_index: 0,
            current_xlabel: None,
            current_ylabel: None,
            current_title: None,
            current_xlim: None,
            current_ylim: None,
        }
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.figure = std::mem::take(&mut self.figure).title(title.to_string());
        self
    }

    /// 设置 subplot 网格，后续调用 next_subplot 进入下一格
    pub fn grid(&mut self, rows: usize, cols: usize) -> &mut Self {
        self.grid = Some((rows, cols));
        self.cell_index = 0;
        self
    }

    /// 进入下一格子并准备绘图区域
    pub fn next_subplot(&mut self) -> &mut Self {
        // 提交当前子图
        if self.current_scene.is_some() {
            self.commit_subplot();
        }
        
        let (w, h) = self.figure.size();
        let padding = 40.0;
        let (rows, cols) = self.grid.unwrap_or((1, 1));
        let cw = (w - padding * 2.0) / cols as f32;
        let ch = (h - padding * 2.0) / rows as f32;
        let idx = self.cell_index.min(rows * cols - 1);
        let r = idx / cols;
        let c = idx % cols;
        let x = padding + c as f32 * cw;
        let y = padding + r as f32 * ch;
        self.current_scene = Some(Scene::new(PlotArea::new(x, y, cw - 20.0, ch - 20.0)));
        self.cell_index += 1;
        
        // 重置当前子图设置
        self.current_xlabel = None;
        self.current_ylabel = None;
        self.current_title = None;
        self.current_xlim = None;
        self.current_ylim = None;
        
        self
    }

    /// 直接开一个满幅子图
    pub fn subplot_full(&mut self) -> &mut Self {
        // 提交之前的子图
        if self.current_scene.is_some() {
            self.commit_subplot();
        }
        
        let (w, h) = self.figure.size();
        let pa = PlotArea::new(80.0, 80.0, w - 160.0, h - 160.0);
        self.current_scene = Some(Scene::new(pa));
        
        // 重置当前子图设置
        self.current_xlabel = None;
        self.current_ylabel = None;
        self.current_title = None;
        self.current_xlim = None;
        self.current_ylim = None;
        
        self
    }

    pub fn scatter(&mut self, data: &[(f32, f32)], color: Color, size: f32) -> &mut Self {
        let scatter = ScatterPlot::new().data(data).color(color).size(size).auto_scale();
        if let Some(scene) = self.current_scene.take() {
            self.current_scene = Some(scene.add_scatter_plot(scatter));
        }
        self
    }

    pub fn plot(&mut self, data: &[(f32, f32)], color: Color, width: f32) -> &mut Self {
        let line = LinePlot::new().data(data).color(color).line_width(width).auto_scale();
        if let Some(scene) = self.current_scene.take() {
            self.current_scene = Some(scene.add_line_plot(line));
        }
        self
    }

    // ================= 轴标签和标题 =================
    
    /// 设置X轴标签
    pub fn xlabel(&mut self, label: &str) -> &mut Self {
        self.current_xlabel = Some(label.to_string());
        self
    }
    
    /// 设置Y轴标签
    pub fn ylabel(&mut self, label: &str) -> &mut Self {
        self.current_ylabel = Some(label.to_string());
        self
    }
    
    /// 设置子图标题
    pub fn subplot_title(&mut self, title: &str) -> &mut Self {
        self.current_title = Some(title.to_string());
        self
    }
    
    /// 设置X轴范围
    pub fn xlim(&mut self, min: f32, max: f32) -> &mut Self {
        self.current_xlim = Some((min, max));
        self
    }
    
    /// 设置Y轴范围
    pub fn ylim(&mut self, min: f32, max: f32) -> &mut Self {
        self.current_ylim = Some((min, max));
        self
    }

    // ================= 更多图表类型 =================
    
    /// 添加条形图
    pub fn bar(&mut self, categories: &[&str], values: &[f32], color: Color) -> &mut Self {
        let bar = BarPlot::new().categories_values(categories, values).fill_color(color).auto_scale();
        if let Some(scene) = self.current_scene.take() {
            self.current_scene = Some(scene.add_bar_plot(bar));
        }
        self
    }
    
    /// 添加直方图
    pub fn hist(&mut self, data: &[f32], bins: usize, color: Color) -> &mut Self {
        use vizuara_plots::BinningStrategy;
        let hist = Histogram::new().data(data).binning(BinningStrategy::FixedCount(bins)).fill_color(color).auto_scale();
        if let Some(scene) = self.current_scene.take() {
            self.current_scene = Some(scene.add_histogram(hist));
        }
        self
    }
    
    /// 添加箱线图
    pub fn boxplot(&mut self, data: &[Vec<f32>], labels: &[&str]) -> &mut Self {
        let mut boxplot = BoxPlot::new();
        for (i, group_data) in data.iter().enumerate() {
            let label = labels.get(i).unwrap_or(&"").to_string();
            let group = vizuara_plots::BoxPlotGroup::from_data(label, group_data.clone());
            boxplot = boxplot.add_group(group);
        }
        if let Some(scene) = self.current_scene.take() {
            self.current_scene = Some(scene.add_boxplot(boxplot));
        }
        self
    }
    
    /// 添加小提琴图
    pub fn violin(&mut self, data: &[Vec<f32>], labels: &[&str]) -> &mut Self {
        let data_groups: Vec<(&str, Vec<f32>)> = data.iter()
            .enumerate()
            .map(|(i, group_data)| {
                let label = labels.get(i).unwrap_or(&"");
                (*label, group_data.clone())
            })
            .collect();
        let violin = ViolinPlot::new().from_data_groups(&data_groups);
        if let Some(scene) = self.current_scene.take() {
            self.current_scene = Some(scene.add_violin_plot(violin));
        }
        self
    }
    
    /// 添加饼图
    pub fn pie(&mut self, data: &[(&str, f32)], _colors: &[Color]) -> &mut Self {
        let labels: Vec<&str> = data.iter().map(|(label, _)| *label).collect();
        let values: Vec<f32> = data.iter().map(|(_, value)| *value).collect();
        let pie = PieChart::new().labels_values(&labels, &values);
        if let Some(scene) = self.current_scene.take() {
            self.current_scene = Some(scene.add_pie_chart(pie));
        }
        self
    }
    
    /// 添加热力图
    pub fn heatmap(&mut self, data: &[Vec<f32>], x_labels: &[&str], y_labels: &[&str]) -> &mut Self {
        let heatmap = Heatmap::new().data(data).x_labels(x_labels).y_labels(y_labels);
        if let Some(scene) = self.current_scene.take() {
            self.current_scene = Some(scene.add_heatmap(heatmap));
        }
        self
    }
    
    /// 添加面积图
    pub fn area(&mut self, data: &[(f32, f32)], _color: Color, _alpha: f32) -> &mut Self {
        let area = AreaChart::new().single_series("area", data).auto_scale();
        if let Some(scene) = self.current_scene.take() {
            self.current_scene = Some(scene.add_area_chart(area));
        }
        self
    }
    
    /// 添加密度图
    pub fn density(&mut self, data: &[f32], color: Color) -> &mut Self {
        let density = DensityPlot::new().data(data).fill_color(Some(color));
        if let Some(scene) = self.current_scene.take() {
            self.current_scene = Some(scene.add_density_plot(density));
        }
        self
    }
    
    /// 添加等高线图（暂时简化实现）
    pub fn contour(&mut self, _x: &[f32], _y: &[f32], _z: &[Vec<f32>]) -> &mut Self {
        // 等高线图需要专门的数据格式，暂时创建空的图表
        let contour = ContourPlot::new();
        if let Some(scene) = self.current_scene.take() {
            self.current_scene = Some(scene.add_contour_plot(contour));
        }
        self
    }
    
    /// 添加雷达图
    pub fn radar(&mut self, data: &[f32], labels: &[&str], _color: Color) -> &mut Self {
        let radar = RadarChart::new()
            .simple_dimensions(labels, 0.0, data.iter().fold(0.0f32, |a, &b| a.max(b)))
            .add_data("data", data.to_vec());
        if let Some(scene) = self.current_scene.take() {
            self.current_scene = Some(scene.add_radar_chart(radar));
        }
        self
    }

    // ================= 快捷绘图方法 =================
    
    /// 快速绘制带标签的线图
    pub fn plot_with_label(&mut self, data: &[(f32, f32)], color: Color, width: f32, _label: &str) -> &mut Self {
        let line = LinePlot::new().data(data).color(color).line_width(width).auto_scale();
        if let Some(scene) = self.current_scene.take() {
            self.current_scene = Some(scene.add_line_plot(line));
        }
        self
    }
    
    /// 快速绘制带标签的散点图
    pub fn scatter_with_label(&mut self, data: &[(f32, f32)], color: Color, size: f32, _label: &str) -> &mut Self {
        let scatter = ScatterPlot::new().data(data).color(color).size(size).auto_scale();
        if let Some(scene) = self.current_scene.take() {
            self.current_scene = Some(scene.add_scatter_plot(scatter));
        }
        self
    }
    
    /// 多条线图（不同颜色）
    pub fn multiplot(&mut self, datasets: &[(&[(f32, f32)], Color, &str)], width: f32) -> &mut Self {
        for (data, color, label) in datasets {
            self.plot_with_label(data, *color, width, label);
        }
        self
    }
    
    /// 多组散点图
    pub fn multiscatter(&mut self, datasets: &[(&[(f32, f32)], Color, &str)], size: f32) -> &mut Self {
        for (data, color, label) in datasets {
            self.scatter_with_label(data, *color, size, label);
        }
        self
    }

    pub fn commit_subplot(&mut self) -> &mut Self {
        if let Some(scene) = self.current_scene.take() {
            // 暂时只提交场景，将来可以添加轴标签等功能
            // 轴标签功能需要在Scene API中实现
            self.figure = std::mem::take(&mut self.figure).add_scene(scene);
        }
        self
    }

    pub fn show(mut self) -> Result<()> {
        if self.current_scene.is_some() { self.commit_subplot(); }
        show_figure(self.figure).map_err(|e| vizuara_core::VizuaraError::RenderError(e.to_string()))
    }

    // ================= 主题和样式 =================
    
    /// 应用默认样式（网格、坐标轴等）
    pub fn apply_default_style(&mut self) -> &mut Self {
        // 这里可以设置默认的网格、背景色等
        self
    }
    
    /// 应用科学论文风格
    pub fn scientific_style(&mut self) -> &mut Self {
        // 设置适合科学论文的样式
        self
    }
    
    /// 应用暗色主题
    pub fn dark_theme(&mut self) -> &mut Self {
        // 设置暗色主题
        self
    }
}

// ================= 便捷函数 =================

/// 预定义颜色集合
pub struct Colors;

impl Colors {
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const ORANGE: Color = Color { r: 1.0, g: 0.65, b: 0.0, a: 1.0 };
    pub const PURPLE: Color = Color { r: 0.5, g: 0.0, b: 0.5, a: 1.0 };
    pub const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const PINK: Color = Color { r: 1.0, g: 0.75, b: 0.8, a: 1.0 };
    
    /// 获取默认颜色序列
    pub fn default_sequence() -> Vec<Color> {
        vec![
            Self::BLUE,
            Self::ORANGE,
            Self::GREEN,
            Self::RED,
            Self::PURPLE,
            Self::CYAN,
            Self::YELLOW,
            Self::PINK,
        ]
    }
    
    /// 获取科学绘图颜色序列
    pub fn scientific_sequence() -> Vec<Color> {
        vec![
            Color::rgb(0.0, 0.4, 0.8),    // 深蓝
            Color::rgb(0.8, 0.4, 0.0),    // 深橙
            Color::rgb(0.0, 0.6, 0.4),    // 深绿
            Color::rgb(0.8, 0.2, 0.2),    // 深红
            Color::rgb(0.4, 0.0, 0.6),    // 深紫
            Color::rgb(0.6, 0.4, 0.0),    // 棕色
        ]
    }
}

/// 生成测试数据的便捷函数
pub mod testdata {
    /// 生成正弦波数据
    pub fn sine_wave(n_points: usize, amplitude: f32, frequency: f32) -> Vec<(f32, f32)> {
        (0..n_points)
            .map(|i| {
                let x = i as f32 * 2.0 * std::f32::consts::PI / n_points as f32 * frequency;
                (x, amplitude * x.sin())
            })
            .collect()
    }
    
    /// 生成余弦波数据
    pub fn cosine_wave(n_points: usize, amplitude: f32, frequency: f32) -> Vec<(f32, f32)> {
        (0..n_points)
            .map(|i| {
                let x = i as f32 * 2.0 * std::f32::consts::PI / n_points as f32 * frequency;
                (x, amplitude * x.cos())
            })
            .collect()
    }
    
    /// 生成线性数据
    pub fn linear(n_points: usize, slope: f32, intercept: f32) -> Vec<(f32, f32)> {
        (0..n_points)
            .map(|i| {
                let x = i as f32;
                (x, slope * x + intercept)
            })
            .collect()
    }
    
    /// 生成随机散点数据
    pub fn random_scatter(n_points: usize) -> Vec<(f32, f32)> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        (0..n_points)
            .map(|i| {
                let mut hasher = DefaultHasher::new();
                i.hash(&mut hasher);
                let seed = hasher.finish();
                let x = (seed % 1000) as f32 / 100.0;
                let y = ((seed / 1000) % 1000) as f32 / 100.0;
                (x, y)
            })
            .collect()
    }
}
