//! 树状图实现
//!
//! 用于可视化层次数据，显示数据项的相对大小

use crate::PlotArea;
use vizuara_core::{Color, Primitive, HorizontalAlign, VerticalAlign};
use nalgebra::Point2;

/// 颜色方案
#[derive(Debug, Clone)]
pub enum ColorScheme {
    Category,
    Blues,
    Greens,
    Reds,
}

/// 树状图项目
#[derive(Debug, Clone)]
pub struct TreemapItem {
    pub label: String,
    pub value: f32,
    pub color: Option<Color>,
}

impl TreemapItem {
    /// 创建新项目
    pub fn new(label: String, value: f32) -> Self {
        Self {
            label,
            value,
            color: None,
        }
    }

    /// 设置颜色
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

/// 树状图样式
#[derive(Debug, Clone)]
pub struct TreemapStyle {
    pub border_width: f32,
    pub padding: f32,
    pub show_labels: bool,
    pub show_values: bool,
    pub label_size: f32,
    pub label_color: Color,
    pub border_color: Color,
}

impl Default for TreemapStyle {
    fn default() -> Self {
        Self {
            border_width: 1.0,
            padding: 2.0,
            show_labels: true,
            show_values: false,
            label_size: 10.0,
            label_color: Color::rgb(0.2, 0.2, 0.2),
            border_color: Color::rgb(0.8, 0.8, 0.8),
        }
    }
}

/// 树状图
#[derive(Debug, Clone)]
pub struct Treemap {
    items: Vec<TreemapItem>,
    style: TreemapStyle,
    color_scheme: ColorScheme,
    title: Option<String>,
}

impl Treemap {
    /// 创建新的树状图
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            style: TreemapStyle::default(),
            color_scheme: ColorScheme::Category,
            title: None,
        }
    }

    /// 添加项目
    pub fn add_item(mut self, item: TreemapItem) -> Self {
        self.items.push(item);
        self
    }

    /// 设置标题
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// 设置颜色方案
    pub fn color_scheme(mut self, scheme: ColorScheme) -> Self {
        self.color_scheme = scheme;
        self
    }

    /// 设置边框宽度
    pub fn border_width(mut self, width: f32) -> Self {
        self.style.border_width = width;
        self
    }

    /// 设置内边距
    pub fn padding(mut self, padding: f32) -> Self {
        self.style.padding = padding;
        self
    }

    /// 设置是否显示标签
    pub fn show_labels(mut self, show: bool) -> Self {
        self.style.show_labels = show;
        self
    }

    /// 设置是否显示数值
    pub fn show_values(mut self, show: bool) -> Self {
        self.style.show_values = show;
        self
    }

    /// 生成颜色
    fn get_item_color(&self, index: usize) -> Color {
        match self.color_scheme {
            ColorScheme::Category => {
                // 使用不同的颜色组合
                let colors = [
                    Color::rgb(0.8, 0.4, 0.4),
                    Color::rgb(0.4, 0.8, 0.4),
                    Color::rgb(0.4, 0.4, 0.8),
                    Color::rgb(0.8, 0.8, 0.4),
                    Color::rgb(0.8, 0.4, 0.8),
                    Color::rgb(0.4, 0.8, 0.8),
                    Color::rgb(0.6, 0.6, 0.6),
                ];
                colors[index % colors.len()]
            }
            ColorScheme::Blues => {
                let intensity = 0.3 + (index as f32 / self.items.len() as f32) * 0.7;
                Color::rgb(0.2, 0.4, intensity)
            }
            ColorScheme::Greens => {
                let intensity = 0.3 + (index as f32 / self.items.len() as f32) * 0.7;
                Color::rgb(0.2, intensity, 0.3)
            }
            ColorScheme::Reds => {
                let intensity = 0.3 + (index as f32 / self.items.len() as f32) * 0.7;
                Color::rgb(intensity, 0.2, 0.2)
            }
        }
    }

    /// 简单的平铺布局算法
    fn compute_layout(&self, plot_area: PlotArea) -> Vec<(f32, f32, f32, f32, Color, String)> {
        let mut layouts = Vec::new();
        
        let total_value: f32 = self.items.iter().map(|item| item.value).sum();
        if total_value <= 0.0 {
            return layouts;
        }

        let available_width = plot_area.width - 2.0 * self.style.padding;
        let available_height = plot_area.height - 2.0 * self.style.padding - if self.title.is_some() { 40.0 } else { 0.0 };
        let total_area = available_width * available_height;

        let mut current_x = plot_area.x + self.style.padding;
        let mut current_y = plot_area.y + self.style.padding + if self.title.is_some() { 40.0 } else { 0.0 };
        let mut row_height = 0.0;
        let mut remaining_width = available_width;

        for (i, item) in self.items.iter().enumerate() {
            let area_ratio = item.value / total_value;
            let item_area = total_area * area_ratio;
            
            // 计算矩形尺寸
            let aspect_ratio = available_width / available_height;
            let width = (item_area * aspect_ratio).sqrt().min(remaining_width);
            let height = if width > 0.0 { item_area / width } else { 0.0 };

            // 如果当前行放不下，换行
            if width > remaining_width && current_x > plot_area.x + self.style.padding {
                current_x = plot_area.x + self.style.padding;
                current_y += row_height + self.style.padding;
                remaining_width = available_width;
                row_height = 0.0;
            }

            let color = item.color.unwrap_or_else(|| self.get_item_color(i));
            
            layouts.push((
                current_x,
                current_y,
                width.max(20.0), // 最小宽度
                height.max(15.0), // 最小高度
                color,
                item.label.clone(),
            ));

            current_x += width + self.style.padding;
            remaining_width -= width + self.style.padding;
            row_height = row_height.max(height);
        }

        layouts
    }
}

impl Treemap {
    /// 生成渲染图元
    pub fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        let layouts = self.compute_layout(plot_area);

        // 渲染矩形
        for (x, y, width, height, color, label) in &layouts {
            // 矩形背景
            primitives.push(Primitive::RectangleStyled {
                min: Point2::new(*x, *y),
                max: Point2::new(x + width, y + height),
                fill: *color,
                stroke: Some((self.style.border_color, self.style.border_width)),
            });

            // 标签
            if self.style.show_labels && width > &20.0 && height > &15.0 {
                primitives.push(Primitive::Text {
                    position: Point2::new(x + width / 2.0, y + height / 2.0),
                    content: label.clone(),
                    size: self.style.label_size,
                    color: self.style.label_color,
                    h_align: HorizontalAlign::Center,
                    v_align: VerticalAlign::Middle,
                });
            }
        }

        // 添加标题
        if let Some(title) = &self.title {
            primitives.push(Primitive::Text {
                position: Point2::new(plot_area.x + plot_area.width / 2.0, plot_area.y + 20.0),
                content: title.clone(),
                size: 16.0,
                color: Color::rgb(0.2, 0.2, 0.2),
                h_align: HorizontalAlign::Center,
                v_align: VerticalAlign::Middle,
            });
        }

        primitives
    }
}
